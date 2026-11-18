import {test,expect} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as core from "../../🟦️.ts";
import * as glue from "../../💾️storage/🟦️.ts";

const protocol=core.incrementalProtocolV1,idle={check:()=>{},tick:async()=>{}};
const kindOf=(facts:{isSymbolicLink():boolean;isDirectory():boolean;isFile():boolean}):string=>facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":facts.isFile()?"file":"foreign";
/** 🏎️ The thread-pool ports the verification used before: the independent reference every verdict is compared with. */
const reference=(root:string,control:core.IncrementalControlV1):core.InstalledPortsV1=>{
 const path=(portable:string)=>portable==="."?root:join(root,...portable.split("/"));
 return{control,
  stat:async portable=>{try{const facts=await lstat(path(portable),{bigint:true});return{kind:kindOf(facts),stamp:core.installedStampV1(facts)};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}},
  children:async portable=>(await readdir(path(portable),{withFileTypes:true})).map(entry=>({name:entry.name,kind:kindOf(entry)}))};
};
const tree=(fan:number):string=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"ports-")));
 for(let a=0;a<fan;a++)for(let b=0;b<fan;b++){const dir=join(root,"a"+a,"b"+b);mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"f"),"x");for(let c=0;c<fan;c++)mkdirSync(join(dir,"c"+c));}
 return root;
};
const ledger=(root:string)=>{
 const sink=glue.installedLedgerSinkV1(Date.now()+10000);
 const walk=(path:string,portable:string):void=>{
  const facts=lstatSync(path,{bigint:true}),names=readdirSync(path).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),kids=names.map(name=>({name,kind:kindOf(lstatSync(join(path,name)))}));
  for(const kid of kids)if(kid.kind==="directory")walk(join(path,kid.name),portable==="."?kid.name:portable+"/"+kid.name);
  sink.directory(portable,facts,kids);
 };
 walk(root,".");
 const rows=[...sink.rows].sort((left,right)=>Buffer.compare(Buffer.from(left.path),Buffer.from(right.path))),counts=core.installedCountsV1(rows),encoded=core.encodeInstalledLedgerV1(rows);
 return{rows,receipt:core.sealInstalledReceiptV1({version:1,purpose:"dependency-installed-paths-receipt",generation:"a".repeat(64),device:rows[0]!.stamp[0],shape:{...counts,maximumPathUnits:64,sha256:"b".repeat(64)},evidence:encoded.evidence})};
};

test("ports law: the production ports answer every tamper exactly like the thread-pool reference (verdict, class, path, relisted)",async()=>{
 const cases:Record<string,(root:string)=>void>={
  unchanged:()=>{},
  missing:root=>rmSync(join(root,"a1","b1","c1"),{recursive:true}),
  swapped:root=>{const path=join(root,"a1","b0","c0");rmSync(path,{recursive:true});mkdirSync(path);},
  foreignLink:root=>{const path=join(root,"a0","b1","c1");rmSync(path,{recursive:true});symlinkSync(join(root,"a0"),path);},
  extraFile:root=>writeFileSync(join(root,"a2","b2","extra"),"y"),
  renamed:root=>renameSync(join(root,"a1","b2","c0"),join(root,"a1","b2","c9")),
  mode:root=>chmodSync(join(root,"a0","b0","c2"),0o500),
  dirToFile:root=>{const path=join(root,"a2","b0","c1");rmSync(path,{recursive:true});writeFileSync(path,"f");},
 };
 for(const[name,change]of Object.entries(cases)){
  const root=tree(3);
  try{
   const{rows,receipt}=ledger(root);change(root);
   const live=await core.verifyInstalledLedgerV1(receipt,rows,glue.installedPortsV1(root,idle)),expected=await core.verifyInstalledLedgerV1(receipt,rows,reference(root,idle));
   expect({name,verdict:live}).toEqual({name,verdict:expected});
   expect(live.status).toBe(name==="unchanged"?"match":"mismatch");
  }finally{chmodSync(join(root,"a0","b0","c2"),0o700);rmSync(root,{recursive:true,force:true});}
 }
},120000);

test("ports law: a verification yields every progress batch and never runs more than one batch of stats between two yields",async()=>{
 const root=tree(6);
 try{
  const{rows,receipt}=ledger(root);let statsSinceTick=0,longest=0,ticks=0;
  const control={check:()=>{},tick:async()=>{ticks++;longest=Math.max(longest,statsSinceTick);statsSinceTick=0;}},ports=glue.installedPortsV1(root,control),counted={...ports,stat:(portable:string)=>{statsSinceTick++;return ports.stat(portable);}};
  const result=await core.verifyInstalledLedgerV1(receipt,rows,counted);
  expect(result.status).toBe("match");
  expect(rows.length).toBeGreaterThan(200);
  expect(ticks).toBe(Math.floor(rows.length/protocol.installed.progressWork));
  expect(longest).toBeLessThanOrEqual(protocol.installed.progressWork);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("ports law: the production ports verify a ledger at least twice as fast as the thread-pool reference",async()=>{
 const root=tree(14);
 try{
  const{rows,receipt}=ledger(root),best=async(make:()=>core.InstalledPortsV1)=>{let low=Infinity;for(let round=0;round<3;round++){const start=performance.now();const result=await core.verifyInstalledLedgerV1(receipt,rows,make());expect(result.status).toBe("match");low=Math.min(low,performance.now()-start);}return low;};
  const slow=await best(()=>reference(root,idle)),fast=await best(()=>glue.installedPortsV1(root,idle));
  expect(rows.length).toBeGreaterThan(2500);
  expect(fast*2).toBeLessThan(slow);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
