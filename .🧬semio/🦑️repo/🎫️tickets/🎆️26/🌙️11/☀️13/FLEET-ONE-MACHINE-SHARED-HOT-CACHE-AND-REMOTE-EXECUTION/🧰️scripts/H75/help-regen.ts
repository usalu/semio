import{readFileSync,writeFileSync,renameSync}from"node:fs";
import{join}from"node:path";
const B=process.argv[2]!,out=process.argv[3]!,catalogPath=process.argv[4]??join(B,"⚙️operations","🔣️catalog.json");
const{buildBackendIdsV1}=await import(join(B,"🔌️backends","🟦️.ts")),{readBuildHelpRequestV1,renderBuildHelpV1}=await import(join(B,"⚙️operations","📖️help","🟦️.ts"));
const help=JSON.parse(readFileSync(catalogPath,"utf8")),entry=(name:string)=>name==="root"?[]:name.split(" ");
const rows=["root",...help.operations.flatMap((row:any)=>[row.name,...(row.subcommands??[]).map((sub:any)=>row.name+" "+sub.name)])];
const snapshots:Record<string,Record<string,string[]>>={};
for(const language of help.languages){snapshots[language]={};for(const row of rows)snapshots[language]![row]=renderBuildHelpV1(help,readBuildHelpRequestV1(help,entry(row)),language).split("\n");}
const old=JSON.parse(readFileSync(join(B,"⚙️operations","🧪️tests","🧫️fixtures","📖️help","🔣️.json"),"utf8"));
const text=JSON.stringify({version:old.version,snapshots},null,2)+"\n";
writeFileSync(out+".tmp",text);renameSync(out+".tmp",out);
console.log("rows",rows.length,"languages",help.languages.length);
