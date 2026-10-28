import {readFileSync} from "node:fs";import {join} from "node:path";
import {checkRuntimeLawV1,listRuntimeLawFilesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts";
const root="/Users/ueli/Documents/semio";
const files=listRuntimeLawFilesV1(root,"🧰️framework").map(path=>({path,source:readFileSync(join(root,path),"utf8")}));
const report=checkRuntimeLawV1(files);
console.log(files.length,"files",report.violations.length,"violations");
for(const v of report.violations) console.log(v);
