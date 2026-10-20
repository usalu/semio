// 🧠️ Scale harness of the graph build AFTER inference: S synthetic owners built from the real persisted project records of a bench run (values + observation rows), pushed through shape -> generation refresh -> assemble in one process with a heap poll per phase.
// env: S owners (default 3735), POOL dir of records (np), UNIQUE unique rows per owner (default 40), VARIANT=live|<ops.json>, SCRATCH, RSSMAX_MB, INTERN=1 interns value strings like the staged observation edit, FREE=1 drops node references after fragments are stored.
import {readFileSync,rmSync,mkdirSync,readdirSync,statSync,writeFileSync} from "node:fs";import {join,dirname} from "node:path";import {createHash} from "node:crypto";import {heapStats} from "bun:jsc";import {brotliDecompressSync} from "node:zlib";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),env=process.env;
const S=Number(env.S??"3735"),POOL=env.POOL??"/private/tmp/graph-mem/down100/np",UNIQUE=Number(env.UNIQUE??"40"),VARIANT=env.VARIANT??"live",SCRATCH=env.SCRATCH??"/private/tmp/graph-mem/scale",RSSMAX=Number(env.RSSMAX_MB??"3700")*1e6;
rmSync(SCRATCH,{recursive:true,force:true});mkdirSync(SCRATCH,{recursive:true});
type Op={anchor:string;replacement:string};const spec=VARIANT==="live"?{edits:{},newModules:{}}:JSON.parse(readFileSync(VARIANT,"utf8")),opsByPath:Record<string,Op[]>=spec.edits??{},newModules:Record<string,string>=spec.newModules??{};
const applyOps=(text:string,ops:Op[],path:string)=>{let out=text;for(const {anchor,replacement} of ops){if(out.includes(replacement)&&!out.includes(anchor))continue;if(out.split(anchor).length!==2)throw Error("anchor drift in "+path+": "+anchor.slice(0,70));out=out.replace(anchor,()=>replacement);}return out;};
Bun.plugin({name:"graph-mem-scale",setup(b){
 b.onResolve({filter:/\/🧠️[^/]*\/🟨️\.mjs$/},args=>{const hit=Object.keys(newModules).find(k=>args.path.endsWith(k.split("/").slice(-2).join("/")));return hit?{path:newModules[hit]}:undefined;});
 b.onLoad({filter:/(?:🕰️generations\/[^/]*|🔌️load\/[^/]*|observation\/[^/]*|project-nodes\/[^/]*)\.mjs$/},async args=>{const key=Object.keys(opsByPath).find(k=>args.path.endsWith(k));let t=await Bun.file(args.path).text();if(key)t=applyOps(t,opsByPath[key]!,key);return {contents:t,loader:"js"};});
}});
const generations:any=await import(join(L,"🏗️build/🕸️graph/🕰️generations/🟨️.mjs")),load:any=await import(join(L,"🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs"));
const observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const mb=(n:number)=>Math.round(n/1e6),used=()=>heapStats().heapSize+heapStats().extraMemorySize;
const pool:{value:any;rows:any[]}[]=[];
for(const f of readdirSync(POOL)){if(!f.endsWith(".json"))continue;const path=join(POOL,f);let header:any,value:any;try{header=observation.readInferenceHeaderV1(path);const t=brotliDecompressSync(readFileSync(path)).toString("utf8");value=JSON.parse(t.slice(t.indexOf("\n")+1));}catch{continue;}if(value?.failed||!value?.targets)continue;pool.push({value,rows:header.observations});}
if(!pool.length)throw Error("empty pool");
const sha=(s:string)=>createHash("sha256").update(s).digest("hex");
const memory:any=VARIANT==="live"?undefined:await import(Object.values(newModules)[0]!);const owned=memory?(v:any)=>memory.internJsonStringsV1(v,memory.graphStringInternerV1):(v:any)=>v;
const shared=new SharedArrayBuffer(16),cells=new Float64Array(shared);new Worker(new URL("./rss-worker.mjs",import.meta.url),{workerData:{buffer:shared,limit:RSSMAX}});
const phases:any[]=[];
const phase=async<T>(name:string,run:()=>Promise<T>|T):Promise<T>=>{const g0=performance.now();Bun.gc(true);const gcMs=Math.round(performance.now()-g0);const stats0=heapStats();const before=stats0.heapSize+stats0.extraMemorySize;cells[0]=process.memoryUsage().rss;const t=performance.now(),c=process.cpuUsage();const out=await run();const cpu=process.cpuUsage(c);phases.push({phase:name,gcBeforeMs:gcMs,objectsBefore:stats0.objectCount,ms:Math.round(performance.now()-t),cpuMs:Math.round((cpu.user+cpu.system)/1e3),beforeMB:mb(before),peakRssMB:mb(cells[0]),afterMB:mb(used()),rssMB:mb(process.memoryUsage().rss)});return out;};
const part:any=await phase("synthesize",()=>{
 const all=new Map<string,any>(),owners:any[]=[];
 for(let i=0;i<S;i++){
  const base=pool[i%pool.length]!,name=`syn-${i}`,root=`syn/${i}`;
  const value=owned(JSON.parse(JSON.stringify({...base.value,name,root})));
  const rows=base.rows.slice();
  for(let k=0;k<UNIQUE;k++){const path=`${W}/syn/${i}/f${k}.ts`;rows.push({kind:"file",path,fact:{size:k,mtimeMs:1,ctimeMs:1,ino:i*1000+k,sha256:sha(path)}});}
  for(const row of rows){const key=generations.rowKeyV1(row);if(!all.has(key))all.set(key,row);}
  owners.push({name,root,node:value,nodeRows:rows});
 }
 return {owners,edgeOnly:[],edges:[],externalNodes:{},parseUnavailable:[],rows:[...all.values()],env:{digest:"e".repeat(64),rows:[]},replaceTags:[]};
});
const distinct=part.rows.length;
const shaped:any=await phase("shape",()=>load.shapeComputeResultV1({workspaceRoot:W,part,stubs:{}}));
part.owners.length=0;part.rows=[];

if(env.MICRO==="1"){
 const ownerRows=new Map<string,any[]>(shaped.owners.map((o:any)=>[o.name,o.rows]));
 const index:any=await phase("micro-index",()=>generations.createFactIndexV1(ownerRows));
 const text:any=await phase("micro-canonical",()=>generations.canonicalJsonV1(index));
 console.log("MICRO "+JSON.stringify({indexJsonMB:mb(text.length),rows:index.rows.length,classes:index.classes.length}));
 const nodeText:any=await phase("micro-fragments",()=>{let bytes=0;for(const o of shaped.owners){bytes+=generations.canonicalJsonV1({version:1,name:o.name,node:o.node,edges:o.edges}).length;}return bytes;});
 console.log("MICRO fragments JSON MB "+mb(nodeText));
 if(env.STOP==="micro"){console.log(JSON.stringify({owners:S,distinctRows:distinct,phases}));process.exit(0);}
}
const frontier=generations.createFrontierV1({directories:[],members:[],capturedAt:0});
const engine=generations.createGenerationStoreV1({root:join(SCRATCH,"generations"),holds:async()=>true,resources:{facts:"facts",builder:"builder"}});
const result:any=await phase("refresh",()=>engine.refresh({identity:"a".repeat(64),policy:"fresh",membership:async()=>({digest:"b".repeat(64),frontier,changes:[]}),compute:async()=>shaped,signal:undefined,progress:undefined}));
const dirBytes=(d:string)=>{let n=0;try{for(const f of readdirSync(join(SCRATCH,"generations",d)))n+=statSync(join(SCRATCH,"generations",d,f)).size;}catch{}return n;};
const assembled:any=await phase("assemble",()=>engine.assemble(result.current,{compare:(a:string,b:string)=>a<b?-1:a>b?1:0}));
const projects=Object.keys(assembled.projects).length;

console.log(JSON.stringify({variant:VARIANT==="live"?"live":VARIANT.split("/").slice(-2).join("/"),owners:S,pool:pool.length,distinctRows:distinct,fragmentsMB:mb(dirBytes("o")),indexPackedMB:mb(dirBytes("x")),projectsAssembled:projects,peakRssMB:mb(process.memoryUsage().rss),phases}));
process.exit(0);
