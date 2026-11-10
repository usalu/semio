import {readFileSync,writeFileSync} from "node:fs";
import {renderBuildHelpV1,readBuildHelpRequestV1,type BuildHelpCatalogV1} from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/📖️help/🟦️.ts";
const dir="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/",catalog=JSON.parse(readFileSync(dir+"🔣️catalog.json","utf8")) as BuildHelpCatalogV1;
const path=dir+"🧪️tests/🧫️fixtures/📖️help/🔣️.json",old=JSON.parse(readFileSync(path,"utf8")),entry=(n:string)=>n==="root"?[]:n.split(" ");
const rows=["root",...catalog.operations.flatMap((r:any)=>[r.name,...(r.subcommands??[]).map((s:any)=>r.name+" "+s.name)])];
const changed:string[]=[];
for(const language of Object.keys(old.snapshots)){
 const next:Record<string,string[]>={};
 for(const row of rows){next[row]=renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,entry(row)),language as "en").split("\n");if(JSON.stringify(next[row])!==JSON.stringify(old.snapshots[language][row]))changed.push(language+":"+row);}
 old.snapshots[language]=next;
}
console.log(changed.join("\n"));
writeFileSync(path,JSON.stringify(old,null,2)+"\n");
