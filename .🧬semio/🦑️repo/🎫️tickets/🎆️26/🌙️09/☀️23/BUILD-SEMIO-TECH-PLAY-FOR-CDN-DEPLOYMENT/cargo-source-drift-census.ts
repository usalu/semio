import {dirname,relative} from "node:path";
import {decodeCargoProvenanceV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import limits from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/👁️drift/🔣️.json" with {type:"json"};

const receipt=decodeCargoProvenanceV1(await Bun.file(process.argv[2]!).text());
const log=await Bun.file(process.argv[3]!).text();
const admission:Map<string,{order:number;inputs:number}>=new Map();
let unit=0,files=0;
for(const line of log.split("\n")){
  if(!line.includes('cargo:source-admission:'))continue;
  const raw=JSON.parse(line.slice(line.indexOf("{"))).activity.resource;
  const row=JSON.parse(raw.slice("cargo:source-admission:".length));
  if(row.phase!=="complete")continue;
  if(row.unit!==unit+1)break;
  unit++;
  if(!admission.has(row.packageName))admission.set(row.packageName,{order:row.unit,inputs:row.files-files});
  files=row.files;
}
const inventory=new Map<string,string[]>();
for(const unit of receipt.units){
  const id=unit.message.package_id,manifest=unit.message.manifest_path;
  if(!id?.startsWith("path+")||!manifest)continue;
  const suffix=id.split("#").at(-1)!;
  const name=suffix.includes("@")?suffix.split("@")[0]!:Bun.TOML.parse(await Bun.file(manifest).text()).package.name as string;
  const sources=inventory.get(name)??[];
  for(const input of unit.inputs)if(input.kind==="file"&&input.path.endsWith(".rs")&&input.path.startsWith(receipt.cwd+"/")&&!input.path.startsWith(receipt.cwd+"/.cache/"))sources.push(input.path);
  inventory.set(name,sources);
}
const paths=new Set<string>(),directoryRanks=new Map<string,number>();
for(const name of admission.keys())for(const path of [...new Set(inventory.get(name)??[])].sort()){
  paths.add(path);
  const directory=dirname(path);
  if(!directoryRanks.has(directory))directoryRanks.set(directory,directoryRanks.size+1);
}
console.log({unitRows:unit,admissionEntries:files,uniquePackages:admission.size,physicalRustCandidates:paths.size,physicalCandidateDirectories:directoryRanks.size,watchedDirectories:limits.maximumDirectories});
for(const path of paths)if(/job\/.*context\/|job\/.*failure\/.*work\/|actor\/.*retained-turn\/.*context\/🦀️|value\/.*retirement\/.*boxed\/🦀️|manifest\/.*view-model\/.*selection\/🦀️|document\/.*hydration\/.*fold\//u.test(path))console.log(directoryRanks.get(dirname(path)),relative(receipt.cwd,path));
