import {readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
export const ROOT="/Users/ueli/Documents/semio/";
export const L=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
export const SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R134/";
export const HERE=fileURLToPath(new URL("./",import.meta.url));
const rd=name=>readFileSync(HERE+"new/"+name,"utf8");
export function applyEdits(text,edits,file){
 for(const e of edits){
  if(e.append!==undefined){text=text.replace(/\s*$/u,"\n")+e.append;continue;}
  if(e.between!==undefined){const a=text.indexOf(e.between[0]);if(a<0||text.indexOf(e.between[0],a+1)>=0)throw Error("between start anchor not unique: "+e.between[0].slice(0,60)+" in "+file);const b=text.indexOf(e.between[1],a);if(b<0)throw Error("between end anchor missing: "+e.between[1].slice(0,60));text=text.slice(0,a)+e.replace+text.slice(b+e.between[1].length);continue;}
  const count=text.split(e.find).length-1;
  if(e.all?count<1:count!==1)throw Error("anchor "+(count?"not unique ("+count+")":"missing")+": "+e.find.slice(0,70)+" in "+file);
  text=e.all?text.split(e.find).join(e.replace):text.replace(e.find,()=>e.replace);
 }
 return text;
}
const B="🏗️build/",GS=B+"🪪️graph-state/",INF=B+"🕸️graph/📥️inference/",RES=B+"🏡️resident/",SEL="⚡️caching/🕸️graph/🔏️seal/🎯️selection/";
const num=(key,from,to)=>({find:'"'+key+'": '+from+",",replace:'"'+key+'": '+to+","});
export const PATCHES=[
 {file:L+GS+"🧬️schema/🔣️.json",edits:[num("entryMaximumDocuments",4096,32768),{find:'"maxItems": 4096,',replace:'"maxItems": 32768,'}]},
 {file:L+GS+"🙋️wanted/🧬️schema/🔣️.json",edits:[{find:'"maxItems": 8192,',replace:'"maxItems": 16384,',all:true},num("maximumProjects",8192,16384)]},
 {file:L+B+"🕸️graph/🕰️generations/🌐️remote-exchange/🧬️schema/🔣️.json",edits:[num("maximumDocuments",20000,32768)]},
 {file:L+GS+"📍️cursor/🧬️schema/🔣️.json",edits:[num("maximumRows",400000,1048576)]},
 {file:L+GS+"📦️project-records/🌱️fresh/🧬️schema/🔣️.json",edits:[num("maximumRows",400000,1048576)]},
 {file:L+B+"🕸️graph/🧮️derived-nodes/🧬️schema/🔣️.json",edits:[num("maximumNodes",262144,1048576)]},
 {file:L+B+"🕸️graph/🕰️generations/✂️cutoff/🧬️schema/🔣️.json",edits:[num("maximumSuspects",65536,131072)]},
 {file:L+INF+"🧬️schema/🔣️.json",edits:[
  num("closureStoreMaximumLayers",262144,524288),num("closureStoreMaximumValues",8192,131072),
  {find:'"closureStoreMaximumEntries": "The most closure entry records kept on disk; the oldest are removed first.",',replace:'"closureStoreMaximumEntries": "The most closure entry records kept on disk; once over it the oldest are removed first, but never one younger than the grace period, so a reader or peer that holds it still finds it.",'},
  {find:'"closureStoreMaximumLayers": "The most shared row layer records kept on disk; the oldest are removed first.",',replace:'"closureStoreMaximumLayers": "The most shared row layer records kept on disk; once over it the oldest are removed first, but never one younger than the grace period, so a reader or peer that holds it still finds it.",'},
  {find:'the oldest are removed first.",\n          "closureStoreGraceMilliseconds": "How long',replace:'once over it the oldest are removed first, but never one younger than the grace period.",\n          "closureStoreGraceMilliseconds": "How long'},
  {find:'"closureStoreMaximumEntries": "Die meisten Closure-Eintragsdatensätze auf der Platte; die ältesten werden zuerst entfernt.",',replace:'"closureStoreMaximumEntries": "Die meisten Closure-Eintragsdatensätze auf der Platte; darüber werden die ältesten zuerst entfernt, aber nie einer, der jünger als die Schonfrist ist, damit ein Leser oder Peer, der ihn hält, ihn noch findet.",'},
  {find:'"closureStoreMaximumLayers": "Die meisten Datensätze gemeinsamer Zeilenschichten auf der Platte; die ältesten werden zuerst entfernt.",',replace:'"closureStoreMaximumLayers": "Die meisten Datensätze gemeinsamer Zeilenschichten auf der Platte; darüber werden die ältesten zuerst entfernt, aber nie einer, der jünger als die Schonfrist ist, damit ein Leser oder Peer, der ihn hält, ihn noch findet.",'},
  {find:'einmal gespeichert, wie viele Einträge ihn auch nennen); die ältesten werden zuerst entfernt.",',replace:'einmal gespeichert, wie viele Einträge ihn auch nennen); darüber werden die ältesten zuerst entfernt, aber nie einer, der jünger als die Schonfrist ist.",'},
 ]},
 {file:L+INF+"🧭️script-inputs/💾️entries/🟨️.mjs",edits:[
  {find:'  for(const file of dated.sort((a,b)=>a.time-b.time).slice(0,dated.length-maximum))await rm(file.path,{force:true});',replace:'  const youngest=Date.now()-limits.closureStoreGraceMilliseconds;\n  for(const file of dated.sort((a,b)=>a.time-b.time).slice(0,dated.length-maximum))if(file.time<youngest)await rm(file.path,{force:true});'},
 ]},
 {file:L+INF+"🧭️script-inputs/🧪️tests/closure-store-limits.test.ts",edits:[
  {find:'toBeGreaterThanOrEqual(PROJECTS*LAYERS_PER_PROJECT);',replace:'toBeGreaterThanOrEqual(PROJECTS*LAYERS_PER_PROJECT);\n expect(INFERENCE_LIMITS_V1.closureStoreMaximumValues).toBeGreaterThanOrEqual(PROJECTS*ENTRIES_PER_PROJECT);'},
  {find:'closureStoreMaximumEntries:3,closureStoreMaximumLayers:3}',replace:'closureStoreMaximumEntries:3,closureStoreMaximumLayers:3,closureStoreGraceMilliseconds:0}'},
  {append:rd("sweep-test.ts")},
 ]},
 {file:L+RES+"🧬️schema/🔣️.json",edits:[num("memoMaximumEntries",65536,262144),num("digestCacheMaximumFiles",262144,1048576),{find:'    "outputSignatureMaximumEntries": 2048,\n',replace:''},num("journalMaximumEvents",262144,1048576)]},
 {file:L+RES+"🗃️state/🟨️.mjs",edits:[{between:["/** 🖊 Signature of a task's outputs by bounded stat; undefined when the output tree exceeds the bound. */",'return sha(rows.join("\\n"));\n}'],replace:'/** 🖊 Signature of a task\'s outputs by stat, hashed row by row as the tree is walked, so no entry ceiling and no list of rows is needed however many files the outputs hold. */\nexport async function outputSignatureV1(root,outputs){\n const hash=createHash("sha256");let first=true;\n const row=text=>{if(!first)hash.update("\\n");first=false;hash.update(text);};\n const walk=async rel=>{\n  let status;try{status=await lstat(root+"/"+rel);}catch(error){if(missing(error)){row(rel+"\\0missing");return;}throw error;}\n  row(rel+"\\0"+status.mtimeMs+"\\0"+status.size+"\\0"+status.ino);\n  if(status.isDirectory())for(const name of(await readdir(root+"/"+rel)).sort())await walk(rel+"/"+name);\n };\n for(const output of outputs)await walk(output);\n return hash.digest("hex");\n}'}]},
 {file:L+RES+"🗃️state/🟨️.d.mts",edits:[{find:'outputs:readonly string[],maximumEntries:number):Promise<string|undefined>;',replace:'outputs:readonly string[]):Promise<string>;'}]},
 {file:L+RES+"🏛️service/🟨️.mjs",edits:[{find:'entry.outputs,policy.outputSignatureMaximumEntries)',replace:'entry.outputs)'},{find:'await outputSignatureV1(root,outputs,policy.outputSignatureMaximumEntries)',replace:'await outputSignatureV1(root,outputs)'}]},
 {file:L+RES+"🧪️tests/🗃️state/🟦️.ts",edits:[
  {find:'import {chmodSync,existsSync,mkdirSync,readdirSync,renameSync,rmSync,writeFileSync,appendFileSync} from "node:fs";',replace:'import {chmodSync,existsSync,lstatSync,mkdirSync,readdirSync,renameSync,rmSync,writeFileSync,appendFileSync} from "node:fs";\nimport {spawnSync} from "node:child_process";'},
  {find:'test("output signature by bounded stat changes with content and refuses large trees"',replace:'test("output signature by stat changes with content"'},
  {find:'["out/a","out/b"],100)',replace:'["out/a","out/b"])',all:true},
  {find:'  expect(await outputSignatureV1(root,["out"],2)).toBeUndefined();\n',replace:''},
  {find:'  rmSync(root+"/out/b");expect(await outputSignatureV1(root,["out/b"],100)).toBeDefined();\n });',replace:'  rmSync(root+"/out/b");expect(await outputSignatureV1(root,["out/b"])).toBeDefined();\n });\n\n'+rd("signature-test.ts").trimEnd()},
 ]},
 {file:L+SEL+"🧬️schema/🔣️.json",edits:[num("maximumRecords",2048,16384),num("maximumAggregateBytes",67108864,536870912)]},
 {file:L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth/🟦️.ts",edits:[
  {find:'import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";',replace:'import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";\nimport scaleDeclaration from"../../../../../../🏗️build/📏️workspace-scale/🧬️schema/🔣️.json";'},
  {append:rd("law.ts")},
 ]},
 {file:L+GS+"📦️project-records/🌱️fresh/🧪️tests/🟦️.ts",edits:[{append:rd("fresh-test.ts")}]},
 {file:L+GS+"📦️project-records/🧬️schema/🔣️.json",edits:[{find:'"maximumEntries": 24,',replace:'"maximumEntries": 16384,'}]},
 {file:L+GS+"📦️project-records/📮️unsent/🧪️tests/🟦️.ts",edits:[{find:'import {mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";',replace:'import {spawnSync} from "node:child_process";\nimport {mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";'},{append:rd("unsent-test.ts")}]},
];
export const NEWS=[
 {to:L+B+"📏️workspace-scale/🧬️schema/🔣️.json",from:"workspace-scale.json"},
];
