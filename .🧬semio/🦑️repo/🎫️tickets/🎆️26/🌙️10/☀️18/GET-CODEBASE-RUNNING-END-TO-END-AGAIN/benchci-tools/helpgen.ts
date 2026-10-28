/** 🧫️ Regenerates the help snapshot of the operations catalogue exactly the way the help law reads it (root, every operation, every subcommand, every language); run inside a mirror with the batch applied: `bun helpgen.ts <out.json>`. */
import {writeFileSync} from "node:fs";
import {renderBuildHelpV1,readBuildHelpRequestV1,type BuildHelpCatalogV1} from "@@HELP@@";
import catalog from "@@CATALOG@@" with {type:"json"};
const help=catalog as BuildHelpCatalogV1,entry=(name:string)=>name==="root"?[]:name.split(" ");
const rows=["root",...help.operations.flatMap(row=>[row.name,...(row.subcommands??[]).map(sub=>row.name+" "+sub.name)])],snapshots:Record<string,Record<string,string[]>>={};
for(const language of help.languages){snapshots[language]={};for(const row of rows)snapshots[language]![row]=renderBuildHelpV1(help,readBuildHelpRequestV1(help,entry(row)),language).split("\n");}
writeFileSync(process.argv[2]!,JSON.stringify({version:1,snapshots},null,2)+"\n");
