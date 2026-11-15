import ts from "/Users/Shared/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync} from "node:fs";
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/";
for(const f of ["🟨️.d.mts","🟨️.mjs","🧪️tests/🧱️compaction-helpers/🟦️.ts"]){
 const text=readFileSync(process.argv[2]+"/"+L+f,"utf8");const name=f.endsWith(".mjs")?"a.mjs":f.endsWith(".d.mts")?"a.d.mts":"a.ts";const sf=ts.createSourceFile(name,text,ts.ScriptTarget.Latest,true);console.log(f,"parse diagnostics",(sf as any).parseDiagnostics.length);
}
