import {pathToFileURL} from "node:url";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const m=await import(pathToFileURL(lib+"🟨️.mjs").href),e=await import(pathToFileURL(lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts").href);
const files=m.nxBuildEngineClosureV1().files;
for(const f of e.nxEngineFamiliesV1){const s=e.splitNxEngineFilesV1(files,f);console.log(f,"closure",files.length,"keyed",s.keyed.length,"unkeyed",s.unkeyed.length,"prefixes",e.nxEngineUnkeyedPrefixesV1(f).length);}
