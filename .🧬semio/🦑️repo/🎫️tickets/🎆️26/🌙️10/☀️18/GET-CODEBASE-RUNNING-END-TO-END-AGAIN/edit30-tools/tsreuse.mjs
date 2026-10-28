/** 🪁 Read-only, graph-free: what the course's derived TypeScript-program question costs cold and with the previous program kept (TypeScript `createProgram(…, oldProgram)`), after an in-memory one-line edit of one file; prints times, `structureIsReused` and whether the program answer (files, packages, configs) is identical.
 * usage: bun tsreuse.mjs <tsconfig> <edited file> */
import ts from "typescript";
import {resolve,dirname} from "node:path";
const [config,edited]=process.argv.slice(2).map(p=>resolve(p));
const parsed=ts.getParsedCommandLineOfConfigFile(config,{},{...ts.sys,onUnRecoverableConfigFileDiagnostic:d=>{throw Error(String(d.messageText));}});
const cache=new Map(),base=ts.createCompilerHost(parsed.options,true);let edit="";
const host={...base,getSourceFile(name,language,onError,fresh){const key=resolve(name);let text=ts.sys.readFile(name);if(text===undefined)return undefined;if(key===edited)text+=edit;const hit=cache.get(key);if(hit&&hit.text===text)return hit.file;const file=base.getSourceFile(name,language,onError,fresh);if(file&&key===edited&&edit){const changed=ts.createSourceFile(name,text,language,true);cache.set(key,{text,file:changed});return changed;}if(file)cache.set(key,{text,file});return file;}};
const answer=p=>JSON.stringify(p.getSourceFiles().map(f=>f.fileName).sort());
let t=performance.now();const cold=ts.createProgram({rootNames:parsed.fileNames,options:parsed.options,host});const coldMs=performance.now()-t,coldFiles=cold.getSourceFiles().length,coldAnswer=answer(cold);
for(const [label,text,old] of [["comment","\n// probe edit30\n",true],["import",'\nimport "./does-not-exist-edit30";\n',true],["comment-parsecache","\n// probe edit30 b\n",false]]){
 edit=text;t=performance.now();const warm=ts.createProgram({rootNames:parsed.fileNames,options:parsed.options,host,...(old?{oldProgram:cold}:{})});const warmMs=performance.now()-t;
 console.log(JSON.stringify({edit:label,coldMs:Math.round(coldMs),coldFiles,warmMs:Math.round(warmMs),reused:["Not","SafeModules","Completely"][warm.structureIsReused]??warm.structureIsReused,sameAnswer:answer(warm)===coldAnswer,rssMB:Math.round(process.memoryUsage().rss/1048576)}));
}
