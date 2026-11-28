/** H91: rewrites the help snapshot rows of the staged tree that differ from the rendering of the staged catalog; run inside the mirror (bun regen-help-snapshot.ts <staged-fixture>). */
import{readFileSync,writeFileSync}from"node:fs";
import catalog from"/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H91/mirror/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"with{type:"json"};
import{readBuildHelpRequestV1,renderBuildHelpV1,type BuildHelpCatalogV1}from"/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H91/mirror/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/📖️help/🟦️.ts";
const path=process.argv[2]!,help=catalog as BuildHelpCatalogV1,document=JSON.parse(readFileSync(path,"utf8")) as {version:number;snapshots:Record<string,Record<string,string[]>>};
const rows=["root",...help.operations.flatMap(row=>[row.name,...(row.subcommands??[]).map(sub=>row.name+" "+sub.name)])],changed:string[]=[];
for(const language of help.languages){
 const next:Record<string,string[]>={};
 for(const row of rows){const lines=renderBuildHelpV1(help,readBuildHelpRequestV1(help,row==="root"?[]:row.split(" ")),language).split("\n");if(JSON.stringify(document.snapshots[language]![row])!==JSON.stringify(lines))changed.push(language+" "+row);next[row]=lines;}
 document.snapshots[language]=next;
}
writeFileSync(path,JSON.stringify(document,null,2)+"\n");
console.log(changed.join("\n"));
