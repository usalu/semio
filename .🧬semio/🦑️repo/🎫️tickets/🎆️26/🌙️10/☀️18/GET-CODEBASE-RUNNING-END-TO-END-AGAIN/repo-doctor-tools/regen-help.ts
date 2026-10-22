import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";

const root=process.argv[2]??process.cwd(),check=process.argv.includes("--check"),B=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations");
const {readBuildHelpRequestV1,renderBuildHelpV1}=await import(pathToFileURL(join(B,"📖️help/🟦️.ts")).href);
const catalog=JSON.parse(readFileSync(join(B,"🔣️catalog.json"),"utf8")),path=join(B,"🧪️tests/🧫️fixtures/📖️help/🔣️.json"),previous=JSON.parse(readFileSync(path,"utf8"));
const rows=["root",...catalog.operations.flatMap((row:{name:string;subcommands?:{name:string}[]})=>[row.name,...(row.subcommands??[]).map(sub=>row.name+" "+sub.name)])];
const snapshots=Object.fromEntries(catalog.languages.map((language:string)=>[language,Object.fromEntries(rows.map(row=>[row,renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,row==="root"?[]:row.split(" ")),language).split("\n")]))]));
const text=JSON.stringify({version:previous.version,snapshots},null,2)+"\n";
if(check){if(text!==readFileSync(path,"utf8")){console.error("help snapshots differ from the catalogue");process.exit(1);}console.log("help snapshots match the catalogue");}
else{writeFileSync(path,text);console.log(`help snapshots written: ${rows.length} entries x ${catalog.languages.length} languages`);}
