import {spawnSync} from "node:child_process";
import {accessSync,constants,realpathSync} from "node:fs";
import {HERMETIC_TABLE_V1} from "../../🏃️execution/🔒️hermetic/🟦️.ts";
import type {RemoteExecutionSandboxV1} from "../🟦️.ts";

/** 🧱️ What a sandbox must confine: the command, its exact environment, where it runs, the one tree it may write (the private input root with its scratch directory) and whether it may use the network. */
export type RemoteExecutionSandboxSpecV1=Readonly<{argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string;root:string;network:boolean}>;

/** 🔒️ The command line a sandbox turns a spec into. */
export type RemoteExecutionSandboxedCommandV1=Readonly<{argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string}>;

/** 🛡️ One confinement strategy of the hermetic table, with the same names the table uses for builds. */
export interface RemoteExecutionSandboxStrategyV1{
 readonly name:RemoteExecutionSandboxV1;
 available():boolean;
 wrap(spec:RemoteExecutionSandboxSpecV1):RemoteExecutionSandboxedCommandV1;
}

const quote=(value:string):string=>`"${value.replace(/\\/gu,"\\\\").replace(/"/gu,'\\"')}"`;

/** 📜️ The macOS sandbox profile: everything stays allowed except network access (unless asked for), writes outside the input root and the system temporary locations, and reading file contents under the home directories and volumes except the input root. */
export function remoteExecutionDarwinProfileV1(root:string,network:boolean):string{
 return [
  "(version 1)","(allow default)",
  ...(network?[]:["(deny network*)"]),
  "(deny file-write*)",
  `(allow file-write* (subpath ${quote(root)}) (subpath "/dev") (literal "/private/var/run/utmpx"))`,
  '(deny file-read-data (subpath "/Users") (subpath "/Volumes") (subpath "/private/tmp") (subpath "/private/var/folders") (subpath "/private/var/tmp"))',
  `(allow file-read-data (subpath ${quote(root)}))`,
 ].join("\n");
}

/** 🧊️ The bubblewrap command line of the Linux sandbox: a read-only view of the host, the input root writable, fresh devices and process table, no network namespace access unless asked for, and everything dies with the parent. */
export function remoteExecutionBubblewrapArgumentsV1(spec:RemoteExecutionSandboxSpecV1):string[]{
 return ["bwrap",...(spec.network?[]:["--unshare-net"]),"--unshare-pid","--unshare-ipc","--die-with-parent","--new-session","--ro-bind","/","/","--dev","/dev","--proc","/proc","--bind",spec.root,spec.root,"--chdir",spec.cwd,"--clearenv",...Object.entries(spec.env).flatMap(([name,value])=>["--setenv",name,value]),"--",...spec.argv];
}

const probe=(command:string,args:readonly string[]):boolean=>{
 try{return spawnSync(command,args,{stdio:"ignore",timeout:10000}).status===0;}catch{return false;}
};
const executable=(path:string):boolean=>{try{accessSync(path,constants.X_OK);return true;}catch{return false;}};

const STRATEGIES:Readonly<Record<RemoteExecutionSandboxV1,RemoteExecutionSandboxStrategyV1>>=Object.freeze({
 "darwin-sandbox":{
  name:"darwin-sandbox",
  available:()=>process.platform==="darwin"&&executable("/usr/bin/sandbox-exec")&&probe("/usr/bin/sandbox-exec",["-p","(version 1)(allow default)","/usr/bin/true"]),
  wrap:spec=>({argv:["/usr/bin/sandbox-exec","-p",remoteExecutionDarwinProfileV1(realpathSync(spec.root),spec.network),...spec.argv],env:spec.env,cwd:spec.cwd}),
 },
 "linux-sandbox":{
  name:"linux-sandbox",
  available:()=>process.platform==="linux"&&probe("bwrap",["--unshare-net","--ro-bind","/","/","/bin/true"]),
  wrap:spec=>({argv:remoteExecutionBubblewrapArgumentsV1(spec),env:{PATH:spec.env.PATH??"/usr/bin:/bin"},cwd:spec.root}),
 },
 "isolated-root":{
  name:"isolated-root",
  available:()=>true,
  wrap:spec=>({argv:spec.argv,env:spec.env,cwd:spec.cwd}),
 },
});

/** 🧭️ The strategies of a host in the order the hermetic table lists them (`sandboxed` becomes the host's own sandbox; `remote` is the scheduler's business and is skipped). */
export function remoteExecutionSandboxOrderV1(platform:string=process.platform):readonly RemoteExecutionSandboxV1[]{
 const row=(HERMETIC_TABLE_V1.strategies as Record<string,readonly string[]>)[platform];
 if(!row)throw Error(`Hermetic strategies required for platform: ${platform}`);
 const sandbox:RemoteExecutionSandboxV1|undefined=platform==="darwin"?"darwin-sandbox":platform==="linux"?"linux-sandbox":undefined;
 return row.flatMap(name=>name==="sandboxed"&&sandbox?[sandbox]:name==="isolated-root"?["isolated-root" as const]:[]);
}

/** 🎛️ Picks the strategy for a worker: an explicit name must be available, `auto` takes the first available one of the host's table. */
export function selectRemoteExecutionSandboxV1(requested:"auto"|RemoteExecutionSandboxV1,platform:string=process.platform):RemoteExecutionSandboxStrategyV1{
 if(requested!=="auto"){
  const strategy=STRATEGIES[requested];
  if(!strategy.available())throw Error(`Sandbox strategy not available on this host: ${requested}`);
  return strategy;
 }
 for(const name of remoteExecutionSandboxOrderV1(platform)){const strategy=STRATEGIES[name];if(strategy.available())return strategy;}
 return STRATEGIES["isolated-root"];
}
