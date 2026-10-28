// [mem2] Read-only probe of what loading the current graph generation costs a process (not a library file): pointer + manifest, packed fact index (open, then every row touched as verification does), frontier, every owner fragment; RSS, ru_maxrss, JSC heap live after a collection, macOS physical footprint per step. usage: bun genprobe.ts <library root (mirror)> <generations dir> [steps: manifest,index,rows,release,frontier,fragments]
import{readFileSync,readdirSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
import{execFileSync}from"node:child_process";
import{heapStats}from"bun:jsc";
const[lib,dir,stepList]=process.argv.slice(2) as [string,string,string?],steps=new Set((stepList??"manifest,index,rows,frontier,fragments").split(","));
const G=await import(lib+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const unit:Record<string,number>={B:1/1048576,KB:1/1024,MB:1,GB:1024},rows:unknown[]=[],t0=performance.now();
const mark=(label:string,extra:Record<string,unknown>={})=>{const before=process.memoryUsage().rss;Bun.gc(true);const h=heapStats(),text=execFileSync("/usr/bin/footprint",["-p",String(process.pid)]).toString(),num=(re:RegExp)=>{const x=re.exec(text);return x?Math.round(Number(x[1])*unit[x[2]!]!):undefined;};const row={label,s:Math.round(performance.now()-t0)/1000,rss:Math.round(before/1048576),maxRss:Math.round(process.resourceUsage().maxRSS/1024),live:Math.round((h.heapSize+h.extraMemorySize)/1048576),fp:num(/phys_footprint: ([\d.]+) (B|KB|MB|GB)/),fpPeak:num(/phys_footprint_peak: ([\d.]+) (B|KB|MB|GB)/),...extra};rows.push(row);console.log("[DEBUG] GEN "+JSON.stringify(row));};
mark("start");
const pointer=JSON.parse(readFileSync(join(dir,"current.json"),"utf8")),manifest=steps.has("manifest")?JSON.parse(readFileSync(join(dir,"m",pointer.manifest+".json"),"utf8")):undefined;
mark("manifest",{generation:pointer.generation,owners:manifest?Object.keys(manifest.owners).length:0});
let index:{rows:{length:number;[n:number]:{path:string}}}|undefined;
if(steps.has("index")){const bytes=readFileSync(join(dir,"x",manifest.factIndex+".facts.br"));index=G.openPackedFactIndexV1(bytes,1<<30);mark("index-open",{packedMB:Math.round(bytes.length/1048576*10)/10,decompressedMB:Math.round(brotliDecompressSync(bytes).length/1048576*10)/10});}
if(index&&steps.has("rows")){let n=0,chars=0;for(let i=0;i<index.rows.length;i++){const r=index.rows[i]!;n++;chars+=r.path.length;}mark("index-rows",{rows:n,pathChars:chars});}
if(index&&steps.has("release")){index=undefined;mark("index-released");}
let frontier:unknown;if(steps.has("frontier")){frontier=JSON.parse(brotliDecompressSync(readFileSync(join(dir,"f",manifest.frontier+".json.br"))).toString());mark("frontier");}
const fragments:unknown[]=[];if(steps.has("fragments")){for(const name of readdirSync(join(dir,"o")))fragments.push(JSON.parse(brotliDecompressSync(readFileSync(join(dir,"o",name))).toString()));mark("fragments",{fragments:fragments.length});}
console.log("[DEBUG] GENKEEP "+[manifest,index,frontier,fragments].filter(Boolean).length);
