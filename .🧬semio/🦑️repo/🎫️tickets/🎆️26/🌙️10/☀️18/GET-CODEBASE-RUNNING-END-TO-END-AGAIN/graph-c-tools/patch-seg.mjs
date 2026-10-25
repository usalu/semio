import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const gen=await import(join(L,"🏗️build/🕸️graph/🕰️generations/🟨️.mjs"));
const P={compactionFactor:2,compactionFloor:1024};const compareCodePointsV1=gen.compareCodePointsV1;
const ownerIdMap_=index=>new Map(index.owners.map((name,id)=>[name,id]));const refine_=x=>x;
globalThis.__mem=l=>{const u=process.memoryUsage();console.log(l.padEnd(18),"rss",Math.round(u.rss/1048576),"heap",Math.round(u.heapUsed/1048576),Date.now()-T0,"ms");};
let T0=Date.now();
function patchFactIndexV1(index,{drop=[],upserts=new Map(),append=new Map()}){
 const ownerIds=new Map(ownerIdMap_(index)),rows=index.rows.slice(),rowIds=new Map(rows.map((row,id)=>[gen.rowKeyV1(row),id])),owners=index.owners.slice();
 const idOf=name=>{let id=ownerIds.get(name);if(id===undefined){id=owners.length;owners.push(name);ownerIds.set(name,id);}return id;};
 const touched=new Set(),gone=new Set();
 globalThis.__mem("after line 3");
 for(const name of drop)if(ownerIds.has(name)){touched.add(ownerIds.get(name));if(!upserts.has(name))gone.add(ownerIds.get(name));}
 for(const name of upserts.keys())touched.add(idOf(name));
 const classes=[];
 globalThis.__mem("after line 6");
 for(const cls of index.classes){const kept=cls.o.filter(id=>!touched.has(id));if(kept.length)classes.push({r:cls.r,o:kept});}
 const rowIdsFor=list=>{const out=new Set();for(const row of list){const key=gen.rowKeyV1(row);let id=rowIds.get(key);if(id===undefined){id=rows.length;rowIds.set(key,id);rows.push(row);}out.add(id);}return[...out].sort((a,b)=>a-b);};
 for(const name of[...upserts.keys()].sort(compareCodePointsV1))classes.push({r:rowIdsFor(upserts.get(name)),o:[ownerIds.get(name)]});
 globalThis.__mem("after line 9");
 for(const name of[...append.keys()].sort(compareCodePointsV1)){const extra=rowIdsFor(append.get(name));classes.push({r:extra,o:[idOf(name)]});}
 const used=new Set(),live=new Set();
 globalThis.__mem("after line 11");
 for(const cls of classes){for(const id of cls.r)used.add(id);for(const id of cls.o)if(!gone.has(id))live.add(id);}
 const rowMap=new Map(),ownerMap=new Map(),rowsOut=[],ownersOut=[];
 [...used].sort((a,b)=>a-b).forEach(id=>{rowMap.set(id,rowsOut.length);rowsOut.push(rows[id]);});
 globalThis.__mem("after line 14");
 [...live].sort((a,b)=>a-b).forEach(id=>{ownerMap.set(id,ownersOut.length);ownersOut.push(owners[id]);});
 globalThis.__mem("after line 15");
 const compact=classes.filter(cls=>cls.o.some(id=>live.has(id))).map(cls=>({r:cls.r.map(id=>rowMap.get(id)),o:cls.o.filter(id=>live.has(id)).map(id=>ownerMap.get(id))}));
 globalThis.__mem("after line 16");
 const result={version:1,owners:ownersOut,rows:rowsOut,classes:compact};
 globalThis.__mem("after line 17");
 return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor?refine_(result):result;
}

const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),m=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const index=JSON.parse(brotliDecompressSync(readFileSync(g+"x/"+m.factIndex+".json.br")).toString());
const N=600,ownerRows=new Map();index.classes.forEach(c=>{for(const o of c.o){const name=index.owners[o];if(ownerRows.size<N||ownerRows.has(name)){let l=ownerRows.get(name);if(!l)ownerRows.set(name,l=[]);for(const r of c.r)l.push(index.rows[r]);}}});
const clones=new Map();const clone=r=>{let c=clones.get(r);if(!c){c=structuredClone(r);clones.set(r,c);}return c;};const upserts=new Map([...ownerRows].map(([n,l])=>[n,l.map(clone)]));
Bun.gc(true);T0=Date.now();globalThis.__mem("start patch");
patchFactIndexV1(index,{drop:[...upserts.keys()],upserts,append:new Map()});globalThis.__mem("done");
