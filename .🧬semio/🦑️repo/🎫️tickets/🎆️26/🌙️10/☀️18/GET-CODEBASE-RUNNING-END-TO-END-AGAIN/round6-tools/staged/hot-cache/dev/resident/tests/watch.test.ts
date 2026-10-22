import {afterAll,describe,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {PassThrough} from "node:stream";
import {appendFileSync,mkdirSync,renameSync,rmSync,writeFileSync,lstatSync} from "node:fs";
import {spawn,spawnSync} from "node:child_process";
import {createWatchSupervisorV1} from "../watch/index.mjs";
import {POLICY_V1,validatorV1} from "../protocol/policy.mjs";
import {createFilterV1} from "../state/index.mjs";
import {WATCHER,cleanup,haveBinary,sleep,snapshot,tempRoot,waitFor,writeTree} from "./common.ts";

const native=haveBinary(WATCHER);
const roots:string[]=[];
afterAll(()=>roots.forEach(cleanup));
const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
const policy={...POLICY_V1,watcherRestartBackoffMilliseconds:[10,10,10],fenceTimeoutMilliseconds:600};

function make(root:string,extra:Record<string,unknown>={}){
 const seen:[string,number,string][]=[],losses:string[]=[],trusts:string[]=[];
 const supervisor=createWatchSupervisorV1({root,policy,filter,rooted:[".git"],names:["node_modules"],binary:WATCHER,backend:"native",fenceDirectory:".semio-fence",instance:"t",
  onBatch:async rows=>{seen.push(...rows);},onLost:async(scope,reason)=>{losses.push(reason+":"+scope);},onTrust:t=>trusts.push(t.trust),...extra});
 return{supervisor,seen,losses,trusts};
}

describe.skipIf(!native)("native watcher",()=>{
 test("reports create, modify, remove, rename and directory creation; never excluded paths",async()=>{
  const root=tempRoot("watch-basic-");roots.push(root);writeTree(root,{"src/a.ts":"a","src/old.ts":"o"});
  const{supervisor,seen}=make(root);const state=await supervisor.start();
  expect(state.trust).toBe("exact");
  writeFileSync(root+"/src/new.ts","n");appendFileSync(root+"/src/a.ts","x");rmSync(root+"/src/old.ts");renameSync(root+"/src/new.ts",root+"/src/moved.ts");mkdirSync(root+"/lib/deep",{recursive:true});writeFileSync(root+"/lib/deep/f.ts","f");
  mkdirSync(root+"/node_modules/x",{recursive:true});writeFileSync(root+"/node_modules/x/y.js","y");mkdirSync(root+"/.git",{recursive:true});writeFileSync(root+"/.git/HEAD","h");
  await supervisor.fence();await supervisor.drained();
  const paths=new Set(seen.map(r=>r[2]));
  for(const expected of["src/a.ts","src/old.ts","src/moved.ts","lib","lib/deep/f.ts"])expect(paths.has(expected)).toBe(true);
  expect(seen.find(r=>r[2]==="lib")![1]).toBe(1);
  expect([...paths].some(p=>p.includes("node_modules")||p.startsWith(".git"))).toBe(false);
  await supervisor.stop();
 });

 test("a fence resolves only after every earlier write has been delivered",async()=>{
  const root=tempRoot("watch-fence-");roots.push(root);mkdirSync(root+"/d");
  const{supervisor,seen}=make(root);await supervisor.start();
  for(let round=0;round<25;round++){
   const names:string[]=[];
   for(let i=0;i<40;i++){const p="d/r"+round+"_"+i;names.push(p);writeFileSync(root+"/"+p,"x");}
   const result=await supervisor.fence();await supervisor.drained();
   expect(result.ok).toBe(true);
   const got=new Set(seen.map(r=>r[2]));
   expect(names.filter(p=>!got.has(p))).toEqual([]);
  }
  expect(supervisor.stats.fenceMisses).toBe(0);
  await supervisor.stop();
 },30000);

 test("concurrent fences share work but each covers writes made before its own call",async()=>{
  const root=tempRoot("watch-fence-share-");roots.push(root);
  const{supervisor,seen}=make(root);await supervisor.start();
  const results=await Promise.all(Array.from({length:12},async(_,i)=>{
   writeFileSync(root+"/c"+i,"x");const p="c"+i;
   const r=await supervisor.fence();await supervisor.drained();
   return{ok:r.ok,delivered:seen.some(row=>row[2]===p)};
  }));
  expect(results.every(r=>r.ok&&r.delivered)).toBe(true);
  expect(supervisor.stats.fences).toBeLessThan(12+2);
  await supervisor.stop();
 });

 test("closing stdin ends the watcher; killing the parent cannot leave an orphan",async()=>{
  const root=tempRoot("watch-orphan-");roots.push(root);
  const child=spawn(WATCHER,["watch","--root",root],{stdio:["pipe","pipe","ignore"]});
  await new Promise(resolve=>child.stdout.once("data",resolve));
  const exited=new Promise<number|null>(resolve=>child.once("exit",resolve));
  child.stdin.end();
  expect(await Promise.race([exited,sleep(2000).then(()=>"timeout")])).toBe(0);
  const script=`import {spawn} from "node:child_process";const c=spawn(${JSON.stringify(WATCHER)},["watch","--root",${JSON.stringify(root)}],{stdio:["pipe","pipe","ignore"]});c.stdout.once("data",()=>{console.log(c.pid);setTimeout(()=>process.kill(process.pid,"SIGKILL"),50);});`;
  const out=await new Promise<string>(resolve=>{const p=spawn("bun",["-e",script],{stdio:["ignore","pipe","ignore"]});let text="";p.stdout.on("data",c=>text+=c);p.on("exit",()=>resolve(text.trim()));});
  const pid=Number(out);expect(pid).toBeGreaterThan(1);
  expect(await waitFor(()=>{try{process.kill(pid,0);return false;}catch{return true;}},3000)).toBe(true);
 },15000);

 test("a restarted watcher replays what happened while it was away",async()=>{
  const root=tempRoot("watch-replay-");roots.push(root);mkdirSync(root+"/src");
  const first=make(root);await first.supervisor.start();writeFileSync(root+"/src/a","1");await first.supervisor.fence();await first.supervisor.drained();
  const saved={cursor:first.supervisor.state().cursor,uuid:first.supervisor.state().uuid};await first.supervisor.stop();
  for(let i=0;i<30;i++)writeFileSync(root+"/src/off"+i,"x");
  await sleep(300);
  const second=make(root,{restore:saved});const state=await second.supervisor.start();
  expect(state.replay).toBe(true);
  expect(await waitFor(()=>second.seen.filter(r=>r[2].startsWith("src/off")).length>=30,3000)).toBe(true);
  expect(second.losses).toEqual([]);
  await second.supervisor.stop();
 });

 test("scan equals a naive walk: same files, modes, directory stamps",()=>{
  const root=tempRoot("watch-scan-");roots.push(root);
  writeTree(root,{"a/b/c.ts":"c","a/d.ts":"d","e.txt":"e","node_modules/x/y.js":"y",".git/HEAD":"h","ü/ß.ts":"u"});
  const out=spawnSync(WATCHER,["scan","--root",root,"--exclude",".git","--exclude-name","node_modules","--threads","3"],{encoding:"utf8"});
  const frame=JSON.parse(out.stdout);
  expect(validatorV1("WatcherFrame")).toBeDefined();
  const naive=snapshot(root,rel=>filter.excluded(rel));
  expect(frame.members.map((m:any)=>m[0])).toEqual([...naive.files.keys()].sort());
  for(const[path,mode]of frame.members)expect(naive.files.get(path)).toBe(mode);
  expect(frame.directories.map((d:any)=>d[0])).toEqual([...naive.directories.keys()].sort());
  for(const[path,mtime,ino]of frame.directories){const n=naive.directories.get(path)!;expect(mtime).toBe(n[0]);expect(ino).toBe(n[1]);}
 });

 test("bad arguments produce one error frame and exit 2; version is a valid frame",()=>{
  const bad=spawnSync(WATCHER,["watch"],{encoding:"utf8"});
  expect(bad.status).toBe(2);expect(validatorV1("WatcherFrame")(JSON.parse(bad.stdout.trim().split("\n")[0]!))).toEqual([]);
  expect(spawnSync(WATCHER,["frobnicate"],{encoding:"utf8"}).status).toBe(2);
  expect(validatorV1("WatcherFrame")(JSON.parse(spawnSync(WATCHER,["version"],{encoding:"utf8"}).stdout))).toEqual([]);
  expect(spawnSync(WATCHER,["watch","--root","/definitely/not/here"],{encoding:"utf8"}).status).toBe(2);
 });

 test("every frame the real watcher prints validates against the schema",async()=>{
  const root=tempRoot("watch-schema-");roots.push(root);
  const child=spawn(WATCHER,["watch","--root",root,"--latency-ms","5"],{stdio:["pipe","pipe","ignore"]});
  let text="";child.stdout.on("data",c=>text+=c);
  await sleep(300);for(let i=0;i<20;i++){writeFileSync(root+"/f"+i,"x");if(i%5===0)mkdirSync(root+"/d"+i);}
  await sleep(500);child.stdin.end();await new Promise(r=>child.once("exit",r));
  const validate=validatorV1("WatcherFrame");const lines=text.trim().split("\n");
  expect(lines.length).toBeGreaterThan(2);
  for(const line of lines)expect(validate(JSON.parse(line))).toEqual([]);
 });
});

/** 🎭 A scripted stand-in for the watcher process. */
function fakeChild(script:(child:any)=>void){
 const child:any=new EventEmitter();child.stdout=new PassThrough();child.stdin={end(){}};child.kill=()=>child.emit("exit",null);
 queueMicrotask(()=>script(child));return child;
}
const frame=(child:any,value:unknown)=>child.stdout.write(JSON.stringify(value)+"\n");
const ready={v:1,t:"ready",backend:"fsevents",root:"/x",cursor:"1",uuid:"U",replay:false,latencyMs:5,native:true};

describe("supervisor against a scripted watcher",()=>{
 test("a watch limit makes the trust permanent verify: unwatched directories can never be vouched for",async()=>{
  const root=tempRoot("watch-limit-");roots.push(root);
  const{supervisor,losses,trusts}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>{frame(child,ready);frame(child,{v:1,t:"lost",at:1,reason:"watch-limit",scope:"deep/dir"});})});
  await supervisor.start();await sleep(50);
  expect(losses).toContain("watch-limit:deep/dir");expect(trusts).toContain("verify");expect(supervisor.state().trust).toBe("verify");
  await supervisor.stop();
 },10000);

 test("lost frames, malformed batches and path escapes reach onLost, never onBatch",async()=>{
  const root=tempRoot("watch-fake-");roots.push(root);
  const{supervisor,seen,losses}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>{
   frame(child,ready);
   frame(child,{v:1,t:"lost",at:1,reason:"overflow",scope:"src"});
   frame(child,{v:1,t:"batch",n:1,at:1,cursor:"2",c:[["m",0,"../etc/passwd"]]});
   frame(child,{v:1,t:"batch",n:2,at:1,cursor:"3",c:[["m",0,"/abs"]]});
   frame(child,{v:1,t:"batch",n:3,at:1,cursor:"4",c:[["m",0,"ok/file"]]});
  })});
  const started=await supervisor.start();void started;
  await sleep(100);await supervisor.drained();
  expect(losses).toContain("overflow:src");expect(losses.filter(l=>l.startsWith("read-error")).length).toBe(2);
  expect(seen.map(r=>r[2])).toEqual(["ok/file"]);
  expect(supervisor.state().cursor).toBe("4");
  await supervisor.stop();
 });

 test("without any fence event the canary degrades trust instead of lying",async()=>{
  const root=tempRoot("watch-canary-");roots.push(root);
  const{supervisor}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>frame(child,ready))});
  const state=await supervisor.start();
  expect(state.trust).toBe("verify");expect(supervisor.stats.fenceMisses).toBe(1);
  await supervisor.stop();
 },10000);

 test("a watcher that keeps dying is restarted with backoff, then replaced by the portable backend",async()=>{
  const root=tempRoot("watch-restart-");roots.push(root);let spawned=0;
  const{supervisor,losses,trusts}=make(root,{binary:"fake",spawnProcess:()=>{spawned++;return fakeChild(child=>{if(spawned===1){frame(child,ready);setTimeout(()=>child.emit("exit",1),20);}else child.emit("exit",1);});}});
  await supervisor.start().catch(()=>{});
  await waitFor(()=>supervisor.state().backend==="portable",8000);
  expect(spawned).toBeGreaterThanOrEqual(policy.watcherRestartsBeforePortable);
  expect(supervisor.state().backend).toBe("portable");expect(trusts).toContain("verify");expect(losses.some(l=>l.startsWith("watcher-exit"))).toBe(true);
  await supervisor.stop();
 },15000);

 test("poll and none backends never claim exact trust and have no fence",async()=>{
  const root=tempRoot("watch-none-");roots.push(root);
  for(const backend of["poll","none"]){
   const{supervisor}=make(root,{backend});const state=await supervisor.start();
   expect(state.trust).not.toBe("exact");expect((await supervisor.fence()).ok).toBe(false);await supervisor.stop();
  }
 });
});
