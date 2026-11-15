import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {join,relative,resolve,sep} from "node:path";
const M="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";
const {openReplicaV1}=await import(`${M}/🔁️replica/🟦️.ts`);
const {moduleSourcesV1,importClosureV1}=await import(`${M}/🛡️resilience/🟦️.ts`);
const S=process.argv[2]!;
const machine=JSON.parse(await readFile(`${S}/mir/.cache/f/machine.json`,"utf8"));
const replica=await openReplicaV1({worktree:`${S}/mir`,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
const root="/Users/Shared/semio";
const seal=JSON.parse(await readFile(`${root}/.cache/f/code/${process.argv[3]}/sealed.json`,"utf8"));
const before=new Map<string,string>(seal.files.map((f:any)=>[f.path,f.sha256]));
const sources=await moduleSourcesV1(M),paths=[...sources,...await importClosureV1(M,sources)].map((p:string)=>relative(root,resolve(M,...p.split("/"))).split(sep).join("/"));
const now=new Set(paths);
const info=(path:string)=>{const id=replica.book.find(path);if(id===undefined)return "unknown";return `${replica.book.origins[replica.files.origin[id]!]} ${replica.files.operationAt(id)} live=${replica.files.live(id)} hlc=${replica.files.entryAt(id).hlc}`;};
console.log("closure files",paths.length,"sealed",before.size);
for(const p of paths){const d=await readFile(join(root,p)).catch(()=>undefined);const h=d&&createHash("sha256").update(d).digest("hex");if(h!==before.get(p))console.log("CHANGED",p.replace(/^.*shared-worktree\//,""),"|",info(p));}
for(const p of before.keys())if(!now.has(p))console.log("GONE",p.replace(/^.*shared-worktree\//,""),"|",info(p));
process.exit(0);
