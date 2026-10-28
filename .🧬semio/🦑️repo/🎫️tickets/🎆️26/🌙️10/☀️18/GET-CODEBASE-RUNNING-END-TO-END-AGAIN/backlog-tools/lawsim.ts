/** ⚖️ [backlog] the whole-tree runtime-name law with the file CONTENT of a MIRROR root (live listing, mirror bytes where the mirror has the file, mirror-only files added): prints `<files> files <n> violations` and the violations. usage: bun lawsim.ts <mirror-root> [extra-relative-path-list-file] */
import {existsSync,readFileSync} from "node:fs";import {join} from "node:path";
const live="/Users/ueli/Documents/semio",root=process.argv[2]!,law=await import(join(root,"🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts"));
const listed=new Set<string>(law.listRuntimeLawFilesV1(live,"🧰️framework"));
if(process.argv[3])for(const line of readFileSync(process.argv[3],"utf8").split("\n"))if(/\.[cm]?[jt]sx?$/u.test(line)&&line.startsWith("🧰️framework")&&existsSync(join(root,line)))listed.add(line);
const files=[...listed].filter(path=>existsSync(join(root,path))||existsSync(join(live,path))).map(path=>({path,source:readFileSync(join(existsSync(join(root,path))?root:live,path),"utf8")}));
const report=law.checkRuntimeLawV1(files);console.log(files.length,"files",report.violations.length,"violations");for(const v of report.violations)console.log(v);
