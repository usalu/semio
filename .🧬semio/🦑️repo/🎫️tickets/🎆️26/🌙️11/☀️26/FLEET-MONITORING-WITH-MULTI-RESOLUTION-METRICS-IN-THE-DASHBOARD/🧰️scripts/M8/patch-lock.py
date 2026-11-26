import os
p='🟦️.ts'
s=open(p,encoding='utf8').read()
a=s.index('/** 🔒️ Takes the writer lock')
b=s.index('export interface FleetMetricsOptionsV1')
new='''/** 🪪️ Who asks for the writer lock of a ring directory: the `role` it collects for (`origin`, or `fleet:<source>`) and the `checkout` it runs for. @see ./🧬️schema/🔣️.json */
export interface MetricsLockClaimV1{readonly role:string;readonly checkout?:string}
/** 🔒️ The answer to a claim: whether this process now writes, and, when not, why (`refused`: the claim may never hold this lock; `foreign`: another live process holds it, named in `detail`). */
export interface MetricsLockResultV1{readonly held:boolean;readonly refusal?:"refused"|"foreign";readonly detail?:string}
interface LockRecordV1{readonly version?:number;readonly pid?:number;readonly startMs?:number;readonly role?:string;readonly checkout?:string}

/** 🧪️ Whether a path lies inside the operating system's temporary directory (by name or by real path): the checkout of a test or a scratch member, never the worktree of a machine. */
export function metricsScratchPathV1(path:string|undefined):boolean{
 if(path===undefined||path==="")return false;
 const roots=new Set<string>([tmpdir()]);
 try{roots.add(realpathSync(tmpdir()));}catch{}
 return[...roots].some(root=>{const inside=relative(root,path);return inside!==""&&!inside.startsWith("..")&&!isAbsolute(inside);});
}

const insideV1=(path:string,root:string):boolean=>{const inside=relative(root,path);return inside===""||(!inside.startsWith("..")&&!isAbsolute(inside));};
const ROLE=new RegExp(P.lockRolePattern,"u");

/** 🛡️ Why a claim may not hold the lock of `directory`, or `undefined` when it may: the role must be a known one, and the machine-wide ring directory (the host capacity directory of this machine) belongs only to a real collector: not to a claim whose checkout is inside the temporary directory, not to a claim without a checkout for the machine role and not to a process under a test runner. A private directory is free for everyone.
 * @see ./🧬️schema/🔣️.json */
export function metricsLockRefusalV1(directory:string,claim:MetricsLockClaimV1,environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 if(!ROLE.test(claim.role))return `unknown role ${claim.role}`;
 let machine:string|undefined;
 try{machine=join(hostCapacityDirectoryV1(),METRICS_FILES_V1.directory);}catch{}
 if(machine===undefined||!insideV1(resolve(directory),machine))return undefined;
 if(metricsScratchPathV1(claim.checkout))return `checkout ${claim.checkout} is inside the temporary directory`;
 if(environment["NODE_ENV"]==="test")return "a test runner never writes the machine rings";
 if(claim.role==="origin"&&(claim.checkout===undefined||claim.checkout===""))return "no checkout named";
 return undefined;
}

const holderName=(holder:LockRecordV1):string=>`${holder.role??"unnamed collector"} pid ${holder.pid??"?"}${holder.checkout===undefined?"":` of ${holder.checkout}`}`;

/** 🔒️ Takes the writer lock of a ring directory for `claim` (`collector.lock`, written whole under a temporary name and linked into place, so a reader never sees half a record). The record names the process by its identity (pid and start time), its role and its checkout. A lock whose holder is gone (by identity, a reused pid is no holder) is taken over once; a live holder keeps it, and so does a record that cannot be read until it is `lockUnreadableMilliseconds` old. Answers whether this process now writes and, if not, why.
 * @see ./🧬️schema/🔣️.json */
export function acquireMetricsLockV1(directory:string,claim:MetricsLockClaimV1):MetricsLockResultV1{
 const refusal=metricsLockRefusalV1(directory,claim);
 if(refusal!==undefined)return {held:false,refusal:"refused",detail:refusal};
 const path=join(directory,P.lockFile),mine=ownProcessIdentityV1();
 mkdirSync(directory,{recursive:true});
 const record:LockRecordV1={version:1,pid:mine.pid,startMs:mine.startMs,role:claim.role,...claim.checkout===undefined?{}:{checkout:claim.checkout}};
 for(let attempt=0;attempt<2;attempt++){
  const temporary=`${path}.${mine.pid}.tmp`;
  try{
   try{writeFileSync(temporary,JSON.stringify(record));linkSync(temporary,path);return {held:true};}
   catch(error){
    if((error as NodeJS.ErrnoException).code!=="EEXIST")return {held:false,refusal:"refused",detail:(error as Error).message};
   }
   let holder:LockRecordV1;
   try{holder=JSON.parse(readFileSync(path,"utf8")) as LockRecordV1;if(typeof holder!=="object"||holder===null)throw new Error("not a record");}
   catch{
    try{if(Date.now()-statSync(path).mtimeMs<P.lockUnreadableMilliseconds)return {held:false,refusal:"foreign",detail:"unreadable record"};unlinkSync(path);}catch{}
    continue;
   }
   if(holder.pid===mine.pid&&holder.startMs===mine.startMs)return holder.role===claim.role?{held:true}:{held:false,refusal:"foreign",detail:holderName(holder)};
   if(processRecordAliveV1(holder.pid,holder.startMs))return {held:false,refusal:"foreign",detail:holderName(holder)};
   try{unlinkSync(path);}catch{}
  }finally{try{unlinkSync(temporary);}catch{}}
 }
 return {held:false,refusal:"foreign",detail:"lock contended"};
}

/** 🔓️ Gives the writer lock back when this process holds it (same identity and, when `claim` is given, the same role). */
export function releaseMetricsLockV1(directory:string,claim?:MetricsLockClaimV1):void{
 const path=join(directory,P.lockFile),mine=ownProcessIdentityV1();
 try{const holder=JSON.parse(readFileSync(path,"utf8")) as LockRecordV1;if(holder.pid===mine.pid&&holder.startMs===mine.startMs&&(claim===undefined||holder.role===claim.role))unlinkSync(path);}catch{}
}

'''
s=s[:a]+new+s[b:]
s=s.replace('import {mkdirSync,readFileSync,renameSync,rmSync,unlinkSync,writeFileSync} from "node:fs";\nimport {join} from "node:path";','import {linkSync,mkdirSync,readFileSync,realpathSync,renameSync,rmSync,statSync,unlinkSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {isAbsolute,join,relative,resolve} from "node:path";',1)
s=s.replace('import {ownProcessIdentityV1,processRecordAliveV1} from "../../../🔖️identity/🟦️.ts";','import {ownProcessIdentityV1,processRecordAliveV1} from "../../../🔖️identity/🟦️.ts";\nimport {hostCapacityDirectoryV1} from "../../🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";',1)
s=s.replace(' readonly ids?:readonly string[];\n}',' readonly ids?:readonly string[];\n readonly checkout?:string;\n}',1)
open(p+'.tmp','w',encoding='utf8').write(s);os.rename(p+'.tmp',p)
