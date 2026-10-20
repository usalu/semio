// regenerates the language-agnostic help snapshot from a catalog: bun regen-help.ts CATALOG.json HELP_MODULE.ts OUT.json
import {readFileSync,writeFileSync} from "node:fs";
const [catalogPath,modulePath,out]=process.argv.slice(2) as [string,string,string];
const catalog=JSON.parse(readFileSync(catalogPath,"utf8")),{renderBuildHelpV1,readBuildHelpRequestV1}=await import(modulePath);
const rows=["root",...catalog.operations.flatMap((row:any)=>[row.name,...(row.subcommands??[]).map((sub:any)=>row.name+" "+sub.name)])];
const snapshots:Record<string,Record<string,string[]>>={};
for(const language of catalog.languages){snapshots[language]={};for(const row of rows)snapshots[language]![row]=renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,row==="root"?[]:row.split(" ")),language).split("\n");}
writeFileSync(out,JSON.stringify({version:1,snapshots},null,2)+"\n");
