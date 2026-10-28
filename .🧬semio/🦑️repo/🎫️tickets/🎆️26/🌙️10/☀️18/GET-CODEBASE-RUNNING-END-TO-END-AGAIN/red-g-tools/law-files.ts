import {readFileSync} from "node:fs";import {join} from "node:path";
import {checkRuntimeLawV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts";
const [root,manifest]=process.argv.slice(2);
const paths=(JSON.parse(readFileSync(manifest!,"utf8")).files as {file:string}[]).map(f=>f.file);
const report=checkRuntimeLawV1(paths.map(path=>({path,source:readFileSync(join(root!,path),"utf8")})));
console.log(paths.length,"files",report.violations.length,"violations");for(const v of report.violations)console.log(v);
