import {existsSync,readFileSync} from "node:fs";import {join} from "node:path";
import {checkRuntimeLawV1,listRuntimeLawFilesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts";
const live="/Users/ueli/Documents/semio",[mirror,...manifests]=process.argv.slice(2);
const staged=new Set<string>();for(const m of manifests)for(const f of JSON.parse(readFileSync(m,"utf8")).files)staged.add(f.file);
const files=listRuntimeLawFilesV1(live,"🧰️framework").map(path=>({path,source:readFileSync(join(staged.has(path)&&existsSync(join(mirror!,path))?mirror!:live,path),"utf8")}));
for(const path of staged)if(!files.some(f=>f.path===path)&&existsSync(join(mirror!,path))&&!path.includes("/🧪️tests/")&&!path.endsWith("/🍞️bun/🔣️.json"))files.push({path,source:readFileSync(join(mirror!,path),"utf8")});
const report=checkRuntimeLawV1(files);console.log(files.length,"files",report.violations.length,"violations");for(const v of report.violations)console.log(v);
