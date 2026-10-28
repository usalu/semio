/** ♨️ Laws of the warm deriver: over the adversarial corpus `./🔣️.json` the warm program question (reused parsed files, the previous program while the guard holds) answers exactly what a cold derivation answers after every step, with real TypeScript and the planner's own observation check, and an always-armed deriver is wrong where the corpus says; the request protocol (identity, one at a time, idle exit, cancellation, a live endpoint refuses a second server); the detached process refuses an implementation that is not on disk; the memo asks the warm deriver before the derivation child and starts one after it. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,isAbsolute,join} from "node:path";
import ts from "typescript";
import {declaredTypeScriptProjectsV1} from "../../../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import {createTypeScriptProgramReuseV1,type TypeScriptProgramReuseV1} from "../../../../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs";
import {observationsHold,observeDetachedV1,observedTypeScriptHost} from "../../../../../../👁️observation/🟨️.mjs";
import type {MemoRowV1} from "../../../🟦️.ts";
import type {MemoDeriverV1} from "../../🟦️.ts";
import {askWarmDeriverV1,serveWarmDeriverV1,warmClosureStampsHoldV1,warmClosureStampsV1,warmDeriverEndpointV1,warmProgramDeriverV1} from "../🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

const scratch=(prefix:string)=>realpathSync(mkdtempSync(join(tmpdir(),prefix)));
const directoryOf=(path:string)=>{try{return readdirSync(path,{withFileTypes:true}).map(entry=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
/** 🪁️ The program question of the corpus project, asked like the planner's deriver asks it (detached observation, support = rows and layers), with or without a reuse store. */
const deriverOf=(root:string,store?:TypeScriptProgramReuseV1):MemoDeriverV1=>async()=>{
 const asked=await observeDetachedV1(()=>declaredTypeScriptProjectsV1(corpus.projects,root,corpus.projectRoot,{compiler:ts,host:observedTypeScriptHost(ts.sys),readdir:directoryOf,join,dirname,isAbsolute,...(store?{reuse:store}:{})}));
 return{fact:JSON.stringify(asked.value),rows:[...new Set([...asked.rows,...asked.layers.flat()])] as MemoRowV1[]};
};
const question:MemoRowV1={kind:"derived",path:JSON.stringify({program:1,projects:corpus.projects}),fact:""};
const holds=(rows:readonly MemoRowV1[],signal?:AbortSignal)=>observationsHold(rows,64,new Set(),new Set(),signal) as Promise<boolean>;
/** 🔫️ Wraps a store to record whether each question armed it and how TypeScript reused the previous program (`structureIsReused`: 0 not, 1 safe modules, 2 completely). */
const spied=(store:TypeScriptProgramReuseV1,armed:boolean[],reused:number[][]):TypeScriptProgramReuseV1=>({...store,begin:(arm:boolean)=>{armed.push(arm);store.begin(arm);},end:()=>{const done=store.end();reused.push([...done.reused]);return done;}});

async function walk(guarded:boolean):Promise<{wrong:string[];armed:boolean[];reused:number[][]}>{
 const root=scratch("warm-corpus-"),armed:boolean[]=[],wrong:string[]=[],reused:number[][]=[];
 try{
  for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  const store=spied(createTypeScriptProgramReuseV1(),armed,reused),warm=warmProgramDeriverV1(deriverOf(root,store),store,new Map(),guarded?holds:async()=>true);
  await Bun.sleep(30);const first=await warm(question);expect(first.fact).toBe((await deriverOf(root)(question)).fact);armed.length=0;reused.length=0;
  for(const step of corpus.steps){
   await Bun.sleep(30);for(const path of (step as {remove?:string[]}).remove??[])rmSync(join(root,path));for(const[path,text]of Object.entries(step.write??{})){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
   const answer=await warm(question),cold=await deriverOf(root)(question);
   if(answer.fact!==cold.fact)wrong.push(step.name);
  }
  return{wrong,armed,reused};
 }finally{rmSync(root,{recursive:true,force:true});}
}

test("after every step of the corpus the warm deriver answers exactly the cold derivation, reusing the previous program exactly where the guard holds, and an always-armed deriver is wrong where the corpus says",async()=>{
 const guarded=await walk(true);
 expect(guarded.wrong).toEqual([]);
 expect(guarded.armed).toEqual(corpus.steps.map(step=>step.armed));
 expect(guarded.reused.map(row=>row.length>0&&row.every(kind=>kind>0))).toEqual(corpus.steps.map(step=>step.armed));
 expect(guarded.reused[0]).toEqual([2]);
 const unguarded=await walk(false);
 for(const name of corpus.unguarded)expect(unguarded.wrong).toContain(name);
},120000);

test("the store parses an unchanged file once and keeps only the files of the latest program",()=>{
 const store=createTypeScriptProgramReuseV1();let parsed=0;const texts:Record<string,string>={"/a.ts":"a","/b.ts":"b"};
 const original=(name:string)=>{parsed++;return{fileName:name,text:texts[name]};};
 store.begin(false);let get=store.sourceFile("c",original,name=>texts[name]);get("/a.ts",99);get("/b.ts",99);store.end();
 store.begin(false);get=store.sourceFile("c",original,name=>texts[name]);get("/a.ts",99);texts["/b.ts"]="b2";get("/b.ts",99);store.end();
 expect(parsed).toBe(3);
 store.begin(false);get=store.sourceFile("c",original,name=>texts[name]);get("/b.ts",99);store.end();
 store.begin(false);get=store.sourceFile("c",original,name=>texts[name]);get("/a.ts",99);store.end();
 expect(parsed).toBe(4);
 store.begin(false);expect(store.oldProgram("c")).toBeUndefined();store.keep("c",{getSourceFiles:()=>[{fileName:"/a.ts"}]});store.end();
 store.begin(true);expect(store.oldProgram("c")).toBeDefined();expect(store.end().sources.has("/a.ts")).toBe(false);
});

test("a parsed file is reused only under the compiler options it was parsed with",()=>{
 const store=createTypeScriptProgramReuseV1();let parsed=0;const texts:Record<string,string>={"/a.ts":"a"};
 const original=(name:string)=>{parsed++;return{fileName:name,text:texts[name]};};
 for(const settings of ["{}","{}","{\"importHelpers\":true}","{\"importHelpers\":true}","{}"]){store.begin(false);store.sourceFile("c",original,name=>texts[name],settings)("/a.ts",99);store.end();}
 expect(parsed).toBe(3);
});

test("the stamps of a captured closure hold until a file of it moves on disk",()=>{
 const directory=scratch("warm-stamps-");
 try{
  writeFileSync(join(directory,"a.ts"),"export {};\n");writeFileSync(join(directory,"b.ts"),"export {};\n");
  const closure={root:directory,sources:[{path:"a.ts",sha256:""},{path:"b.ts",sha256:""}]},stamps=warmClosureStampsV1(closure);
  expect(warmClosureStampsHoldV1(closure,stamps)).toBe(true);
  writeFileSync(join(directory,"b.ts"),"export {};\n");expect(warmClosureStampsHoldV1(closure,stamps)).toBe(false);
  const again=warmClosureStampsV1(closure);rmSync(join(directory,"a.ts"));expect(warmClosureStampsHoldV1(closure,again)).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the request protocol: answers its identity one request at a time, refuses another identity, a second server and a cancelled request, falls back without a server and closes when idle",async()=>{
 const directory=scratch("warm-proto-"),endpoint=warmDeriverEndpointV1(directory,"id-"+process.pid,process.platform,directory);let running=0,most=0,closed=false;
 try{
  const server=await serveWarmDeriverV1({endpoint,identity:"one",idleMilliseconds:1500,onClose:()=>{closed=true;},answer:async request=>{running++;most=Math.max(most,running);await Bun.sleep(150);writeFileSync(request+".answer","held");running--;}});
  const ask=(identity:string,request:string,signal?:AbortSignal)=>askWarmDeriverV1({endpoint,identity,request,connectMilliseconds:250,answerMilliseconds:5000,...(signal?{signal}:{})});
  const answers=await Promise.all([ask("one",join(directory,"r1")),ask("one",join(directory,"r2")),ask("two",join(directory,"r3"))]);
  expect(answers).toEqual([true,true,false]);expect(most).toBe(1);
  expect(existsSync(join(directory,"r1.answer"))&&existsSync(join(directory,"r2.answer"))&&!existsSync(join(directory,"r3.answer"))).toBe(true);
  const abort=new AbortController();setTimeout(()=>abort.abort(),20);expect(await ask("one",join(directory,"r4"),abort.signal)).toBe(false);
  await expect(serveWarmDeriverV1({endpoint,identity:"one",idleMilliseconds:1000,answer:async()=>{}})).rejects.toThrow("already serves");
  await Bun.sleep(2200);expect(closed).toBe(true);
  expect(await ask("one",join(directory,"r5"))).toBe(false);
  await server.close();
 }finally{rmSync(directory,{recursive:true,force:true});}
},20000);

test("the detached process refuses an implementation whose captured bytes are not on disk and leaves nothing behind",()=>{
 const directory=scratch("warm-main-"),state=join(directory,"state"),file=join(directory,"module.ts");mkdirSync(state);writeFileSync(file,"export {};\n");
 writeFileSync(join(state,"closure.json"),JSON.stringify({root:directory,sources:[{path:"module.ts",sha256:"0".repeat(64)}]}));writeFileSync(join(state,"request.json"),"{}");
 const endpoint=warmDeriverEndpointV1(directory,"refused",process.platform,directory);
 const run=spawnSync(process.execPath,[join(import.meta.dir,"..","⌨️main","🟦️.ts"),"--root",directory,"--identity","refused","--endpoint",endpoint,"--directory",state],{encoding:"utf8",timeout:60000});
 expect(run.status,run.stderr).toBe(3);expect(existsSync(state)).toBe(false);expect(existsSync(endpoint)).toBe(false);
 rmSync(directory,{recursive:true,force:true});
},60000);

test("wiring: the memo asks the warm deriver before the derivation child and starts one after a cold answer; the program question takes the installed store",()=>{
 const glue=readFileSync(join(import.meta.dir,"..","..","..","🚪️command","🟦️.ts"),"utf8"),inputs=readFileSync(join(import.meta.dir,"..","..","..","..","..","..","🕸️graph","📥️inference","🧭️script-inputs","🟨️.mjs"),"utf8");
 const ask=glue.indexOf("if(warm&&await warm.ask(file))"),child=glue.indexOf("await child({[GRAPH_MEMO_POLICY_V1.derive]:file})"),start=glue.indexOf("warm?.start(file)");
 expect(ask>0&&child>ask&&start>child).toBe(true);
 const main=readFileSync(join(import.meta.dir,"..","⌨️main","🟦️.ts"),"utf8");expect(main).toContain("if(!warmClosureStampsHoldV1(closure,stamps)||process.memoryUsage().rss>GRAPH_MEMO_POLICY_V1.warm.maximumResidentBytes)void server?.close();");
 expect(inputs).toContain("reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, JSON.stringify(options))");
 expect(inputs).toContain("reuse: activeTypeScriptProgramReuseV1()");expect(inputs).toContain("...(oldProgram ? { oldProgram } : {})");expect(inputs).toContain("reuse?.keep(config, program);");
});
