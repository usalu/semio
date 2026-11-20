import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {selectRustSourceV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {inspectRustCompileReferences,rustTokens,rustTokenPairs} from "C:/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
const store="C:/semio/.cache/b/r/m/u/a",paths=new Set<string>();let n=0;
for(const d of readdirSync(store))for(const f of readdirSync(join(store,d)))if(f.endsWith(".json")&&n++<80){const m=JSON.parse(readFileSync(join(store,d,f),"utf8"));for(const s of m.stamps)if(s.present&&s.path.endsWith(".rs"))paths.add(s.path);}
let bytes=0,t=performance.now(),select=0,refs=0,tok=0;const texts=[...paths].map(p=>readFileSync(p,"utf8"));
for(const x of texts){bytes+=x.length;let a=performance.now();let sel=selectRustSourceV1(x,["unix","target_os=windows"],"f.rs");select+=performance.now()-a;a=performance.now();inspectRustCompileReferences(sel);refs+=performance.now()-a;a=performance.now();rustTokenPairs(rustTokens(sel));tok+=performance.now()-a;}
console.log({files:texts.length,MB:(bytes/1e6).toFixed(1),selectMs:Math.round(select),refsMs:Math.round(refs),tokMs:Math.round(tok),MBps:(bytes/1e6/((select+refs+tok)/1000)).toFixed(2)});
