import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{types}from"node:util";
const policy={maximumDepth:64,maximumValues:1048576,maximumMembers:65536,maximumTextBytes:67108864,maximumReferences:65536};
const reference=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}$/u,prefix=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_/u;
function capture(value:any){
 let values=0,text=0;const active=new Set(),names=new Set(),charge=(value:string)=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
 const visit=(current:any,depth:number)=>{
  if(++values>policy.maximumValues||depth>policy.maximumDepth)throw Error("cap");
  if(typeof current==="string"){charge(current);return;}if(current===null||typeof current==="boolean"||typeof current==="number"&&Number.isFinite(current))return;
  if(typeof current!=="object"||types.isProxy(current)||active.has(current))throw Error("Inert acyclic inference DATA required");
  const array=Array.isArray(current),prototype=Object.getPrototypeOf(current);if(array?prototype!==Array.prototype:prototype!==Object.prototype&&prototype!==null)throw Error("Plain");const keys=Reflect.ownKeys(current);if(keys.length>policy.maximumMembers+(array?1:0)||keys.some(key=>typeof key!=="string"))throw Error("Bounded");if(array&&(current.length>policy.maximumMembers||keys.length!==current.length+1))throw Error("Dense");
  active.add(current);try{for(const key of keys){if(array&&key==="length")continue;if(array&&(!/^(?:0|[1-9][0-9]*)$/u.test(key)||Number(key)>=current.length))throw Error("Dense idx");const row=Object.getOwnPropertyDescriptor(current,key);if(!row||!("value"in row)||!row.enumerable)throw Error("Own");charge(key);if(key==="env"&&typeof row.value==="string"&&prefix.test(row.value)){if(!reference.test(row.value))throw Error("Closed");names.add(row.value);}visit(row.value,depth+1);}}finally{active.delete(current);}
 };visit(value,0);return[...names].sort();
}
const fast=(text:string)=>{const names=new Set<string>();for(const m of text.matchAll(/"env":"(SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64})"/gu))names.add(m[1]!);return[...names].sort();};
const dir="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np";
const files=readdirSync(dir).filter(n=>n.endsWith(".json")).slice(0,300);let tc=0,tf=0,tj=0,n=0,nodes=0,sizes:number[]=[];let mism=0;
for(const f of files){try{const t=brotliDecompressSync(readFileSync(dir+"/"+f)).toString("utf8");const rt=t.slice(t.indexOf("\n")+1);let a=performance.now();const v=JSON.parse(rt);tj+=performance.now()-a;a=performance.now();const names=capture(v);tc+=performance.now()-a;a=performance.now();const fr=fast(rt);tf+=performance.now()-a;if(JSON.stringify(fr)!==JSON.stringify(names))mism++;n++;sizes.push(rt.length);}catch(e){}}
console.log(JSON.stringify({n,meanBytes:sizes.reduce((a,b)=>a+b,0)/n,parseMs:tj/n,captureMs:tc/n,regexMs:tf/n,mismatch:mism}));
