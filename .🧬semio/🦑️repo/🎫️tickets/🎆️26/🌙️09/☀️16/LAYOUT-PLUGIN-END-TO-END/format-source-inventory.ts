import {readFile,writeFile,mkdir} from "node:fs/promises";
import {resolve,dirname,relative} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const manifest="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml";
const text=await readFile(manifest,"utf8"),normal=text.split("[dependencies]")[1]!.split(/^[[]/m)[0]!,rows=[];
for(const line of normal.split("\n")){
 if(!line.startsWith("semio-s-artifact-stdio-"))continue;
 const path=line.match(/path\s*=\s*"([^"]+)"/)![1]!,file=resolve(dirname(manifest),path,"Cargo.toml"),source=await readFile(file,"utf8"),name=source.match(/^name\s*=\s*"([^"]+)"/m)![1]!,entry=source.split("[lib]")[1]!.split(/^[[]/m)[0]!.match(/^path\s*=\s*"([^"]+)"/m)![1]!;
 const defaults=source.split("[features]")[1]?.split(/^[[]/m)[0]?.match(/^default\s*=([^\n]+)/m)?.[1];if(defaults)throw Error("Inspect declared feature defaults before measuring "+name);
 const inputs=await readRustSourceClosureV1([relative(process.cwd(),resolve(dirname(file),entry))],process.cwd(),["target_arch=aarch64","target_os=macos","target_family=unix","unix"],async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 const row={name,manifest:relative(process.cwd(),file),cfgTest:false,defaultFeatures:[],files:inputs.length,bytes:inputs.reduce((n,x)=>n+x.bytes,0),lines:inputs.reduce((n,x)=>n+x.activeLines,0)};rows.push({...row,fits:row.files<=32&&row.bytes<=262144&&row.lines<=4000});
}
const output=resolve(import.meta.dirname,"🗑️generated/format-source-inventory.json");await mkdir(dirname(output),{recursive:true});await writeFile(output,JSON.stringify(rows,null,2));console.log(JSON.stringify(rows));
