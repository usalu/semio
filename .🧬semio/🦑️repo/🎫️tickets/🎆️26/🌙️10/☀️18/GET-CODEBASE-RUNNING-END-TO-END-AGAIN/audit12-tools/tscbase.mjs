/** 🧷 [audit12] type-checks a tsconfig with every G46-touched file read at its pre-G46 bytes (the first batch's base in landing order; files G46 created read as absent): node tscbase.mjs <root> <tsconfig> <C> → `file(line,col): error TSxxxx: …` lines like tsc --pretty false */
import {createRequire} from "node:module";
import {existsSync,readFileSync} from "node:fs";
import {join,resolve} from "node:path";
const [root,config,C]=process.argv.slice(2);const ts=createRequire(join(root,"package.json"))("typescript");
const order=[...readFileSync(join(C,"audit12-tools/compose.py"),"utf8").split("\n").find(l=>l.startsWith("ORDER")).matchAll(/"([a-z0-9-]+-tools\/staged\/[a-z0-9]+)"/gu)].map(m=>m[1]).concat("umlaut lang status rows sigint usage phases say".split(" ").map(b=>"progress-tools/staged/"+b),["winps-tools/staged/native","winps-tools/staged/pin"]);
const before=new Map();
for(const batch of order){const rows=JSON.parse(readFileSync(join(C,batch,"manifest.json"),"utf8"));for(const [rel,row] of Object.entries(rows)){const path=resolve(root,rel);if(!before.has(path))before.set(path,row.base===null?null:readFileSync(join(C,batch,"base",row.file),"utf8"));}}
const parsed=ts.getParsedCommandLineOfConfigFile(config,{},{...ts.sys,onUnRecoverableConfigFileDiagnostic:d=>{throw Error(String(d.messageText));}});
const host=ts.createCompilerHost(parsed.options);const read=host.readFile.bind(host),exists=host.fileExists.bind(host);
host.readFile=p=>{const r=resolve(p);return before.has(r)?before.get(r)??undefined:read(p);};host.fileExists=p=>{const r=resolve(p);return before.has(r)?before.get(r)!==null:exists(p);};
const program=ts.createProgram({rootNames:parsed.fileNames.filter(f=>host.fileExists(f)),options:parsed.options,host});
for(const d of ts.getPreEmitDiagnostics(program)){if(!d.file){console.log("error TS"+d.code+": "+ts.flattenDiagnosticMessageText(d.messageText," "));continue;}const {line,character}=d.file.getLineAndCharacterOfPosition(d.start);console.log(`${d.file.fileName.startsWith(root+"/")?d.file.fileName.slice(root.length+1):d.file.fileName}(${line+1},${character+1}): error TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText," ").slice(0,300)}`);}
