// ts2-g prototype: warm in-memory builder-program engine vs cold createProgram on the course edit corpus (virtual edits, no tree writes).
// usage: node proto.mjs <warm|cold|warm-listing> <out.jsonl> [stepNames,comma]
import {createRequire} from "node:module";
import {readdirSync,statSync} from "node:fs";
import {appendFileSync,writeFileSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
const R="/Users/ueli/Documents/semio",ts=createRequire(R+"/")("typescript"),[mode,out,only]=process.argv.slice(2),P=join(R,"🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript"),CONFIG=join(P,"tsconfig.json");
const f=x=>join(R,x),COURSE=f("🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts"),FW=f("🧰️framework/📦️packages/🟦️typescript/🟦️.ts"),BACKOFF=f("🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/🔁️jittered-backoff/🟦️.ts"),DEPLOYJSON=f("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json"),AMBIENT=f("🎓️teaching/🏛️architecture/📚️course/🧪️tests/tsg-ambient.ts");
const app=(p,t)=>["append",p,t],rep=(p,a,b)=>["replace",p,a,b];
const STEPS=[["original",[]],["noop",[]],["leaf-comment",[app(COURSE,"\n// probe\n")]],["leaf-body",[app(COURSE,"\nconst tsGInternal = 1 + 1;\n")]],["leaf-export",[app(COURSE,"\nexport const tsGProbe: number = 1;\n")]],["leaf-error",[app(COURSE,'\nexport const tsGBad: number = "x";\n')]],["leaf-error-fixed",[]],["mid-body",[app(BACKOFF,"\nconst tsGInternal = 1 + 1;\n")]],["mid-export",[app(BACKOFF,"\nexport const tsGProbe: number = 1;\n")]],["mid-error",[app(BACKOFF,'\nexport const tsGBad: number = "x";\n')]],["api-break",[rep(FW,"export { retryWithJitteredBackoff } from","export { retryWithJitteredBackoff as tsGRenamed } from")]],["api-break-fixed",[]],["json-key",[rep(DEPLOYJSON,'"legal": {}','"legalX": {}')]],["json-key-fixed",[]],["delete-file",[["delete",COURSE]]],["delete-file-restored",[]],["ambient-global",[["create",AMBIENT,"declare global { interface Array<T> { at(index: number): never } }\nexport {};\n"]]],["ambient-global-removed",[]],["tsconfig-option",[rep(CONFIG,'"types":','"noUncheckedIndexedAccess": true, "types":')]],["tsconfig-option-removed",[]],["tsconfig-paths",[rep(CONFIG,'"@semio-tech/course": ["../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts"]','"@semio-tech/course": ["../../../../../🧰️framework/nonexistent.ts"]')]],["tsconfig-paths-fixed",[]]];
const real=new Map(),edits=new Map(),baseRead=p=>{if(!real.has(p)){const t=ts.sys.readFile(p);real.set(p,t)}return real.get(p)};
const frozenText=new Map(),frozen=p=>{if(!frozenText.has(p))frozenText.set(p,ts.sys.readFile(p));return frozenText.get(p)};
const sys={...ts.sys,
 readFile:p=>edits.has(p)?(edits.get(p)??undefined):frozen(p),
 fileExists:p=>edits.has(p)?edits.get(p)!==null:ts.sys.fileExists(p),
 readDirectory(path,exts,excl,incl,depth){let r=ts.sys.readDirectory(path,exts,excl,incl,depth).filter(x=>edits.get(x)!==null);for(const[x,v]of edits)if(v!==null&&!ts.sys.fileExists(x)&&x.startsWith(path+"/")&&/\.tsx?$/.test(x)&&!r.includes(x))r.push(x);return r}};
function applyStep(list){edits.clear();for(const e of list){const[k,p]=e;if(k==="delete")edits.set(p,null);else if(k==="create")edits.set(p,e[2]);else{const cur=edits.has(p)?edits.get(p):ts.sys.readFile(p);if(k==="append")edits.set(p,cur+e[2]);else{if(!cur.includes(e[2]))throw Error("anchor "+e[2].slice(0,40));edits.set(p,cur.replace(e[2],e[3]))}}}}
const cmd=ts.parseCommandLine(["--noEmit","-p",CONFIG]);
function parsed(){const c=ts.getParsedCommandLineOfConfigFile(CONFIG,cmd.options,{...sys,onUnRecoverableConfigFileDiagnostic:d=>{throw Error(ts.flattenDiagnosticMessageText(d.messageText,"\n"))}});return c}
const fmt=d=>d.file?`${d.file.fileName.replace(R+"/","")}(${d.file.getLineAndCharacterOfPosition(d.start).line+1}):TS${d.code}`:`TS${d.code}`;
const cache=new Map();let prev,stats={hit:0,miss:0};
function warmCheck(listing){
 const c=parsed(),options=c.options,base=ts.createIncrementalCompilerHost(options,sys),seen=new Set();
 const gsf=base.getSourceFile.bind(base);
 base.getSourceFile=(fileName,lv,onError,should)=>{const text=sys.readFile(fileName);if(text===undefined)return undefined;const key=typeof lv==="object"?lv.languageVersion+":"+lv.impliedNodeFormat:String(lv),hit=cache.get(fileName);seen.add(fileName);if(!should&&hit&&hit.text===text&&hit.key===key){stats.hit++;return hit.sf}stats.miss++;const sf=gsf(fileName,lv,onError,should);if(sf)cache.set(fileName,{text,key,sf});return sf};
 base.hasInvalidatedResolutions=()=>listing?false:true;
 stats={hit:0,miss:0};const t0=performance.now();
 const builder=ts.createSemanticDiagnosticsBuilderProgram(c.fileNames,options,base,prev,ts.getConfigFileParsingDiagnostics(c),c.projectReferences);
 const t1=performance.now();
 const all=[...builder.getConfigFileParsingDiagnostics(),...builder.getSyntacticDiagnostics(),...builder.getOptionsDiagnostics(),...builder.getGlobalDiagnostics(),...builder.getSemanticDiagnostics()];
 const t2=performance.now();
 for(const k of cache.keys())if(!seen.has(k))cache.delete(k);
 prev=builder;
 return {diag:all,build:t1-t0,check:t2-t1,files:seen.size,hits:stats.hit,misses:stats.miss};
}
function coldCheck(){
 const c=parsed(),host=ts.createCompilerHost(c.options);host.readFile=sys.readFile;host.fileExists=sys.fileExists;
 const t0=performance.now(),program=ts.createProgram({rootNames:c.fileNames,options:c.options,host,projectReferences:c.projectReferences,configFileParsingDiagnostics:ts.getConfigFileParsingDiagnostics(c)}),t1=performance.now(),all=ts.getPreEmitDiagnostics(program),t2=performance.now();
 return {diag:all,build:t1-t0,check:t2-t1,files:program.getSourceFiles().length};
}
writeFileSync(out,"");
const wanted=only?only.split(","):undefined;
for(const[name,list]of STEPS){
 if(wanted&&!wanted.includes(name))continue;
 applyStep(list);
 let r,cold;
 if(mode==="both"){const cc0=process.cpuUsage(),ct=performance.now();cold=coldCheck();const cc=process.cpuUsage(cc0);cold.wallMs=Math.round(performance.now()-ct);cold.cpuMs=Math.round((cc.user+cc.system)/1000);cold.lines=[...new Set(cold.diag.map(fmt))].sort();cold.diag=undefined;if(global.gc)global.gc();}
 const cpuW0=process.cpuUsage(),tw=performance.now();
 r=mode==="cold"?coldCheck():warmCheck(mode==="warm-listing");
 const wall=performance.now()-tw,cpu=process.cpuUsage(cpuW0);
 let gcMs=0;if(process.env.GC){const g0=performance.now();if(globalThis.Bun)Bun.gc(true);else if(global.gc)global.gc();gcMs=Math.round(performance.now()-g0);}
 const lines=[...new Set(r.diag.map(fmt))].sort();
 const row={mode,step:name,...(cold?{coldWallMs:cold.wallMs,coldCpuMs:cold.cpuMs,same:JSON.stringify(cold.lines)===JSON.stringify([...new Set(r.diag.map(fmt))].sort()),coldErrors:cold.lines.length}:{}),wallMs:Math.round(wall),cpuMs:Math.round((cpu.user+cpu.system)/1000),buildMs:Math.round(r.build),checkMs:Math.round(r.check),files:r.files,hits:r.hits,misses:r.misses,errors:lines.length,gcMs,rssMb:Math.round(process.memoryUsage.rss()/1048576),heapMb:Math.round(process.memoryUsage().heapUsed/1048576),diag:lines};
 appendFileSync(out,JSON.stringify(row)+"\n");
 console.log(JSON.stringify({...row,diag:undefined}));
 if(mode==="cold"&&global.gc)global.gc();
}

if(process.env.IDLE){for(let i=0;i<12;i++){await new Promise(r=>setTimeout(r,3000));console.log(JSON.stringify({idleS:(i+1)*3,rssMb:Math.round(process.memoryUsage.rss()/1048576),heapMb:Math.round(process.memoryUsage().heapUsed/1048576)}));}}
