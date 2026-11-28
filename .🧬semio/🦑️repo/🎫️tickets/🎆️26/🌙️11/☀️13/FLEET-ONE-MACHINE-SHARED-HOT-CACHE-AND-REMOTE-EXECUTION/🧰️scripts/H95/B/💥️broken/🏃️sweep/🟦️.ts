import {spawn} from "node:child_process";
import {readFileSync} from "node:fs";
import {setPriority} from "node:os";
import {join} from "node:path";
import policy from "../🔣️.json" with {type:"json"};
import {brokenDependentsV1,brokenDirtyCratesV1,brokenIndexCratesV1,brokenMergeV1,brokenParseCargoV1,brokenWriteBoardV1,validateBrokenBoardV1,type BrokenBoardV1,type BrokenErrorV1,type BrokenIndexV1,type BrokenPolicyV1} from "../🟦️.ts";
import {brokenTextV1} from "../📖️texts/🟦️.ts";
import {journalFindEditorV1,journalReadV1,type JournalCursorV1,type JournalEditorV1,type JournalEventV1} from "../../📜️change-journal/🟦️.ts";

const rules=policy as BrokenPolicyV1;

/** 🔌️ The seams of one sweep; tests replace them, the defaults read the real journal, capture the real snapshot and run the real check. */
export type BrokenSweepPortsV1=Readonly<{
 now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;signal?:AbortSignal;
 index?:(root:string)=>BrokenIndexV1;
 journal?:(store:string,cursor:JournalCursorV1)=>Readonly<{events:readonly JournalEventV1[];cursor:JournalCursorV1;fresh:readonly string[]}>;
 snapshot?:(root:string)=>Promise<Readonly<{root:string;digest:string}>>;
 check?:(request:Readonly<{argv:readonly string[];cwd:string;environment:NodeJS.ProcessEnv;signal?:AbortSignal;timeoutMilliseconds:number;priority:number}>)=>Promise<Readonly<{code:number;stdout:string;stderr:string}|undefined>>;
 editor?:(store:string,path:string)=>JournalEditorV1|undefined;
 publish?:(store:string,board:BrokenBoardV1)=>void;
 notice?:(line:string)=>void;
 environment?:NodeJS.ProcessEnv;
 languages?:readonly ("en"|"de")[];
}>;

/** 🧾️ What one sweep did. */
export type BrokenSweepResultV1=Readonly<{board:BrokenBoardV1;mode:"workspace"|"crates"|"idle";checked:number;deferred:number}>;

const readBoard=(store:string,origin:string,limits:BrokenPolicyV1):BrokenBoardV1|undefined=>{
 try{const value=JSON.parse(readFileSync(join(store,...limits.store.directory,origin,limits.store.boardFile),"utf8"));return validateBrokenBoardV1(value).length===0?value as BrokenBoardV1:undefined;}catch{return undefined;}
};

/** 🏃️ Runs the check as a low priority child (the operating system priority of the declared nice level, a time limit, cancellation kills it) and collects its output; a child that cannot start or runs out of time answers nothing. */
export function brokenRunCheckV1(request:Readonly<{argv:readonly string[];cwd:string;environment:NodeJS.ProcessEnv;signal?:AbortSignal;timeoutMilliseconds:number;priority:number}>,binary:string):Promise<Readonly<{code:number;stdout:string;stderr:string}>|undefined>{
 return new Promise(done=>{
  let stdout="",stderr="",finished=false;
  const child=spawn(binary,[...request.argv],{cwd:request.cwd,env:request.environment,stdio:["ignore","pipe","pipe"],windowsHide:true});
  const finish=(value:Readonly<{code:number;stdout:string;stderr:string}>|undefined):void=>{if(finished)return;finished=true;clearTimeout(timer);request.signal?.removeEventListener("abort",abort);done(value);};
  const abort=():void=>{child.kill("SIGTERM");finish(undefined);};
  const timer=setTimeout(()=>{child.kill("SIGTERM");finish(undefined);},request.timeoutMilliseconds);
  timer.unref();
  request.signal?.addEventListener("abort",abort,{once:true});
  child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});
  child.on("error",()=>finish(undefined));
  child.on("spawn",()=>{try{if(child.pid!==undefined)setPriority(child.pid,request.priority);}catch{}});
  child.on("close",code=>finish({code:code??1,stdout,stderr}));
 });
}

/** 🧹️ Sweeps once: reads what the replica journal gained, maps it to crates (and their dependents), waits for crates that are still being edited, checks the rest on a consistent source snapshot with `cargo check --keep-going` (offloaded to the fleet by the repo command, at the lowest priority) and publishes the board of this machine with, per crate, status, first error and last editor. Nothing changed means no check. A check that did not run (cancelled, out of time) keeps the old rows and the old journal cursor so the changes are replayed, never lost. */
export async function brokenSweepOnceV1(request:Readonly<{root:string;origin:string;store:string;full?:boolean}>,ports:BrokenSweepPortsV1={},limits:BrokenPolicyV1=rules):Promise<BrokenSweepResultV1>{
 const now=ports.now??Date.now,notice=ports.notice??(()=>{}),languages=ports.languages??["en","de"],say=(key:Parameters<typeof brokenTextV1>[1],values:Record<string,string|number>={})=>{for(const language of languages)notice(brokenTextV1(language,key,values));};
 const began=now(),prior=readBoard(request.store,request.origin,limits),environment=ports.environment??process.env;
 const journal=(ports.journal??((store,cursor)=>journalReadV1(store,cursor)))(request.store,prior?.cursor??{}),index=(ports.index??(root=>brokenIndexCratesV1(root,limits)))(request.root);
 const dirty:Record<string,number>={...prior?.dirty},touched=brokenDirtyCratesV1(journal.events,index,limits);
 const lastEvent=new Map<string,number>();
 for(const event of journal.events){const owner=[...index.byDirectory].filter(([dir])=>event.path.startsWith(dir+"/")).sort((a,b)=>b[0].length-a[0].length)[0];if(owner!==undefined)lastEvent.set(owner[1],Math.max(lastEvent.get(owner[1])??0,event.atMs));}
 for(const crate of touched.direct)dirty[crate]=Math.max(dirty[crate]??0,lastEvent.get(crate)??began);
 const cold=prior===undefined||prior.rows.length===0,stale=prior?.lastSweep===undefined||began-prior.lastSweep.atMs>limits.sweep.fullAfterHours*3600_000;
 const workspace=request.full===true||cold||touched.workspace||stale&&Object.keys(dirty).length>0;
 const forced=prior?.lastSweep!==undefined&&began-prior.lastSweep.atMs>=limits.sweep.maximumQuietSeconds*1000;
 const ready=Object.keys(dirty).filter(crate=>forced||began-dirty[crate]!>=limits.sweep.quietSeconds*1000),waiting=Object.keys(dirty).filter(crate=>!ready.includes(crate));
 const publishBoard=(board:BrokenBoardV1):BrokenBoardV1=>{(ports.publish??((store,value)=>{brokenWriteBoardV1(store,value,limits);}))(request.store,board);return board;};
 if(!workspace&&!ready.length){
  if(waiting.length)say("sweep.deferred",{count:waiting.length,seconds:limits.sweep.quietSeconds});else say("sweep.idle");
  const board:BrokenBoardV1={version:1,origin:request.origin,atMs:began,cursor:journal.cursor,rows:prior?.rows??[],dirty,...prior?.lastSweep===undefined?{}:{lastSweep:prior.lastSweep}};
  return {board:publishBoard(board),mode:"idle",checked:0,deferred:waiting.length};
 }
 const closure=brokenDependentsV1(ready,index),crates=workspace||closure.size>limits.sweep.workspaceThreshold?undefined:[...closure].sort(),checkedSet=new Set(crates??index.byDirectory.values());
 const mode:"workspace"|"crates"=crates===undefined?"workspace":"crates";
 const view=await (ports.snapshot??(async root=>({root,digest:""})))(request.root);
 say("sweep.begin",{count:checkedSet.size,mode,digest:view.digest===""?"-":view.digest.slice(0,19)});
 const binary=environment[limits.check.environment]??limits.check.binary,argv=[...limits.check.wrapper,...limits.check.prefix,...limits.check.arguments,...crates===undefined?["--workspace"]:crates.flatMap(crate=>["-p",crate])];
 const outcome=await (ports.check??(request2=>brokenRunCheckV1(request2,binary)))({argv,cwd:view.root,environment:{...environment,SEMIO_AGENT:"broken-sweep",SEMIO_SOURCE_SNAPSHOT:view.digest===""?"live":view.digest},signal:ports.signal,timeoutMilliseconds:limits.sweep.timeoutSeconds*1000,priority:limits.sweep.priority});
 if(outcome===undefined){
  say("sweep.failed",{reason:"check did not finish"});
  const board:BrokenBoardV1={version:1,origin:request.origin,atMs:now(),cursor:prior?.cursor??{},rows:prior?.rows??[],dirty,lastSweep:{atMs:began,mode,seconds:Math.round((now()-began)/10)/100,crates:checkedSet.size,code:-1,...view.digest===""?{}:{digest:view.digest}}};
  return {board:publishBoard(board),mode,checked:0,deferred:waiting.length};
 }
 const parsed=brokenParseCargoV1(outcome.stdout,[view.root,request.root],limits);
 let errors:ReadonlyMap<string,BrokenErrorV1>=parsed.errors;const checkedNow=new Set(checkedSet);
 if(outcome.code!==0&&parsed.errors.size===0&&parsed.finished!==false&&!parsed.built.size){
  const text=outcome.stderr.split("\n").filter(line=>line.trim()!=="").slice(-3).join(" ").slice(0,limits.limits.stderrCharacters);
  errors=new Map([...parsed.errors,["(check)",{message:text===""?"check failed without output":text}]]);checkedNow.add("(check)");
 }
 const events=journal.events,editor=ports.editor??((store,path)=>journalFindEditorV1(store,path));
 const rows=brokenMergeV1(prior?.rows??[],checkedNow,{errors,built:parsed.built,finished:parsed.finished},index,{nowMs:now(),origin:request.origin,...view.digest===""?{}:{digest:view.digest},editor:(file,dir)=>{
  if(file!==undefined){const found=editor(request.store,file);if(found!==undefined)return found;}
  if(dir===undefined)return undefined;
  const near=events.filter(event=>event.path.startsWith(dir+"/")).sort((a,b)=>b.atMs-a.atMs)[0];
  return near===undefined?undefined:{origin:near.origin,atMs:near.atMs,path:near.path};
 }},limits);
 for(const crate of checkedNow)delete dirty[crate];
 const counts={broken:0,blocked:0,ok:0};for(const row of rows)if(row.status!=="unchecked")counts[row.status]++;
 say("sweep.end",{seconds:Math.round((now()-began)/10)/100,...counts});
 const board:BrokenBoardV1={version:1,origin:request.origin,atMs:now(),cursor:journal.cursor,rows,dirty,lastSweep:{atMs:began,mode,seconds:Math.round((now()-began)/10)/100,crates:checkedNow.size,code:outcome.code,...view.digest===""?{}:{digest:view.digest}}};
 return {board:publishBoard(board),mode,checked:checkedNow.size,deferred:waiting.length};
}

/** 🔁️ Sweeps again and again until cancelled, pausing the declared seconds between sweeps; the work of a quiet period is only reading the journal. */
export async function brokenWatchV1(request:Readonly<{root:string;origin:string;store:string}>,ports:BrokenSweepPortsV1={},limits:BrokenPolicyV1=rules):Promise<void>{
 const sleep=ports.sleep??(milliseconds=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);ports.signal?.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}));
 while(!ports.signal?.aborted){
  try{await brokenSweepOnceV1(request,ports,limits);}catch(error){(ports.notice??(()=>{}))(brokenTextV1("en","sweep.failed",{reason:String((error as Error).message??error).slice(0,200)}));}
  await sleep(limits.sweep.pollSeconds*1000);
 }
}
