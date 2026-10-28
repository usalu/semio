/** 🧾 Read-only: for every dirty owner of a delta generation, the JSON paths at which its fragment differs from the parent's, aggregated over owners (path with array indices collapsed), plus the roster-chunk files added/removed for `SEMIO_SOURCE_CLOSURE_*` names. Usage: bun fragdiff.mjs <root> <child-manifest-digest> [owner...] */
import{readFileSync,existsSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,childDigest,...only]=process.argv.slice(2);
const store=join(root,".cache/b/g/m/generations"),man=d=>JSON.parse(readFileSync(join(store,"m",d+".json"),"utf8"));
const frag=d=>JSON.parse(brotliDecompressSync(readFileSync(join(store,"o",d+".json.br"))));
const child=man(childDigest),parent=man(child.parent);
const closures=join(root,".cache/b/g/m/build-input-closures");
const roster=key=>{const p=join(closures,key+".json");if(!existsSync(p))return undefined;const v=JSON.parse(readFileSync(p,"utf8"));return v.files??v;};
const diffs=new Map(),envAdded=new Map(),envRemoved=new Map(),owners=only.length?only:child.built.dirty;
const walk=(a,b,path,out)=>{if(JSON.stringify(a)===JSON.stringify(b))return;if(a&&b&&typeof a==="object"&&typeof b==="object"&&Array.isArray(a)===Array.isArray(b)&&!Array.isArray(a)){for(const k of new Set([...Object.keys(a),...Object.keys(b)]))walk(a[k],b[k],path+"."+k,out);return;}out.add(path);};
const envs=n=>{const s=new Set();JSON.stringify(n,(k,v)=>{if(typeof v==="string"&&/^SEMIO_SOURCE_CLOSURE_[0-9a-f]{64}$/.test(v))s.add(v.slice(21));return v;});return s;};
for(const name of owners){const a=parent.owners[name],b=child.owners[name];if(!a||!b)continue;const fa=frag(a.fragment),fb=frag(b.fragment),out=new Set();walk(fa,fb,"",out);for(const p of out){const k=p.replace(/\.targets\.[^.]+/,".targets.*").replace(/\.namedInputs\.[^.]+/,".namedInputs.*");diffs.set(k,(diffs.get(k)??0)+1);}
 const ea=envs(fa),eb=envs(fb);for(const k of eb)if(!ea.has(k))envAdded.set(k,(envAdded.get(k)??0)+1);for(const k of ea)if(!eb.has(k))envRemoved.set(k,(envRemoved.get(k)??0)+1);}
console.log("owners",owners.length);for(const[k,v]of[...diffs].sort((x,y)=>y[1]-x[1]).slice(0,30))console.log(" ",v,k);
const delta=new Map();
for(const[k]of envAdded){const r=roster(k);if(!r)continue;for(const[o]of envRemoved){const q=roster(o);if(!q)continue;const plus=r.filter(x=>!q.includes(x)),minus=q.filter(x=>!r.includes(x));if(plus.length+minus.length<6&&(plus.length||minus.length)){const key=JSON.stringify({plus,minus});delta.set(key,(delta.get(key)??0)+1);}}}
console.log("closure envs added",envAdded.size,"removed",envRemoved.size);
for(const[k,v]of[...delta].sort((x,y)=>y[1]-x[1]).slice(0,20))console.log(" ",v,k.slice(0,600));
