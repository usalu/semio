import {spawn,type ChildProcess} from "node:child_process";
import {CargoController} from "../🎛️control/🟦️.ts";
import type {CargoDiscoveryOperation} from "../📁️physical/🟦️.ts";

export type CargoExecutionRequest=Readonly<{arguments:readonly string[];directory:string;environment:Readonly<Record<string,string|undefined>>}>;
export type CargoExecutionResult=Readonly<{code:number;signal:string|null}>;
export interface CargoExecutionPort {readonly maximumMilliseconds:number;terminate(pid:number):Promise<void>;forward(channel:"stdout"|"stderr",bytes:Uint8Array):Promise<void>;}

/** 🏃️ Conserves the literal command, stream and terminal owners across execution and refusal. */
export class CargoExecutionOwner {
 state:"prepared"|"running"|"complete"|"refused"="prepared";failure:unknown=null;terminationFailure:unknown=null;terminalFailure:unknown=null;result:CargoExecutionResult|null=null;
 private child:ChildProcess|null=null;private readonly pending=new Set<Uint8Array>();private termination:Promise<void>|null=null;private terminalObservation:Promise<void>|null=null;
 constructor(readonly request:CargoExecutionRequest){}
 async execute(operation:CargoDiscoveryOperation,port:CargoExecutionPort):Promise<CargoExecutionResult>{
  const control=new CargoController(operation,operation.workspace);
  if(this.state!=="prepared"||!Number.isSafeInteger(port.maximumMilliseconds)||port.maximumMilliseconds<=0)throw Error("Cargo execution requires its prepared owner and finite time capability");
  await control.step("command-arguments-owner",this.request.directory,1,128);
  const args:string[]=[];operation.workspace.partialRecords.push(args);
  for(const value of this.request.arguments){if(!value||value.length>4096||value.includes("\0"))throw Error("Cargo execution has an invalid literal argument");await control.step("command-argument",this.request.directory,value.length+1,16);args.push(value);}
  if(!args.length||!this.request.directory||this.request.directory.length>4096)throw Error("Cargo execution requires a literal command and directory");
  await control.step("command-environment-owner",this.request.directory,1,128);
  const environment:Record<string,string>={};operation.workspace.partialRecords.push(environment);
  for(const key in this.request.environment){if(!Object.hasOwn(this.request.environment,key))continue;const value=this.request.environment[key];if(value===undefined)continue;if(!key||key.length>4096||value.length>65536||key.includes("\0")||value.includes("\0"))throw Error("Cargo execution environment exceeds its owned scope");await control.step("command-environment",this.request.directory,key.length+value.length+1,32);environment[key]=value;}
  await control.step("command-spawn",this.request.directory,1,256);
  this.child=spawn("cargo",args,{cwd:this.request.directory,env:environment,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32"});this.state="running";
  const child=this.child,terminal=new Promise<CargoExecutionResult>((accept,reject)=>{child.once("error",reject);child.once("close",(code,signal)=>{if(code===null&&!signal)reject(Error("Cargo terminal has no exit status"));else accept({code:code??1,signal});});});
  let stopped:string|null=null;let refuseStop!:(error:Error)=>void;
  const cancellation=new Promise<never>((_,reject)=>{refuseStop=reject;});
  const stop=(reason:string):void=>{if(stopped===null){stopped=reason;refuseStop(Error("Cargo command stopped: "+reason));}if(this.termination||!child.pid||child.exitCode!==null||child.signalCode!==null)return;const pid=child.pid;this.termination=Promise.resolve().then(()=>port.terminate(pid)).catch(error=>{this.terminationFailure=error;});};
  const abort=()=>stop("cancelled"),timer=setTimeout(()=>stop("time budget refused"),port.maximumMilliseconds);operation.signal.addEventListener("abort",abort,{once:true});if(operation.signal.aborted)abort();
  const forward=async(channel:"stdout"|"stderr",stream:NonNullable<ChildProcess["stdout"]>):Promise<void>=>{for await(const bytes of stream){if(!(bytes instanceof Uint8Array))throw Error("Cargo execution output requires an owned binary chunk");this.pending.add(bytes);await control.step("command-output",this.request.directory,bytes.byteLength+1,bytes.byteLength+32);await port.forward(channel,bytes);control.check();this.pending.delete(bytes);operation.workspace.ownedBytes-=bytes.byteLength+32;}};
  try{if(!child.stdout||!child.stderr)throw Error("Cargo execution stream owners are absent");const [result]=await Promise.race([Promise.all([terminal,forward("stdout",child.stdout),forward("stderr",child.stderr)]),cancellation]);this.result=result;if(stopped||result.signal)throw Error("Cargo command stopped: "+(stopped??result.signal));await control.finish("command-complete",this.request.directory);this.state="complete";return result;}catch(error){this.state="refused";this.failure=error;stop("refused");if(this.termination)await this.termination;if(this.terminationFailure!==null){child.stdout?.destroy();child.stderr?.destroy();this.terminalObservation=terminal.then(result=>{this.result=result;},failure=>{this.terminalFailure=failure;});}else try{this.result=await terminal;}catch(failure){this.terminalFailure=failure;}throw error;}finally{clearTimeout(timer);operation.signal.removeEventListener("abort",abort);}
 }
}

/** 🎛️ Admits the subprocess owner before the private system provider receives any command. */
export async function executeCargoCommand(request:CargoExecutionRequest,operation:CargoDiscoveryOperation,port:CargoExecutionPort):Promise<CargoExecutionResult>{await new CargoController(operation,operation.workspace).step("command-owner",request.directory,1,256);const owner=new CargoExecutionOwner(request);operation.workspace.partialRecords.push(owner);return owner.execute(operation,port);}
