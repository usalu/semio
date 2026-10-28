import{execFile}from"node:child_process";
import{readdirSync}from"node:fs";
import{platform as osPlatform}from"node:os";
import{resolveHostCapacityLanguageV1}from"../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import{terminateOwnedProcessTree}from"../../../🪓️termination/🟦️.ts";
import{runtimeNativeSystemV1}from"../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import{runtimeNativeLibcV1}from"../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts";
import{fillTestOrphansMessageV1 as fill,openFilesCeilingV1,testOrphansMessagesV1 as messages,testOrphansProtocolV1 as protocol}from"./👶️child/🟦️.ts";

export{limitOpenFilesV1,openFilesCeilingV1,startTestOrphanGuardV1,testOrphansProtocolV1}from"./👶️child/🟦️.ts";
export{parentGoneV1}from"./👷️parent/🟦️.ts";

const words=(environment:Readonly<Record<string,string|undefined>>)=>messages[resolveHostCapacityLanguageV1(environment)];
const ROW=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+([A-Za-z]{3}\s+[A-Za-z]{3}\s+\d+\s+[\d:]{8}\s+\d{4})\s+(.*)$/u;

/** 🪦️ One row of the process table: identity is the pid together with its start time, so a reused pid is never taken for the old process. */
export type TestProcessRowV1=Readonly<{pid:number;parent:number;group:number;start:string;command:string}>;
/** 🔌️ What the runner side observes and does: the process table, the open files of a pid (nothing where the host cannot tell), the end of a process tree and a pause; a test replaces them. */
export type TestOrphansHostV1=Readonly<{rows:()=>Promise<readonly TestProcessRowV1[]>;descriptors:(pid:number)=>number|undefined;terminate:(pid:number)=>void;sleep:(milliseconds:number)=>Promise<void>}>;
/** 🧾️ How one watched process ended: the most open files any process of its tree held and the processes that were still running after it ended. */
export type TestProcessEndV1=Readonly<{peakDescriptors:number;orphans:readonly Readonly<{pid:number;command:string}>[]}>;

/** 📋️ Reads the rows of `ps -A -o pid=,ppid=,pgid=,lstart=,command=` (the same columns on macOS and Linux); a line that does not parse is skipped. */
export function parseTestProcessRowsV1(text:string):readonly TestProcessRowV1[]{
 return text.split("\n").flatMap(line=>{const match=ROW.exec(line);return match?[Object.freeze({pid:Number(match[1]),parent:Number(match[2]),group:Number(match[3]),start:match[4]!.replace(/\s+/gu," "),command:match[5]!.slice(0,160)})]:[];});
}

/** 🗂️ Reads the process table of the host; a host without `ps` (Windows) answers no rows, so nothing is tracked there. */
export function readTestProcessRowsV1(platform:string=osPlatform()):Promise<readonly TestProcessRowV1[]>{
 if(platform==="win32")return Promise.resolve([]);
 return new Promise(resolve=>execFile("ps",["-A","-o","pid=,ppid=,pgid=,lstart=,command="],{encoding:"utf8",maxBuffer:128<<20,env:{...process.env,LC_ALL:"C"}},(error,stdout)=>resolve(error?[]:parseTestProcessRowsV1(stdout))));
}

let darwin:((pid:number)=>number|undefined)|undefined;
/** 🔢️ Counts the open files of a process: the descriptor table through `proc_pidinfo(PROC_PIDLISTFDS)` on macOS, `/proc/<pid>/fd` on Linux, nothing elsewhere or when the process is not ours. See https://man7.org/linux/man-pages/man5/proc_pid_fd.5.html */
export function countOpenDescriptorsV1(pid:number,platform:string=osPlatform()):number|undefined{
 if(platform==="linux")try{return readdirSync(`/proc/${pid}/fd`).length;}catch{return undefined;}
 if(platform!=="darwin")return undefined;
 darwin??=(()=>{
  try{
   const system=runtimeNativeSystemV1(),library=system.dlopen(runtimeNativeLibcV1(),{proc_pidinfo:{args:["i32","i32","u64","ptr","i32"],returns:"i32"}});
   return(target:number)=>{const size=library.symbols.proc_pidinfo(target,1,0,null,0);if(size<=0)return undefined;const buffer=new Uint8Array(size),got=library.symbols.proc_pidinfo(target,1,0,system.ptr(buffer),size);return got>0?got/8:undefined;};
  }catch{return()=>undefined;}
 })();
 return darwin(pid);
}

/** 🗣️ Words why a test process was stopped for its open files, in the language of the environment. */
export function describeOpenFilesExceededV1(file:string,count:number,pid:number,command:string,ceiling:number,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return fill(words(environment).descriptors,{file,count,pid,command:command.slice(0,80),ceiling});
}

/** 🗣️ Words the processes a test file left behind, in the language of the environment. */
export function describeOrphansV1(file:string,orphans:TestProcessEndV1["orphans"],environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return fill(words(environment).orphans,{file,count:orphans.length,list:orphans.slice(0,protocol.listedOrphans).map(row=>`${row.pid} ${row.command.slice(0,60)}`).join("; ")+(orphans.length>protocol.listedOrphans?`; +${orphans.length-protocol.listedOrphans}`:"")});
}

const defaultHost:TestOrphansHostV1=Object.freeze({rows:()=>readTestProcessRowsV1(),descriptors:pid=>countOpenDescriptorsV1(pid),terminate:pid=>terminateOwnedProcessTree(pid),sleep:milliseconds=>new Promise(done=>setTimeout(done,milliseconds))});

/** 👀️ Follows the process trees of `bun test` runs with one timer and one process table read per tick: it remembers every descendant (and every member of the process group) of a watched pid, stops a tree above its open files ceiling once, and when the run ends names, stops and returns what it left running. */
export function createTestProcessWatchV1(options:Readonly<{host?:Partial<TestOrphansHostV1>;environment?:Readonly<Record<string,string|undefined>>;intervalMilliseconds?:number;graceMilliseconds?:number}>={}){
 const host:TestOrphansHostV1={...defaultHost,...options.host},environment=options.environment??process.env,interval=options.intervalMilliseconds??protocol.observeMilliseconds,grace=options.graceMilliseconds??protocol.graceMilliseconds;
 const entries=new Map<number,{seen:Map<number,{start:string;command:string}>;peak:number;stopped:boolean;ceiling:number;file:string;terminate:(reason:string)=>void}>();
 let timer:ReturnType<typeof setInterval>|undefined,reading=false;
 const members=(table:readonly TestProcessRowV1[],root:number)=>{
  const wanted=new Set<number>([root]);
  for(let changed=true;changed;){changed=false;for(const row of table)if(!wanted.has(row.pid)&&(wanted.has(row.parent)||row.group===root)){wanted.add(row.pid);changed=true;}}
  return table.filter(row=>wanted.has(row.pid));
 };
 const tick=async()=>{
  if(reading||!entries.size)return;reading=true;
  try{
   const table=await host.rows();
   for(const[root,entry]of[...entries]){
    for(const row of members(table,root)){
     entry.seen.set(row.pid,{start:row.start,command:row.command});
     const count=host.descriptors(row.pid);
     if(count===undefined)continue;
     entry.peak=Math.max(entry.peak,count);
     if(count>=entry.ceiling*protocol.stopBasisPoints/10000&&!entry.stopped){entry.stopped=true;entry.terminate(describeOpenFilesExceededV1(entry.file,count,row.pid,row.command,entry.ceiling,environment));}
    }
   }
  }catch{}finally{reading=false;}
 };
 return Object.freeze({watch:(pid:number,input:Readonly<{file:string;ceiling?:number;terminate:(reason:string)=>void;report?:(text:string)=>void}>)=>{
  const entry={seen:new Map<number,{start:string;command:string}>(),peak:0,stopped:false,ceiling:input.ceiling??openFilesCeilingV1(environment),file:input.file,terminate:input.terminate};
  entries.set(pid,entry);
  if(timer===undefined){timer=setInterval(()=>{void tick();},interval);timer.unref();}
  void tick();
  return async():Promise<TestProcessEndV1>=>{
   await host.sleep(grace);
   const table=await host.rows(),alive=new Map(table.map(row=>[row.pid,row])),orphans:{pid:number;command:string}[]=[];
   entries.delete(pid);if(!entries.size&&timer!==undefined){clearInterval(timer);timer=undefined;}
   for(const[seenPid,identity]of entry.seen)if(seenPid!==pid&&alive.get(seenPid)?.start===identity.start)orphans.push({pid:seenPid,command:identity.command});
   for(const row of table)if(row.group===pid&&row.pid!==pid&&!entry.seen.has(row.pid))orphans.push({pid:row.pid,command:row.command});
   for(const orphan of orphans)host.terminate(orphan.pid);
   const end=Object.freeze({peakDescriptors:entry.peak,orphans:Object.freeze(orphans)});
   if(orphans.length)(input.report??(text=>{process.stderr.write(text+"\n");}))(describeOrphansV1(input.file,orphans,environment));
   return end;
  };
 }});
}
