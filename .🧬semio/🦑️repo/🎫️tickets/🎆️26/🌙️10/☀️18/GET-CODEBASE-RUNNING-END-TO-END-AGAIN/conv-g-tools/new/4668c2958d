import {spawn} from "node:child_process";
import {randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalRootV1,endpointsV1} from "../📨️protocol/📍️endpoint/🟨️.mjs";
import {createChangeJournalClientV1,type ChangeJournalClientV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {
 CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1,changeHeartbeatAgeV1,createChangeLogWriterV1,readChangeStateV1,touchChangeFileV1,writeChangeStateV1,
 type ChangeJournalPolicyV1,type ChangeRowV1,type ChangeStateV1,
} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";

/** 🔭️ The slice of the native watch supervisor this service needs: it is injected, so the service owns no watcher of its own. */
export interface ChangeWatchSupervisorV1{start():Promise<{backend:string;trust:string;reason?:string}>;stop():Promise<void>}
/** 🏭️ Builds the supervisor from the options the existing `createWatchSupervisorV1` takes. */
export type ChangeWatchFactoryV1=(options:Readonly<{
 root:string;policy:Record<string,unknown>;filter:{excluded(path:string):boolean};rooted:readonly string[];names:readonly string[];binary?:string;backend:"auto"|"native"|"portable";latencyMs:number;fenceDirectory:string;instance:string;
 onBatch:(rows:readonly(readonly[string,number,string])[],at:number)=>Promise<void>;onLost:(scope:string,reason:string)=>void;onTrust:(state:{backend:string;trust:string;reason:string})=>void;log:(kind:string,detail:string)=>void;
}>)=>ChangeWatchSupervisorV1;

/** 🎛️ Everything one service run needs. */
export type ChangeJournalServiceOptionsV1=Readonly<{
 root:string;directory:string;fence:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>;binary?:string;watcherPolicy:Record<string,unknown>;createSupervisor:ChangeWatchFactoryV1;
 policy?:ChangeJournalPolicyV1;now?:()=>number;log?:(kind:string,detail:string)=>void;residentBytes?:()=>number;
}>;
/** 🛑️ Another live service already owns the journal directory. */
export class ChangeJournalRunningErrorV1 extends Error{constructor(directory:string){super("A change journal service already runs for "+directory);this.name="ChangeJournalRunningErrorV1";}}

const excludedBy=(exclude:ChangeJournalServiceOptionsV1["exclude"])=>(path:string):boolean=>{
 if(!path)return false;
 for(const prefix of exclude.rooted)if(path===prefix||path.startsWith(prefix+"/"))return true;
 return exclude.names.length>0&&path.split("/").some(part=>exclude.names.includes(part));
};

function takeLock(directory:string,policy:ChangeJournalPolicyV1,now:number):void{
 const lock=join(directory,CHANGE_JOURNAL_FILES_V1.lock);
 for(let attempt=0;attempt<2;attempt++){
  try{mkdirSync(lock,{mode:0o700});writeFileSync(join(lock,"owner.json"),JSON.stringify({pid:process.pid,at:now}),{mode:0o600});return;}
  catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   let owner:{pid?:number;at?:number}={};
   try{owner=JSON.parse(readFileSync(join(lock,"owner.json"),"utf8"));}catch{}
   const alive=(()=>{try{process.kill(owner.pid??0,0);return true;}catch(failure){return(failure as NodeJS.ErrnoException).code==="EPERM";}})();
   if(alive&&(changeHeartbeatAgeV1(directory,now)<policy.heartbeatStaleMilliseconds||now-(owner.at??0)<policy.startTimeoutMilliseconds))throw new ChangeJournalRunningErrorV1(directory);
   rmSync(lock,{recursive:true,force:true});
  }
 }
 throw new ChangeJournalRunningErrorV1(directory);
}

/** 📜️ Runs the change journal service of one workspace: one new epoch, one writer, one watcher supervisor, a heartbeat, idle shutdown. It keeps no membership and no task state, only the log, so its memory is the runtime plus the supervisor's queue. */
export async function runChangeJournalServiceV1(options:ChangeJournalServiceOptionsV1):Promise<Readonly<{epoch:string;stop(reason:string):Promise<void>;closed:Promise<string>}>>{
 const {root,directory,fence,exclude,binary,watcherPolicy,createSupervisor,policy=CHANGE_JOURNAL_POLICY_V1,now=Date.now,log=()=>{},residentBytes=()=>process.memoryUsage().rss}=options;
 mkdirSync(directory,{recursive:true,mode:0o700});
 takeLock(directory,policy,now());
 const epoch=randomBytes(8).toString("hex"),instance=randomBytes(4).toString("hex"),startedAt=now(),excluded=excludedBy(exclude);
 rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.unavailable),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.state),{force:true});
 const writer=createChangeLogWriterV1({directory,epoch,policy});
 let current:ChangeStateV1={version:1,epoch,instance,pid:process.pid,startedAt,root,fence,exclude:{rooted:[...exclude.rooted],names:[...exclude.names]},backend:"none",trust:"none",ready:false};
 const publish=(change:Partial<ChangeStateV1>):void=>{current={...current,...change};writeChangeStateV1(directory,current);};
 let closeResolve!:(reason:string)=>void,stopping:Promise<void>|undefined,timer:ReturnType<typeof setInterval>|undefined,supervisor:ChangeWatchSupervisorV1|undefined,trust="none";
 const closed=new Promise<string>(resolve=>{closeResolve=resolve;});
 const stop=(reason:string):Promise<void>=>stopping??=(async()=>{
  if(timer)clearInterval(timer);
  try{await supervisor?.stop();}catch{}
  writer.close();
  try{publish({ready:false,trust:"none",reason});}catch{}
  rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.heartbeat),{force:true});rmSync(join(directory,CHANGE_JOURNAL_FILES_V1.lock),{recursive:true,force:true});
  closeResolve(reason);
 })();
 const unavailable=async(reason:string):Promise<never>=>{
  writeFileSync(join(directory,CHANGE_JOURNAL_FILES_V1.unavailable),JSON.stringify({at:now(),reason}),{mode:0o600});
  await stop(reason);
  throw Error("The change journal cannot vouch: "+reason+" / Der Änderungsjournal-Dienst kann nicht einstehen: "+reason);
 };
 try{
  supervisor=createSupervisor({
   root,policy:{...watcherPolicy,watcherLatencyMilliseconds:policy.watcherLatencyMilliseconds},filter:{excluded},rooted:exclude.rooted,names:exclude.names,binary,backend:binary?"native":"portable",latencyMs:policy.watcherLatencyMilliseconds,fenceDirectory:fence,instance,log,
   onBatch:async rows=>{writer.append(rows as readonly ChangeRowV1[],now());},
   onLost:(scope,reason)=>{writer.lost(reason,scope,now());},
   onTrust:state=>{
    const lost=trust==="exact"&&state.trust!=="exact";trust=state.trust;
    if(lost)writer.lost("trust-"+(state.reason||state.trust),"",now());
    if(current.ready)publish({trust:state.trust as ChangeStateV1["trust"],backend:state.backend as ChangeStateV1["backend"],reason:state.reason});
   },
  });
  const started=await supervisor.start();
  trust=started.trust;
  if(!["fsevents","inotify","rdcw"].includes(started.backend)||started.trust!=="exact")await unavailable(started.reason||started.backend+"/"+started.trust);
  publish({backend:started.backend as ChangeStateV1["backend"],trust:started.trust as ChangeStateV1["trust"],ready:true,reason:undefined});
 }catch(error){
  if(!stopping)await stop("failed");
  throw error;
 }
 touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,now());touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.activity,now());
 let swept=0;
 timer=setInterval(()=>{
  const at=now();
  touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.heartbeat,at);
  let activity=at;
  try{activity=statSync(join(directory,CHANGE_JOURNAL_FILES_V1.activity)).mtimeMs;}catch{}
  if(at-activity>policy.idleShutdownMilliseconds){void stop("idle");return;}
  if(residentBytes()>policy.maximumResidentBytes){void stop("memory");return;}
  if(at-swept>10000){
   swept=at;
   const base=join(root,...fence.split("/"));
   try{for(const name of readdirSync(base))if(at-statSync(join(base,name)).mtimeMs>30000)rmSync(join(base,name),{force:true});}catch{}
  }
 },policy.heartbeatMilliseconds);
 timer.unref?.();
 log("ready",epoch);
 return{epoch,stop,closed};
}

/** 🚀️ Starts the service detached when none is live and waits until its state is ready; resolves quietly when it cannot (the client then walks the tree). */
export async function startChangeJournalServiceV1(options:Readonly<{directory:string;root:string;execPath?:string;script?:string;arguments?:readonly string[];policy?:ChangeJournalPolicyV1;now?:()=>number;environment?:Readonly<Record<string,string|undefined>>}>):Promise<void>{
 const {directory,root,execPath=process.execPath,script=fileURLToPath(new URL("./⌨️main/🟦️.ts",import.meta.url)),policy=CHANGE_JOURNAL_POLICY_V1,now=Date.now,environment=process.env}=options;
 const live=():boolean=>{const state=readChangeStateV1(directory);return !!state&&changeHeartbeatAgeV1(directory,now())<policy.heartbeatStaleMilliseconds&&state.ready;};
 if(live())return;
 mkdirSync(directory,{recursive:true,mode:0o700});
 const child=spawn(execPath,[script,"--root",root,"--directory",directory,...(options.arguments??[])],{cwd:root,env:{...environment},stdio:"ignore",detached:true,windowsHide:true});
 child.unref();
 let exited=false;child.once("exit",()=>{exited=true;});
 const deadline=now()+policy.startTimeoutMilliseconds;
 while(now()<deadline&&!exited){
  if(live())return;
  await new Promise(resolve=>setTimeout(resolve,policy.startPollMilliseconds));
 }
}

/** 🏁️ The command line of the detached service process: `--root <workspace> --directory <journal directory> [--binary <watcher>]`. */
export async function runChangeJournalMainV1(argv:readonly string[],dependencies:Readonly<{createSupervisor:ChangeWatchFactoryV1;watcherPolicy:Record<string,unknown>;exclude:ChangeJournalServiceOptionsV1["exclude"];fence:string;locate:()=>Promise<string|undefined>}>):Promise<number>{
 const value=(name:string):string|undefined=>{const at=argv.indexOf(name);return at<0?undefined:argv[at+1];};
 const root=value("--root"),directory=value("--directory");
 if(!root||!directory)throw Error("--root and --directory required / --root und --directory erforderlich");
 const binary=value("--binary")??await dependencies.locate();
 let service;
 try{service=await runChangeJournalServiceV1({root,directory,fence:dependencies.fence,exclude:dependencies.exclude,binary,watcherPolicy:dependencies.watcherPolicy,createSupervisor:dependencies.createSupervisor});}
 catch(error){return error instanceof ChangeJournalRunningErrorV1?0:1;}
 for(const signal of["SIGTERM","SIGINT"] as const)process.once(signal,()=>void service.stop(signal));
 await service.closed;
 return 0;
}


/** 📍️ The journal directory of a workspace: below the private runtime directory of the resident service endpoints (one per worktree and user). */
export function changeJournalDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return join(endpointsV1(canonicalRootV1(root),environment as NodeJS.ProcessEnv).directory,"journal");
}

/** 🔌️ The client every consumer uses: it starts the service on demand and falls back (by refusing) whenever the journal cannot vouch. */
export function connectChangeJournalV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):ChangeJournalClientV1{
 const canonical=canonicalRootV1(root),directory=changeJournalDirectoryV1(canonical,environment);
 return createChangeJournalClientV1({directory,root:canonical,environment,start:()=>startChangeJournalServiceV1({directory,root:canonical,environment})});
}
