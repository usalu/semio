// [obs-g] regenerates the help snapshot fixture of the operations catalogue in a workspace tree. usage: bun apply_help.ts <workspace dir>
import {readFileSync,writeFileSync} from "node:fs";
const W=process.argv[2]!.replace(/\/$/u,"")+"/",B=W+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/";
const {buildHelpForArgumentsV1,readBuildHelpRequestV1,renderBuildHelpV1}=await import(B+"📖️help/🟦️.ts");
const catalog=JSON.parse(readFileSync(B+"🔣️.catalog.json".replace("🔣️.catalog","🔣️catalog"),"utf8"));
const path=B+"🧪️tests/🧫️fixtures/📖️help/🔣️.json",current=JSON.parse(readFileSync(path,"utf8"));
const entry=(name:string)=>name==="root"?[]:name.split(" ");
const names=["root",...catalog.operations.flatMap((row:any)=>[row.name,...(row.subcommands??[]).map((sub:any)=>row.name+" "+sub.name)])];
const out:any={version:current.version,snapshots:{}};
for(const language of catalog.languages){out.snapshots[language]={};for(const name of names)out.snapshots[language][name]=renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,entry(name)),language).split("\n");}
let changed:string[]=[];for(const language of catalog.languages)for(const name of names)if(JSON.stringify(out.snapshots[language][name])!==JSON.stringify(current.snapshots[language]?.[name]))changed.push(language+":"+name);
writeFileSync(path,JSON.stringify(out,null,2)+"\n");console.log("changed snapshots:",changed.join(", "));
