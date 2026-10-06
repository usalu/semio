import { spawn } from "node:child_process";
import { lstat, mkdir, mkdtemp, open } from "node:fs/promises";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { acquireCargoBuildLeaseV1 } from "../../../../🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../🏃️process/🪓️termination/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import policySchema from "./🧬️schema/🔣️.json" with { type: "json" };

export type MutationInventoryExecutionPolicy = Readonly<{
  version: 1; maximumUnits: number; maximumOwnedBytes: number; maximumCaptureBytes: number; budgetMs: number;
  compilerStorage: Readonly<{ buildDirectory: string; leaseDirectory: string; targetDirectory: string; captureDirectory: string }>;
}>;
export type MutationInventoryProcessRequest = Readonly<{
  command: string; argv: readonly string[]; cwd: string; environment: Readonly<Record<string,string|undefined>>;
  captureDirectory: string; budgetMs: number; maximumCaptureBytes: number;
  compiler: Readonly<{ buildDirectory: string; leaseDirectory: string }> | null;
}>;
export type MutationInventoryProcessReason = "exit" | "spawn-error" | "deadline" | "cancelled" | "capture-budget" | "work-budget" | "byte-budget" | "operation-error";
export type MutationInventoryCapture = Readonly<{ path: string; bytes: number }>;
export type MutationInventoryProcessResult = Readonly<{ code: number|null; signal: string|null; reason: MutationInventoryProcessReason; stdout: MutationInventoryCapture; stderr: MutationInventoryCapture; receiptPath: string }>;
export type MutationInventoryProcessProgress = Readonly<{ stage: string; path: string; completed: number; ownedBytes: number; capturedBytes: number }>;
export type MutationInventoryProcessOperation = Readonly<{
  signal: AbortSignal; maximumUnits: number; maximumOwnedBytes: number; workspace: MutationInventoryProcessWorkspace;
  onProgress(progress: MutationInventoryProcessProgress): void|Promise<void>; yieldContinuation(): Promise<void>;
}>;
type BackingOwner = { path: string; bytes: number; closed: boolean; identity: {device: number; inode: number; modified: number}; close(): Promise<void> };
type ProcessOwner = { pid?: number; closed: boolean; terminate(): void; join(): Promise<{code: number|null; signal: string|null}> };
/** 🪢️ Retains admitted inputs and real process, lease and backing owners through every refusal. */
export class MutationInventoryProcessWorkspace {
  input: MutationInventoryProcessRequest|undefined;
  request: MutationInventoryProcessRequest|undefined;
  reads: string[] = [];
  readOwners: {path:string; buffer?:Uint8Array; text?:string; closed:boolean; close?:()=>Promise<void>}[] = [];
  admittedArguments: string[] = [];
  admittedEnvironment: Record<string,string|undefined> = Object.create(null);
  process: ProcessOwner|undefined;
  lease: { release(): void }|undefined;
  stdout: BackingOwner|undefined;
  stderr: BackingOwner|undefined;
  directory: string|undefined;
  receiptPath: string|undefined;
  result: MutationInventoryProcessResult|undefined;
  refusal: unknown;
  completed = 0;
  ownedBytes = 0;
  capturedBytes = 0;
  deadline = 0;
}
/** 🛑️ Carries the same operation workspace when controlled admission or execution refuses. */
export class MutationInventoryProcessError extends Error {
  constructor(readonly reason: MutationInventoryProcessReason, readonly workspace: MutationInventoryProcessWorkspace) { super("Mutation inventory process refused: "+reason); }
}
/** 🔐️ Reads the single closed, explicitly authored execution policy. */
export function readMutationInventoryExecutionPolicy(environment: Readonly<Record<string,string|undefined>>): MutationInventoryExecutionPolicy {
  const text=environment.SEMIO_MUTATION_INVENTORY_POLICY;
  if(!text)throw Error("Explicit mutation inventory execution policy required");
  if(text.length>100000)throw Error("Mutation inventory execution policy exceeds its closed envelope");
  const value: unknown=JSON.parse(text),errors=validateJsonSchemaSubset(policySchema,value);
  if(errors.length)throw Error("Invalid mutation inventory execution policy: "+errors.join("; "));
  return value as MutationInventoryExecutionPolicy;
}
/** 🎛️ Emits the single canonical collector argument policy. */
export function mutationInventoryPolicyArguments(policy:MutationInventoryExecutionPolicy): readonly string[] {
  if(!positive(policy.maximumUnits)||!positive(policy.maximumOwnedBytes)||!positive(policy.budgetMs)||policy.budgetMs>86400000)throw Error("Positive mutation inventory collector policy required");
  return ["--maximum-units",String(policy.maximumUnits),"--maximum-owned-bytes",String(policy.maximumOwnedBytes),"--budget-ms",String(policy.budgetMs)];
}
/** 🔗️ Requires the artifact collector to use the exact authored process policy. */
export function admitMutationInventoryProducerArguments(argv:readonly string[],policy:MutationInventoryExecutionPolicy): readonly string[] {
  const expected=mutationInventoryPolicyArguments(policy);
  if((argv.length!==10&&argv.length!==11)||argv[0]!=="list-mutations"||expected.some((value,index)=>argv[argv.length-6+index]!==value))throw Error("Mutation inventory producer arguments differ from the authored policy");
  return argv;
}
function positive(value: number): boolean { return Number.isSafeInteger(value)&&value>0; }
function reason(error: unknown): MutationInventoryProcessReason { return error instanceof MutationInventoryProcessError?error.reason:"operation-error"; }
async function checkpoint(operation: MutationInventoryProcessOperation,stage: string,path: string,units=1,owned=operation.workspace.ownedBytes): Promise<void> {
  const w=operation.workspace;
  if(operation.signal.aborted)throw new MutationInventoryProcessError("cancelled",w);
  if(w.deadline&&Date.now()>=w.deadline)throw new MutationInventoryProcessError("deadline",w);
  if(w.completed+units>operation.maximumUnits)throw new MutationInventoryProcessError("work-budget",w);
  if(owned>operation.maximumOwnedBytes)throw new MutationInventoryProcessError("byte-budget",w);
  w.completed+=units;w.ownedBytes=owned;
  await operation.onProgress({stage,path,completed:w.completed,ownedBytes:w.ownedBytes,capturedBytes:w.capturedBytes});
  await operation.yieldContinuation();
  if(operation.signal.aborted)throw new MutationInventoryProcessError("cancelled",w);
}
async function regularDirectory(path: string,operation: MutationInventoryProcessOperation): Promise<void> {
  if(!isAbsolute(path))throw Error("Mutation inventory storage must be absolute");
  for(let cursor=resolve(path);;cursor=dirname(cursor)){
    await checkpoint(operation,"storage",cursor);
    try{const stat=await lstat(cursor);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Mutation inventory storage is not a regular directory: "+cursor);}
    catch(error){if((error as {code?:string}).code!=="ENOENT")throw error;}
    if(dirname(cursor)===cursor)break;
  }
  await mkdir(path,{recursive:true});
}
async function backing(path: string): Promise<BackingOwner & {write(bytes: Uint8Array):Promise<void>}> {
  const file=await open(path,"wx+",0o600),stat=await file.stat();
  const owner={path,bytes:0,closed:false,identity:{device:stat.dev,inode:stat.ino,modified:stat.mtimeMs},async close(){if(owner.closed)return;await file.sync();const stat=await file.stat();owner.identity.modified=stat.mtimeMs;await file.close();owner.closed=true;},async write(bytes:Uint8Array){for(let offset=0;offset<bytes.length;){const written=await file.write(bytes,offset,bytes.length-offset,null);if(!written.bytesWritten)throw Error("Mutation inventory capture made no progress");offset+=written.bytesWritten;owner.bytes+=written.bytesWritten;}}};
  return owner;
}
/** 🏃️ Runs one selected executable with caller budgets, durable paged capture and optional compiler ownership. */
export async function runMutationInventoryProcess(request: MutationInventoryProcessRequest,operation: MutationInventoryProcessOperation): Promise<MutationInventoryProcessResult> {
  const w=operation.workspace;
  if(w.input||w.request||w.process||w.directory)throw Error("Mutation inventory process requires a fresh workspace");
  if(!positive(operation.maximumUnits)||!positive(operation.maximumOwnedBytes)||!positive(request.budgetMs)||request.budgetMs>86400000||!positive(request.maximumCaptureBytes))throw Error("Mutation inventory process requires positive finite limits");
  w.input=request;w.deadline=Date.now()+request.budgetMs;
  const controller=new AbortController();
  const operationStarted=operation;operation={...operation,signal:controller.signal};
  let heartbeat: ReturnType<typeof setInterval>|undefined,observeTail=Promise.resolve();
  let failure: unknown,code:number|null=null,signal:string|null=null,out: Awaited<ReturnType<typeof backing>>|undefined,err:Awaited<ReturnType<typeof backing>>|undefined,expiry:ReturnType<typeof setTimeout>|undefined;
  const abort=()=>{controller.abort();w.process?.terminate();};
  operationStarted.signal.addEventListener("abort",abort,{once:true});if(operationStarted.signal.aborted)abort();
  expiry=setTimeout(abort,request.budgetMs);
  try{
    await checkpoint(operation,"admission",request.cwd);
    if(!request.command||!isAbsolute(request.cwd)||!isAbsolute(request.captureDirectory))throw Error("Mutation inventory command and absolute owner/storage required");
    let storage=4096+request.captureDirectory.length*24;
    await checkpoint(operation,"retention",request.cwd,1,storage);
    const admit=async(value: string|undefined)=>{
      if(value===undefined)return;
      for(let index=0;index<value.length;index+=1024){await checkpoint(operation,"inputs",request.cwd,Math.min(1024,value.length-index),storage);for(let unit=index;unit<Math.min(index+1024,value.length);unit++)if(value.charCodeAt(unit)===0)throw Error("Mutation inventory process input contains NUL");}
      storage+=value.length*2+32;
      await checkpoint(operation,"retention",request.cwd,1,storage);
    };
    for(const value of [request.command,request.cwd,request.captureDirectory])await admit(value);
    if(request.compiler){for(const value of [request.compiler.buildDirectory,request.compiler.leaseDirectory]){await admit(value);if(value.length>4096||!isAbsolute(value))throw Error("Mutation inventory compiler storage must be an absolute bounded path");}}
    for(const value of request.argv){await admit(value);w.admittedArguments.push(value);}
    for(const key in request.environment)if(Object.hasOwn(request.environment,key)){const value=request.environment[key];await admit(key);await admit(value);w.admittedEnvironment[key]=value;}
    w.request=Object.freeze({...request,argv:Object.freeze(w.admittedArguments),environment:Object.freeze(w.admittedEnvironment),compiler:request.compiler?Object.freeze({...request.compiler}):null});
    await regularDirectory(request.captureDirectory,operation);
    w.directory=await mkdtemp(join(request.captureDirectory,"mutation-inventory-"));
    w.receiptPath=join(w.directory,"receipt.json");
    out=await backing(join(w.directory,"stdout"));w.stdout=out;
    err=await backing(join(w.directory,"stderr"));w.stderr=err;
    heartbeat=setInterval(()=>{observeTail=observeTail.then(()=>checkpoint(operation,"waiting",request.command)).catch(error=>{failure??=error;abort();});},1000);
    if(request.compiler){
      await regularDirectory(request.compiler.leaseDirectory,operation);
      await checkpoint(operation,"lease",request.compiler.buildDirectory);
      w.lease=await acquireCargoBuildLeaseV1({directory:request.compiler.leaseDirectory,buildDirectory:request.compiler.buildDirectory,args:request.argv,signal:operation.signal,onWait:p=>{if(controller.signal.aborted)throw new MutationInventoryProcessError(Date.now()>=w.deadline?"deadline":"cancelled",w);}});
    }
    await checkpoint(operation,"spawn",request.command);
    const child=spawn(request.command,[...w.request.argv],{cwd:w.request.cwd,env:w.request.environment,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true});
    let spawnFailure: unknown;
    const joined=new Promise<{code:number|null;signal:string|null}>(accept=>{child.once("error",e=>{spawnFailure=e;});child.once("close",(code,signal)=>accept({code,signal}));});
    w.process={pid:child.pid,closed:false,terminate:()=>terminateOwnedChildTree(child),join:async()=>{const result=await joined;w.process!.closed=true;return result;}};
    if(operation.signal.aborted||Date.now()>=w.deadline)abort();
    let captureTail=Promise.resolve();
    const drain=async(stream:AsyncIterable<Uint8Array>,owner:NonNullable<typeof out>)=>{
      try{for await(const chunk of stream){
        const task=captureTail.then(async()=>{
        const available=request.maximumCaptureBytes-w.capturedBytes,accepted=Math.max(0,Math.min(available,chunk.length));
        for(let offset=0;offset<accepted;offset+=4096){
          const length=Math.min(4096,accepted-offset);
          await checkpoint(operation,"capture",owner.path,length);
          await owner.write(chunk.subarray(offset,offset+length));w.capturedBytes+=length;
        }
        if(accepted!==chunk.length)throw new MutationInventoryProcessError("capture-budget",w);
        });captureTail=task.catch(()=>{});await task;
      }}catch(error){failure??=error;abort();throw error;}
    };
    const drained=await Promise.allSettled([drain(child.stdout!,out),drain(child.stderr!,err)]);
    ({code,signal}=await w.process.join());
    for(const result of drained)if(result.status==="rejected")failure??=result.reason;
    if(spawnFailure)failure??=new MutationInventoryProcessError("spawn-error",w);
    if(Date.now()>=w.deadline)failure=new MutationInventoryProcessError("deadline",w);
    else if(operationStarted.signal.aborted)failure=new MutationInventoryProcessError("cancelled",w);
  }catch(error){failure??=error;if(Date.now()>=w.deadline)failure=new MutationInventoryProcessError("deadline",w);else if(operationStarted.signal.aborted)failure=new MutationInventoryProcessError("cancelled",w);abort();if(w.process)({code,signal}=await w.process.join());}
  finally{
    if(heartbeat)clearInterval(heartbeat);await observeTail;
    if(expiry)clearTimeout(expiry);operationStarted.signal.removeEventListener("abort",abort);
    for(const owner of [out,err])if(owner)try{await owner.close();}catch(error){failure??=error;}
    if(w.lease)try{w.lease.release();w.lease=undefined;}catch(error){failure??=error;}
  }
  if(failure)w.refusal=failure;
  if(!w.stdout||!w.stderr||!w.receiptPath)throw failure??Error("Mutation inventory capture was not admitted");
  w.result=Object.freeze({code,signal,reason:failure?reason(failure):"exit",stdout:{path:w.stdout.path,bytes:w.stdout.bytes},stderr:{path:w.stderr.path,bytes:w.stderr.bytes},receiptPath:w.receiptPath});
  const receipt=await open(w.receiptPath,"wx",0o600);
  try{await receipt.writeFile(JSON.stringify({schema:"semio.mutation-inventory.process/v1",...w.result,completed:w.completed,ownedBytes:w.ownedBytes,capturedBytes:w.capturedBytes,processClosed:w.process?.closed??true,leaseReleased:w.lease===undefined,stdoutClosed:w.stdout.closed,stderrClosed:w.stderr.closed})+"\n");await receipt.sync();}finally{await receipt.close();}
  return w.result;
}
/** 📖️ Reads only a capture admitted by this workspace, sharing its remaining units and owned payload budget. */
export async function readMutationInventoryCapture(capture:MutationInventoryCapture,operation:MutationInventoryProcessOperation):Promise<string>{
  const w=operation.workspace;
  if(!w.result||!([w.result.stdout,w.result.stderr].includes(capture)))throw Error("Capture does not belong to this mutation inventory workspace");
  const reservation=capture.bytes*3+32;
  await checkpoint(operation,"read-admission",capture.path,1,w.ownedBytes+reservation);
  const ownerRecord:{path:string;buffer?:Uint8Array;text?:string;closed:boolean;close?:()=>Promise<void>}={path:capture.path,closed:false};w.readOwners.push(ownerRecord);
  const file=await open(capture.path,"r");ownerRecord.close=async()=>{if(ownerRecord.closed)return;await file.close();ownerRecord.closed=true;};
  try{
    const buffer=new Uint8Array(capture.bytes);ownerRecord.buffer=buffer;
    const stat=await file.stat(),owner=capture===w.result.stdout?w.stdout!:w.stderr!;if(!stat.isFile()||stat.nlink!==1||stat.size!==capture.bytes||stat.dev!==owner.identity.device||stat.ino!==owner.identity.inode||stat.mtimeMs!==owner.identity.modified)throw Error("Mutation inventory capture backing advanced");
    for(let offset=0;offset<buffer.length;){await checkpoint(operation,"read",capture.path,Math.min(4096,buffer.length-offset));const {bytesRead}=await file.read(buffer,offset,Math.min(4096,buffer.length-offset),offset);if(!bytesRead)throw Error("Mutation inventory capture ended early");offset+=bytesRead;}
    await checkpoint(operation,"decode",capture.path,buffer.length);
    const after=await file.stat();if(after.mtimeMs!==owner.identity.modified||after.size!==capture.bytes)throw Error("Mutation inventory capture backing advanced while reading");
    const text=new TextDecoder("utf-8",{fatal:true}).decode(buffer);ownerRecord.text=text;w.reads.push(text);return text;
  }finally{await ownerRecord.close();}
}
