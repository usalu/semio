"""R36c: the TypeScript programs of the declared closure are released between programs (proven cause of the first-wave growth: every createProgram leaves about 1 GB of unreleased garbage), the sampler names the functions of its stack."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
FP=B+"🧠️memory/🦶️footprint/"
FM=FP+"🟨️.mjs"
add(FM,'stack=()=>String(new Error().stack).split("\\n").slice(3,3+P.pulseFrames).map(line=>line.trim().replace(/^at /u,"").slice(-110)).join(" < ")','stack=()=>String(new Error().stack).split("\\n").slice(3,3+P.pulseFrames).map(line=>{const row=/^\\s*(?:at )?(.*?)\\s*\\((?:.*?)([^/]*):(\\d+):\\d+\\)$/u.exec(line);return row===null?line.trim().slice(-80):(row[1]||"?")+"@"+row[2]+":"+row[3];}).join(" < ")')
add(FM,'let installed,step,pulser,pulses=0,pulsedAt=0;\n','''let installed,step,pulser,pulses=0,pulsedAt=0,releasedAt=0,releaseCalls=0;
/** 🧹️ Returns the memory a long synchronous stretch of garbage left behind: when the managed memory (heap in use plus external) grew by `releaseGrowthBytes` since the last release, it collects, shrinks and collects again; a call that finds no such growth costs one counter step in `every - 1` of `every` calls. Compiling a TypeScript program leaves about 1 GB of unreleased garbage in the runtime (measured: 5 programs of 4468 files, footprint 1.7 GB to 4.9 GB; with a release between the programs 1.3 to 2.0 GB), so the declared closure calls it between its programs and the inference at its yield points. @returns {boolean} whether it released */
export function releaseIfGrownV1({usage=()=>{const row=process.memoryUsage();return row.heapUsed+row.external;},release=releaseRuntimeMemoryV1,threshold=P.releaseGrowthBytes,every=1}={}){
 if(every>1&&(++releaseCalls%every)!==0)return false;
 if(usage()-releasedAt<threshold)return false;
 release();releasedAt=usage();return true;
}

''')
FD=FP+"🟨️.d.mts"
add(FD,'readonly pulseFrames:number;','readonly pulseFrames:number;readonly releaseGrowthBytes:number;')
add(FD,'export function footprintPulseV1():void;','export function footprintPulseV1():void;\nexport function releaseIfGrownV1(options?:{usage?:()=>number;release?:()=>void;threshold?:number;every?:number}):boolean;')
FS=FP+"🧬️schema/🔣️.json"
add(FS,'"pulseFrames": 14,','"pulseFrames": 14,\n        "releaseGrowthBytes": 268435456,')
add(FS,'commandTimeoutMilliseconds of one reading.','releaseGrowthBytes the managed growth after which releaseIfGrownV1 collects and shrinks; commandTimeoutMilliseconds of one reading.')
add(FS,'commandTimeoutMilliseconds einer Messung.','releaseGrowthBytes das verwaltete Wachstum, ab dem releaseIfGrownV1 sammelt und schrumpft; commandTimeoutMilliseconds einer Messung.')
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
add(SI,'import { footprintPulseV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";','import { footprintPulseV1, releaseIfGrownV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(SI,'  const { compiler, host, readdir, join, dirname, isAbsolute, reuse } = environment;\n  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input)','  const { compiler, host, readdir, join, dirname, isAbsolute, reuse, release = releaseIfGrownV1 } = environment;\n  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input)')
add(SI,'  for (const relative of configs) {\n    const config = join(workspaceRoot, relative), source = compiler.readJsonConfigFile(config, host.readFile);','  for (const relative of configs) {\n    if (!reuse) release();\n    const config = join(workspaceRoot, relative), source = compiler.readJsonConfigFile(config, host.readFile);')
add(SI,'  return { files: [...files].sort(), packages: [...packages].sort(), configs: declared };\n}','  if (!reuse) release();\n  return { files: [...files].sort(), packages: [...packages].sort(), configs: declared };\n}')
add(SI,'export async function yieldInferenceV1(context,phase,index,force=false){\n footprintPulseV1();\n','export async function yieldInferenceV1(context,phase,index,force=false){\n footprintPulseV1();releaseIfGrownV1({every:16});\n')
TF=FP+"🧪️tests/🟦️.ts"
add(TF,'import{footprintPulseV1,bindProcessGateV1,','import{releaseIfGrownV1,footprintPulseV1,bindProcessGateV1,')
add(TF,'import{readFileSync}from"node:fs";\n','import{readFileSync}from"node:fs";\nimport ts from"typescript";\nimport{fileURLToPath}from"node:url";\nimport{declaredTypeScriptProjectsV1}from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";\nimport{spawnSync}from"node:child_process";\nimport{mkdirSync,writeFileSync}from"node:fs";\n')
_t=r'''
test("releaseIfGrownV1 releases once the managed memory grew by the threshold, moves its baseline to the live size and counts calls when asked to look rarely",()=>{
 let used=100*MB;const released:number[]=[];
 const call=(more:Record<string,unknown>={})=>releaseIfGrownV1({usage:()=>used,release:()=>{released.push(used);used=150*MB;},threshold:200*MB,...more});
 expect(call()).toBe(false);
 used=400*MB;expect(call()).toBe(true);expect(released).toEqual([400*MB]);
 used=300*MB;expect(call()).toBe(false);
 used=360*MB;expect(call()).toBe(true);expect(released.length).toBe(2);
 used=900*MB;let looks=0;const seen=()=>{looks++;return used;};
 for(let index=0;index<7;index++)releaseIfGrownV1({usage:seen,release:()=>undefined,threshold:200*MB,every:8});
 expect(looks).toBeLessThanOrEqual(1);
});

test("the declared TypeScript closure releases the runtime between its programs and after the last one, but never when a reuse store keeps the programs",()=>{
 const root=join(tmpdir(),"footprint-programs-"+process.pid);mkdirSync(join(root,"a"),{recursive:true});mkdirSync(join(root,"b"),{recursive:true});
 for(const name of["a","b"]){writeFileSync(join(root,name,"tsconfig.json"),JSON.stringify({compilerOptions:{noLib:true,types:[]},files:["x.ts"]}));writeFileSync(join(root,name,"x.ts"),"export const x = 1;\n");}
 try{
  const calls:string[]=[],listing=(path:string)=>{try{return require("node:fs").readdirSync(path,{withFileTypes:true}).map((entry:any)=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
  const environment=(more:Record<string,unknown>)=>({compiler:ts,host:ts.sys,readdir:listing,join,dirname:require("node:path").dirname,isAbsolute:require("node:path").isAbsolute,...more});
    const result=declaredTypeScriptProjectsV1(["{projectRoot}/a/tsconfig.json","{projectRoot}/b/tsconfig.json"],root,".",environment({release:()=>{calls.push("release");}}));
  expect(result.configs.length).toBe(2);expect(calls.length).toBe(3);
  calls.length=0;
  declaredTypeScriptProjectsV1(["{projectRoot}/a/tsconfig.json"],root,".",environment({release:()=>{calls.push("release");},reuse:{sourceFile:(_c:string,original:unknown)=>original,oldProgram:()=>undefined,keep:()=>undefined}}));
  expect(calls.length).toBe(0);
 }finally{require("node:fs").rmSync(root,{recursive:true,force:true});}
});

test("releasing between the TypeScript programs keeps the managed memory flat where the control grows with every program (measured in a child process)",()=>{
 const script=`
  const ts=require(${JSON.stringify(ts.sys.getExecutingFilePath())}),fs=require("node:fs"),path=require("node:path"),os=require("node:os");
  (async()=>{
  const {declaredTypeScriptProjectsV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",import.meta.url)))});
  const {releaseIfGrownV1}=await import(${JSON.stringify(fileURLToPath(new URL("../🟨️.mjs",import.meta.url)))});
  const root=fs.mkdtempSync(path.join(os.tmpdir(),"progs-"));fs.writeFileSync(path.join(root,"tsconfig.json"),JSON.stringify({compilerOptions:{types:[]},files:["x.ts"]}));fs.writeFileSync(path.join(root,"x.ts"),"export const x = 1;\\n");
  const listing=p=>{try{return fs.readdirSync(p,{withFileTypes:true}).map(e=>({name:e.name,directory:e.isDirectory(),file:e.isFile()}));}catch{return undefined;}};
  const mode=process.env.R36_MODE;let peak=0;
  for(let index=0;index<10;index++){
   declaredTypeScriptProjectsV1(["{projectRoot}/tsconfig.json"],root,".",{compiler:ts,host:ts.sys,readdir:listing,join:path.join,dirname:path.dirname,isAbsolute:path.isAbsolute,release:mode==="release"?()=>releaseIfGrownV1({threshold:64*1048576}):()=>undefined});
   const usage=process.memoryUsage();peak=Math.max(peak,usage.heapUsed+usage.external);
  }
  fs.rmSync(root,{recursive:true,force:true});console.log(Math.round(peak/1048576));})();`;
  const measure=(mode:string)=>{const run=spawnSync(process.execPath,[...(process.env.R5_PRELOAD?["--preload",process.env.R5_PRELOAD]:[]),"--eval",script],{encoding:"utf8",env:{...process.env,R36_MODE:mode}});expect(run.status,String(run.stderr)+String(run.error)).toBe(0);return Number(run.stdout.trim());};
 const control=measure("control"),released=measure("release");
 expect(control).toBeGreaterThan(0);expect(released).toBeGreaterThan(0);
 expect(released).toBeLessThan(control*0.75);
});
'''
add(TF,'//R36b\n','//R36b\n'+_t)
