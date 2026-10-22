import {existsSync,readdirSync,rmSync,statSync} from "node:fs";
import {join,resolve,sep} from "node:path";
import {buildLeaseReportV1,readLeaseQueuesV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts";
import {classifyLeaseResourceV1,leaseOrderPolicyV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import {leaseRegistryRootV1,readLeaseRegistryEntriesV1,readLeaseViolationsV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import {REPOSITORY_LEASE_DIRECTORY_V1} from "../../../🔒️leases/⌨️command/🟦️.ts";
import {redactCommandV1,type HolderFactsV1,type LeasesFactsV1,type WaiterFactsV1} from "../../🟦️.ts";
import {readJsonFileV1,type DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../🟦️.ts";

const ENTRY=/^\d+-\d+\.json$/u;

type RegistryFolder=Readonly<{path:string;directory:string;names:readonly string[]}>;

/** 🗂️ Lists the registry folders that name their lease directory, each with its entry files. */
function folders(root:string):readonly RegistryFolder[]{
 const found:RegistryFolder[]=[];let names:string[];
 try{names=readdirSync(root);}catch{return found;}
 for(const name of names){
  const path=join(root,name),directory=(readJsonFileV1(join(path,"directory.json"),4096) as {directory?:unknown}|undefined)?.directory;
  if(typeof directory!=="string")continue;
  let files:string[]=[];try{files=readdirSync(path).filter(file=>ENTRY.test(file));}catch{}
  found.push({path,directory,names:files});
 }
 return found;
}

const gone=(directory:string,root:string):boolean=>!directory.startsWith(root+sep)&&!existsSync(directory);
const sizeOf=(path:string):number=>{try{return statSync(path).size;}catch{return 0;}};

/** 🧹️ Removes the rows of dead or silent processes and the folders of lease directories that are gone, by the registry's own rule. */
async function repair(ports:DoctorPortsV1,dryRun:boolean):Promise<DoctorRepairRunV1>{
 const root=leaseRegistryRootV1(ports.environment);if(!existsSync(root))return {removed:0,bytes:0,status:"unavailable",reason:"The lease registry does not exist."};
 let removed=0,bytes=0;
 for(const folder of folders(root)){
  ports.signal.throwIfAborted();
  const live=readLeaseRegistryEntriesV1(root,folder.directory,ports.now(),false),liveNames=new Set(live.map(row=>`${row.pid}-${row.startedAtMs}.json`));
  const dead=folder.names.filter(name=>!liveNames.has(name));
  if(gone(folder.directory,ports.root)&&live.length===0){
   for(const name of [...folder.names,"directory.json","violations.jsonl"])bytes+=sizeOf(join(folder.path,name));
   removed+=dead.length+1;if(!dryRun)rmSync(folder.path,{recursive:true,force:true});
   continue;
  }
  for(const name of dead){bytes+=sizeOf(join(folder.path,name));removed++;if(!dryRun)rmSync(join(folder.path,name),{force:true});}
 }
 return removed===0?{removed:0,bytes:0,status:"nothing"}:{removed,bytes};
}

/** 🔒️ The lease facts of the checkout: holders, waiters and their blockers, deadlock cycles, order violations, the machine registry's stale rows and queue tickets of dead processes. */
export const leasesProbeV1:DoctorProbeV1=Object.freeze({
 area:"leases",phase:1,
 repairs:[Object.freeze({id:"leases-registry",area:"leases" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repair(ports,dryRun)})],
 async read(ports,context){
  const directory=resolve(ports.root,REPOSITORY_LEASE_DIRECTORY_V1);
  if(!existsSync(directory))return {unavailable:"The lease directory does not exist yet; the leases image was never attached"};
  const now=ports.now(),root=leaseRegistryRootV1(ports.environment),policy=leaseOrderPolicyV1(ports.environment);
  const entries=readLeaseRegistryEntriesV1(root,directory,now,false),queues=readLeaseQueuesV1(directory),violations=readLeaseViolationsV1(root,directory,context.policy.leases.violationsLimit);
  const report=buildLeaseReportV1({directory,atMs:now,entries,queues,violations},ports.environment);
  const owner=(resource:string):string=>{const row=resource?classifyLeaseResourceV1(policy,resource).owner:"leases";return row==="unclassified"?"leases":row;};
  const name=(row:{resource:string;hash:string}):string=>row.resource||"hash:"+row.hash.slice(0,12);
  const deadPids=new Set(report.holders.filter(row=>!ports.alive(row.pid)).map(row=>row.pid)),living=report.holders.filter(row=>ports.alive(row.pid));
  const holders:HolderFactsV1[]=living.map(row=>({resource:name(row),hash:row.hash,mode:row.mode,pid:row.pid,heldSeconds:row.heldSeconds,class:row.class,owner:owner(row.resource),command:redactCommandV1(row.command??"")}));
  const waiters:WaiterFactsV1[]=report.waiters.filter(row=>ports.alive(row.pid)).map(row=>({resource:name(row),hash:row.hash,mode:row.mode,pid:row.pid,waitedSeconds:row.waitedSeconds,class:row.class,owner:owner(row.resource),position:row.position??null,queued:row.queued??null,blockers:[...new Set(row.blockers.map(id=>Number.parseInt(id,10)).filter(pid=>Number.isSafeInteger(pid)&&pid>0))],command:redactCommandV1(row.command??"")}));
  const all=folders(root);let orphanFolders=0,deadEntries=0,liveEntries=0;
  for(const folder of all){
   const live=readLeaseRegistryEntriesV1(root,folder.directory,now,false).length;
   liveEntries+=live;deadEntries+=Math.max(0,folder.names.length-live);
   if(gone(folder.directory,ports.root)&&live===0)orphanFolders++;
  }
  const facts:LeasesFactsV1={
   holders,waiters,
   cycles:report.cycles.map(row=>({members:[...row.members],victim:String(row.victim)})),
   violations:report.violations.map(row=>({kind:row.kind,held:row.held.slice(0,200),requested:row.requested.slice(0,200)})),
   registry:{folders:all.length,orphanFolders,deadEntries,liveEntries},
   deadTickets:queues.filter(row=>!ports.alive(row.pid)).length,
   deadHolders:[...deadPids].sort((left,right)=>left-right),
  };
  return {sections:{leases:facts}};
 },
});
