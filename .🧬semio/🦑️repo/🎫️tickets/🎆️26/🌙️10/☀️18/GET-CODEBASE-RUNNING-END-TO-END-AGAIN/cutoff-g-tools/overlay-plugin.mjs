// bun --preload: overlay of file contents without touching the disk. Repository modules that import node:fs / node:fs/promises get shims that answer overlaid paths from memory.
import{readFileSync,writeFileSync}from"node:fs";import{dirname,join}from"node:path";import{fileURLToPath}from"node:url";
const here=dirname(fileURLToPath(import.meta.url)),contents=new Map();let epoch=1;
globalThis.__overlay={get:p=>contents.get(p),set(map){contents.clear();for(const[p,b]of map)contents.set(p,b);epoch++;},epoch:()=>epoch,size:()=>contents.size,calls:{read:0}};
const fsShim=join(here,"overlay-fs-shim.mjs"),fspShim=join(here,"overlay-fsp-shim.mjs"),ROOT="/Users/ueli/Documents/semio/";
Bun.plugin({name:"overlay-fs",setup(build){build.onLoad({filter:/^\/Users\/ueli\/Documents\/semio\/(?!\.cache\/)(?!.*\/node_modules\/)(?!.*cutoff-g-tools).*\.(mjs|ts)$/},args=>{
 const loader=args.path.endsWith(".ts")?"ts":"js";
 let text=readFileSync(args.path,"utf8");
 if(!/["'](?:node:)?fs(?:\/promises)?["']/.test(text))return{contents:text,loader};
 const out=text.replace(/(["'])(?:node:)?fs\/promises\1/g,(_m,q)=>q+fspShim+q).replace(/(["'])(?:node:)?fs\1/g,(_m,q)=>q+fsShim+q);
 return{contents:out,loader};
});}});

// OVERLAY_PERTURB=numbers|all: the sections of the data files declared in the revision schema as ignored are served with every number +1 (and, with `all`, every boolean flipped and every string suffixed)
if(process.env.OVERLAY_PERTURB){
 const lib=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",declaration=JSON.parse(readFileSync(lib+"🏗️build/🕸️graph/📥️inference/♻️revision/🧬️schema/🔣️.json","utf8")).definitions.Sections.const.files;
 const mode=process.env.OVERLAY_PERTURB,bend=value=>Array.isArray(value)?value.map(bend):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([k,v])=>[k,bend(v)])):typeof value==="number"?value+1:mode==="all"&&typeof value==="boolean"?!value:mode==="all"&&typeof value==="string"?value+"~":value;
 const map=new Map();
 for(const[local,ignore]of Object.entries(declaration)){const document=JSON.parse(readFileSync(lib+local,"utf8"));for(const key of ignore)if(!(process.env.OVERLAY_PERTURB_SKIP??"").split(",").includes(key))document[key]=bend(document[key]);map.set(lib+local,Buffer.from(JSON.stringify(document,null,2)));}
 globalThis.__overlay.set(map);
}

// OVERLAY_TRACE=<out.json>: every top level section of the declared policy files that the process reads (through JSON.parse of the file text or through a JSON import) is written to <out.json> at exit; `*` marks an enumeration of all sections
if(process.env.OVERLAY_TRACE){
 const lib=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",files={"⚡️caching/🔣️policy.json":null,"🏗️build/🔣️policy.json":null},seen=new Set(),texts=new Map();
 for(const local of Object.keys(files))texts.set(readFileSync(lib+local,"utf8"),local);
 const wrap=(local,value)=>new Proxy(value,{get(target,key,receiver){if(typeof key==="string")seen.add(local+"\0"+key);return Reflect.get(target,key,receiver);},has(target,key){if(typeof key==="string")seen.add(local+"\0"+key);return Reflect.has(target,key);},ownKeys(target){seen.add(local+"\0*");return Reflect.ownKeys(target);}});
 const parse=JSON.parse;
 JSON.parse=function(text,...rest){const value=parse.call(this,text,...rest);if(typeof text==="string"&&text.length>100){const local=texts.get(text);if(local&&value&&typeof value==="object"&&!Array.isArray(value))return wrap(local,value);}return value;};
 Bun.plugin({name:"trace-json",setup(build){build.onLoad({filter:/\/(?:⚡️caching|🏗️build)\/🔣️policy\.json$/},args=>{const local=args.path.slice(lib.length);if(!(local in files))return{contents:readFileSync(args.path,"utf8"),loader:"json"};return{exports:{default:wrap(local,parse(readFileSync(args.path,"utf8")))},loader:"object"};});}});
 process.on("exit",()=>{try{writeFileSync(process.env.OVERLAY_TRACE,JSON.stringify([...seen].map(row=>row.split("\0")).sort()));}catch{}});
}
