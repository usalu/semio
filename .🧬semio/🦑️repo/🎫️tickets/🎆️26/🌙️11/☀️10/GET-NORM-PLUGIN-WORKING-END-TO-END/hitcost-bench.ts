import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import{readdirSync,statSync}from"node:fs";import{createHash}from"node:crypto";
const B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const{createClosureStoreV1}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const directory=process.argv[2],store=createClosureStoreV1({directory,readOnly:true});
const ents=readdirSync(directory+"/e").map(n=>[n,statSync(directory+"/e/"+n).size] as const).filter(x=>x[1]>20000&&x[1]<60000).slice(0,40);
const layers=readdirSync(directory+"/l").map(n=>[n,statSync(directory+"/l/"+n).size] as const).sort((a,b)=>b[1]-a[1]).slice(0,60).map(x=>x[0].replace(".json.br",""));
const sha=(t:string)=>createHash("sha256").update(t).digest("hex");
let t=performance.now(),loaded=0,layerCount=0;
for(const[n]of ents){const entry=JSON.parse((await import("node:zlib")).brotliDecompressSync(await Bun.file(directory+"/e/"+n).bytes()).toString("utf8"));const got=await store.load(entry.id);if(got){loaded++;layerCount+=got.digests.length;}}
const loadMs=performance.now()-t;t=performance.now();let rows=0,heap0=process.memoryUsage().heapUsed,peak=0;
for(const digest of layers)for(const batch of store.layerBatchesSync(digest,5000)){rows+=batch.length;}
const layerMs=performance.now()-t;
console.log(JSON.stringify({entriesLoaded:loaded,layersChecked:layerCount,loadMsPerEntry:+(loadMs/ents.length).toFixed(2),bigLayers:layers.length,layerRows:rows,layerMsPerLayer:+(layerMs/layers.length).toFixed(1)}));
