import {afterAll,describe,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,writeFileSync} from "node:fs";
import {WATCHER,cleanup,haveBinary,tempRoot,waitFor} from "../../🟦️.ts";

/** 🕳️ Laws of events lost between fseventsd and the watcher (`UserDropped`), measured under fleet load in C/result-jloss.md: the callback queue must not run below the build processes, and a drop must be repaired from the volume history without a loss and without a fence row overtaking an earlier change. `SEMIO_REPO_WATCH_STALL_BINARY` is the watcher built with `--cfg semio_watch_stall` (its first stream sleeps in every callback, so fseventsd drops events for it). https://developer.apple.com/documentation/coreservices/1455361-fseventstreameventflags */
const STALL=process.env.SEMIO_REPO_WATCH_STALL_BINARY??"";
const roots:string[]=[];
afterAll(()=>roots.forEach(cleanup));

type Watch=Readonly<{pid:number;covered:(path:string)=>boolean;seen:(path:string)=>boolean;lost:string[];historyDone:()=>number;stop:()=>Promise<void>}>;

/** 🛰️ Runs one watcher on `root` and keeps every reported path, every structural directory row (a subtree), every loss and the replays it finished; `arrived` sees the paths of each batch right after they are kept. */
async function watch(binary:string,root:string,arrived:(paths:readonly string[])=>void=()=>{}):Promise<Watch>{
 const child=spawn(binary,["watch","--root",root,"--latency-ms","5"],{stdio:["pipe","pipe","ignore"]});
 const paths=new Set<string>(),subtrees=new Set<string>(),lost:string[]=[];let replays=0,ready=false,rest="";
 child.stdout.on("data",chunk=>{
  const lines=(rest+chunk).split("\n");rest=lines.pop()!;
  for(const line of lines){
   const frame=JSON.parse(line);
   if(frame.t==="ready")ready=true;
   else if(frame.t==="lost")lost.push(frame.reason+":"+frame.scope);
   else if(frame.t==="history-done")replays++;
   else if(frame.t==="batch"){for(const[kind,dir,path]of frame.c){paths.add(path);if(dir===1&&kind!=="m")subtrees.add(path);}arrived(frame.c.map((row:[string,number,string])=>row[2]));}
  }
 });
 expect(await waitFor(()=>ready,10000)).toBe(true);
 const covered=(path:string)=>paths.has(path)||path.split("/").some((_,index,parts)=>index>0&&subtrees.has(parts.slice(0,index).join("/")));
 return{pid:child.pid!,covered,seen:path=>paths.has(path),lost,historyDone:()=>replays,stop:async()=>{child.stdin.end();await new Promise(resolve=>child.once("exit",resolve));}};
}

/** 🗂️ A root with `directories` empty directories made before any watcher starts, so only file rows (or a replayed subtree) can cover the files written later. */
function prepared(prefix:string,directories:number):string{
 const root=tempRoot(prefix);roots.push(root);
 for(let index=0;index<directories;index++)mkdirSync(root+"/d"+index);
 return root;
}

describe.skipIf(process.platform!=="darwin"||!haveBinary(STALL))("FSEvents drops are replayed from the volume history",()=>{
 test("a callback stalled until fseventsd drops events (writes go on until the drop was replayed, at most 60000 files, a fence every 500) reports, whenever a fence row arrives, every file written before that fence, and never a loss",async()=>{
  const root=prepared("watch-drop-",60),written:string[]=[],fences=new Map<string,number>(),overtaken:string[]=[];
  let run!:Watch;
  run=await watch(STALL,root,paths=>{for(const path of paths){const before=fences.get(path);if(before!==undefined&&written.slice(0,before).some(file=>!run.covered(file)))overtaken.push(path);}});
  for(let index=0;index<60000&&(index<12000||run.historyDone()+run.lost.length===0);index++){
   const path="d"+(index%60)+"/f"+index;writeFileSync(root+"/"+path,"x");written.push(path);
   if(index%500===499){const fence="fence"+index;fences.set(fence,written.length);writeFileSync(root+"/"+fence,"");await new Promise(resolve=>setTimeout(resolve,0));}
  }
  fences.set("fence",written.length);writeFileSync(root+"/fence","");
  expect(await waitFor(()=>run.seen("fence"),90000,10)).toBe(true);
  await run.stop();
  cleanup(root);
  expect({lost:run.lost,overtaken}).toEqual({lost:[],overtaken:[]});
  expect(run.historyDone()).toBeGreaterThanOrEqual(1);
 },120000);
});

describe.skipIf(process.platform!=="darwin"||!haveBinary(WATCHER))("the FSEvents callback queue",()=>{
 test("no watcher thread that works during a burst runs at utility priority (20) or below: the callback queue competes with the build processes (31)",async()=>{
  const root=prepared("watch-qos-",20);
  const run=await watch(WATCHER,root);
  const working:number[]=[];
  let index=0;const until=performance.now()+2000;
  while(performance.now()<until){
   for(let batch=0;batch<400;batch++,index++)writeFileSync(root+"/d"+(index%20)+"/f"+index,"x");
   const sample=spawnSync("ps",["-M","-p",String(run.pid)],{encoding:"utf8"}).stdout.split("\n").slice(1);
   for(const line of sample){const match=/\s(\d+\.\d+)\s+\S+\s+(\d+)[A-Z]*\s+\d+:\d+\.\d+\s+\d+:\d+\.\d+/u.exec(line);if(match&&Number(match[1])>0)working.push(Number(match[2]));}
  }
  await run.stop();
  expect(working.length).toBeGreaterThan(0);
  expect(working.filter(priority=>priority<=20)).toEqual([]);
 },20000);
});
