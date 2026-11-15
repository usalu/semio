import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {performance} from "node:perf_hooks";

/** ⏱️ Restart benchmark of the shared-worktree replica: run with `bun 1113-bench-restart.ts <module-root> <phases|load|checkpoint|e2e>`. `<module-root>` is the `🤝️shared-worktree` directory of the code under test (the live one for the baseline, a temporary copy for the new code). The live state of `SEMIO_BENCH_LIVE` (default `/Users/Shared/semio`) is only read: its `.cache/f/*` is copied into a temporary worktree, the walk reads the live tree, git runs against copies of the index and a private object directory. */
const [root="",mode="phases"]=process.argv.slice(2);
const live=process.env.SEMIO_BENCH_LIVE??"/Users/Shared/semio",store=process.env.SEMIO_BENCH_STORE??"/Users/Shared/semio-store";
const mod=async<T>(path:string):Promise<T>=>await import(join(root,path)) as T;
const ms=(from:number):string=>`${(performance.now()-from).toFixed(0)} ms`;
const mib=(bytes:number):string=>`${(bytes/1048576).toFixed(1)} MiB`;
const rss=():string=>mib(process.memoryUsage().rss);
const row=(name:string,value:string):void=>{process.stdout.write(`${name.padEnd(44)} ${value}\n`);};

function stalls():Readonly<{stop:()=>number}>{
 let last=performance.now(),worst=0;
 const timer=setInterval(()=>{const now=performance.now();worst=Math.max(worst,now-last-1);last=now;},1);
 return {stop:()=>{clearInterval(timer);return worst;}};
}

function stage():string{
 const base=mkdtempSync(join(tmpdir(),"bench-restart-")),cache=join(base,".cache","f");
 mkdirSync(cache,{recursive:true});
 const source=join(live,".cache","f");
 for(const name of ["state.json","state.log","state.ckp","state.wal","cursor.json","clock.json","machine.json"])if(existsSync(join(source,name)))cpSync(join(source,name),join(cache,name));
 if(existsSync(join(live,".gitignore")))cpSync(join(live,".gitignore"),join(base,".gitignore"));
 return base;
}

type Replica={settings:{worktree:string};files:{size:number};ignore:unknown;book:{count:number}};
type ReplicaModule={openReplicaV1:(options:object)=>Promise<Replica>;closeReplicaV1:(replica:Replica)=>Promise<void>;walkLocalV1:(replica:Replica)=>Promise<{scanned:number;changed:{length:number}}>};

const options=(worktree:string):object=>({worktree,storeRoot:store,originId:process.env.SEMIO_BENCH_ORIGIN??"g1-server",groupId:"g1",lock:false});

function run(command:string,args:readonly string[],env:Record<string,string>):Promise<Readonly<{code:number|null;out:number}>>{
 return new Promise(done=>{
  const child=spawn(command,args,{env:{...process.env,...env},stdio:["ignore","pipe","ignore"]});
  let out=0;
  child.stdout.on("data",(chunk:Buffer)=>{out+=chunk.length;});
  child.on("close",code=>{done({code,out});});
 });
}

async function phases():Promise<void>{
 const replicaModule=await mod<ReplicaModule>("🔁️replica/🟦️.ts"),worktree=stage();
 row("rss before open",rss());
 let mark=performance.now();
 const probe=stalls(),replica=await replicaModule.openReplicaV1(options(worktree));
 row("openReplicaV1 (load+folds+requeue+tail)",ms(mark));
 row("  longest event-loop stall during open",`${probe.stop().toFixed(0)} ms`);
 row("  files / paths",`${replica.files.size} / ${replica.book.count}`);
 row("rss after open",rss());
 const walkProbe=stalls();mark=performance.now();
 const walk=await replicaModule.walkLocalV1({...replica,settings:{...replica.settings,worktree:live}} as unknown as Replica);
 row("first walk of the live tree",`${ms(mark)} (${walk.scanned} entries, ${walk.changed.length} differ)`);
 row("  longest event-loop stall during walk",`${walkProbe.stop().toFixed(0)} ms`);
 const indexDir=mkdtempSync(join(tmpdir(),"bench-index-"));
 cpSync(join(live,".git","index"),join(indexDir,"index"));
 const env={GIT_DIR:join(live,".git"),GIT_WORK_TREE:live,GIT_INDEX_FILE:join(indexDir,"index"),GIT_OBJECT_DIRECTORY:join(indexDir,"objects"),GIT_ALTERNATE_OBJECT_DIRECTORIES:join(live,".git","objects"),GIT_OPTIONAL_LOCKS:"0"};
 mkdirSync(join(indexDir,"objects"),{recursive:true});
 for(const [name,args] of [["git rev-parse --git-dir",["rev-parse","--git-dir"]],["git symbolic-ref HEAD",["symbolic-ref","-q","HEAD"]],["git rev-parse HEAD",["rev-parse","-q","--verify","HEAD^{commit}"]],["git write-tree (private objects)",["write-tree"]],["git config --local --list",["config","--local","--list","-z"]]] as const){
  mark=performance.now();const outcome=await run("git",args,env);row(name,`${ms(mark)} (exit ${outcome.code})`);
 }
 mark=performance.now();
 const view=await run("git",["--no-optional-locks","status","--porcelain=v1","-z","--untracked-files=all","--no-renames"],env);
 row("git status --porcelain (first view)",`${ms(mark)} (exit ${view.code}, ${mib(view.out)} out)`);
 mark=performance.now();
 const again=await run("git",["--no-optional-locks","status","--porcelain=v1","-z","--untracked-files=all","--no-renames"],env);
 row("git status again (warm)",`${ms(mark)} (exit ${again.code})`);
 mark=performance.now();await replicaModule.closeReplicaV1(replica);
 row("closeReplicaV1 (clean stop persist)",ms(mark));
 for(const name of ["state.json","state.log","state.ckp","state.wal"]){const file=join(worktree,".cache","f",name);if(existsSync(file))row(`  ${name}`,mib(statSync(file).size));}
 rmSync(worktree,{recursive:true,force:true});rmSync(indexDir,{recursive:true,force:true});
}

async function load():Promise<void>{
 const replicaModule=await mod<ReplicaModule>("🔁️replica/🟦️.ts"),worktree=stage();
 for(let round=0;round<3;round++){
  const mark=performance.now(),probe=stalls(),replica=await replicaModule.openReplicaV1(options(worktree));
  row(`open #${round+1}`,`${ms(mark)}, stall ${probe.stop().toFixed(0)} ms, rss ${rss()}, files ${replica.files.size}`);
  const closing=performance.now();await replicaModule.closeReplicaV1(replica);row(`close #${round+1}`,ms(closing));
 }
 rmSync(worktree,{recursive:true,force:true});
}

type Tables={book:{count:number;name:(id:number)=>string};files:{ids:()=>Int32Array;entryAt:(id:number)=>unknown;manifestHex:() => string;shared:number};manifest:{ids:()=>Int32Array;entryAt:(id:number)=>unknown};queue:{snapshot:()=>Int32Array}};

function fingerprint(replica:Tables):string{
 const hash=createHash("sha256");
 for(const id of replica.files.ids())hash.update(`F${id}\0${replica.book.name(id)}\0${JSON.stringify(replica.files.entryAt(id))}\n`);
 for(const id of replica.queue.snapshot())hash.update(`Q${id}\n`);
 for(const id of replica.manifest.ids())hash.update(`M${id}\0${JSON.stringify(replica.manifest.entryAt(id))}\n`);
 hash.update(`${replica.files.manifestHex()}\0${replica.files.shared}\0${replica.book.count}`);
 return hash.digest("hex").slice(0,16);
}

async function roundtrip():Promise<void>{
 const replicaModule=await mod<ReplicaModule>("🔁️replica/🟦️.ts"),worktree=stage();
 const first=await replicaModule.openReplicaV1(options(worktree)),before=fingerprint(first as unknown as Tables);
 await replicaModule.closeReplicaV1(first);
 const second=await replicaModule.openReplicaV1(options(worktree)),after=fingerprint(second as unknown as Tables);
 row("fingerprint after takeover / after reload",`${before} / ${after} ${before===after?"EQUAL":"DIFFERENT"}`);
 await replicaModule.closeReplicaV1(second);
 rmSync(worktree,{recursive:true,force:true});
}

async function checkpointBench():Promise<void>{
 const replicaModule=await mod<ReplicaModule&{checkpointReplicaV1:(replica:Replica)=>Promise<void>}>("🔁️replica/🟦️.ts"),worktree=stage();
 const replica=await replicaModule.openReplicaV1(options(worktree));
 for(let round=0;round<3;round++){
  const probe=stalls(),mark=performance.now();
  await replicaModule.checkpointReplicaV1(replica);
  row(`checkpoint chunked+yielding #${round+1}`,`${ms(mark)}, longest stall ${probe.stop().toFixed(0)} ms, file ${mib(statSync(join(worktree,".cache","f","state.ckp")).size)}`);
 }
 const file=join(worktree,".cache","f","state.ckp"),bytes=readFileSync(file);
 {const probe=stalls(),mark=performance.now();writeFileSync(join(worktree,"sync.bin"),bytes);row("comparison: one blocking writeFileSync",`${ms(mark)}, longest stall ${probe.stop().toFixed(0)} ms`);}
 {
  const probe=stalls(),mark=performance.now(),copy=new Uint8Array(bytes.length);copy.set(bytes);
  const worker=new Worker(`data:text/javascript,self.onmessage=async event=>{const fs=await import("node:fs");fs.writeFileSync(event.data.file,event.data.bytes);postMessage("done");};`);
  await new Promise<void>(done=>{worker.onmessage=()=>{done();};worker.postMessage({file:join(worktree,"worker.bin"),bytes:copy},[copy.buffer]);});
  worker.terminate();row("comparison: copy + worker thread write",`${ms(mark)}, longest stall ${probe.stop().toFixed(0)} ms (plus a second ${mib(bytes.length)} copy)`);
 }
 await replicaModule.closeReplicaV1(replica);
 rmSync(worktree,{recursive:true,force:true});
}

async function e2e():Promise<void>{
 const count=Number(process.env.SEMIO_BENCH_FILES??"40000"),base=mkdtempSync(join(tmpdir(),"bench-e2e-")),worktree=join(base,"wt"),storeDir=join(base,"store");
 mkdirSync(worktree,{recursive:true});
 for(let index=0;index<count;index++){const directory=join(worktree,`g${index%200}`,`d${index%37}`);if(index<400)mkdirSync(directory,{recursive:true});else if(!existsSync(directory))mkdirSync(directory,{recursive:true});writeFileSync(join(directory,`f${index}.txt`),`content ${index}\n`);}
 const git=(...args:string[])=>Bun.spawnSync(["git",...args],{cwd:worktree,env:{...process.env,GIT_AUTHOR_NAME:"b",GIT_AUTHOR_EMAIL:"b@b",GIT_COMMITTER_NAME:"b",GIT_COMMITTER_EMAIL:"b@b"},stdout:"ignore",stderr:"ignore"});
 git("init","-q");git("add","-A");git("commit","-q","-m","x");
 const replicaModule=await mod<{runV1:(options:object,signal:AbortSignal,sink?:(event:{event:string})=>void)=>Promise<void>;seedV1:(options:object)=>Promise<unknown>}>("🔁️replica/🟦️.ts");
 const attest=await mod<{readAttestationV1:(store:string,origin:string)=>Promise<{wallMs:number;view:{stale?:boolean}|null}|undefined>}>("🩺️attest/🟦️.ts");
 const options={worktree,storeRoot:storeDir,originId:"bench",groupId:"bg",attestMilliseconds:500};
 let mark=performance.now();await replicaModule.seedV1(options);row(`seed of ${count} files`,ms(mark));
 for(let round=0;round<3;round++){
  const controller=new AbortController(),events:string[]=[],started=Date.now(),begin=performance.now();
  mark=begin;
  let firstAttest:number|undefined,firstFollowAt:number|undefined;
  const running=replicaModule.runV1(options,controller.signal,event=>{if(event.event==="started"&&firstFollowAt===undefined)firstFollowAt=performance.now()-begin;events.push(event.event);});
  for(;;){
   const found=await attest.readAttestationV1(storeDir,"bench");
   if(found!==undefined&&found.wallMs>=started){firstAttest=performance.now()-begin;row(`run #${round+1}: process start -> started event`,`${(firstFollowAt??0).toFixed(0)} ms`);row(`run #${round+1}: process start -> first attestation`,`${firstAttest.toFixed(0)} ms (view ${found.view===null?"none":found.view.stale===true?"stale":"fresh"})`);break;}
   if(performance.now()-begin>120000){row(`run #${round+1}`,"no attestation within 120 s");break;}
   await Bun.sleep(20);
  }
  const closing=performance.now();controller.abort();await running;row(`run #${round+1}: stop`,ms(closing));
 }
 rmSync(base,{recursive:true,force:true});
}

const modes:Record<string,()=>Promise<void>>={phases,load,roundtrip,checkpoint:checkpointBench,e2e};
if(modes[mode]===undefined){process.stderr.write(`unknown mode ${mode}\n`);process.exit(2);}
await modes[mode]!();
void readFileSync;void writeFileSync;
