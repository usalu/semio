import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import{readdirSync,statSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const{createClosureStoreV1}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const obs=await import(B+"👁️observation/🟨️.mjs") as any;
const directory=process.argv[2],N=Number(process.argv[3]??30),store=createClosureStoreV1({directory,readOnly:true});
const names=readdirSync(directory+"/e").map(n=>[n,statSync(directory+"/e/"+n).mtimeMs] as const).sort((a,b)=>b[1]-a[1]).slice(0,3000).filter((_,i)=>i%37===0).map(x=>x[0]);
const picked:any[]=[];for(const n of names){const r=JSON.parse(brotliDecompressSync(readFileSync(directory+"/e/"+n)).toString("utf8"));if(r.layers.length>=1)picked.push(r.id);if(picked.length>=N)break;}
console.log("picked",picked.length);const{createHash}=await import("node:crypto");const ph:any={read:0,brotli:0,sha:0,parse:0,n:0,bytes:0,rows:0};const phase=(d:string)=>{let a=performance.now();const raw=readFileSync(directory+"/l/"+d+".json.br");ph.read+=performance.now()-a;a=performance.now();const b=brotliDecompressSync(raw);ph.brotli+=performance.now()-a;a=performance.now();createHash("sha256").update(b).digest("hex");ph.sha+=performance.now()-a;a=performance.now();const rows=JSON.parse(b.toString("utf8"));ph.parse+=performance.now()-a;ph.n++;ph.bytes+=b.length;ph.rows+=rows.length;};
const step=async()=>{};
const run=async(label:string)=>{
 const t={load:0,rows:0,layers:0},t0=performance.now();let layersRead=0;
 for(const id of picked){
  await obs.withObservationRoundV1(async()=>{
   let a=performance.now();const e=await store.load(id);t.load+=performance.now()-a;if(!e)return;
   a=performance.now();await obs.observationRowsHoldAsyncV1(e.rows,step);t.rows+=performance.now()-a;
   a=performance.now();const verdicts=new Map();
   for(const d of e.digests){let ok=true;layersRead++;phase(d);for(const batch of store.layerBatchesSync(d,5000))if(!await obs.observationLayerHoldsAsyncV1(batch,step)){ok=false;break;}}
   t.layers+=performance.now()-a;
  });
 }
 const total=performance.now()-t0;console.log(label,JSON.stringify({perEntryMs:+(total/picked.length).toFixed(0),loadMs:+(t.load/picked.length).toFixed(0),rowsMs:+(t.rows/picked.length).toFixed(0),layersMs:+(t.layers/picked.length).toFixed(0),layersPerEntry:layersRead/picked.length}));
};
await run("cold");console.log(JSON.stringify(ph));
