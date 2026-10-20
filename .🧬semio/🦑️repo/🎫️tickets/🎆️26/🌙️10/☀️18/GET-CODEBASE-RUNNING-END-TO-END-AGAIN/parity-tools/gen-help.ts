import {readFileSync,writeFileSync} from "node:fs";
import {readBuildHelpRequestV1,renderBuildHelpV1,type BuildHelpCatalogV1} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/📖️help/🟦️.ts";
const help=JSON.parse(readFileSync(process.argv[2]!,"utf8")) as BuildHelpCatalogV1,entry=(name:string)=>name==="root"?[]:name.split(" "),rows=["root",...help.operations.flatMap(row=>[row.name,...(row.subcommands??[]).map(sub=>row.name+" "+sub.name)])];
const snapshots:Record<string,Record<string,string[]>>={};
for(const language of help.languages){snapshots[language]={};for(const row of rows)snapshots[language]![row]=renderBuildHelpV1(help,readBuildHelpRequestV1(help,entry(row)),language).split("\n");}
const text=JSON.stringify({version:1,snapshots},null,2)+"\n",target=process.argv[3]!;
if(process.argv[4]==="--check")console.log(readFileSync(target,"utf8")===text?"identical":"differs");else writeFileSync(target,text);
