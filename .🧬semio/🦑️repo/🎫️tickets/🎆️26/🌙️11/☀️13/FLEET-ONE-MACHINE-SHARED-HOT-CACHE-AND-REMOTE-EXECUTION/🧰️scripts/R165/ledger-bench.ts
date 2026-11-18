import fs from "node:fs"; import {join} from "node:path"; import {lstat,readdir} from "node:fs/promises";
const c=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🟦️.ts");
const root="/Users/ueli/Documents/semio/.cache/b/d/m/j/"+(process.env.GEN??"d22a22b6385877c52eb2");
// read-only scan building rows (recording wrapper counts)
const rows:any[]=[];const thr=c.installedThresholdNsV1(Date.now());let scanL=0;
function walk(abs:string,portable:string){const f=fs.lstatSync(abs,{bigint:true});scanL++;const ents=fs.readdirSync(abs,{withFileTypes:true});const ch=ents.map(e=>({name:e.name,kind:e.isSymbolicLink()?"link":e.isDirectory()?"directory":e.isFile()?"file":"foreign"}));rows.push(c.installedRowV1(portable,f,ch,thr));for(const e of ents)if(e.isDirectory()&&!e.isSymbolicLink())walk(join(abs,e.name),portable==="."?e.name:portable+"/"+e.name);}
walk(root,".");rows.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
const counts=c.installedCountsV1(rows);const receipt={device:rows[0].stamp[0],shape:{entries:counts.entries,directories:counts.directories,files:counts.files,links:counts.links}};
const kindOf=(f:any)=>f.isSymbolicLink()?"link":f.isDirectory()?"directory":f.isFile()?"file":"foreign";
const n={lstat:0,readdir:0};
const path=(p:string)=>p==="."?root:join(root,...p.split("/"));
const asyncPorts=(control:any)=>({control,stat:async(p:string)=>{n.lstat++;try{const f=await lstat(path(p),{bigint:true});return{kind:kindOf(f),stamp:c.installedStampV1(f)};}catch(e:any){if(e.code==="ENOENT")return undefined;throw e;}},children:async(p:string)=>{n.readdir++;return(await readdir(path(p),{withFileTypes:true})).map(e=>({name:e.name,kind:kindOf(e)}));}});
const syncPorts=(control:any)=>({control,stat:async(p:string)=>{n.lstat++;try{const f=fs.lstatSync(path(p),{bigint:true});return{kind:kindOf(f),stamp:c.installedStampV1(f)};}catch(e:any){if(e.code==="ENOENT")return undefined;throw e;}},children:async(p:string)=>{n.readdir++;return fs.readdirSync(path(p),{withFileTypes:true}).map(e=>({name:e.name,kind:kindOf(e)}));}});
const mk=(tick:()=>Promise<void>)=>({check(){},tick});
console.log("rows",rows.length,"scan lstat",scanL,"load",(await import("node:os")).loadavg().map(x=>x.toFixed(1)).join(" "));
for(const [label,ports,tick] of [["async+nop tick",asyncPorts,async()=>{}],["sync+nop tick",syncPorts,async()=>{}],["async+setImmediate tick",asyncPorts,()=>new Promise<void>(r=>setImmediate(r))],["sync+setImmediate tick",syncPorts,()=>new Promise<void>(r=>setImmediate(r))]] as any[]){
 n.lstat=0;n.readdir=0;const t=performance.now();const res=await c.verifyInstalledLedgerV1(receipt as any,rows,ports(mk(tick)));const ms=performance.now()-t;console.log(label,res.status,Math.round(ms),"ms",n.lstat,"lstat",n.readdir,"readdir","relisted",(res as any).relisted);}
