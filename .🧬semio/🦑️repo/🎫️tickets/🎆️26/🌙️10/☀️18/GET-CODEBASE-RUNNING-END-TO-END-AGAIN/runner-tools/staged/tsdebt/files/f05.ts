/** 🔐️ Contract: at most one process tree compiles into a Cargo build directory and profile at a time. `-Zfine-grain-locking` lets two Cargo processes whose freshness views differ lock each other for ever; this law reproduces that with the repository's pinned Cargo (negative control, no lease), proves the same pair completes with serialised, non-overlapping compilations under `acquireCargoBuildLeaseV1` (judged by an independent Python interval oracle), and proves the lease semantics: queueing, cancellation, crash recovery, process-tree and in-process sharing, independent profiles and directories. */
import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),root=resolve(here,"../../../../../../../.."),lease=fileURLToPath(new URL("../../🟦️.ts",import.meta.url));
const toolchains=join(root,".cache/b/t/m/r/toolchains"),nightly=existsSync(toolchains)?readdirSync(toolchains).filter(name=>name.startsWith("nightly-")).sort().at(-1):undefined,cargo=nightly?join(toolchains,nightly,"bin/cargo"):undefined;
const scratch=():string=>realpathSync(mkdtempSync(join(tmpdir(),"cu-")));
type Child={process:Bun.Subprocess<"pipe","pipe","pipe">;output:string;done:Promise<number>};
const worker=(directory:string):string=>{const path=join(directory,"worker.ts");writeFileSync(path,`import {acquireCargoBuildLeaseV1} from ${JSON.stringify(lease)};
const [leases,build,target,profile,mode]=process.argv.slice(2),controller=new AbortController(),children=[],owned=[];let finish;const finished=new Promise(accept=>{finish=accept;});
const take=()=>acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,targetDirectory:target,args:["--profile",profile],signal:controller.signal,onWait:()=>console.log("waiting")});
process.stdin.on("data",async data=>{for(const word of String(data).trim().split(/\\s+/)){
 if(word==="cancel")controller.abort();if(word==="release")finish();
 if(word==="child")children.push(Bun.spawn([process.execPath,${JSON.stringify(path)},leases,build,target,profile,"child"],{stdin:"pipe",stdout:"inherit",stderr:"inherit"}));
 if(word==="stop")for(const child of children.splice(0)){child.stdin.write("release\\n");child.stdin.end();await child.exited;console.log("child-gone");}
 if(word==="again"){owned.push(await take());console.log("again-held");}
 if(word==="drop"){owned.shift().release();console.log("dropped");}}});
const timeout=setTimeout(()=>controller.abort(),60000);
try{owned.push(await take());
 try{console.log(mode==="child"?"child-held":"held");await finished;}finally{for(const held of owned.splice(0))held.release();console.log("released");}}
catch(error){if(controller.signal.aborted)console.log("cancelled");else throw error;}finally{clearTimeout(timeout);process.stdin.destroy();}
`);return path;};
const start=(script:string,leases:string,build:string,target:string,profile:string,children:Child[]):Child=>{const process_=Bun.spawn([process.execPath,script,leases,build,target,profile,"root"],{stdin:"pipe",stdout:"pipe",stderr:"pipe"}),child:Child={process:process_,output:"",done:undefined as unknown as Promise<number>};
 const drain=async(stream:ReadableStream<Uint8Array>)=>{for await(const bytes of stream)child.output+=new TextDecoder().decode(bytes);};child.done=Promise.all([drain(process_.stdout),drain(process_.stderr),process_.exited]).then(rows=>rows[2]);children.push(child);return child;};
const until=async(child:Child,event:string,milliseconds=20000)=>{const deadline=Date.now()+milliseconds;while(!child.output.split("\n").includes(event)){expect(child.process.exitCode,child.output).toBeNull();expect(Date.now(),event+": "+child.output).toBeLessThan(deadline);await Bun.sleep(20);}};
const send=(child:Child,word:string)=>{child.process.stdin.write(word+"\n");child.process.stdin.flush();};
const finish=async(child:Child)=>{send(child,"release");child.process.stdin.end();expect(await child.done,child.output).toBe(0);};

test("the build-directory custody queues a second process tree, hands over on release, cancellation and crash, shares with its descendants and its own process, and leaves other profiles and directories alone",async()=>{
 const work=scratch(),leases=join(work,"l"),script=worker(work),children:Child[]=[];mkdirSync(leases);
 try{
  const first=start(script,leases,join(work,"b"),join(work,"t1"),"prototype",children);await until(first,"held");
  const second=start(script,leases,join(work,"b"),join(work,"t2"),"prototype",children);await until(second,"waiting");expect(second.output.split("\n")).not.toContain("held");
  send(first,"child");await until(first,"child-held");
  send(first,"stop");await until(first,"child-gone");expect(second.output.split("\n")).not.toContain("held");
  await finish(first);await until(second,"held");
  const third=start(script,leases,join(work,"b"),join(work,"t3"),"prototype",children);await until(third,"waiting");send(third,"cancel");third.process.stdin.end();expect(await third.done,third.output).toBe(0);expect(third.output.split("\n")).not.toContain("held");expect(third.output.split("\n")).toContain("cancelled");
  const crash=start(script,leases,join(work,"b"),join(work,"t4"),"prototype",children);await until(crash,"waiting");
  const otherProfile=start(script,leases,join(work,"b"),join(work,"t5"),"release",children);await until(otherProfile,"held");
  const otherDirectory=start(script,leases,join(work,"b2"),join(work,"t6"),"prototype",children);await until(otherDirectory,"held");
  second.process.kill("SIGKILL");await second.done;await until(crash,"held");
  await finish(crash);await finish(otherProfile);await finish(otherDirectory);
 }finally{for(const child of children)if(child.process.exitCode===null)child.process.kill("SIGKILL");await Promise.all(children.map(child=>child.done));rmSync(work,{recursive:true,force:true});}
},120000);

test("a process tree shares the custody of its root and an own-process second lease shares it too, while an unrelated process waits for both",async()=>{
 const work=scratch(),leases=join(work,"l"),script=worker(work),children:Child[]=[];mkdirSync(leases);
 try{
  const root_=start(script,leases,join(work,"b"),join(work,"t1"),"prototype",children);await until(root_,"held");
  send(root_,"child");await until(root_,"child-held");
  const outsider=start(script,leases,join(work,"b"),join(work,"t2"),"prototype",children);await until(outsider,"waiting");expect(outsider.output.split("\n")).not.toContain("held");
  send(root_,"stop");await until(root_,"child-gone");expect(outsider.output.split("\n")).not.toContain("held");
  send(root_,"again");await until(root_,"again-held");send(root_,"drop");await until(root_,"dropped");await Bun.sleep(300);expect(outsider.output.split("\n")).not.toContain("held");
  await finish(root_);await until(outsider,"held");await finish(outsider);
 }finally{for(const child of children)if(child.process.exitCode===null)child.process.kill("SIGKILL");await Promise.all(children.map(child=>child.done));rmSync(work,{recursive:true,force:true});}
},120000);

test.skipIf(!process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)("a queued custody wait under a real issued admission control completes once the holder releases (the control signal is the lease signal)",async()=>{
 const work=scratch(),leases=join(work,"l"),script=worker(work),children:Child[]=[];mkdirSync(leases);
 try{
  await receiveScriptProcessInvocation(process.env,async original=>{
   const control=scriptCompilerAdmissionControlV1(original,"test:cargo-custody",120000),holder=start(script,leases,join(work,"b"),join(work,"t1"),"prototype",children);await until(holder,"held");
   const waiting=acquireCargoBuildLeaseV1({directory:leases,buildDirectory:join(work,"b"),targetDirectory:join(work,"t2"),args:["--profile","prototype"],signal:control.signal,control});let settled=false;waiting.then(()=>{settled=true;},()=>{settled=true;});
   await Bun.sleep(1500);expect(settled).toBe(false);await finish(holder);
   const lease=await waiting;lease.release();
  });
 }finally{for(const child of children)if(child.process.exitCode===null)child.process.kill("SIGKILL");await Promise.all(children.map(child=>child.done));rmSync(work,{recursive:true,force:true});}
},180000);

const probe={
 "Cargo.toml":'[workspace]\nresolver = "3"\nmembers = ["x", "y", "t"]\n[profile.dev]\ndebug = 0\nincremental = false\n',
 ".cargo/config.toml":'[unstable]\nembed-metadata = "DoNotEmbed"\nfine-grain-locking = true\nbuild-dir-new-layout = true\nchecksum-freshness = true\n',
 "x/Cargo.toml":'[package]\nname = "x"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n',"x/build.rs":'fn main() { let _ = option_env!("XVAR"); }\n',"x/src/lib.rs":"pub fn f() -> u32 { 1 }\n",
 "y/Cargo.toml":'[package]\nname = "y"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n',"y/build.rs":'fn main() { let _ = option_env!("YVAR"); }\n',"y/src/lib.rs":"pub fn f() -> u32 { 1 }\n",
 "t/Cargo.toml":'[package]\nname = "t"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nx = { path = "../x" }\ny = { path = "../y" }\n',"t/src/lib.rs":"pub fn f() -> u32 { 1 }\n",
};
const prepare=():string=>{const work=scratch();for(const [name,text] of Object.entries(probe)){mkdirSync(dirname(join(work,name)),{recursive:true});writeFileSync(join(work,name),text);}return work;};
const cargoEnvironment=(work:string,variant:"a"|"b"|"base",target:string)=>({...process.env,PATH:dirname(cargo!)+":"+(process.env.PATH??""),RUSTUP_TOOLCHAIN:"",RUSTC:join(dirname(cargo!),"rustc"),RUSTC_WRAPPER:"",CARGO_TARGET_DIR:join(work,target),CARGO_BUILD_BUILD_DIR:join(work,"b"),XVAR:variant==="a"?"1":"0",YVAR:variant==="b"?"1":"0"});
const baseline=(work:string)=>new Promise<number>(accept=>{spawn(cargo!,["build","-p","t"],{cwd:work,env:cargoEnvironment(work,"base","t-base"),stdio:"ignore"}).once("close",code=>accept(code??1));});

test.skipIf(!cargo||!existsSync(cargo)||!python)("two Cargo processes with opposite freshness views on one build directory sleep for ever without the lease (negative control) and finish one after the other with it, as judged by an independent interval oracle",async()=>{
 const work=prepare(),log=join(work,"log.txt");writeFileSync(log,"");
 try{
  let deadlocked=false;
  for(let round=0;round<14&&!deadlocked;round++){
   expect(await baseline(work)).toBe(0);
   const pair=(["a","b"] as const).map(variant=>spawn("sh",["-c",'read go; exec "$@"',"sh",cargo!,"build","-p","t"],{cwd:work,env:cargoEnvironment(work,variant,"t-"+variant),stdio:["pipe","ignore","ignore"]}));
   for(const child of pair)child.stdin!.write("go\n");
   const exits=pair.map(child=>new Promise<number>(accept=>child.once("close",code=>accept(code??1))));
   const outcome=await Promise.race([Promise.all(exits).then(()=>"finished"),Bun.sleep(25000).then(()=>"asleep")]);
   if(outcome==="asleep"){deadlocked=pair.every(child=>child.exitCode===null&&child.signalCode===null);for(const child of pair)child.kill("SIGKILL");await Promise.all(exits);}
  }
  expect(deadlocked,"the pinned Cargo never locked itself in 14 rounds: the negative control no longer reproduces the hazard").toBe(true);
  const script=join(work,"cargo-worker.ts"),leases=join(work,"l");mkdirSync(leases);
  writeFileSync(script,`import {acquireCargoBuildLeaseV1} from ${JSON.stringify(lease)};import {appendFileSync} from "node:fs";
const [leases,work,variant,log,cargo]=process.argv.slice(2),note=(what)=>appendFileSync(log,what+" "+process.pid+" "+variant+" "+process.hrtime.bigint()+"\\n");
const held=await acquireCargoBuildLeaseV1({directory:leases,buildDirectory:work+"/b",targetDirectory:work+"/t-"+variant,args:[],signal:new AbortController().signal});
try{note("start");const child=Bun.spawn([cargo,"build","-p","t"],{cwd:work,env:{...process.env,PATH:cargo.replace(/\\/cargo$/,"")+":"+process.env.PATH,RUSTUP_TOOLCHAIN:"",RUSTC:cargo.slice(0,-5)+"rustc",RUSTC_WRAPPER:"",CARGO_TARGET_DIR:work+"/t-"+variant,CARGO_BUILD_BUILD_DIR:work+"/b",XVAR:variant==="a"?"1":"0",YVAR:variant==="b"?"1":"0"},stdout:"ignore",stderr:"ignore"});process.exitCode=await child.exited;note("end");}finally{held.release();}
`);
  for(let round=0;round<3;round++){
   expect(await baseline(work)).toBe(0);writeFileSync(log,"");
   const pair=(["a","b"] as const).map(variant=>spawn("sh",["-c",'read go; exec "$@"',"sh",process.execPath,script,leases,work,variant,log,cargo!],{cwd:work,stdio:["pipe","ignore","inherit"]}));
   for(const child of pair)child.stdin!.write("go\n");
   const exits=pair.map(child=>new Promise<number>(accept=>child.once("close",code=>accept(code??1))));
   const outcome=await Promise.race([Promise.all(exits),Bun.sleep(90000).then(()=>"asleep" as const)]);
   if(outcome==="asleep")for(const child of pair)child.kill("SIGKILL");
   expect(outcome,"leased Cargo processes must never sleep for ever").toEqual([0,0]);
   const run=Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),log]),report=JSON.parse(run.stdout.toString().split("\n")[0]!);
   expect(run.exitCode,run.stdout.toString()).toBe(0);expect(report).toEqual({runs:2,unfinished:0,overlaps:0});
  }
  const overlapping=join(work,"overlap.txt");writeFileSync(overlapping,"start 1 a 100\nstart 2 b 150\nend 1 a 200\nend 2 b 250\n");
  expect(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),overlapping]).exitCode).toBe(1);
 }finally{rmSync(work,{recursive:true,force:true});}
},600000);

const laneWorker=`import {appendFileSync} from "node:fs";import {acquireCargoCustodyV1} from ${JSON.stringify(fileURLToPath(new URL("../../🔐️custody/🟦️.ts",import.meta.url)))};
const [leases,build,log]=process.argv.slice(2),custody=await acquireCargoCustodyV1({directory:leases,buildDirectory:build,profile:"prototype",signal:new AbortController().signal}),note=what=>appendFileSync(log,what+" "+process.pid+" x "+process.hrtime.bigint()+"\\n");
note("start");await Bun.sleep(1500);note("end");custody.release();`;
test("builds of distinct lane directories overlap while builds of one directory queue one after the other, as judged by the independent interval oracle (the throughput law of the lanes)",async()=>{
 const work=scratch();
 try{
  const script=join(work,"worker.ts");writeFileSync(script,laneWorker);
  for(const mode of ["one-directory","two-lanes"] as const){
   const leases=join(work,mode+"-leases"),log=join(work,mode+".log");mkdirSync(leases);writeFileSync(log,"");
   const directories=mode==="one-directory"?[join(work,"m/b"),join(work,"m/b")]:[join(work,"m/u/l/d/b"),join(work,"m/u/l/d1/b")];
   const children=directories.map(directory=>Bun.spawn([process.execPath,script,leases,directory,log],{stdout:"pipe",stderr:"pipe"}));
   const exits=await Promise.all(children.map(child=>child.exited));expect(exits,await new Response(children[0]!.stderr).text()).toEqual([0,0]);
   const run=Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),log]),report=JSON.parse(run.stdout.toString().split("\n")[0]!);
   expect(report).toEqual({runs:2,unfinished:0,overlaps:mode==="one-directory"?0:1});
   expect(run.exitCode).toBe(mode==="one-directory"?0:1);
  }
 }finally{rmSync(work,{recursive:true,force:true});}
},60000);
