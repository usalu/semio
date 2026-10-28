"""🧹 [audit7] Appends the law of batch `reclaim` to the farm-close suite (anchored; idempotent). usage: edit_law.py <root>"""
import os, sys
root = sys.argv[1]
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🔒️farm-close/🟦️.ts"
IMPORT = ('import{join}from"node:path";', 'import{join}from"node:path";\nimport{pathToFileURL}from"node:url";')
LAW = r'''
/** 🧟️ Runs one farm in a process of its own that starts its pool from a captured snapshot and then either closes it or is killed before it can, and answers the farm directories it created. */
const farmProcess=(temporary:string,killed:boolean):string[]=>{
 const plan=JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536}),farm=pathToFileURL(join(import.meta.dir,"..","..","🟨️.mjs")).href;
 const code=`const{EventEmitter}=await import("node:events"),fs=await import("node:fs"),{openProjectFarmV1}=await import(${JSON.stringify(farm)}),temporary=${JSON.stringify(temporary)},before=new Set(fs.readdirSync(temporary));class Child extends EventEmitter{connected=true;exitCode=null;signalCode=null;send(message){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}}const farm=openProjectFarmV1({workspaceRoot:temporary,context:{registry:null},contracts:{},projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:${JSON.stringify(plan)}},processed:()=>24,cores:10,write:()=>undefined,launch:{command:"/semio-law/runtime",arguments:[],environment:{},implementation:"d".repeat(64),text:()=>"captured snapshot text"},spawnProcess:()=>new Child(),request:()=>({granted:Promise.resolve(undefined),release:()=>undefined})});for(let index=0;index<24;index++)await farm.prepare({name:"m"+index,root:"m",json:{},native:false,pipeline:false,nativeRoot:"m",selectedInputs:[],head:[],cacheKey:"0".repeat(24)});fs.writeSync(1,JSON.stringify(fs.readdirSync(temporary).filter(name=>name.startsWith("semio-")&&!before.has(name))));if(${killed})process.kill(process.pid,"SIGKILL");await farm.close();`;
 const run=Bun.spawnSync([process.execPath,"-e",code],{env:{...process.env,TMPDIR:temporary},stderr:"pipe"});
 if(!killed)expect(run.exitCode,run.stderr.toString().slice(-2000)).toBe(0);
 return JSON.parse(run.stdout.toString()||"[]") as string[];
};

test("a farm removes the snapshot copy, staging and session directories a killed predecessor left behind, and keeps those of a running process and every name it does not write",()=>{
 const temporary=realpathSync(mkdtempSync(join(tmpdir(),"farm-reclaim-"))),running=`semio-graph-farm-image-${process.pid}-a7live`,foreign="semio-graph-farm-image-1-short",kept=[running,foreign];
 for(const name of kept)mkdirSync(join(temporary,name),{recursive:true});
 const left=farmProcess(temporary,true);
 try{
  expect(left.length,"the killed farm left its directories").toBeGreaterThanOrEqual(3);
  expect(left.every(name=>existsSync(join(temporary,name)))).toBe(true);
  farmProcess(temporary,false);
  expect(left.filter(name=>existsSync(join(temporary,name))),"the successor removed what the killed farm left").toEqual([]);
  expect(kept.filter(name=>existsSync(join(temporary,name))),"directories of running processes and foreign names stay").toEqual(kept);
 }finally{rmSync(temporary,{recursive:true,force:true});}
},60000);
'''
path = os.path.join(root, REL)
text = open(path, encoding="utf8").read()
if "a farm removes the snapshot copy, staging and session directories a killed predecessor left behind" in text: sys.exit(0)
if text.count(IMPORT[0]) != 1: print("[audit7] import anchor lost"); sys.exit(1)
old_fs = 'import{existsSync,mkdtempSync,readdirSync,realpathSync,rmSync}from"node:fs";'
if text.count(old_fs) != 1: print("[audit7] fs import anchor lost"); sys.exit(1)
text = text.replace(old_fs, 'import{existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync}from"node:fs";').replace(IMPORT[0], IMPORT[1]).rstrip("\n") + "\n" + LAW
mode = os.stat(path).st_mode & 0o7777
with open(path, "w", encoding="utf8") as handle: handle.write(text)
os.chmod(path, mode)
