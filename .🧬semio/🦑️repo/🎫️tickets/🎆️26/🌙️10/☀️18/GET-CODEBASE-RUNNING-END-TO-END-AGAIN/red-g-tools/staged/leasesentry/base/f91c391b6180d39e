import {mkdirSync,readdirSync} from "node:fs";
import {readHostCapacityStatusV1,hostCapacityDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {readLeaseReportV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/📋️status/🟦️.ts";
import {AgentSessionErrorV1,agentSessionAddressV1,agentSessionEnvironmentV1,agentSessionMessageV1,agentSessionPathsV1,agentSessionRegistryRootV1,agentSessionStoreV1,agentSessionWorktreeKeyV1,closeAgentSessionV1,collectAgentSessionsV1,currentAgentSessionV1,readAgentSessionTokenV1,isAgentSessionIdV1,listAgentSessionsV1,listSessionSandboxesV1,renderAgentSessionListV1,renderAgentSessionStatusV1,testSandboxParentV1,AGENT_SESSION_POLICY_V1,type AgentSessionRowV1,type AgentSessionV1,type AncestryPortV1,type CurrentAgentSessionOptionsV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../../🌡️host-capacity/🟦️.ts";
import {REPOSITORY_LEASE_DIRECTORY_V1} from "../../🔒️leases/⌨️command/🟦️.ts";
import {agentSessionReclaimV1} from "../🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

/** 📊️ What one session holds right now across the tools. */
export type AgentSessionUsageV1=Readonly<{leases:number;grants:number;sandboxes:number}>;
/** 🔌️ What the session command touches: output, environment, registry, clock, process facts and the usage probe; tests replace them. */
export type SessionCommandContextV1=Readonly<{environment:NodeJS.ProcessEnv;write:(line:string)=>void;registry?:string;parent?:string;now?:()=>number;alive?:(pid:number)=>boolean;ancestry?:AncestryPortV1;usage?:(session:AgentSessionV1)=>Promise<AgentSessionUsageV1|undefined>}>;

const valued=new Set(["agent","client","client-pid","owner-pid","weight"]),flagged=new Set(["export","json","dead","force","all"]);

function parse(args:readonly string[]):Readonly<{values:Record<string,string>;switches:ReadonlySet<string>;positional:readonly string[]}>{
 const values:Record<string,string>={},switches=new Set<string>(),positional:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(!argument.startsWith("--")){positional.push(argument);continue;}
  const equals=argument.indexOf("="),name=argument.slice(2,equals<0?undefined:equals);
  if(valued.has(name)){const value=equals>=0?argument.slice(equals+1):args[++index];if(value===undefined||value==="")throw new AgentSessionErrorV1("unknownOption",{value:argument});values[name]=value;}
  else if(flagged.has(name)&&equals<0)switches.add(name);
  else throw new AgentSessionErrorV1("unknownOption",{value:argument});
 }
 return {values,switches,positional};
}
const whole=(text:string|undefined,code:string):number|undefined=>{
 if(text===undefined)return undefined;const value=Number(text);
 if(!Number.isSafeInteger(value)||value<1)throw new AgentSessionErrorV1(code,{value:text});return value;
};
const weightOf=(text:string|undefined):number|undefined=>{
 if(text===undefined)return undefined;const rules=AGENT_SESSION_POLICY_V1.resources.weight,value=Number(text);
 if(!Number.isSafeInteger(value)||value<rules.minimum||value>rules.maximum)throw new AgentSessionErrorV1("invalidWeight",{value:text,minimum:rules.minimum,maximum:rules.maximum});return value;
};

/** 🤖️ Counts what a session holds: leases of the checkout's lease store, capacity grants and live test sandboxes. A probe that fails leaves its count out of the answer. */
export async function readAgentSessionUsageV1(root:string,session:AgentSessionV1,environment:NodeJS.ProcessEnv,parent?:string):Promise<AgentSessionUsageV1|undefined>{
 try{
  const address=agentSessionAddressV1(session),report=readLeaseReportV1(`${root}/${REPOSITORY_LEASE_DIRECTORY_V1}`,environment,Date.now(),0),control=createCapacityControlV1();
  try{
   const snapshot=await readHostCapacityStatusV1({directory:hostCapacityDirectoryV1(environment),policy:BUILD_HOST_CAPACITY_POLICY_V1,control:control.control});
   const sandboxes=listSessionSandboxesV1(agentSessionPathsV1(session,testSandboxParentV1(parent)).sandboxes).length;
   return {leases:report.holders.filter(row=>row.session===address).length,grants:snapshot.grants.filter(row=>(row as {session?:string}).session===address).length,sandboxes};
  }finally{control.dispose();}
 }catch{return undefined;}
}

const worktreeStores=(registry:string)=>{try{return readdirSync(registry,{withFileTypes:true}).filter(entry=>entry.isDirectory()&&/^[0-9a-f]{12}$/u.test(entry.name)).map(entry=>agentSessionStoreV1(registry,entry.name));}catch{return [];}};

/** 🚪️ Dispatches `repo session open|status|close|list`. */
export async function runSessionCommandV1(root:string,args:readonly string[],context:SessionCommandContextV1):Promise<number>{
 const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],context.environment).language,say=(id:string,values:Readonly<Record<string,string|number>>={})=>agentSessionMessageV1(id,values,language);
 try{return await dispatch(root,args,context,language,say);}
 catch(error){if(error instanceof AgentSessionErrorV1)throw new Error(say(error.code,error.values));throw error;}
}

async function dispatch(root:string,args:readonly string[],context:SessionCommandContextV1,language:"en"|"de",say:(id:string,values?:Readonly<Record<string,string|number>>)=>string):Promise<number>{
 const [command,...rest]=args,{values,switches,positional}=parse(rest),json=switches.has("json"),now=(context.now??Date.now)(),registry=context.registry??agentSessionRegistryRootV1(context.environment);
 const ports={now,...context.alive===undefined?{}:{alive:context.alive}},store=agentSessionStoreV1(registry,agentSessionWorktreeKeyV1(root));
 const parent=testSandboxParentV1(context.parent),paths=(session:AgentSessionV1)=>agentSessionPathsV1(session,parent);
 const out=(value:unknown)=>context.write(JSON.stringify(value,null,2)),reclaim=agentSessionReclaimV1({...context.parent===undefined?{}:{parent:context.parent},...context.alive===undefined?{}:{alive:context.alive}});
 const named:CurrentAgentSessionOptionsV1={environment:context.environment,registry,worktree:root,now,...context.alive===undefined?{}:{alive:context.alive},...context.ancestry===undefined?{}:{ancestry:context.ancestry}};
 if(command==="open"){
  const ownerPid=whole(values["owner-pid"],"invalidPid"),clientPid=whole(values["client-pid"],"invalidPid"),weight=weightOf(values.weight);
  const session=currentAgentSessionV1({...named,
   ...values.agent===undefined?{}:{agent:values.agent},...values.client===undefined?{}:{client:values.client},
   ...values.agent===undefined&&values.client!==undefined?{connection:{client:values.client,clientProcess:clientPid??0,process:ownerPid??0}}:{},
   ...ownerPid===undefined?{}:{ownerPid},...weight===undefined?{}:{weight}});
  const created=session.openedAtMs>=now,address=agentSessionAddressV1(session);
  mkdirSync(paths(session).temp,{recursive:true,mode:0o700});
  if(json){out({version:1,created,address,session,environment:agentSessionEnvironmentV1(session),paths:paths(session)});return 0;}
  context.write(say(created?"opened":"attached",{id:session.id,agent:session.agent.id,client:session.agent.client,origin:say("origin."+session.agent.origin),lane:session.resources.cargo.lane,tests:paths(session).sandboxes}));
  if(switches.has("export"))for(const [name,value] of Object.entries(agentSessionEnvironmentV1(session)))context.write(`${name}=${value}`);
  return 0;
 }
 if(command==="status"){
  if(positional.length>1)throw new AgentSessionErrorV1("unknownOption",{value:positional[1]!});
  const id=positional[0];if(id!==undefined&&!isAgentSessionIdV1(id))throw new AgentSessionErrorV1("invalidSession",{value:id});
  const session=id===undefined?currentAgentSessionV1(named):listAgentSessionsV1(store,ports).find(row=>row.session.id===id)?.session;
  if(session===undefined)throw new AgentSessionErrorV1("unknown",{id:id!});
  const rows=listAgentSessionsV1(store,ports),row:AgentSessionRowV1=rows.find(candidate=>candidate.session.id===session.id)??{session,state:"live",lastSeenMs:now,shareBasisPoints:0},live=rows.filter(candidate=>candidate.state==="live").length;
  const usage=await(context.usage?context.usage(session):readAgentSessionUsageV1(root,session,context.environment,context.parent));
  const token=readAgentSessionTokenV1(agentSessionStoreV1(registry,session.worktree),session.id)!==undefined;
  if(json){out({version:1,...row,live,address:agentSessionAddressV1(session),paths:paths(session),token,...usage===undefined?{}:{usage}});return 0;}
  for(const line of renderAgentSessionStatusV1({row,live,nowMs:now,paths:paths(session),token,...usage===undefined?{}:{usage}},language))context.write(line);
  return 0;
 }
 if(command==="list"){
  const stores=switches.has("all")?worktreeStores(registry):[store];
  const rows=stores.flatMap(candidate=>listAgentSessionsV1(candidate,ports));
  if(json){out({version:1,sessions:rows});return 0;}
  for(const line of renderAgentSessionListV1(rows,now,language))context.write(line);
  return 0;
 }
 if(command==="close"){
  if(switches.has("dead")){
   const done=collectAgentSessionsV1(store,reclaim,{...ports,reason:"dead"});
   if(json){out({version:1,collected:done.map(row=>({id:row.session.id,agent:row.session.agent.id,released:row.released,kept:row.kept,removed:row.removed}))});return 0;}
   context.write(say("collected",{count:done.filter(row=>row.removed).length}));return 0;
  }
  const id=positional[0];if(id===undefined)throw new AgentSessionErrorV1("usage");if(positional.length>1)throw new AgentSessionErrorV1("unknownOption",{value:positional[1]!});
  if(!isAgentSessionIdV1(id))throw new AgentSessionErrorV1("invalidSession",{value:id});
  const row=listAgentSessionsV1(store,ports).find(candidate=>candidate.session.id===id);if(row===undefined)throw new AgentSessionErrorV1("unknown",{id});
  const anchor=row.session.anchors.at(-1),alive=context.alive??((pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}});
  if(!switches.has("force")&&row.state!=="gone"&&row.session.agent.origin!=="default"&&row.session.anchors.some(candidate=>alive(candidate.pid)))throw new AgentSessionErrorV1("closeLive",{id,state:say("state."+row.state),pid:anchor!.pid});
  const result=reclaim(row.session),closed=result.kept===0||switches.has("force");
  if(closed)closeAgentSessionV1(store,id,"closed",now);
  if(json){out({version:1,id,closed,released:result.released,kept:result.kept});return 0;}
  context.write(say("closed",{id,released:result.released,kept:result.kept}));return 0;
 }
 throw new AgentSessionErrorV1("usage");
}
