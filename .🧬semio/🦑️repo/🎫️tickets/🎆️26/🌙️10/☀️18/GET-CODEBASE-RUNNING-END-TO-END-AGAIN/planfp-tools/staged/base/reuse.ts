/** ♻️ The record of a finished Bazel plan and the check that lets a later run reuse it without starting the planner: the planner child writes a witness of everything it looked at, the parent seals it with the key of everything it was given and the identity of what it wrote, and a run whose key is the same and whose witness still holds gets the very same workspace. */
import {createHash} from "node:crypto";
import {createReadStream} from "node:fs";
import {lstat,readFile,readdir,rename,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readWorldWitnessV1,verifyWorldWitnessV1,type WorldWitnessPortsV1,type WorldWitnessV1}from"../../../🔭️witness/🟨️.mjs";
import type {BazelImplementationV1} from "../🎬️executor/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const;
const cursorFile="plan.cursor.json";
/** 📛️ The names of the files the record and its preload use in a workspace. */
export const bazelPlanFilesV1=Object.freeze({receipt:policy.receipt,witness:policy.witness,preload:policy.preload});
const sortKeys=(value:unknown):unknown=>Array.isArray(value)?value.map(sortKeys):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sortKeys((value as Record<string,unknown>)[key])])):value;

/** 🎚️ The granted job count is an execution setting: only the worker plan after the planner reads it, so it leaves the key and a run admitted with fewer jobs reuses the plan. Drops the top-level `parallel` of the request and every `--parallel=<n>` / `--parallel <n>` from the capabilities. */
export function bazelPlanJobFreeV1<T>(value:T):T{
 const strip=(item:unknown):unknown=>{if(Array.isArray(item)){const out:unknown[]=[];for(let index=0;index<item.length;index++){const entry=item[index];if(typeof entry==="string"&&/^--parallel=\d+$/u.test(entry))continue;if(entry==="--parallel"&&typeof item[index+1]==="string"&&/^\d+$/u.test(item[index+1] as string)){index++;continue;}out.push(strip(entry));}return out;}return item!==null&&typeof item==="object"?Object.fromEntries(Object.entries(item).map(([name,entry])=>[name,strip(entry)])):item;};
 return (value!==null&&typeof value==="object"&&!Array.isArray(value)?strip(Object.fromEntries(Object.entries(value).filter(([name])=>name!=="parallel"))):strip(value)) as T;
}

/** 🗝️ Digests everything a plan is a function of, in a canonical form: the same parts give the same key in every process and every language. */
export function bazelPlanKeyV1(parts:Readonly<Record<string,unknown>>):string{return createHash("sha256").update(JSON.stringify(sortKeys(parts))).digest("hex");}

/** 🧾️ One thing the planner left in the root of the workspace, by its identity and its content digest. */
export interface BazelPlanOutputV1{readonly name:string;readonly identity:string;readonly sha256:string}
/** 📑️ The sealed record of one finished plan. */
export interface BazelPlanReceiptV1{readonly version:1;readonly key:string;readonly recordedAtMs:number;readonly witness:WorldWitnessV1;readonly implementation:BazelImplementationV1;readonly adapter:BazelImplementationV1;readonly scopes:Readonly<Record<string,BazelImplementationV1>>;readonly outputs:readonly BazelPlanOutputV1[]}
/** 🚦️ Why a plan could not be reused, or the proof that it can. */
export type BazelPlanReuseVerdictV1=Readonly<{reuse:true;receipt:BazelPlanReceiptV1;checked:number}>|Readonly<{reuse:false;reason:"absent"|"corrupt"|"key"|"outputs"|"world";kind?:string;subject?:string}>;

/** 🔍️ Names the entries the planner owns in the root of a workspace; everything the executor, Bazel or the record itself writes there is left out. */
export function bazelPlanOwnsV1(name:string,ignore:ReadonlySet<string>):boolean{
 return !policy.excluded.includes(name)&&!ignore.has(name)&&!policy.excludedPrefixes.some(prefix=>name.startsWith(prefix))&&!policy.excludedSuffixes.some(suffix=>name.endsWith(suffix));
}

const identityOf=(facts:import("node:fs").BigIntStats):string=>[facts.dev,facts.ino,facts.mode,facts.size,facts.mtimeNs,facts.ctimeNs].join(":");
async function digestFileV1(path:string,signal:AbortSignal):Promise<string>{
 const digest=createHash("sha256");
 for await(const chunk of createReadStream(path)){signal.throwIfAborted();digest.update(chunk);}
 return digest.digest("hex");
}

/** 🗂️ Lists what the planner left in the root of a workspace with identities and, when asked, content digests. */
export async function bazelPlanOutputsV1(workspace:string,ignore:ReadonlySet<string>,digest:boolean,signal:AbortSignal):Promise<BazelPlanOutputV1[]>{
 const rows:BazelPlanOutputV1[]=[];
 for(const name of(await readdir(workspace)).sort()){
  signal.throwIfAborted();if(!bazelPlanOwnsV1(name,ignore))continue;
  const path=join(workspace,name),facts=await lstat(path,{bigint:true});
  rows.push({name,identity:identityOf(facts),sha256:digest&&facts.isFile()?await digestFileV1(path,signal):""});
 }
 return rows;
}

/** 📖️ Reads the record of the last plan in a workspace; a missing record is nothing, an unreadable one is corrupt. */
export async function readBazelPlanReceiptV1(workspace:string):Promise<BazelPlanReceiptV1|undefined|"corrupt">{
 let text:string;
 try{text=await readFile(join(workspace,policy.receipt),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;return "corrupt";}
 try{
  if(Buffer.byteLength(text)>policy.maximumReceiptBytes)return "corrupt";
  const value=JSON.parse(text) as BazelPlanReceiptV1;
  if(validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length)return "corrupt";
  readWorldWitnessV1(JSON.stringify(value.witness));
  return Object.freeze(value);
 }catch{return "corrupt";}
}

/** 🧭️ The state of the change journal a recorded plan relies on: the cursor taken before the planner started and the symbolic-link directories above the observed paths. */
export interface BazelPlanJournalV1{readonly cursor:string;readonly linked:readonly string[]}

/** 📍️ Takes the cursor of the change journal of a checkout before a computation starts, or nothing when the journal cannot vouch (disabled, no service, untrusted watcher); the journal only ever narrows a check, so nothing here may fail a run. */
export async function takeBazelPlanCursorV1(root:string,signal?:AbortSignal):Promise<string|undefined>{
 try{
  const[{connectChangeJournalV1},{formatChangeCursorV1}]=await Promise.all([import("../../../🏡️resident/📜️journal/🟦️.ts"),import("../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts")]);
  const taken=await connectChangeJournalV1(root).cursor(signal);
  return"exact"in taken?undefined:formatChangeCursorV1(taken);
 }catch{signal?.throwIfAborted();return undefined;}
}

/** 💾️ Reads the journal state kept beside a record: the cursor of the last check that held and the symbolic-link directories. */
async function readJournalStateV1(workspace:string,key:string):Promise<BazelPlanJournalV1|undefined>{
 try{const value=JSON.parse(await readFile(join(workspace,cursorFile),"utf8")) as {key?:string;cursor?:string;linked?:string[]};return value.key===key&&typeof value.cursor==="string"&&Array.isArray(value.linked)?{cursor:value.cursor,linked:value.linked}:undefined;}catch{return undefined;}
}
async function writeJournalStateV1(workspace:string,key:string,state:BazelPlanJournalV1):Promise<void>{
 const path=join(workspace,cursorFile),temporary=path+"."+process.pid+".tmp";
 await writeFile(temporary,JSON.stringify({key,...state}),{mode:0o600});await rename(temporary,path);
}

/** 🧹️ Forgets the record and the witness of the last plan of a workspace, before anything in it changes, so that an interrupted plan never leaves a record that describes another workspace. */
export async function removeBazelPlanRecordV1(workspace:string):Promise<void>{
 await Promise.all([policy.receipt,policy.witness,cursorFile].map(name=>rm(join(workspace,name),{force:true})));
}

/** ✅️ Decides whether the last plan of a workspace can be reused: the key of everything the plan was given is the same, the workspace root is as the planner left it, and every fact the planner looked at is still true. */
export async function checkBazelPlanReuseV1(input:Readonly<{workspace:string;key:string;ignore:ReadonlySet<string>;signal:AbortSignal;ports?:WorldWitnessPortsV1;environment?:Readonly<Record<string,string|undefined>>;root?:string;progress?:(checked:number)=>Promise<void>|void}>):Promise<BazelPlanReuseVerdictV1>{
 const receipt=await readBazelPlanReceiptV1(input.workspace);
 if(receipt===undefined)return{reuse:false,reason:"absent"};
 if(receipt==="corrupt")return{reuse:false,reason:"corrupt"};
 if(receipt.key!==input.key)return{reuse:false,reason:"key"};
 for(const row of receipt.outputs){
  if(!bazelPlanOwnsV1(row.name,input.ignore))continue;
  input.signal.throwIfAborted();
  let facts:import("node:fs").BigIntStats;
  try{facts=await lstat(join(input.workspace,row.name),{bigint:true});}catch{return{reuse:false,reason:"outputs"};}
  if(identityOf(facts)!==row.identity)return{reuse:false,reason:"outputs"};
 }
 const options={signal:input.signal,...(input.ports?{ports:input.ports}:{}),...(input.environment?{environment:input.environment}:{}),...(input.progress?{progress:input.progress}:{})};
 let verdict=undefined as Awaited<ReturnType<typeof verifyWorldWitnessV1>>|undefined;
 const state=input.root===undefined?undefined:await readJournalStateV1(input.workspace,input.key);
 if(state!==undefined&&input.root!==undefined)try{
  const[{connectChangeJournalV1},{checkWorldViaJournalV1},{parseChangeCursorV1,formatChangeCursorV1}]=await Promise.all([import("../../../🏡️resident/📜️journal/🟦️.ts"),import("../../../🏡️resident/📜️journal/🔭️world/🟦️.ts"),import("../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts")]);
  const answer=await checkWorldViaJournalV1({client:connectChangeJournalV1(input.root),cursor:parseChangeCursorV1(state.cursor),workspace:input.root,rows:receipt.witness.observations,linked:state.linked,stat:rows=>verifyWorldWitnessV1({...receipt.witness,observations:rows},options),signal:input.signal});
  verdict=answer.verdict;
  if(verdict.holds&&answer.cursor!==undefined)await writeJournalStateV1(input.workspace,input.key,{cursor:formatChangeCursorV1(answer.cursor),linked:state.linked}).catch(()=>undefined);
 }catch{input.signal.throwIfAborted();verdict=undefined;}
 verdict??=await verifyWorldWitnessV1(receipt.witness,options);
 return verdict.holds?{reuse:true,receipt,checked:verdict.checked}:{reuse:false,reason:"world",kind:verdict.kind,subject:verdict.subject};
}

/** 🖋️ Seals the witness the planner child wrote together with what the plan produced; a witness that cannot prove completeness is not sealed and the reason is returned. */
export async function recordBazelPlanV1(input:Readonly<{workspace:string;key:string;implementation:BazelImplementationV1;adapter:BazelImplementationV1;scopes:Readonly<Record<string,BazelImplementationV1>>;ignore:ReadonlySet<string>;signal:AbortSignal;nowMs:number;root?:string;cursor?:string}>):Promise<Readonly<{recorded:true;observations:number}>|Readonly<{recorded:false;reason:"absent"|"racy"|"unstable"|"tainted";detail:string}>>{
 let witness:WorldWitnessV1;
 try{witness=readWorldWitnessV1(await readFile(join(input.workspace,policy.witness),"utf8"));}catch{return{recorded:false,reason:"absent",detail:""};}
 finally{await rm(join(input.workspace,policy.witness),{force:true});}
 if(witness.tainted.length)return{recorded:false,reason:"tainted",detail:witness.tainted.join("; ")};
 if(witness.unstable.length)return{recorded:false,reason:"unstable",detail:witness.unstable.slice(0,3).join("; ")};
 if(witness.racy!==null)return{recorded:false,reason:"racy",detail:witness.racy};
 const receipt:BazelPlanReceiptV1={version:1,key:input.key,recordedAtMs:input.nowMs,witness,implementation:input.implementation,adapter:input.adapter,scopes:input.scopes,outputs:await bazelPlanOutputsV1(input.workspace,input.ignore,true,input.signal)};
 const path=join(input.workspace,policy.receipt),temporary=path+"."+process.pid+".tmp";
 await writeFile(temporary,JSON.stringify(receipt),{mode:0o600});await rename(temporary,path);
 if(input.root!==undefined&&input.cursor!==undefined)try{
  const{symlinkedDirectoriesV1}=await import("../../../🏡️resident/📜️journal/🔭️world/🟦️.ts");
  await writeJournalStateV1(input.workspace,input.key,{cursor:input.cursor,linked:await symlinkedDirectoriesV1(input.root,witness.observations.filter(row=>/^(?:[lLsSdfrp]|a\d*)$/u.test(row[0])).map(row=>row[1]))});
 }catch{input.signal.throwIfAborted();}
 return{recorded:true,observations:witness.observations.length};
}

/** 🔬️ Names the outputs whose content differs between two listings, in either direction; the oracle of the `verify` mode. */
export function compareBazelPlanOutputsV1(before:readonly BazelPlanOutputV1[],after:readonly BazelPlanOutputV1[]):string[]{
 const names=new Map<string,string>(before.map(row=>[row.name,row.sha256])),seen=new Set<string>(),moved:string[]=[];
 for(const row of after){seen.add(row.name);if(names.get(row.name)!==row.sha256)moved.push(row.name);}
 for(const name of names.keys())if(!seen.has(name))moved.push(name);
 return moved.sort();
}
