/** 🧱️ Publishes reviewed direct owner bindings with durable intent and immediate shared-file guards. */
import {readFileSync,writeFileSync,openSync,closeSync,readSync,writeSync,ftruncateSync,fsyncSync,fstatSync,lstatSync,existsSync,mkdirSync,constants} from "node:fs";
import {resolve,relative,dirname,join,isAbsolute} from "node:path";
import {createHash} from "node:crypto";
const root="/Users/ueli/Documents/semio",g="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-publication-root",selfPath=import.meta.path;
const hash=(v:Uint8Array|string)=>createHash("sha256").update(v).digest("hex");
const planPath=join(g,"publication-plan-1.json"),inputPath=join(g,"publication-input-independent-1.json");
const expectedPlan="2ed0f149f3e303296c0727381bb6beb810ac24a3a67569295794c3e255cc07ea";
const planRaw=readFileSync(planPath),plan=JSON.parse(planRaw.toString()),inputRaw=readFileSync(inputPath),selfRaw=readFileSync(selfPath);
if(hash(planRaw)!==expectedPlan)throw Error("Reviewed plan changed");
const priority:Record<string,number>={"normal-dependency":0,"explicit-owned-exports":1,"defining-owner-import":2};
const rows=plan.pairs.map((x:any,index:number)=>({...x,planIndex:index})).sort((a:any,b:any)=>priority[a.kind]-priority[b.kind]||a.planIndex-b.planIndex);
if(rows.length!==3||new Set(rows.map((x:any)=>x.path)).size!==3||rows.some((x:any)=>priority[x.kind]===undefined))throw Error("Publication ordering changed");
const target=(path:string)=>{
 const p=resolve(root,path),rel=relative(root,p);
 if(isAbsolute(path)||!rel||rel.startsWith("..")||isAbsolute(rel)||path.split("/").includes("AGENTS.md"))throw Error("Unowned path");
 for(let parent=p;parent!==root;parent=dirname(parent))if(existsSync(parent)&&lstatSync(parent).isSymbolicLink())throw Error("Symbolic publication ancestry");
 return p;
};
const read=(path:string):string|null=>{
 const p=target(path);if(!existsSync(p))return null;
 const fd=openSync(p,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
 try{if(!fstatSync(fd).isFile())throw Error("Nonregular target");return fdText(fd)}finally{closeSync(fd)}
};
const fdText=(fd:number)=>{
 const size=fstatSync(fd).size,bytes=Buffer.allocUnsafe(size);let n=0;
 while(n<size){const count=readSync(fd,bytes,n,size-n,n);if(count<=0)throw Error("Opened read made no progress");n+=count}
 if(fstatSync(fd).size!==size)throw Error("Opened size advanced");return bytes.toString("utf8");
};
const mode=process.argv[2],auditPath=join(g,"publisher-source-independent-3.json");
const auditRaw=mode==="publish"?readFileSync(auditPath):null,audit=auditRaw?JSON.parse(auditRaw.toString()):null;
if(mode!=="review"&&mode!=="publish")throw Error("Expected review or publish");
if(mode==="publish"&&(audit.ready!==true||audit.planSha256!==expectedPlan||audit.publisherSourceSha256!==hash(selfRaw)||audit.inputProofSha256!==hash(inputRaw)))throw Error("Publisher admission mismatch");
const evidence=()=>{
 if(hash(readFileSync(planPath))!==expectedPlan||hash(readFileSync(inputPath))!==hash(inputRaw)||hash(readFileSync(selfPath))!==hash(selfRaw))throw Error("Publication input advanced");
 if(auditRaw&&hash(readFileSync(auditPath))!==hash(auditRaw))throw Error("Publisher admission advanced");
 if(hash(readFileSync(plan.sourceAuthority.path))!==plan.sourceAuthority.rawSha256||readFileSync(plan.sourceAuthority.path,"utf8")!==plan.sourceAuthority.rawUtf8)throw Error("Source advanced");
 for(const x of plan.proofs)if(hash(readFileSync(x.path))!==x.rawSha256||readFileSync(x.path,"utf8")!==x.rawUtf8)throw Error("Proof advanced");
 for(const x of plan.contexts)if(read(x.path)!==x.source||hash(x.source)!==x.sha256)throw Error("Context advanced");
 for(const x of plan.rawBindings)if(hash(readFileSync(x.path))!==x.sha256)throw Error("Raw native binding advanced");
};
const completed=new Set<string>();
const endpoints=(except?:string)=>{
 for(const x of rows)if(x.path!==except&&(read(x.path)!==(completed.has(x.path)?x.after:x.before)||x.inverse!==x.before||hash(x.after)!==x.afterSha256))throw Error("Endpoint advanced: "+x.path);
};
evidence();endpoints();
if(mode==="review"){console.log(JSON.stringify({ready:true,planSha256:expectedPlan,inputProofSha256:hash(inputRaw),publisherSourceSha256:hash(selfRaw),pairs:3,contexts:14,order:rows.map((x:any)=>({path:x.path,kind:x.kind})),writes:0}));process.exit(0)}
const journal:any[]=[],journalPath=join(g,"publication-journal-3.json"),receiptPath=join(g,"published-3.json");
if(existsSync(journalPath)||existsSync(receiptPath))throw Error("Publication receipt exists");
const save=()=>{
 const bytes=Buffer.from(JSON.stringify({planSha256:expectedPlan,publisherSourceSha256:hash(selfRaw),atomicIdentityClaimed:false,journal},null,2)+"\n");
 const fd=openSync(journalPath,constants.O_WRONLY|constants.O_CREAT|(constants.O_NOFOLLOW??0),0o644);
 try{let n=0;while(n<bytes.length){const count=writeSync(fd,bytes,n,bytes.length-n,n);if(count<=0)throw Error("Journal made no progress");n+=count}ftruncateSync(fd,bytes.length);fsyncSync(fd)}finally{closeSync(fd)}
};
for(const x of rows){
 const bytes=Buffer.from(x.after);evidence();endpoints();
 const record={path:x.path,planIndex:x.planIndex,before:x.before,after:x.after,inverse:x.inverse,device:null as string|null,inode:null as string|null,intent:true,completed:false};
 journal.push(record);save();evidence();endpoints();
 const p=target(x.path);
 if(x.before===null){mkdirSync(dirname(p),{recursive:true});target(x.path);evidence();endpoints()}
 const flags=x.before===null?constants.O_CREAT|constants.O_EXCL|constants.O_RDWR:constants.O_RDWR|(constants.O_NOFOLLOW??0);
 const fd=openSync(p,flags,0o644);
 try{
  const stat=fstatSync(fd);if(!stat.isFile())throw Error("Nonregular opened target");
  record.device=String(stat.dev);record.inode=String(stat.ino);save();
  evidence();endpoints(x.path);
  const pathStat=lstatSync(target(x.path)),finalStat=fstatSync(fd);
  if(pathStat.isSymbolicLink()||!pathStat.isFile()||pathStat.dev!==stat.dev||pathStat.ino!==stat.ino||finalStat.dev!==stat.dev||finalStat.ino!==stat.ino)throw Error("Opened path identity advanced");
  if(fdText(fd)!==(x.before??""))throw Error("Final opened predecessor advanced");
  let n=0;while(n<bytes.length){const count=writeSync(fd,bytes,n,bytes.length-n,n);if(count<=0)throw Error("Target made no progress");n+=count}
  ftruncateSync(fd,bytes.length);fsyncSync(fd);
  const after=fstatSync(fd);if(after.dev!==stat.dev||after.ino!==stat.ino||fdText(fd)!==x.after)throw Error("Written descriptor advanced");
 }finally{closeSync(fd)}
 if(read(x.path)!==x.after)throw Error("Written endpoint advanced");
 completed.add(x.path);record.completed=true;save();evidence();endpoints();
}
const receipt={planPath,planSha256:expectedPlan,inputProofSha256:hash(inputRaw),publisherSourceSha256:hash(selfRaw),journalPath,journalSha256:hash(readFileSync(journalPath)),writes:3,rows:journal.map(x=>({path:x.path,planIndex:x.planIndex,completed:x.completed,beforeSha256:x.before===null?null:hash(x.before),afterSha256:hash(x.after)})),nativeOwningWholePassed:true,gpuRuntimeReady:false,broaderDeletionReady:false,atomicIdentityClaimed:false};
writeFileSync(receiptPath,JSON.stringify(receipt,null,2)+"\n");console.log(JSON.stringify({path:receiptPath,writes:3,journalSha256:receipt.journalSha256,nativeRuntimeClaimed:true}));
