import {AsyncLocalStorage} from "node:async_hooks";
import {realpathSync} from "node:fs";
import {resolve,sep} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import callerSchema from "../../🧬️schema/📥️caller.json" with {type:"json"};
import catalog from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readIssuedScriptProcessEnvelopeV1,checkScriptInvocation,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {authorizeCapabilityV1,canonicalJsonV1,appendAuditV1,canonicalRootV1,capabilityIdV1,endpointsV1,keyDigestV1,readServiceKeyV1,verifyCapabilityV1,worktreeIdV1,type RevocationRegistryV1} from "./🔑️session/🟨️.mjs";
import type {NativeBuildCallerCapabilitiesV1} from "../🟦️.ts";

/** 📜 The authored admission constants and their English and German refusal texts. */
export const serviceInvocationPolicyV1=schema.definitions.Policy.const;
/** 📐 The closed grant schema that `capabilities.service` of the caller schema must repeat. */
export const serviceGrantSchemaV1=schema.definitions.Grant;
/** 🧬 The whole service invocation schema. */
export const serviceInvocationSchemaV1=schema;

/** 🎟 The grant an issued invocation carries so that its holder may admit plain task commands for authenticated client sessions. */
export interface ServiceGrantV1{readonly version:1;readonly worktree:string;readonly keyDigest:string;readonly operations:readonly("run"|"run-many"|"affected")[];readonly maximumRequests:number;readonly maximumConcurrent:number}
/** 🔤 The refusal codes of the admission. */
export type ServiceRefusalCodeV1=keyof typeof serviceInvocationPolicyV1.wire;
/** 📨 One client command as the authority core re-validates it. */
export interface ServiceRequestV1{readonly token:string;readonly agent:string;readonly cwd:string;readonly args:readonly string[];readonly named?:readonly string[]}
/** 🧾 One audit row of the lease and process journal. */
export interface ServiceAuditRowV1{readonly v:1;readonly at:number;readonly event:"admit"|"refuse"|"finish"|"revoke";readonly session:string;readonly agent:string;readonly worktree:string;readonly operation:string;readonly args:readonly string[];readonly code?:ServiceRefusalCodeV1;readonly launches?:number;readonly exit?:number}
/** 📒 Receives audit rows; an admission is refused when its row cannot be recorded. */
export interface ServiceAuditV1{record(row:ServiceAuditRowV1):void|Promise<void>}
/** 🎫 The live admission of one command: what it may declare and whether it still stands. */
export interface ServiceTicketV1{readonly session:string;readonly agent:string;readonly operation:string;readonly args:readonly string[];readonly named:readonly string[];readonly launches:readonly(readonly string[])[];check():void}

/** 🚨 A refused service request; `code` is one of the authored refusal codes. */
export class ServiceRefusalV1 extends Error{
 readonly code:ServiceRefusalCodeV1;readonly fields:Readonly<Record<string,string>>;
 constructor(code:ServiceRefusalCodeV1,fields:Readonly<Record<string,string>>={}){super("Service request refused ("+code+")"+(fields.reason?": "+fields.reason:""));this.name="ServiceRefusalV1";this.code=code;this.fields=fields;}
}

const policy=serviceInvocationPolicyV1;
const refuse=(code:ServiceRefusalCodeV1,fields:Readonly<Record<string,string>>={})=>new ServiceRefusalV1(code,fields);
const storage=new AsyncLocalStorage<{readonly grant:string;readonly root:string;readonly ticket:ServiceTicketV1}>();

/** 🗝 Creates the grant for `repo service executor` at the door: the worktree identity and the digest of the private key the service already wrote. */
export function createServiceGrantV1(root:string,environment:NodeJS.ProcessEnv):ServiceGrantV1{
 const canonical=canonicalRootV1(root),runtime=endpointsV1(canonical,environment),grant={version:1 as const,worktree:worktreeIdV1(canonical),keyDigest:keyDigestV1(readServiceKeyV1(runtime.key)),operations:[...policy.operations] as ServiceGrantV1["operations"][number][],...policy.defaults};
 return readServiceGrantV1(grant);
}

/** 🔏 Admits a grant as closed data. */
export function readServiceGrantV1(value:unknown):ServiceGrantV1{
 if(validateJsonSchemaSubset({...serviceGrantSchemaV1,definitions:schema.definitions} as object,value).length)throw Error("Closed service grant required");
 return value as ServiceGrantV1;
}

function readArguments(args:readonly string[],grant:ServiceGrantV1,tool:string):{operation:string;named:string[];tasks:string[]}{
 const bad=(reason:string)=>refuse("arguments",{reason});
 if(args.length<1||args.length>policy.maximumArguments)throw bad("1 to "+policy.maximumArguments+" arguments");
 const operation=args[0]!,entry=catalog.operations.find(candidate=>candidate.name===operation);
 if(!entry)throw bad("unknown operation "+operation.slice(0,64));
 if(!grant.operations.includes(operation as ServiceGrantV1["operations"][number]))throw refuse("scope",{reason:operation});
 const allowed=new Set((entry as {flags?:string[]}).flags??[]),positionals:string[]=[];let projects:string[]=[];
 for(let index=1;index<args.length;index++){
  const argument=args[index]!;
  if(!argument.startsWith("-")){positionals.push(argument);continue;}
  if(!argument.startsWith("--"))throw bad("short option "+argument.slice(0,16));
  const equals=argument.indexOf("="),name=equals<0?argument:argument.slice(0,equals),inline=equals<0?undefined:argument.slice(equals+1);
  if(name!=="--tool"&&(policy.forbiddenFlags.includes(name)||!allowed.has(name)))throw bad("flag "+name);
  const specification=(catalog.flags as {name:string;value:string;operations?:string[]}[]).find(flag=>flag.name===name&&(flag.operations===undefined||flag.operations.includes(operation)));
  if(!specification)throw bad("flag "+name);
  let value:string|undefined;
  if(specification.value==="none"){if(inline!==undefined)throw bad(name+" takes no value");}
  else if(specification.value==="required"){value=inline??args[++index];if(value===undefined||value.startsWith("--"))throw bad(name+" needs a value");}
  else if(specification.value==="optional")value=inline;
  else throw bad("flag "+name);
  if(name==="--tool"&&value!==tool)throw refuse("tool");
  if((policy.projectFlags as Record<string,string>)[operation]===name&&value!==undefined)projects=value.split(",").filter(Boolean);
 }
 const[minimum,maximum]=(policy.positionals as Record<string,number[]>)[operation]!;
 if(positionals.length<minimum!||positionals.length>maximum!)throw bad(operation+" takes "+minimum+" to "+maximum+" task names");
 const pattern=new RegExp(policy.taskPattern,"u");
 for(const task of positionals)if(!pattern.test(task))throw bad("task name "+task.slice(0,64));
 return{operation,tasks:positionals,named:operation==="run"?[positionals[0]!.split(":")[0]!]:projects};
}

function insideWorktree(root:string,cwd:string):boolean{
 let real:string;try{real=realpathSync(cwd);}catch{return false;}
 return real===root||real.startsWith(root.endsWith(sep)?root:root+sep);
}

/** 🧰 Everything the admission needs that is not the request: the held grant and caller facts, the key, the revocation registry, the audit sink and the one-shot door computation. */
export interface ServiceAdmissionOptionsV1{
 readonly root:string;readonly grant:ServiceGrantV1;readonly tool:"nx"|"bazel";readonly key:Uint8Array;
 readonly revocations:Pick<RevocationRegistryV1,"has"|"hasAgent">;readonly audit:ServiceAuditV1;
 readonly declare:(args:readonly string[])=>Promise<readonly(readonly string[])[]>;
 readonly closure?:(named:readonly string[])=>Promise<ReadonlySet<string>>;
 readonly now?:()=>number;
}

/** 🏛 Creates the admission of one service invocation. `admit` re-validates a client command and returns its ticket; `run` admits, declares the ticket's launches for the duration of `body`, and audits the end. */
export function createServiceAdmissionV1(options:ServiceAdmissionOptionsV1){
 const{grant,tool,key,revocations,audit,declare,closure,now=()=>Date.now()}=options,root=canonicalRootV1(options.root),worktree=worktreeIdV1(root),encoded=canonicalJsonV1(grant),sessions=new Map<string,{admitted:number;active:number}>();
 const row=(event:ServiceAuditRowV1["event"],request:Partial<ServiceRequestV1>,session:string,extra:Partial<ServiceAuditRowV1>={}):ServiceAuditRowV1=>({v:1,at:now(),event,session,agent:String(request.agent??"-").slice(0,64),worktree,operation:String(request.args?.[0]??"-").slice(0,32),args:(request.args??[]).slice(0,64).map(argument=>String(argument).slice(0,512)),...extra});
 async function admit(request:ServiceRequestV1):Promise<ServiceTicketV1>{
  let session="-";
  try{
   if(typeof request?.token!=="string"||request.token==="")throw refuse("token");
   if(validateJsonSchemaSubset({...schema.definitions.Request,definitions:schema.definitions} as object,request).length)throw refuse("arguments",{reason:"request is not a closed service request"});
   if(grant.worktree!==worktree||keyDigestV1(key)!==grant.keyDigest)throw refuse("grant");
   const caveats=verifyCapabilityV1(key,request.token);if(!caveats)throw refuse("token");
   session=capabilityIdV1(request.token)!;
   for(const required of policy.requiredCaveats)if(!caveats.some(caveat=>caveat.k===required))throw refuse("caveats",{caveat:required});
   const at=now(),notAfter=Math.min(...caveats.filter(caveat=>caveat.k==="notAfter").map(caveat=>caveat.v as number));
   if(at>notAfter)throw refuse("expired");
   if(notAfter-at>policy.maximumLifetimeMilliseconds)throw refuse("unbounded",{hours:String(policy.maximumLifetimeMilliseconds/3600000)});
   if(!insideWorktree(root,request.cwd))throw refuse("worktree");
   const cwd=realpathSync(request.cwd),{operation,named:own,tasks}=readArguments(request.args,grant,tool),named=request.named??own;
   const denial=authorizeCapabilityV1(caveats,{commandClass:"run",cwd,now:at,projects:named,worktree,agent:request.agent});
   if(denial){
    const reason=denial.reason;
    throw refuse(reason==="expired"?"expired":reason==="worktree"?"worktree":reason==="agent"?"agent":denial.code==="capability"?"token":"scope",reason&&reason!=="expired"?{reason}:{});
   }
   if(named.length===0&&caveats.some(caveat=>caveat.k==="projects"))throw refuse("scope",{reason:"projects"});
   if(revocations.has(session)||revocations.hasAgent(request.agent))throw refuse("revoked");
   const target=operation==="run"?tasks[0]!.split(":")[0]!:undefined;
   if(target!==undefined&&!named.includes(target)){
    const reachable=closure?await closure(named):new Set<string>();
    if(!reachable.has(target))throw refuse("closure",{task:tasks[0]!});
   }
   const state=sessions.get(session)??{admitted:0,active:0};sessions.set(session,state);
   if(state.admitted>=grant.maximumRequests||state.active>=grant.maximumConcurrent)throw refuse("busy");
   let launches:readonly(readonly string[])[];
   try{launches=await declare(request.args);}catch(error){if(error instanceof ServiceRefusalV1)throw error;throw refuse("arguments",{reason:String((error as Error)?.message??error).slice(0,160)});}
   try{await audit.record(row("admit",request,session,{launches:launches.length}));}catch{throw refuse("audit");}
   state.admitted++;state.active++;
   const ticket:ServiceTicketV1={session,agent:request.agent,operation,args:[...request.args],named,launches,check:()=>{if(revocations.has(session)||revocations.hasAgent(request.agent))throw refuse("revoked");if(now()>notAfter)throw refuse("expired");}};
   return ticket;
  }catch(error){
   if(error instanceof ServiceRefusalV1){try{await audit.record(row("refuse",request,session,{code:error.code}));}catch{}}
   throw error;
  }
 }
 return{
  admit,
  async run<T>(request:ServiceRequestV1,body:(ticket:ServiceTicketV1)=>Promise<T>):Promise<T>{
   const ticket=await admit(request),state=sessions.get(ticket.session)!;let exit=0;
   try{return await storage.run({grant:encoded,root,ticket},()=>body(ticket));}
   catch(error){exit=1;throw error;}
   finally{state.active--;try{await audit.record(row("finish",request,ticket.session,{exit,launches:ticket.launches.length}));}catch{}}
  },
 };
}

/** 🧭 Returns the capabilities as the same client would have been issued them one-shot: the service grant is removed and, while an admitted command runs beneath a grant equal to the held one, its ticket's launches are declared. Every launch check of the authority core reads the declaration through this one function. */
export function declaredCapabilitiesV1<T extends object>(capabilities:T):Omit<T,"service">{
 const{service,...rest}=capabilities as T&{service?:unknown;launches?:readonly(readonly string[])[];workspaceRoot?:string};
 if(service===undefined)return capabilities;
 const store=storage.getStore();
 if(store===undefined||store.grant!==canonicalJsonV1(service)||canonicalRootV1(rest.workspaceRoot??"")!==store.root)return rest as Omit<T,"service">;
 store.ticket.check();
 const launches:(readonly string[])[]=[...(rest.launches??[])];
 for(const launch of store.ticket.launches)if(!launches.some(owned=>JSON.stringify(owned)===JSON.stringify(launch)))launches.push(launch);
 return{...rest,launches} as unknown as Omit<T,"service">;
}

/** 🪪 The ticket that is live in the current asynchronous scope, if any. */
export function activeServiceTicketV1():ServiceTicketV1|undefined{return storage.getStore()?.ticket;}

/** 🗒 Audit sink for the resident service: appends one JSON line per row to the private audit file and publishes the event through the held invocation's process journal. */
export function createServiceAuditV1(invocation:ScriptInvocation,file:string):ServiceAuditV1{
 return{record(value){
  if(validateJsonSchemaSubset({...schema.definitions.Audit,definitions:schema.definitions} as object,value).length)throw Error("Closed service audit row required");
  appendAuditV1(file,value);
  void invocation.control.publish({owner:invocation.policy.owner,command:"service:"+value.event+":"+value.session,stage:value.event==="finish"?"complete":"running"});
 }};
}

/** 🛰 Holds the issued invocation: reads its grant from the private envelope, binds the door computation to the caller facts of that envelope, and returns the admission. */
export function holdServiceInvocationV1(root:string,invocation:ScriptInvocation,options:{readonly key:Uint8Array;readonly revocations:Pick<RevocationRegistryV1,"has"|"hasAgent">;readonly audit:ServiceAuditV1;readonly closure?:ServiceAdmissionOptionsV1["closure"];readonly now?:()=>number;readonly environment?:()=>NodeJS.ProcessEnv}){
 checkScriptInvocation(invocation);
 const held=readIssuedScriptProcessEnvelopeV1(invocation).capabilities as NativeBuildCallerCapabilitiesV1&{service?:unknown};
 if(held.service===undefined)throw refuse("grant");
 if(validateJsonSchemaSubset(callerSchema,held).length||resolve(held.workspaceRoot)!==resolve(root))throw refuse("grant");
 const grant=readServiceGrantV1(held.service),caller=declaredCapabilitiesV1(held),environment=options.environment??(()=>process.env);
 const declare=async(args:readonly string[])=>{
  const{createBuildBootstrapCapabilitiesV1}=await import("../📤️child/🟦️.ts");
  const door=await createBuildBootstrapCapabilitiesV1(root,["repo",...args],{...environment(),SEMIO_SCRIPT_CAPABILITIES:undefined,SEMIO_BUILD_TOOL:caller.tool,SEMIO_BUILD_MODE:caller.buildMode,CARGO_NET_OFFLINE:String(caller.native.network.offline),CARGO_TARGET_DIR:caller.native.artifactDirectory}) as NativeBuildCallerCapabilitiesV1&{service?:unknown};
  if(door.service!==undefined||door.tool!==caller.tool||door.buildMode!==caller.buildMode||JSON.stringify(door.native)!==JSON.stringify(caller.native))throw refuse("tool");
  return door.launches;
 };
 const admission=createServiceAdmissionV1({root,grant,tool:caller.tool,key:options.key,revocations:options.revocations,audit:options.audit,declare,closure:options.closure,now:options.now});
 return{...admission,warm:()=>declare(["run","warmup:build"]).then(()=>undefined,()=>undefined)};
}
