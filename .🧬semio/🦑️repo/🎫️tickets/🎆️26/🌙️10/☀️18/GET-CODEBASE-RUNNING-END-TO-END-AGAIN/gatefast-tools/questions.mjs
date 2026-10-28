/** ❔ Read-only census of the derived questions of one generation: `bun questions.mjs <root> [manifest-digest]` (default: the current pointer). Groups the derived rows by question kind (walk / derive / program), by view, by the underlying walk (question without `view`) and by holder count, so a cutoff that re-asks N questions can be compared with the number of distinct computations behind them. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,wanted]=process.argv.slice(2);
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),digest=wanted??cur.manifest;
const man=JSON.parse(readFileSync(join(store,"m",digest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const derived=new Map(),pseudo=new Set();
for(let i=0;i<idx.rows.length;i++)if(idx.rowKindOf(i)==="derived")derived.set(i,idx.rows[i]);
idx.owners.forEach((name,id)=>{if(name.startsWith("@derived:"))pseudo.add(id);});
const holders=new Map(),support=new Map();
for(const cls of idx.classes){
 const here=cls.r.filter(id=>derived.has(id));
 const pseudoHere=cls.o.filter(o=>pseudo.has(o));
 for(const o of pseudoHere)support.set(o,(support.get(o)??0)+cls.r.length);
 if(!here.length)continue;
 for(const o of cls.o)if(!pseudo.has(o))for(const id of here){let s=holders.get(id);if(!s)holders.set(id,s=new Set());s.add(idx.owners[o]);}
}
const kinds={},views={},walks=new Map(),hist={};
for(const[id,row]of derived){
 let q;try{q=JSON.parse(row.path);}catch{q={};}
 const kind=q.walk===1?"walk":q.derive===1?"derive":q.program===1?"program":"other";kinds[kind]=(kinds[kind]??0)+1;
 if(kind==="walk"){views[q.view??"(whole)"]=(views[q.view??"(whole)"]??0)+1;const{view,...rest}=q;const key=JSON.stringify(rest);const w=walks.get(key)??{views:[],holders:new Set(),roots:q.roots};w.views.push(q.view??"(whole)");for(const h of holders.get(id)??[])w.holders.add(h);walks.set(key,w);}
 const n=(holders.get(id)??new Set()).size,b=n===0?"0":n===1?"1":n<=4?"2-4":n<=16?"5-16":n<=64?"17-64":">64";hist[b]=(hist[b]??0)+1;
}
const perWalk={};for(const w of walks.values()){const k=w.views.length;perWalk[k]=(perWalk[k]??0)+1;}
const supports=[...support.values()].sort((a,b)=>a-b),median=supports[Math.floor(supports.length/2)]??0;
console.log(JSON.stringify({generation:cur.generation,manifest:digest.slice(0,12),rows:idx.rows.length,owners:idx.owners.length,derivedRows:derived.size,pseudoOwners:pseudo.size,kinds,views,distinctWalks:walks.size,viewsPerWalk:perWalk,holderHistogram:hist,supportRowsMedian:median,supportRowsMax:supports.at(-1)??0}));
const top=[...walks.values()].sort((a,b)=>b.holders.size-a.holders.size).slice(0,12);
for(const w of top)console.log(" walk",w.holders.size,"holders",w.views.join(","),JSON.stringify(w.roots).slice(0,180));
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
