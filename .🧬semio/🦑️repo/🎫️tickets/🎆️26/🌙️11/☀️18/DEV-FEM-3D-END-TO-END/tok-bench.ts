import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {rustTokens,rustTokenPairs,rustDelimiterFaultV1,inspectRustCompileReferences} from "C:/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
const store="C:/semio/.cache/b/r/m/u/a",paths=new Set<string>();let n=0;
for(const d of readdirSync(store))for(const f of readdirSync(join(store,d)))if(f.endsWith(".json")&&n++<80){const m=JSON.parse(readFileSync(join(store,d,f),"utf8"));for(const s of m.stamps)if(s.present&&s.path.endsWith(".rs"))paths.add(s.path);}
const texts=[...paths].map(p=>readFileSync(p,"utf8"));let bytes=0;texts.forEach(t=>bytes+=t.length);
const time=(name:string,f:(t:string)=>unknown)=>{const a=performance.now();for(const t of texts)f(t);console.log(name,Math.round(performance.now()-a),"ms");};
time("rustTokens",rustTokens);time("tokens+pairs",t=>rustTokenPairs(rustTokens(t)));time("delimiterFault",rustDelimiterFaultV1);time("inspectRefs",t=>{try{inspectRustCompileReferences(t,()=>undefined)}catch{}});console.log("MB",(bytes/1e6).toFixed(1));
