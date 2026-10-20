// 🔬 Offline census over the persisted per-project inference records (np): which repo:script targets carry a command import closure, how large, and which carry none.
import {readFileSync,readdirSync,existsSync} from "node:fs";import {join} from "node:path";import {brotliDecompressSync} from "node:zlib";
const G="/Users/ueli/Documents/semio/.cache/b/g/m",NP=join(G,"inference/np"),CL=join(G,"build-input-closures");
const manifests=new Map<string,string[]|null>();
const manifest=(key:string)=>{if(!manifests.has(key)){const p=join(CL,key+".json");manifests.set(key,existsSync(p)?JSON.parse(readFileSync(p,"utf8")).files:null);}return manifests.get(key)!;};
const out:any[]=[];
for(const f of readdirSync(NP).filter(f=>f.endsWith(".json"))){
 const raw=readFileSync(join(NP,f));let text:string;try{text=brotliDecompressSync(raw).toString();}catch{text=raw.toString();}
 const line=text.split("\n")[1];if(!line)continue;let p:any;try{p=JSON.parse(line);}catch{continue;}if(!p.targets)continue;
 const expand=(inputs:any[],seen=new Set<string>()):any[]=>inputs.flatMap(x=>{
  if(typeof x==="string"&&Object.hasOwn(p.namedInputs??{},x)){if(seen.has(x))return[];seen.add(x);return expand(p.namedInputs[x],seen);}
  if(x&&typeof x==="object"&&"input"in x&&Object.hasOwn(p.namedInputs??{},x.input)&&!x.projects&&!x.dependencies){if(seen.has(x.input))return[];seen.add(x.input);return expand(p.namedInputs[x.input],seen);}
  return[x];});
 for(const [name,t] of Object.entries<any>(p.targets)){
  const ex=expand(t.inputs??[]),markers=ex.filter(x=>x&&typeof x==="object"&&typeof x.env==="string"&&x.env.startsWith("SEMIO_SOURCE_CLOSURE_")).map(x=>x.env.slice(21));
  const files=new Set<string>();let missing=0;for(const k of markers){const m=manifest(k);if(!m)missing++;else for(const x of m)files.add(x);}
  const lit=ex.filter(x=>typeof x==="string"&&/\.[cm]?[jt]sx?$/.test(x)&&!/[*?{]/.test(x)).length;
  const fallback=ex.some(x=>typeof x==="string"&&x.includes("/**/*.{ts,tsx,js,mjs,cjs,json}"));
  out.push({p:p.name,r:p.root,t:name,e:t.executor,c:t.cache??null,k:t.metadata?.semio?.cache?.class??null,kind:t.metadata?.semio?.execution?.kind??null,m:markers.length,n:files.size,missing,s:[...files].some(x=>x.endsWith("script.ts")),lit,fallback,cmd:(t.options?.command??"").slice(0,60)});
 }
}
console.log(JSON.stringify(out.length));
await Bun.write(process.argv[2]!,out.map(r=>JSON.stringify(r)).join("\n")+"\n");
