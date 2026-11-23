import {readFileSync} from "node:fs";
import {isAbsolute} from "node:path";
import {reachedFilesV1} from "../../../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts";

const USES=/\b(?:option_)?env!\s*\(\s*"(CARGO_MANIFEST_DIR|CARGO_MANIFEST_PATH|OUT_DIR)"\s*\)/gu;
const INCLUDING=/\binclude(?:_str|_bytes)?!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"(?:CARGO_MANIFEST_DIR|OUT_DIR)"\s*\)\s*,/gu;

/** 🔍️ How many places of one source text turn a manifest or output directory into a VALUE of the program (`env!("CARGO_MANIFEST_DIR")` that is not just the first part of an `include*!(concat!(...))`, whose result is a file, not a path). Such a value is baked into the compiled bytes: the machine's real directory. */
export function embeddedPathValuesV1(text:string):number{
 const uses=[...text.matchAll(USES)].length,including=[...text.matchAll(INCLUDING)].length;
 return Math.max(0,uses-including);
}

/** 🔍️ The real paths a unit's compiled bytes depend on, so the fleet key can include them: empty when no source file the compiler reads (the root source, its `mod` files, `#[path]` and `include!` targets) embeds a manifest or output directory as a value. A tree too large to scan, or an unreadable file, counts as embedding (the unit is then shared only between machines with the same paths, never wrongly). */
export function pathSensitiveV1(input:Readonly<{source:string;manifest:string|undefined;cwd:string}>,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):readonly string[]{
 const reached=reachedFilesV1(input.source,read,64,input.manifest),paths=[...new Set([input.manifest,input.cwd].filter((path):path is string=>path!==undefined&&isAbsolute(path)))].sort();
 if(reached===undefined)return paths;
 for(const file of [input.source,...reached.filter(path=>path.endsWith(".rs"))]){
  const text=read(file);if(text===undefined)return paths;
  if(embeddedPathValuesV1(text)>0)return paths;
 }
 return [];
}
