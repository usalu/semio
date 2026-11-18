import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync,brotliCompressSync,constants}from"node:zlib";import{join}from"node:path";
const cas=join(process.argv[2],"v1/main/cas");let j;
for(const d of readdirSync(cas))for(const f of readdirSync(join(cas,d))){if(!f.startsWith("356e"))continue;j=JSON.parse(brotliDecompressSync(readFileSync(join(cas,d,f))).toString());}
const q=b=>brotliCompressSync(Buffer.from(b),{params:{[constants.BROTLI_PARAM_QUALITY]:5,[constants.BROTLI_PARAM_LGWIN]:22,[constants.BROTLI_PARAM_SIZE_HINT]:b.length}}).length;
const t0=performance.now();
const keyOf=r=>JSON.stringify([r.kind,r.path,r.fact===undefined?null:r.fact]);
const ids=new Map(),table=[];const idx=l=>l.map(r=>{const k=keyOf(r);let i=ids.get(k);if(i===undefined){i=table.length;ids.set(k,i);table.push(r)}return i});
const rows=idx(j.rows),layers=j.layers.map(idx);
console.log("unique",table.length,"of",j.layers.flat().length);
// sort table
const order=table.map((r,i)=>i).sort((a,b)=>{const x=table[a],y=table[b];return x.path<y.path?-1:x.path>y.path?1:keyOf(x)<keyOf(y)?-1:1});
const remap=new Array(table.length);order.forEach((o,n)=>remap[o]=n);
let prev="";const enc=order.map(o=>{const r=table[o];let p=0;while(p<prev.length&&p<r.path.length&&prev[p]===r.path[p])p++;prev=r.path;return[r.kind,p,r.path.slice(p),r.fact===undefined?null:r.fact]});
// layers as delta sorted ids
const lay=layers.map(l=>{const s=l.map(i=>remap[i]).sort((a,b)=>a-b);const out=[];let pv=0;for(const v of s){out.push(v-pv);pv=v}return out});
const doc=JSON.stringify({v:2,recipe:j.recipe,value:j.value,sidecar:j.sidecar,rows:rows.map(i=>remap[i]),table:enc,layers:lay});
console.log("doc raw",doc.length,"packed",q(Buffer.from(doc)),"ms",Math.round(performance.now()-t0));
console.log("table",JSON.stringify(enc).length,"layers",JSON.stringify(lay).length,"value",JSON.stringify(j.value).length,"sidecar",JSON.stringify(j.sidecar).length);
// layer order relevance: do layers hold rows in order significant? check sorted equals original sorted? 
let ordered=0;for(const l of layers){}
// order-preserving ids with first-seen table
{
 const ids2=new Map(),tab=[];const idx2=l=>l.map(r=>{const k=keyOf(r);let i=ids2.get(k);if(i===undefined){i=tab.length;ids2.set(k,i);tab.push(r)}return i});
 const rows2=idx2(j.rows),layers2=j.layers.map(idx2);
 let prev="";const enc2=tab.map(r=>{let p=0;while(p<prev.length&&p<r.path.length&&prev[p]===r.path[p])p++;prev=r.path;return[r.kind,p,r.path.slice(p),r.fact===undefined?null:r.fact]});
 const lay2=layers2.map(l=>{let pv=-1;return l.map(v=>{const d=v-pv-1;pv=v;return d})});
 // pool
 const count=new Map();const walk=(v,f)=>{if(typeof v==="string"){if(v.length>=64)f(v)}else if(Array.isArray(v))v.forEach(x=>walk(x,f));else if(v&&typeof v==="object")Object.values(v).forEach(x=>walk(x,f))};
 walk(j.sidecar,s=>count.set(s,(count.get(s)||0)+1));
 const pool=[...count].filter(([,n])=>n>1).map(([s])=>s),pi=new Map(pool.map((s,i)=>[s,i]));
 const sub=v=>typeof v==="string"?(pi.has(v)?{$:pi.get(v)}:v):Array.isArray(v)?v.map(sub):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,x])=>[k,sub(x)])):v;
 const side=sub(j.sidecar);
 const doc=JSON.stringify({v:2,recipe:j.recipe,value:j.value,rows:rows2,table:enc2,layers:lay2,pool,sidecar:side});
 console.log("v2 doc raw",doc.length,"packed",q(Buffer.from(doc)),"sidecar",JSON.stringify(side).length,"pool",pool.length,JSON.stringify(pool).length,"layers",JSON.stringify(lay2).length,"table",JSON.stringify(enc2).length);
}
{
 const ids2=new Map(),tab=[];const idx2=l=>l.map(r=>{const k=keyOf(r);let i=ids2.get(k);if(i===undefined){i=tab.length;ids2.set(k,i);tab.push(r)}return i});
 const rows2=idx2(j.rows),layers2=j.layers.map(idx2);
 const count=new Map();const walk=(v,f)=>{if(typeof v==="string"){if(v.length>=48)f(v)}else if(Array.isArray(v))v.forEach(x=>walk(x,f));else if(v&&typeof v==="object")Object.values(v).forEach(x=>walk(x,f))};
 walk(j.sidecar,s=>count.set(s,(count.get(s)||0)+1));tab.forEach(r=>walk(r.fact,s=>count.set(s,(count.get(s)||0)+1)));
 const pool=[...count].filter(([,n])=>n>1).map(([s])=>s),pi=new Map(pool.map((s,i)=>[s,i]));
 const sub=v=>typeof v==="string"?(pi.has(v)?{$:pi.get(v)}:v):Array.isArray(v)?v.map(sub):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,x])=>[k,sub(x)])):v;
 let prev="";const enc2=tab.map(r=>{let p=0;while(p<prev.length&&p<r.path.length&&prev[p]===r.path[p])p++;prev=r.path;return[r.kind,p,r.path.slice(p),r.fact===undefined?null:sub(r.fact)]});
 const lay2=layers2.map(l=>{let pv=-1;return l.map(v=>{const d=v-pv-1;pv=v;return d})});
 const doc=JSON.stringify({v:2,recipe:j.recipe,value:j.value,rows:rows2,table:enc2,layers:lay2,pool,sidecar:sub(j.sidecar)});
 console.log("v3 doc raw",doc.length,"packed",q(Buffer.from(doc)),"table",JSON.stringify(enc2).length,"pool",pool.length);
 const kinds={};for(const e of enc2){const k=e[0];kinds[k]=(kinds[k]||0)+JSON.stringify(e).length}console.log(JSON.stringify(kinds));
}
