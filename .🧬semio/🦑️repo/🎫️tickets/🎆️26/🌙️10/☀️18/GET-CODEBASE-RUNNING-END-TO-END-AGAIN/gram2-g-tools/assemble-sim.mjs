// [gram2-g] graph-free simulation of the assembler's last stages on REAL data: the rows and owners of the sealed generation (read only) are expanded into per-owner row lists (what the builder holds after inference), then shape (claim), createFactIndexV1 and packFactIndexV1 run with rss/footprint marks. usage: bun assemble-sim.mjs <gen-dir> [stages=shape,index,pack]
import{readFileSync}from"node:fs";import{join}from"node:path";import{heapStats}from"bun:jsc";
const D=process.argv[2],STAGES=(process.argv[3]??"shape,index,pack").split(","),L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const mb=n=>Math.round(n/1048576);
const fp=()=>{const o=String(Bun.spawnSync(["footprint","-p",String(process.pid)]).stdout),m=/Footprint:\s+(\d+)\s+(\w+)/.exec(o);return m?m[1]+m[2]:"?";};
let peak=0;const mark=label=>{const m=process.memoryUsage(),h=heapStats();peak=Math.max(peak,m.rss);console.log(JSON.stringify({label,rssMB:mb(m.rss),heapMB:mb(h.heapSize),extraMB:mb(h.extraMemorySize),extMB:mb(m.external),fp:fp(),peakRssMB:mb(peak)}));};
globalThis.__m=mark;const timer=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},200);
const g=await import(L+"🕸️graph/🕰️generations/🟨️.mjs"),load=await import(L+"🕸️graph/🕰️generations/🔌️load/🟨️.mjs");

const {rowKeyV1,compareCodePointsV1}=g;
function createFactIndexSharedV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),lists=[],empty=[],byList=new Map();let seen=new Int32Array(1<<16);
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name),known=byList.get(list);
  if(known!==undefined){if(!known.length)empty.push({r:[],o:[owner]});lists.push(known);return;}
  const mine=new Uint32Array(list.length);let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;mine[count++]=id;}
  }
  if(!count)empty.push({r:[],o:[owner]});
  const done=mine.subarray(0,count);byList.set(list,done);lists.push(done);
 });
 globalThis.__m?.("fi:lists");
 const first=new Uint32Array(rows.length+1);
 for(const list of lists)for(const id of list)first[id+1]++;
 for(let id=0;id<rows.length;id++)first[id+1]+=first[id];
 const flat=new Uint32Array(first[rows.length]),fill=first.slice(0,rows.length);
 lists.forEach((list,owner)=>{for(const id of list)flat[fill[id]++]=owner;});
 globalThis.__m?.("fi:flat");
 const buckets=new Map(),classes=[];
 for(let id=0;id<rows.length;id++){
  const from=first[id],to=first[id+1];let hash=to-from;
  for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^flat[at])>>>0;
  let chain=buckets.get(hash),group;
  if(chain===undefined)buckets.set(hash,chain=[]);
  for(const candidate of chain){const other=candidate.span;if(other[1]-other[0]!==to-from)continue;let same=true;for(let at=0;at<to-from;at++)if(flat[other[0]+at]!==flat[from+at]){same=false;break;}if(same){group=candidate;break;}}
  if(group===undefined){group={r:[],o:Array.from(flat.subarray(from,to)),span:[from,to]};chain.push(group);classes.push(group);}
  group.r.push(id);
 }
 return{version:1,owners,rows,classes:[...classes.map(({r,o})=>({r,o})),...empty]};
}
mark("start");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
let index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
mark("index-decoded");
let ownerRows=new Map(index.owners.map(name=>[name,[]]));
if(process.env.SHARE!=="1"&&process.env.SEGMENTS!=="1")for(const c of index.classes){const rows=c.r.map(id=>index.rows[id]);for(const o of c.o){const list=ownerRows.get(index.owners[o]);for(const row of rows)list.push(row);}}
if(process.env.SHARE==="1"){const sig=new Map();const classOf=new Map();index.classes.forEach((c,i)=>{for(const o of c.o){let s=classOf.get(o);if(!s)classOf.set(o,s=[]);s.push(i);}});
 const shareList=new Map();ownerRows=new Map();for(const [o,s] of classOf){const key=s.join(",");let list=shareList.get(key);if(!list){list=[];for(const i of s)for(const id of index.classes[i].r)list.push(index.rows[id]);shareList.set(key,list);}ownerRows.set(index.owners[o],list);}
 for(const name of index.owners)if(!ownerRows.has(name))ownerRows.set(name,[]);console.log(JSON.stringify({distinctLists:shareList.size}));}
let ownerSegments;
if(process.env.SEGMENTS==="1"){const classArrays=index.classes.map(c=>c.r.map(id=>index.rows[id]));ownerSegments=new Map(index.owners.map(n=>[n,[]]));index.classes.forEach((c,i)=>{for(const o of c.o)ownerSegments.get(index.owners[o]).push(classArrays[i]);});ownerRows=new Map();console.log(JSON.stringify({segmentArrays:classArrays.length}));globalThis.__ref=process.env.COMPARE==="1"?index:null;}
let pairs=0;for(const list of ownerRows.values())pairs+=list.length;
console.log(JSON.stringify({owners:ownerRows.size,rows:index.rows.length,pairs}));
const ref=globalThis.__ref;index=null;Bun.gc(true);mark("owner-rows-expanded");
if(process.env.SEGMENTS==="1"){const {createSegmentedFactIndexV1}=await import("./segments.mjs");const built=createSegmentedFactIndexV1(ownerSegments,{rowKey:rowKeyV1,compare:compareCodePointsV1});mark("segmented-built");
 console.log(JSON.stringify({rows:built.rows.length,classes:built.classes.length,owners:built.owners.length}));
 if(ref){const consumers=(idx)=>{const sets=new Array(idx.rows.length);for(const c of idx.classes)for(const r of c.r)(sets[r]??=new Set()).add.apply?.call;const out=new Array(idx.rows.length).fill(null).map(()=>[]);for(const c of idx.classes)for(const r of c.r)for(const o of c.o)out[r].push(idx.owners[o]);return out.map(l=>[...new Set(l)].sort().join("\u0001"));};
  const a=consumers(built),b=consumers(ref),byRow=new Map(ref.rows.map((row,i)=>[row,i]));let bad=0;for(let i=0;i<built.rows.length;i++){const j=byRow.get(built.rows[i]);if(j===undefined||a[i]!==b[j])bad++;}console.log("CONSUMERS-DIFFER-ROWS",bad,"of",built.rows.length,"rows equal order",built.rows.every((r,i)=>r===ref.rows[i]));}
 if(false){const same=JSON.stringify(built.owners)===JSON.stringify(ref.owners)&&built.rows.length===ref.rows.length&&built.rows.every((r,i)=>r===ref.rows[i])&&JSON.stringify(built.classes)===JSON.stringify(ref.classes);console.log("EQUALS-ORIGINAL-INDEX",same);}
 clearInterval(timer);console.log(JSON.stringify({done:true,peakRssMB:mb(peak)}));process.exit(0);}
if(STAGES.includes("shape")){
 const names=[...ownerRows.keys()].filter(n=>!n.startsWith("@")),part={owners:names.map(name=>({name,root:name,node:{targets:{}},nodeRows:ownerRows.get(name)})),edgeOnly:[],edges:[],rows:[...new Set([...ownerRows.values()].flat())],env:{digest:"x",rows:[]},externalNodes:{},parseUnavailable:[],replaceTags:[]};
 mark("part-built");
 const shaped=load.shapeComputeResultV1({workspaceRoot:"/w",part,stubs:{},implementation:{all:[],environment:[]}});
 mark("shaped");globalThis.__keep=shaped;
}
let rebuilt;
if(STAGES.includes("index")){const owners=new Map();for(const [n,l] of ownerRows)owners.set(n,l);if(process.env.VARIANT==="chunked"){const {createFactIndexChunkedV1}=await import("./staged/index-chunks/chunked.mjs");rebuilt=createFactIndexChunkedV1(owners,{rowKeyV1,compareCodePointsV1});}else rebuilt=(process.env.VARIANT==="shared"?createFactIndexSharedV1:g.createFactIndexV1)(owners);mark("index-built");if(process.env.DIGEST==="1"){const {createHash}=await import("node:crypto"),h=createHash("sha256");h.update(JSON.stringify(rebuilt.owners));for(const row of rebuilt.rows)h.update(rowKeyV1(row)+"\n");for(const cls of rebuilt.classes)h.update(JSON.stringify(cls)+"\n");console.log("INDEX-DIGEST",h.digest("hex"),"rows",rebuilt.rows.length,"classes",rebuilt.classes.length);}}
if(STAGES.includes("patch")){const base=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);mark("patch-base");const upserts=new Map();for(const [n,l] of ownerRows)if(!n.startsWith("@"))upserts.set(n,l);const patched=g.patchFactIndexV1(base,{upserts});mark("patched");console.log(JSON.stringify({classes:patched.classes.length,rows:patched.rows.length}));}
if(STAGES.includes("pack")&&rebuilt){const bytes=g.packFactIndexV1(rebuilt,g.GENERATION_POLICY_V1.maximumIndexBytes);console.log("packed bytes",bytes.length);mark("packed");}
clearInterval(timer);console.log(JSON.stringify({done:true,peakRssMB:mb(peak)}));
