/** 📸️ Regenerates the language-agnostic help snapshot of the operations catalogue from a tree whose catalogue was edited; usage: bun snap-help.ts <root> <out.json> */
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const [root,out]=process.argv.slice(2),base=join(root!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations"),help=await import(join(base,"📖️help/🟦️.ts")),catalog=JSON.parse(readFileSync(join(base,"🔣️catalog.json"),"utf8")),old=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/📖️help/🔣️.json"),"utf8"));
const entry=(name:string)=>name==="root"?[]:name.split(" "),rows=["root",...catalog.operations.flatMap((row:any)=>[row.name,...(row.subcommands??[]).map((sub:any)=>row.name+" "+sub.name)])];
const snapshots=Object.fromEntries(catalog.languages.map((language:string)=>[language,Object.fromEntries(rows.map((row:string)=>[row,help.renderBuildHelpV1(catalog,help.readBuildHelpRequestV1(catalog,entry(row)),language).split("\n")]))]));
writeFileSync(out!,JSON.stringify({...old,snapshots},null,2)+"\n");
console.log(JSON.stringify({languages:Object.keys(snapshots),rows:rows.length}));
