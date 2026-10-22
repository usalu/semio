import {existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {systemAncestryV1,type AncestryPortV1} from "../🧭️ancestry/🟦️.ts";
import {AGENT_SESSION_POLICY_V1,deriveAgentSessionV1,type AgentSeedV1,type AgentSessionV1} from "../🪪️identity/🟦️.ts";
import {agentSessionAddressV1,agentSessionRegistryRootV1,agentSessionStoreV1,agentSessionWorktreeKeyV1,collectAgentSessionsIfDueV1,openAgentSessionV1,parseAgentSessionAddressV1,readAgentSessionV1,touchAgentSessionV1,type AgentSessionReclaimV1,type AgentSessionStoreV1} from "../🗄️store/🟦️.ts";

const policy=AGENT_SESSION_POLICY_V1;
/** 🌿️ How a caller names the session it wants; everything optional, nothing read from a variable other than the one address. */
export type CurrentAgentSessionOptionsV1=Readonly<{
 environment?:Readonly<Record<string,string|undefined>>;registry?:string;worktree?:string;ancestry?:AncestryPortV1;now?:number;platform?:NodeJS.Platform;
 agent?:string;client?:string;origin?:"service";connection?:Readonly<{client:string;clientProcess:number;process:number}>;ownerPid?:number;weight?:number;alive?:(pid:number)=>boolean;collect?:AgentSessionReclaimV1;strict?:boolean;
}>;

const cache=new Map<string,AgentSessionV1>();
/** 🧼️ Forgets the sessions this process resolved, for tests. */
export function resetAgentSessionsV1():void{cache.clear();}

/** 🔭️ Finds the worktree that contains a directory: the nearest ancestor with a repository marker, else undefined. */
export function findAgentSessionWorktreeV1(start:string=process.cwd()):string|undefined{
 let directory=resolve(start);
 for(;;){
  if(existsSync(join(directory,".🧬semio"))||existsSync(join(directory,".git")))return directory;
  const parent=dirname(directory);if(parent===directory)return undefined;directory=parent;
 }
}

/** 🪪️ The one place every tool adapter reads the agent session from (a store that cannot be written still yields the derived session for this process, so a read-only home never stops a build; `strict` refuses instead): the session named by the environment address when it exists; otherwise the session identified by the explicit name, the client's own session variable, the connection or the nearest agent-client ancestor (opened on first use, the same session for the same identity afterwards), else the default session. */
export function currentAgentSessionV1(options:CurrentAgentSessionOptionsV1={}):AgentSessionV1{
 const environment=options.environment??process.env,registry=options.registry??agentSessionRegistryRootV1(environment,options.platform),now=options.now,picked=clientVariablesV1(environment);
 const named=options.agent!==undefined||options.connection!==undefined||options.origin!==undefined,address=named?undefined:parseAgentSessionAddressV1(environment[policy.environment.session]);
 if(address!==undefined){
  const found=cacheKey(registry,address.worktree,environment,picked,options),cached=cache.get(found);if(cached!==undefined)return cached;
  const store=agentSessionStoreV1(registry,address.worktree),session=readAgentSessionV1(store,address.id);
  if(session!==undefined){touchAgentSessionV1(store,session.id,now);cache.set(found,session);return session;}
 }
 const worktree=agentSessionWorktreeKeyV1(options.worktree??findAgentSessionWorktreeV1(),options.platform),key=cacheKey(registry,worktree,environment,picked,options),cached=cache.get(key);
 if(cached!==undefined)return cached;
 const identified=options.agent!==undefined||options.connection!==undefined||Object.keys(picked).length>0,ancestry=identified?[]:(options.ancestry??(()=>systemAncestryV1(options.platform)))();
 const seed:AgentSeedV1={environment:picked,ancestry,...options.agent===undefined?{}:{agent:options.agent},...options.client===undefined?{}:{client:options.client},...options.origin===undefined?{}:{origin:options.origin},...options.connection===undefined?{}:{connection:options.connection},...options.ownerPid===undefined?{}:{ownerPid:options.ownerPid},...options.weight===undefined?{}:{weight:options.weight}};
 const store:AgentSessionStoreV1=agentSessionStoreV1(registry,worktree),derived=deriveAgentSessionV1(seed);
 let opened:Readonly<{session:AgentSessionV1;created:boolean}>;
 try{opened=openAgentSessionV1(store,derived,{...now===undefined?{}:{now},...options.alive===undefined?{}:{alive:options.alive},...options.weight===undefined?{}:{weight:options.weight}});}
 catch(error){if(options.strict)throw error;const {anchor,...rest}=derived;opened={session:Object.freeze({...rest,worktree,anchors:Object.freeze(anchor===undefined?[]:[anchor]),openedAtMs:now??Date.now()}),created:false};}
 cache.set(key,opened.session);
 if(options.collect!==undefined)collectAgentSessionsIfDueV1(store,options.collect,{...now===undefined?{}:{now},...options.alive===undefined?{}:{alive:options.alive}});
 return opened.session;
}

function clientVariablesV1(environment:Readonly<Record<string,string|undefined>>):Record<string,string>{
 const picked:Record<string,string>={};
 for(const row of policy.identity.clients){const name=(row as {environment?:string}).environment,value=name===undefined?undefined:environment[name];if(value!==undefined&&value!=="")picked[name!]=value;}
 return picked;
}

function cacheKey(registry:string,worktree:string,environment:Readonly<Record<string,string|undefined>>,picked:Readonly<Record<string,string>>,options:CurrentAgentSessionOptionsV1):string{
 return JSON.stringify([registry,worktree,environment[policy.environment.session]??null,picked,options.agent??null,options.client??null,options.origin??null,options.connection??null,options.ownerPid??null,options.weight??null]);
}

/** 📤️ The environment a child process needs to attach to a session: its address, nothing else. */
export function agentSessionEnvironmentV1(session:Pick<AgentSessionV1,"worktree"|"id">):Readonly<Record<string,string>>{return Object.freeze({[policy.environment.session]:agentSessionAddressV1(session)});}

/** 🤝️ Resolves the current session strictly (the store must take it) and writes its address into the environment, so every child inherits it. */
export function adoptAgentSessionV1(environment:NodeJS.ProcessEnv=process.env,options:CurrentAgentSessionOptionsV1={}):AgentSessionV1{
 const session=currentAgentSessionV1({...options,environment,strict:true});Object.assign(environment,agentSessionEnvironmentV1(session));return session;
}
