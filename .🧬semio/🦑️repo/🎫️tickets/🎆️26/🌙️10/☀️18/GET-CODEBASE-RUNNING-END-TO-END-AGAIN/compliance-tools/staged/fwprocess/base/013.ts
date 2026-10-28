import {createHash} from "node:crypto";
import {appendFileSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {homedir} from "node:os";
import {isAbsolute,join} from "node:path";
import type {LeaseCycleV1,LeaseModeV1,LeaseNodeV1,LeaseOrderPolicyV1,LeaseViolationV1} from "../📋️contract/🟦️.ts";
import {agentSessionAddressOfEnvironmentV1} from "../../../🪪️agent-sessions/🗄️store/🟦️.ts";
import {leaseHistoryKeyV1,leaseStandingSnapshotV1,observeLeaseStandingV1,probeLeaseActivityV1,type LeaseStandingV1} from "../../📈️standing/🟦️.ts";
import standingSchema from "../../📈️standing/🧬️schema/🔣️.json" with {type:"json"};

/** 🗂️ One process's published leases for one lease directory: what it holds and what it waits for. */
export type LeaseRegistryEntryV1=Readonly<{version:1;pid:number;startedAtMs:number;updatedAtMs:number;command:string;directory:string;session?:string;standing?:LeaseStandingV1;holds:readonly Readonly<{resource:string;mode:LeaseModeV1;sinceMs:number;task:number}>[];waits:readonly Readonly<{resource:string;mode:LeaseModeV1;sinceMs:number;task:number;queued:boolean}>[];ancestry?:Readonly<Record<string,readonly number[]>>}>;
type Snapshot=Readonly<{ancestry?:Readonly<Record<string,readonly number[]>>;holds:readonly Readonly<{directory:string;resource:string;mode:LeaseModeV1;atMs:number;task:number}>[];waits:readonly Readonly<{directory:string;resource:string;mode:LeaseModeV1;sinceMs:number;task:number;queued:boolean}>[]}>;
type Source=Readonly<{snapshot:()=>Snapshot;cycle:(nodes:readonly LeaseNodeV1[],from:string)=>LeaseCycleV1|undefined}>;

const HEARTBEAT_MILLISECONDS=60000,STALE_MILLISECONDS=300000;
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const key=(directory:string,resource:string):string=>`${directory}\0${resource}`;
const startedAtMs=Math.round(performance.timeOrigin);
/** 🕰️ When this process started (the operating system's start time within a few ms, so another process can verify it): the identity of its registry entry, its yield request file and its history of holds. */
export const leaseProcessStartedAtMsV1=startedAtMs;

/** 🗺️ The registry root: one directory per user shared by every checkout, next to the host capacity gate. An empty or relative `XDG_CACHE_HOME` is ignored as the XDG Base Directory Specification demands, so no working directory ever has a registry of its own. See https://specifications.freedesktop.org/basedir-spec/latest/ */
export function leaseRegistryRootV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment.SEMIO_LEASE_REGISTRY_DIRECTORY;
 if(override)return override;
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache");
 return join(base,"semio","lease-registry");
}
/** 🔑️ The per-lease-directory subfolder of the registry. */
export function leaseRegistryFolderV1(root:string,directory:string):string{return join(root,createHash("sha256").update(directory).digest("hex").slice(0,24));}

const readJson=(path:string):unknown=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};
const validEntry=(value:unknown):value is LeaseRegistryEntryV1=>{
 const row=value as LeaseRegistryEntryV1|undefined;
 return !!row&&row.version===1&&Number.isSafeInteger(row.pid)&&Array.isArray(row.holds)&&Array.isArray(row.waits)&&typeof row.directory==="string"&&typeof row.updatedAtMs==="number"&&(row.standing===undefined||row.standing.version===1);
};

/** 📖️ Reads every live entry of one lease directory and removes the entries of dead or silent processes. */
export function readLeaseRegistryEntriesV1(root:string,directory:string,now:number=Date.now(),prune=true):readonly LeaseRegistryEntryV1[]{
 const folder=leaseRegistryFolderV1(root,directory),found:LeaseRegistryEntryV1[]=[];
 let names:string[];try{names=readdirSync(folder).filter(name=>/^\d+-\d+\.json$/u.test(name));}catch{return [];}
 for(const name of names){
  const path=join(folder,name),value=readJson(path);
  if(!validEntry(value)||value.directory!==directory||!alive(value.pid)||now-value.updatedAtMs>STALE_MILLISECONDS){if(prune&&pid(name)!==process.pid)try{rmSync(path,{force:true});}catch{}continue;}
  found.push(value);
 }
 return found;
}
const pid=(name:string):number=>Number.parseInt(name,10);

/** 🧾️ Lists the lease directories that ever published to the registry. */
export function listLeaseRegistryDirectoriesV1(root:string):readonly string[]{
 const found:string[]=[];
 try{for(const folder of readdirSync(root)){const value=readJson(join(root,folder,"directory.json")) as {directory?:unknown}|undefined;if(typeof value?.directory==="string")found.push(value.directory);}}catch{}
 return found;
}

/** 📜️ Reads the most recent violations of one lease directory. */
export function readLeaseViolationsV1(root:string,directory:string,limit:number):readonly LeaseViolationV1[]{
 let text="";try{text=readFileSync(join(leaseRegistryFolderV1(root,directory),"violations.jsonl"),"utf8");}catch{return [];}
 const rows:LeaseViolationV1[]=[];
 for(const line of text.split("\n").slice(-limit-1)){if(!line)continue;try{const row=JSON.parse(line);if(row?.version===1)rows.push(row);}catch{}}
 return rows.slice(-limit);
}

/** 🕸️ Builds the wait-for graph nodes of one lease directory from the other processes' entries and this process's own fresh ledger. */
export function leaseNodesV1(entries:readonly LeaseRegistryEntryV1[],own:Snapshot,directory:string,ownPid:number=process.pid):readonly LeaseNodeV1[]{
 const nodes=new Map<string,{holds:{key:string;mode:LeaseModeV1}[];wait?:{key:string;mode:LeaseModeV1;sinceMs:number;queued:boolean};ancestors?:string[]}>(),chains=new Map<string,string[]>();
 const node=(id:string)=>{let row=nodes.get(id);if(!row){row={holds:[]};nodes.set(id,row);}return row;};
 for(const entry of entries){
  if(entry.pid===ownPid)continue;
  for(const hold of entry.holds)node(`${entry.pid}/${hold.task}`).holds.push({key:key(directory,hold.resource),mode:hold.mode});
  for(const wait of entry.waits)node(`${entry.pid}/${wait.task}`).wait={key:key(directory,wait.resource),mode:wait.mode,sinceMs:wait.sinceMs,queued:wait.queued};
  for(const[task,chain]of Object.entries(entry.ancestry??{}))if(Array.isArray(chain))chains.set(`${entry.pid}/${task}`,chain.map(value=>`${entry.pid}/${value}`));
 }
 for(const[task,chain]of Object.entries(own.ancestry??{}))chains.set(`${ownPid}/${task}`,chain.map(value=>`${ownPid}/${value}`));
 for(const hold of own.holds)if(hold.directory===directory)node(`${ownPid}/${hold.task}`).holds.push({key:key(directory,hold.resource),mode:hold.mode});
 for(const wait of own.waits)if(wait.directory===directory)node(`${ownPid}/${wait.task}`).wait={key:key(directory,wait.resource),mode:wait.mode,sinceMs:wait.sinceMs,queued:wait.queued};
 return [...nodes.entries()].map(([id,row])=>Object.freeze({id,holds:Object.freeze(row.holds),...row.wait?{wait:Object.freeze(row.wait)}:{},...chains.has(id)?{ancestors:Object.freeze(chains.get(id)!)}:{}}));
}

let source:Source|undefined;
const dirty=new Set<string>(),policies=new Map<string,LeaseOrderPolicyV1>(),written=new Set<string>(),descriptions=new Map<string,string>();
let timer:ReturnType<typeof setTimeout>|undefined,heartbeat:ReturnType<typeof setInterval>|undefined,disabled=false,exitHooked=false;
const rootNow=():string=>leaseRegistryRootV1();
const entryPath=(directory:string):string=>join(leaseRegistryFolderV1(rootNow(),directory),`${process.pid}-${startedAtMs}.json`);
const command=():string=>process.argv.slice(1).join(" ").slice(0,240);

function flush(directory:string):void{
 if(disabled||!source)return;
 const policy=policies.get(directory),snapshot=source.snapshot(),holds=snapshot.holds.filter(row=>row.directory===directory).map(row=>({resource:row.resource,mode:row.mode,sinceMs:row.atMs,task:row.task})),waits=snapshot.waits.filter(row=>row.directory===directory).map(row=>({resource:row.resource,mode:row.mode,sinceMs:row.sinceMs,task:row.task,queued:row.queued})),tasks=new Set([...holds.map(row=>row.task),...waits.map(row=>row.task)]),ancestry=Object.fromEntries(Object.entries(snapshot.ancestry??{}).filter(([task])=>tasks.has(Number(task))));
 const path=entryPath(directory);
 try{
  if(holds.length===0&&waits.length===0){rmSync(path,{force:true});written.delete(directory);return;}
  const folder=leaseRegistryFolderV1(rootNow(),directory);mkdirSync(folder,{recursive:true,mode:0o700});
  if(!written.has(directory)){writeFileSync(join(folder,"directory.json"),JSON.stringify({version:1,directory}));if(!exitHooked){exitHooked=true;observeLeaseStandingV1(()=>{for(const known of written)leaseRegistryV1.changed(known,policies.get(known)!);});process.once("exit",()=>{for(const known of written)try{rmSync(entryPath(known),{force:true});}catch{}});}}
  probeLeaseActivityV1();
  const address=agentSessionAddressOfEnvironmentV1(process.env),text=JSON.stringify({version:1,pid:process.pid,startedAtMs,updatedAtMs:Date.now(),command:command(),directory,...address===undefined?{}:{session:address},standing:leaseStandingSnapshotV1(address!==undefined),holds,waits,...Object.keys(ancestry).length?{ancestry}:{}});
  if(text.length>(policy?.registry.maximumEntryBytes??65536))throw Error("Bounded lease registry entry required");
  const temporary=`${path}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);written.add(directory);
  if(!heartbeat){heartbeat=setInterval(()=>{for(const known of written)flush(known);},HEARTBEAT_MILLISECONDS);heartbeat.unref();}
 }catch{disabled=true;}
}
function drain():void{timer=undefined;for(const directory of [...dirty]){dirty.delete(directory);flush(directory);}}

/** 📜️ Appends one finished exclusive hold to the history of its kind (resource family and command kind); the file keeps about `maximumRows` rows. */
export function appendLeaseHistoryV1(root:string,directory:string,key:string,heldMs:number,nowMs:number=Date.now()):void{
 const rules=standingSchema.definitions.Policy.const.history,folder=leaseRegistryFolderV1(root,directory),path=join(folder,rules.file);
 mkdirSync(folder,{recursive:true,mode:0o700});appendFileSync(path,JSON.stringify({key,ms:Math.round(heldMs),atMs:nowMs})+"\n");
 const lines=readFileSync(path,"utf8").split("\n").filter(Boolean);
 if(lines.length>rules.maximumRows*2){const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,lines.slice(-rules.maximumRows).join("\n")+"\n");renameSync(temporary,path);}
}
/** 📖️ The recent hold durations of one kind, oldest first. */
export function readLeaseHistoryV1(root:string,directory:string,key:string):readonly number[]{
 let text="";try{text=readFileSync(join(leaseRegistryFolderV1(root,directory),standingSchema.definitions.Policy.const.history.file),"utf8");}catch{return [];}
 const found:number[]=[];
 for(const line of text.split("\n")){if(!line)continue;try{const row=JSON.parse(line) as {key?:unknown;ms?:unknown};if(row.key===key&&typeof row.ms==="number")found.push(row.ms);}catch{}}
 return found;
}

/** 🗒️ The process's side of the registry: publishing, deadlock lookup and violation log. Every operation is best effort; a failing registry never fails a lease. */
export const leaseRegistryV1=Object.freeze({
 attach(next:Source):void{source=next;},
 changed(directory:string,policy:LeaseOrderPolicyV1,immediate=false):void{
  policies.set(directory,policy);
  if(immediate||policy.registry.flushMilliseconds===0){dirty.delete(directory);flush(directory);return;}
  dirty.add(directory);
  if(!timer){timer=setTimeout(drain,policy.registry.flushMilliseconds);timer.unref();}
 },
 findCycle(directory:string,from:string):LeaseCycleV1|undefined{
  if(!source)return undefined;
  const nodes=leaseNodesV1(readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false),source.snapshot(),directory),found=source.cycle(nodes,from);
  descriptions.clear();
  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false))for(const task of new Set([...entry.holds.map(row=>row.task),...entry.waits.map(row=>row.task)]))descriptions.set(`${entry.pid}/${task}`,`process ${entry.pid} (${entry.command.slice(0,60)}) holds [${entry.holds.filter(row=>row.task===task).map(row=>row.resource.slice(-40)).join(", ")}] waits for ${entry.waits.find(row=>row.task===task)?.resource.slice(-40)??"nothing"}`);
  return found;
 },
 describeNode(_directory:string,node:string):string{return descriptions.get(node)??`process ${node}`;},
 holdsShared(directory:string,resource:string,holder:number):boolean{
  return readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false).some(entry=>entry.pid===holder&&entry.holds.some(row=>row.resource===resource&&row.mode==="shared"));
 },
 otherHolders(directory:string,resource:string):readonly number[]{
  return readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false).filter(entry=>entry.pid!==process.pid&&entry.holds.some(row=>row.resource===resource)).map(entry=>entry.pid);
 },
 held(directory:string,resource:string,mode:LeaseModeV1,sinceMs:number):void{
  const heldMs=Date.now()-sinceMs;
  if(mode!=="exclusive"||disabled||heldMs<standingSchema.definitions.Policy.const.history.minimumHoldMilliseconds)return;
  try{appendLeaseHistoryV1(rootNow(),directory,leaseHistoryKeyV1(resource,command()),heldMs);}catch{}
 },
 directories():readonly string[]{return [...written];},
 ownHolds(directory:string):readonly Readonly<{resource:string;mode:LeaseModeV1;sinceMs:number}>[]{
  return source?source.snapshot().holds.filter(row=>row.directory===directory).map(row=>({resource:row.resource,mode:row.mode,sinceMs:row.atMs})):[];
 },
 holderOf(directory:string,resource:string,mode:LeaseModeV1):Readonly<{pid:number;heldSeconds:number}>|undefined{
  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false)){
   if(entry.pid===process.pid)continue;
   const hold=entry.holds.find(row=>row.resource===resource&&(row.mode==="exclusive"||mode==="exclusive"));
   if(hold)return Object.freeze({pid:entry.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-hold.sinceMs)/1000))});
  }
  return undefined;
 },
 recordViolation(directory:string,row:LeaseViolationV1,policy:LeaseOrderPolicyV1):void{
  const folder=leaseRegistryFolderV1(rootNow(),directory),path=join(folder,"violations.jsonl");
  mkdirSync(folder,{recursive:true,mode:0o700});appendFileSync(path,JSON.stringify(row)+"\n");
  const lines=readFileSync(path,"utf8").split("\n").filter(Boolean);
  if(lines.length>policy.registry.maximumViolations*2)writeFileSync(path,lines.slice(-policy.registry.maximumViolations).join("\n")+"\n");
 },
});
