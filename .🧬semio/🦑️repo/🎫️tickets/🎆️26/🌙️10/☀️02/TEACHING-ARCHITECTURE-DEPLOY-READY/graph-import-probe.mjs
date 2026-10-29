/** 🔬️ Bounds a read-only literal runtime import walk without running project inference. */
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
const workspace=process.cwd(),base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",load=file=>import(pathToFileURL(path.join(workspace,file))),parsers=await load(base+"/⚡️caching/📥️inference/🟨️.mjs"),router=await load(base+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🟨️.mjs");
for(const [entry,command] of [["🧰️framework/📦️packages/🦀️rust/📜️script.ts","generate"],["🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts","test"]]){
 const started=performance.now(),pending=[{file:path.join(workspace,entry),depth:0}],seen=new Set();let bytes=0,maximumDepth=0,missing=0,limited=false;
 while(pending.length){
  if(seen.size>=600||performance.now()-started>1500){limited=true;break;}
  const {file,depth}=pending.shift();if(seen.has(file)||!file.startsWith(workspace+path.sep)||!fs.existsSync(file))continue;
  seen.add(file);maximumDepth=Math.max(maximumDepth,depth);if(!/\.[cm]?[jt]sx?$/.test(file))continue;
  const source=fs.readFileSync(file,"utf8");bytes+=Buffer.byteLength(source);if(Buffer.byteLength(source)>524288){limited=true;break;}
  const selected=file===path.join(workspace,entry)?router.routerCommandSpecifiersV1(ts,file,source,command):undefined;
  for(const imported of parsers.commandSourceImports(file,source)){if(selected&&!selected.has(imported))continue;const resolved=path.resolve(path.dirname(file),imported);if(!fs.existsSync(resolved)){missing++;continue;}pending.push({file:resolved,depth:depth+1});}
 }
 console.log(JSON.stringify({entry,command,milliseconds:Math.round(performance.now()-started),files:seen.size,bytes,maximumDepth,pending:pending.length,missing,limited}));
}
