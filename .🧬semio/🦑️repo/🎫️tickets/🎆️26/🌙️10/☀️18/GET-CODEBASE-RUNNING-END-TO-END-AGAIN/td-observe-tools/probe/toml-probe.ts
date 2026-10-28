/** 🔬️ [td-observe] compares Bun.TOML, the portable owned parser and smol-toml over the two TOML corpora (native-manifests law + observation fixture). usage: bun toml-probe.ts <root> */
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
const root=process.argv[2]!,lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {parseTomlV1}=await import(root+"/🧰️framework/🔨️modules/⚙️runtime/🪶️portable/📄️toml/🟦️.ts"),smol=createRequire(lib+"/👁️observation/🧪️tests/x.ts")("smol-toml").parse;
const docs=[...JSON.parse(readFileSync(lib+"/🕸️graph/📥️inference/🦀️native-manifests/🧪️tests/📜️toml/🔣️.json","utf8")).documents.map((r:any)=>({name:r.name,text:r.text,expected:r.expected})),...JSON.parse(readFileSync(lib+"/👁️observation/🧪️tests/🧫️fixtures/🔣️.json","utf8")).tomlReceiving.cases.map((r:any)=>({name:"obs: "+r.name,text:r.source,expected:r.accepted?{path:r.path}:null}))];
const run=(f:any,t:string)=>{try{return{v:JSON.parse(JSON.stringify(f(t)))};}catch(e){return{e:String((e as Error).message).slice(0,80)};}};
const sub=(v:any,e:any)=>e===null?v.e!==undefined:v.e===undefined&&(e.path!==undefined&&Object.keys(e).length===1?v.v?.lib?.path===e.path:JSON.stringify(v.v)===JSON.stringify(e));
let bad=0;for(const d of docs){const b=run(Bun.TOML.parse,d.text),p=run(parseTomlV1,d.text),s=run(smol,d.text),ok=[sub(b,d.expected),sub(p,d.expected),sub(s,d.expected)];if(!ok[1])bad++;if(!ok.every(Boolean))console.log(JSON.stringify(d.name),"bun",ok[0],"portable",ok[1],p.e??"","smol",ok[2]);}
console.log("[DEBUG] documents",docs.length,"portable mismatches",bad);
