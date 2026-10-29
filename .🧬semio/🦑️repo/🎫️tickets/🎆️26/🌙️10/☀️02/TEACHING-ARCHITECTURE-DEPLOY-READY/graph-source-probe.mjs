/** 🔬️ Bounded read-only syntax and Rust source discovery probes for graph CPU diagnosis. */
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
const workspace=process.cwd(),base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",load=file=>import(pathToFileURL(path.join(workspace,file)));
const observations=await load(base+"/👁️observation/🟨️.mjs"),parsers=await load(base+"/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs"),native=await load(base+"/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs");
const parserSource=fs.readFileSync(path.join(workspace,base+"/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs"),"utf8"),parserBody=parserSource.slice(parserSource.indexOf("const RUST_RAW_PATTERN_V1"),parserSource.indexOf("/** 🪶️")),readReferences=new Function(parserBody+";return rustSourceReferences;")();
let started=0,files=0;
const analyze=(_file,bytes)=>{if(++files>250||performance.now()-started>1500)throw Error("Bounded diagnostic source budget reached");if(bytes.byteLength>524288)throw Error("Bounded diagnostic file size reached");return readReferences(bytes.toString("utf8"));};
const provider=observations.registerSourceAnalysisV1({id:"ticket:bounded-rust-source-query",revision:createHash("sha256").update(analyze.toString()).digest("hex"),analyze}),cache=parsers.createRustSourceCache();
native.wire({sourceAnalyses:{prepareSourceAnalysesV1:()=>({rust:provider}),RUST_SOURCE_CACHE:cache,RUST_INPUT_SNAPSHOTS:new WeakMap()},coordinates:{buildPath:value=>value.replaceAll("\\","/"),SCRIPT_BASENAME:"📜️script.ts"},targetPolicy:{}});
for(const root of ["🧰️framework/📦️packages/🦀️rust","🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"]){
 files=0;started=performance.now();
 const value=native.cargoSourceSetV1(root,workspace,new Map(),true,false);
 console.log(JSON.stringify({root,milliseconds:Math.round(performance.now()-started),parsedFiles:files,resolved:value.resolved,reason:value.reason??null,files:value.files?.length??null,message:value.message??null}));
}
