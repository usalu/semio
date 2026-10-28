// [gram4-g] production-faithful simulation of the COLD builder tail on REAL data: the rows and owners of the sealed generation (read only) become (H1) the per-project observation arrays of the records, (H2) the part's `nodeRows` copies, then the real `shapeComputeResultV1` (real workspace root and owner roots, so the claim by path runs as in production), the release of H1/H2/part, then the real createFactIndexV1 over the shaped rows plus pack. Marks: rss / JS heap / extra / footprint. usage: bun sim2.mjs <generation dir> ; env VARIANT=base|nokeys, STAGES=...
import{readFileSync}from"node:fs";import{join}from"node:path";import{heapStats}from"bun:jsc";
const D=process.argv[2],V=process.env.VARIANT??"base",L=(process.env.GRAM_ROOT??"/Users/ueli/Documents/semio")+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=process.env.GRAM_WS??"/Users/ueli/Documents/semio";
const mb=n=>Math.round(n/1048576);
const fp=()=>{const o=String(Bun.spawnSync(["footprint","-p",String(process.pid)]).stdout),m=/Footprint:\s+(\d+)\s+(\w+)/.exec(o);return m?m[1]+m[2]:"?";};
let peak=0;const marks=[];const mark=label=>{if(process.env.MARK_GC==="1")Bun.gc(true);const m=process.memoryUsage(),h=heapStats();peak=Math.max(peak,m.rss);const row={label,rssMB:mb(m.rss),heapMB:mb(h.heapSize),extraMB:mb(h.extraMemorySize),fp:fp(),peakRssMB:mb(peak)};marks.push(row);console.log(JSON.stringify(row));};
const timer=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},100);
const g=await import(L+"🕸️graph/🕰️generations/🟨️.mjs"),load=await import(L+"🕸️graph/🕰️generations/🔌️load/🟨️.mjs");
const{rowKeyV1}=g;
mark("start");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
let index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
mark("index-decoded");
const projectNames=new Set(Object.keys(manifest.owners));
let h1=new Map(),extras=new Map();
for(const name of index.owners)(projectNames.has(name)?h1:extras).set(name,[]);
for(const c of index.classes){const rows=c.r.map(id=>index.rows[id]);for(const o of c.o){const name=index.owners[o],list=(h1.get(name)??extras.get(name));for(const row of rows)list.push(row);}}
let pairs=0;for(const l of h1.values())pairs+=l.length;
console.log(JSON.stringify({projects:h1.size,extras:extras.size,rows:index.rows.length,pairs}));
const distinct=index.rows.slice();index=null;Bun.gc(true);
mark("h1-expanded");
let part={owners:[...h1.keys()].map(name=>({name,root:manifest.owners[name].root,...(manifest.owners[name].nativeRoot?{nativeRoot:manifest.owners[name].nativeRoot}:{}),node:{targets:{}},nodeRows:[...h1.get(name)]})),edgeOnly:[],edges:[],rows:distinct,env:{digest:"x",rows:[]},externalNodes:{},parseUnavailable:[],replaceTags:[]};
mark("h2-part-built");
let shaped=load.shapeComputeResultV1({workspaceRoot:WS,part,stubs:{},implementation:{all:[],environment:[]}});
mark("shaped");
if(process.env.COMPARE_OUT){const{writeFileSync}=await import("node:fs"),{createHash}=await import("node:crypto");const h=createHash("sha256");for(const o of shaped.owners){h.update(o.name+"\n");for(const r of o.rows)h.update(rowKeyV1(r)+"\n");}writeFileSync(process.env.COMPARE_OUT,h.digest("hex")+" owners "+shaped.owners.length+"\n");}
h1=null;part=null;Bun.gc(true);mark("released");
const upserts=new Map();for(const o of shaped.owners)upserts.set(o.name,o.rows);for(const[n,l]of extras)upserts.set(n,l);
const rebuilt=g.createFactIndexV1(upserts);
mark("index-built");
if(process.env.DIGEST==="1"){const{createHash}=await import("node:crypto"),h=createHash("sha256");h.update(JSON.stringify(rebuilt.owners));for(const row of rebuilt.rows)h.update(rowKeyV1(row)+"\n");for(const cls of rebuilt.classes)h.update(JSON.stringify(cls)+"\n");console.log("INDEX-DIGEST",h.digest("hex"),"rows",rebuilt.rows.length,"classes",rebuilt.classes.length);}
const packed=g.packFactIndexV1(rebuilt,g.GENERATION_POLICY_V1.maximumIndexBytes);mark("packed");
clearInterval(timer);console.log(JSON.stringify({done:true,variant:V,peakRssMB:mb(peak),maxFootprint:Math.max(...marks.map(m=>parseInt(m.fp)||0))}));
