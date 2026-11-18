import {mkdirSync,mkdtempSync,rmSync,statSync,utimesSync,writeFileSync,openSync,writeSync,closeSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {withArtifactRelativeDirectoryV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
const which=process.argv[2]!,scopes=Number(process.argv[3]??4),members=Number(process.argv[4]??50),waitMs=Number(process.argv[5]??6000);
const {probeVerify}=await import(`./${which}.probe.ts`);
const base=mkdtempSync(join(tmpdir(),"r99p-"));
const roots:string[]=[],files:{root:number;path:string;bytes:number;sha256:string}[]=[],dirs=new Map<string,string>();
const anchor=(p:string)=>{const s=statSync(p,{bigint:true});return [s.dev,s.ino,s.mode].join(":");};
const put=(root:number,rel:string,text:string)=>{const full=join(base,roots[root]!,rel);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,text);files.push({root,path:rel,bytes:Buffer.byteLength(text),sha256:createHash("sha256").update(text).digest("hex")});};
for(let s=0;s<scopes;s++){roots.push(`roles/g${String(s).padStart(20,"0")}`);put(s,"package.json",`{"s":${s}}`);put(s,"bun.lock",`lock ${s}`);for(let m=0;m<members;m++)put(s,`w/${m}/package.json`,`{"s":${s},"m":${m}}`);}
const all=new Set<string>(["roles"]);for(const r of roots){const parts=r.split("/");for(let d=1;d<=parts.length;d++)all.add(parts.slice(0,d).join("/"));for(const f of files.filter(f=>roots[f.root]===r)){const p=f.path.split("/");p.pop();for(let d=1;d<=p.length;d++)all.add(r+"/"+p.slice(0,d).join("/"));}}
for(const d of all)dirs.set(d,anchor(join(base,d)));
const manifest={roots,files},stamps=new Map<string,string>();
let publishes=0;const control={publish:async()=>{publishes++;},yieldContinuation:async()=>{}},check=()=>{};
const run=async(label:string,args:readonly unknown[])=>{publishes=0;const t=performance.now();let err="";try{await withArtifactRelativeDirectoryV1(base,check,async root=>{await (probeVerify as Function)(root,manifest,dirs,stamps,check,control,...args);});}catch(e){err=(e as Error).message;}console.log(`${which.padEnd(3)} ${label.padEnd(26)} files ${files.length} publishes ${String(publishes).padStart(6)} ${Math.round(performance.now()-t).toString().padStart(5)} ms ${err?"THROWS "+err:"ok"}`);return err;};
await run("record",[true]);
await new Promise(r=>setTimeout(r,waitMs));
await run("fresh all",[false,true]);
await run("changed",[false]);
await run("changed again",[false]);
await run("root 1",[false,true,1]);
const victim=join(base,roots[2]!,"w/3/package.json"),before=statSync(victim);
const text=Buffer.from(`{"s":2,"m":3}`);text[3]=0x58;const fd=openSync(victim,"r+");writeSync(fd,text,0,text.length,0);closeSync(fd);utimesSync(victim,before.atime,before.mtime);
const e1=await run("tampered, changed",[false]);const e2=await run("tampered, root 2",[false,true,2]);const e3=await run("tampered, root 1",[false,true,1]);const e4=await run("tampered, all",[false,true]);
console.log("tamper caught by changed/root2/root1/all:",!!e1,!!e2,!!e3,!!e4);
rmSync(base,{recursive:true,force:true});
