import {lstatSync} from "node:fs";
import {lstat,open} from "node:fs/promises";
import {createHash} from "node:crypto";
import {join,dirname,relative,sep} from "node:path";
const base="/Users/ueli/Documents/semio/.cache/b/t/m/sdk/e921066ebbb960dda44a0bc1564a6691b8757dae055b440ccce291e9c918f62e";
const index=JSON.parse(await (await import("node:fs/promises")).readFile(join(base,".semio-sdk-source.json"),"utf8"));
const entries=index.entries;
const root="/Users/ueli/Documents/semio/.cache/b/t/m";
const prefix=[];let p=root;for(const part of ["",...relative(root,base).split(sep).filter(Boolean)]){if(part)p=join(p,part);prefix.push(p)}
const full=[];{let q="/";for(const c of root.split("/").filter(Boolean)){q=join(q,c);full.push(q)}}
const all=[...full.slice(0,-0||undefined),...prefix.slice(1)];
const t=async(name,f)=>{const s=performance.now();await f();console.log(name,Math.round(performance.now()-s),"ms")};
await t("A rootguard sync x entries ("+all.length+" lstatSync each)",async()=>{for(let i=0;i<entries.length;i++)for(const q of all)lstatSync(q,{bigint:true})});
await t("B ancestors async per entry",async()=>{for(const e of entries){const parts=e.path.split("/");for(let i=1;i<parts.length;i++)await lstat(join(base,parts.slice(0,i).join("/")),{bigint:true})}});
await t("C lstat entry only",async()=>{for(const e of entries)await lstat(join(base,e.path),{bigint:true})});
await t("D hash all files (open/stat/read/stat/close)",async()=>{for(const e of entries){if(e.kind!=="file")continue;const f=await open(join(base,e.path),"r");try{const st=await f.stat({bigint:true});const b=Buffer.alloc(65536);let off=0;const h=createHash("sha256");while(off<Number(st.size)){const {bytesRead}=await f.read(b,0,Math.min(65536,Number(st.size)-off),off);h.update(b.subarray(0,bytesRead));off+=bytesRead;}await f.stat({bigint:true});h.digest("hex")}finally{await f.close()}}});
console.log("entries",entries.length,"load");
