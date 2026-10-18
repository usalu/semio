import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { SEMIO_SCHEMA_VENDOR_VOCABULARY_V1, semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
const R="/Users/ueli/Documents/semio";
const PRUNED = new Set(["node_modules", "target", "dist", "🗑️generated", "🤖️generated", ".git"]);
const files=(root:string):string[]=>readdirSync(root).flatMap((n)=>{ if(PRUNED.has(n))return[]; const p=join(root,n); return statSync(p).isDirectory()?files(p):n==="🔣️.json"?[p]:[]});
const ann=(node:any,f:[string,unknown][]=[])=>{ if(Array.isArray(node))for(const c of node)ann(c,f); else if(node&&typeof node==="object")for(const[k,c]of Object.entries(node)){if(k.startsWith("x-semio-"))f.push([k,c]);ann(c,f)} return f};
const ajv=semioSchemaAjvV1(); const declared=SEMIO_SCHEMA_VENDOR_VOCABULARY_V1.keywords as Record<string,object>;
for(const root of [R+"/🧰️framework/🛍️products/💻️os",R+"/🌎️hub"]) for(const p of files(root)){ let d; try{d=JSON.parse(readFileSync(p,"utf8"))}catch{continue} for(const[k,v]of ann(d)) if(k in declared && !ajv.validate(declared[k]!,v)) console.log(k,p.replace(R+"/",""),JSON.stringify(v).slice(0,200),JSON.stringify(ajv.errors).slice(0,300));}
