/** 🛂️ Contract: a task whose process admitted the sources of the build implementation earlier admits the current closure for itself when a peer moved it, retries a closure that keeps changing under its own capture a bounded number of times with the declared pauses, and ends with the refusal `closure.moving` (exit status 75) only when it never holds still; files outside the list never take part; an independent Python oracle agrees on the moved files and on the pauses. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdtemp,mkdir,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {BUILD_TEXTS_V1,refuseBuildV1} from "../../../../📖️texts/🟦️.ts";
import {handleTextV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorExitCodeV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {concurrentEditOfV1,concurrentEditV1,STALE_IMPLEMENTATION_CODES_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {CLOSURE_READMISSION_POLICY_V1 as policy,closureMovedErrorV1,listedClosureFilesV1,movedClosureFilesV1,readmitMovedClosureV1,tornClosureCaptureV1,withMovedClosureReadmittedV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {nxFailureChurnV1} from "../../../../🔧️tools/🧶️nx/🪵️failure-tail/🟦️.ts";
import {refusalClassOfTextV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧾️refusals/🟦️.ts";
import refusals from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🧾️refusals.json";
import {readFileSync} from "node:fs";

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0),here=fileURLToPath(new URL(".",import.meta.url)),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const area=async()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();await mkdir(base,{recursive:true});return mkdtemp(join(base,"rm-"));};
const torn=()=>new Error("Authorizer source changed during capture");
type Log={sleeps:number[];said:{key:string;values:Record<string,string|number>}[]};
const drive=async<T>(failures:number,extra:{origin?:()=>Promise<string|undefined>;error?:()=>Error;files?:string[];announce?:boolean;signal?:AbortSignal}={})=>{
 const log:Log={sleeps:[],said:[]};let calls=0;
 const result=await readmitMovedClosureV1<string>({capture:async()=>{calls++;if(calls<=failures)throw (extra.error??torn)();return "snapshot-"+calls;},moved:async()=>extra.files??["a/x.ts","a/y.ts"],sleep:async ms=>{log.sleeps.push(ms);},say:(key,values)=>{log.said.push({key,values:{...values}});},origin:extra.origin,announce:extra.announce,signal:extra.signal}).then(value=>({value}),error=>({error:error as Error}));
 return{...result,log,calls};
};

test("the schema is closed and its numbers are consistent: six attempts (five retries), one pause less than attempts, growing, every signal named once",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validateSchema(schema),JSON.stringify(ajv.errors)).toBe(true);
 expect(policy.maximumAttempts).toBe(6);expect(policy.backoffMs).toHaveLength(policy.maximumAttempts-1);
 expect([...policy.backoffMs]).toEqual([...policy.backoffMs].sort((a,b)=>a-b));expect(new Set(policy.backoffMs).size).toBe(policy.backoffMs.length);
 for(const row of [...policy.movedSignals,...policy.tornSignals])expect(row.value.length).toBeGreaterThan(8);
 expect(new Set([...policy.movedSignals,...policy.tornSignals].map(row=>row.value)).size).toBe(policy.movedSignals.length+policy.tornSignals.length);
 expect(policy.maximumListedFiles).toBeGreaterThan(0);
});

test("the moved files are exactly those of the admitted list that differ from the disk or are gone, in byte order, and a file outside the list never takes part; an independent Python oracle agrees",async()=>{
 const root=await area();
 try{
  const files:Record<string,string>={"a.ts":"export const a=1;\n","b/c.ts":"export const c=1;\n","b/d.ts":"export const d=1;\n","é.ts":"export const e=1;\n","gone.ts":"x\n"};
  await mkdir(join(root,"b"),{recursive:true});for(const[path,text]of Object.entries(files))await writeFile(join(root,path),text);
  const rows=Object.entries(files).map(([path,text])=>({path,sha256:sha(text)}));
  await writeFile(join(root,"unrelated.ts"),"export const u=1;\n");
  expect(await movedClosureFilesV1(root,rows)).toEqual([]);
  await writeFile(join(root,"unrelated.ts"),"export const u=2;\n");await writeFile(join(root,"new-file.ts"),"export const n=1;\n");
  expect(await movedClosureFilesV1(root,rows),"edits outside the list change nothing").toEqual([]);
  await writeFile(join(root,"b/d.ts"),"export const d=2;\n");await writeFile(join(root,"é.ts"),"export const e=2;\n");await rm(join(root,"gone.ts"));
  const own=await movedClosureFilesV1(root,rows);
  expect(own).toEqual(["b/d.ts","gone.ts","é.ts"]);
  const rowsFile=join(root,"rows.json");await writeFile(rowsFile,JSON.stringify(rows));
  if(python){const run=spawnSync(python,["-I",here+"🐍️.py",root,rowsFile,fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),"0"],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);expect(JSON.parse(run.stdout).moved).toEqual([...own]);}
  const bounded=await movedClosureFilesV1(root,rows,undefined,2);expect(bounded).toHaveLength(2);for(const path of bounded)expect(own).toContain(path);
 }finally{await rm(root,{recursive:true,force:true});}
});

test("an error says the list is outdated or the capture was torn by its own words; anything else is neither and passes through",()=>{
 const differs=refuseBuildV1("stale","closure.source-differs",{path:"p",expectedSha256:"a",observedSha256:"b"}),notAdmitted=refuseBuildV1("refused","closure.source-not-admitted",{path:"p"});
 expect(closureMovedErrorV1(differs)).toBe(true);expect(closureMovedErrorV1(notAdmitted)).toBe(true);expect(closureMovedErrorV1(new Error("Admitted implementation declaring image and import required"))).toBe(true);
 expect(closureMovedErrorV1(new Error("External implementation runtime dependency refused: node_modules/x"))).toBe(false);expect(closureMovedErrorV1("string")).toBe(false);
 for(const row of policy.tornSignals)expect(tornClosureCaptureV1(new Error("x "+row.value+" y")),row.value).toBe(true);
 expect(tornClosureCaptureV1(concurrentEditV1("bytes","/p"))).toBe(true);expect(tornClosureCaptureV1(Object.assign(new Error("ENOENT: no such file or directory, open '/p'"),{code:"ENOENT"}))).toBe(true);expect(tornClosureCaptureV1(Object.assign(new Error("EACCES"),{code:"EACCES"}))).toBe(false);
 expect(tornClosureCaptureV1(differs)).toBe(false);expect(tornClosureCaptureV1(new Error("Complete immutable compiler import required: ./x"))).toBe(false);
});

test("a closure that holds still after k torn captures is admitted on attempt k+1 with exactly the declared pauses, and an independent oracle agrees for every k",async()=>{
 for(let k=0;k<policy.maximumAttempts;k++){
  const run=await drive(k);
  expect("value"in run&&run.value,"k="+k).toBe("snapshot-"+(k+1));expect(run.calls).toBe(k+1);expect(run.log.sleeps).toEqual(policy.backoffMs.slice(0,k));
  expect(run.log.said.map(row=>row.key)).toEqual(["closure.readmitted",...Array(k).fill("closure.readmit-retry")]);
  if(python){const run2=spawnSync(python,["-I",here+"🐍️.py",here,"/dev/stdin",fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),String(k)],{encoding:"utf8",input:"[]"});expect(run2.status,run2.stderr).toBe(0);const oracle=JSON.parse(run2.stdout);expect(oracle.pauses).toEqual(run.log.sleeps);expect(oracle.refused).toBe(false);expect(oracle.ran).toBe(run.calls);}
 }
});

test("a closure that never holds still ends after six attempts (five retries) with the refusal that exits 75, naming the files and the writer, and nothing runs with a mixed version",async()=>{
 const run=await drive(99,{origin:async()=>"g1-m02",files:["a/1.ts","a/2.ts","a/3.ts","a/4.ts","a/5.ts"]});
 const error=(run as{error:Error}).error as Error&{code:string;key:string;values:Record<string,string|number>;cause?:Error};
 expect(run.calls).toBe(policy.maximumAttempts);expect(run.log.sleeps).toEqual([...policy.backoffMs]);
 expect(error.key).toBe("build.closure.moving");expect(error.code).toBe("unserved");expect(repoErrorExitCodeV1(error.code as "unserved")).toBe(75);
 expect(error.values).toEqual({count:5,files:"a/1.ts, a/2.ts, a/3.ts, a/4.ts",origin:"g1-m02",attempts:6});expect(error.cause?.message).toContain("changed during capture");
 for(const language of ["en","de"] as const){const text=handleTextV1(BUILD_TEXTS_V1,"closure.moving",language,error.values);expect(text).toContain("a/1.ts");expect(text).toContain("g1-m02");}
 expect(handleTextV1(BUILD_TEXTS_V1,"closure.moving","en",error.values)).toContain("try again later");expect(handleTextV1(BUILD_TEXTS_V1,"closure.moving","de",error.values)).toContain("später");
 const retry=run.log.said.filter(row=>row.key==="closure.readmit-retry");expect(retry).toHaveLength(policy.maximumAttempts-1);expect(retry[0]!.values).toMatchObject({attempt:1,maximum:6,seconds:0.25,origin:"g1-m02",count:5});
 if(python){const oracle=JSON.parse(spawnSync(python,["-I",here+"🐍️.py",here,"/dev/stdin",fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),"99"],{encoding:"utf8",input:"[]"}).stdout);expect(oracle.refused).toBe(true);expect(oracle.pauses).toEqual(run.log.sleeps);expect(oracle.ran).toBe(run.calls);}
});

test("an error that is not a torn capture ends the task at once without a pause, a missing writer is a dash, and the announcement is made once per moved closure",async()=>{
 const other=await drive(1,{error:()=>new Error("Complete immutable compiler import required: ./x")});
 expect((other as{error:Error}).error.message).toContain("Complete immutable");expect(other.calls).toBe(1);expect(other.log.sleeps).toEqual([]);
 const quiet=await drive(0,{announce:false});expect(quiet.log.said).toEqual([]);expect("value"in quiet).toBe(true);
 const named=await drive(0,{});expect(named.log.said[0]).toEqual({key:"closure.readmitted",values:{count:2,files:"a/x.ts, a/y.ts",origin:"-"}});
 const failing=await drive(0,{origin:async()=>{throw new Error("no replica");}});expect(failing.log.said[0]!.values.origin).toBe("-");
 expect(listedClosureFilesV1([])).toEqual({count:0,files:""});
});

test("cancellation ends the wait: an aborted signal stops the next attempt without another capture",async()=>{
 const controller=new AbortController();let calls=0;const log:number[]=[];
 const failure=await readmitMovedClosureV1<string>({capture:async()=>{calls++;throw torn();},moved:async()=>["a.ts"],sleep:async ms=>{log.push(ms);controller.abort(new Error("cancelled"));},say:()=>undefined,signal:controller.signal}).catch(error=>error as Error);
 expect(failure.message).toBe("cancelled");expect(calls).toBe(1);expect(log).toEqual([250]);
});

test("the parent classifies what a task prints when the closure still moved: an outdated admission restarts the command, a closure that never held still runs the build again, and the declared sites hold the texts",()=>{
 const differs=refuseBuildV1("stale","closure.source-differs",{path:"p",expectedSha256:"a",observedSha256:"b"}),notAdmitted=refuseBuildV1("refused","closure.source-not-admitted",{path:"p"}),moving=refuseBuildV1("unserved","closure.moving",{count:1,files:"p",origin:"-",attempts:6});
 for(const[error,id,disposition]of[[differs,"moved-source-closure","reexec"],[notAdmitted,"moved-source-closure","reexec"],[moving,"closure-moving","reobserve"]]as const){const hit=refusalClassOfTextV1("task output\nRepoError: "+error.message);expect(hit?.id,id).toBe(id);expect(hit?.disposition).toBe(disposition);}
 const stale=nxFailureChurnV1("RepoError: "+differs.message,"nx run a:b") as Error&{code:string};expect(STALE_IMPLEMENTATION_CODES_V1.has(stale.code)).toBe(true);
 expect(concurrentEditOfV1(nxFailureChurnV1("RepoError: "+moving.message,"nx run a:b"))).toMatchObject({reason:"input",path:"nx run a:b"});
 const table=refusals.definitions.Table.const.classes.filter(row=>row.id==="moved-source-closure"||row.id==="closure-moving");expect(table).toHaveLength(2);
 for(const row of table)for(const message of row.messages)for(const site of message.sites)expect(readFileSync(fileURLToPath(new URL("../../../../../../../../../../"+site,import.meta.url)),"utf8"),message.text).toContain(message.text);
});

type Run=(sources:unknown,resolutions:unknown,fresh:boolean)=>Promise<string>;
const moved=()=>refuseBuildV1("stale","closure.source-differs",{path:"a/x.ts",expectedSha256:"a",observedSha256:"b"});
const movable=(entry:string,run:Run,scope:{kind:"closure"}|{kind:"static";roots:string[];leaves:string[]}={kind:"closure"})=>{
 const log={fresh:[] as string[][],sleeps:[] as number[],said:[] as string[]};
 const deps={fresh:async(_root:string,seeds:readonly string[])=>{log.fresh.push([...seeds]);return{sources:"fresh-sources-"+log.fresh.length,resolutions:"fresh-resolutions-"+log.fresh.length};},moved:async()=>["a/x.ts"],sleep:async(ms:number)=>{log.sleeps.push(ms);},say:(key:string)=>{log.said.push(key);}};
 return{log,call:()=>withMovedClosureReadmittedV1({workspace:"/w",entry,scope,signal:new AbortController().signal,baked:{sources:"baked-sources",resolutions:"baked-resolutions"},run},deps)};
};

test("a capture against the admitted list that holds goes through untouched: no fresh admission, no announcement",async()=>{
 const seen:string[]=[],task=movable("e/hold.ts",async(sources,_r,fresh)=>{seen.push(sources+"|"+fresh);return "ok";});
 expect(await task.call()).toBe("ok");expect(seen).toEqual(["baked-sources|false"]);expect(task.log.fresh).toEqual([]);expect(task.log.said).toEqual([]);
});

test("a moved closure is admitted again from the disk with the seeds of its scope, announced once, kept for the next capture of the process, and dropped when it moves again",async()=>{
 let phase=0;const seen:string[]=[],task=movable("e/moved.ts",async(sources,_r,fresh)=>{seen.push(sources+"|"+fresh);if(sources==="baked-sources"||phase===1&&sources==="fresh-sources-1")throw moved();return "ok:"+sources;},{kind:"static",roots:["r/one.ts","r/two.ts"],leaves:["l/leaf.json"]});
 expect(await task.call()).toBe("ok:fresh-sources-1");expect(task.log.fresh).toEqual([["e/moved.ts","r/one.ts","r/two.ts","l/leaf.json"]]);expect(task.log.said).toEqual(["closure.readmitted"]);
 expect(await task.call()).toBe("ok:fresh-sources-1");expect(task.log.fresh).toHaveLength(1);expect(task.log.said).toHaveLength(1);
 phase=1;expect(await task.call()).toBe("ok:fresh-sources-2");expect(task.log.fresh).toHaveLength(2);expect(task.log.said).toHaveLength(1);
 expect(seen).toEqual(["baked-sources|false","fresh-sources-1|true","fresh-sources-1|true","fresh-sources-1|true","fresh-sources-2|true"]);
});

test("a fresh admission that moves again under its own capture is retried with the declared pauses and then refused as `unserved`; a different error ends the task at once",async()=>{
 const task=movable("e/never.ts",async()=>{throw moved();}),failure=await task.call().catch(error=>error as Error&{code:string;key:string});
 expect(failure.key).toBe("build.closure.moving");expect(failure.code).toBe("unserved");expect(task.log.fresh).toHaveLength(policy.maximumAttempts);expect(task.log.sleeps).toEqual([...policy.backoffMs]);
 expect(task.log.said.filter(key=>key==="closure.readmit-retry")).toHaveLength(policy.maximumAttempts-1);
 let calls=0;const settles=movable("e/settles.ts",async(sources)=>{calls++;if(sources==="baked-sources"||calls<=3)throw moved();return "ok";});
 expect(await settles.call()).toBe("ok");expect(settles.log.fresh).toHaveLength(3);expect(settles.log.sleeps).toEqual(policy.backoffMs.slice(0,2));
 const other=movable("e/other.ts",async()=>{throw new Error("External implementation runtime dependency refused: node_modules/x");});
 expect((await other.call().catch(error=>error as Error)).message).toContain("External");expect(other.log.fresh).toEqual([]);expect(other.log.sleeps).toEqual([]);
});

test("a vanished file during the fresh admission is a torn capture and is retried, while an error of the fresh admission that names no movement ends the task at once",async()=>{
 let calls=0;
 const depsFresh=async()=>{calls++;if(calls<3)throw Object.assign(new Error("ENOENT: no such file or directory, open '/w/a.ts'"),{code:"ENOENT"});return{sources:"s",resolutions:"r"};};
 const log={sleeps:[] as number[]};
 const value=await withMovedClosureReadmittedV1({workspace:"/w",entry:"e/vanished.ts",scope:{kind:"closure"},signal:new AbortController().signal,baked:{sources:"baked-sources",resolutions:"baked-resolutions"},run:async(sources)=>{if(sources==="baked-sources")throw moved();return "ok";}},{fresh:depsFresh,moved:async()=>[],sleep:async ms=>{log.sleeps.push(ms);},say:()=>undefined});
 expect(value).toBe("ok");expect(calls).toBe(3);expect(log.sleeps).toEqual(policy.backoffMs.slice(0,2));
 const broken=await withMovedClosureReadmittedV1({workspace:"/w",entry:"e/broken.ts",scope:{kind:"closure"},signal:new AbortController().signal,baked:{sources:"baked-sources",resolutions:"baked-resolutions"},run:async()=>{throw moved();}},{fresh:async()=>{throw new Error("Complete immutable compiler import required: ./x");},moved:async()=>[],sleep:async ms=>{log.sleeps.push(ms);},say:()=>undefined}).catch(error=>error as Error);
 expect(broken.message).toContain("Complete immutable compiler import required");expect(log.sleeps).toHaveLength(2);
});
