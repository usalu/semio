import {execFile} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import type {ProcessFactsV1} from "../🟦️.ts";

/** 📦️ Modules of sets that may not be installed on a checkout; a probe reads them only when they exist. */
export type DoctorOptionalModuleV1="tools"|"sessions-core"|"sessions-build";
const OPTIONAL:Readonly<Record<DoctorOptionalModuleV1,string>>={
 tools:"../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts",
 "sessions-core":"../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🟦️.ts",
 "sessions-build":"../../🪪️agent-sessions/🟦️.ts",
};

/** 🔌️ Everything a probe may touch outside the files of the checkout; tests replace each port. */
export type DoctorPortsV1=Readonly<{
 root:string;environment:Readonly<Record<string,string|undefined>>;platform:"darwin"|"linux"|"win32";signal:AbortSignal;
 now():number;alive(pid:number):boolean;progress(stage:string):void;
 processes():Promise<readonly ProcessFactsV1[]>;command(program:string,args:readonly string[]):Promise<string>;
 optional(name:DoctorOptionalModuleV1):Promise<Readonly<Record<string,unknown>>|undefined>;
}>;

/** 🧟️ Asks the operating system whether a process still exists; a process we may not signal still exists. */
export function doctorPidAliveV1(pid:number):boolean{
 try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}
}

/** 📖️ Reads a small JSON file; a missing, oversized or malformed file reads as `undefined`. */
export function readJsonFileV1(path:string,maximumBytes:number=4194304):unknown{
 try{const text=readFileSync(path,"utf8");return text.length>maximumBytes?undefined:JSON.parse(text);}catch{return undefined;}
}

/** ⏱️ Reads `[[dd-]hh:]mm:ss` of `ps` into seconds. */
export function parseElapsedSecondsV1(text:string):number{
 const match=/^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)$/u.exec(text.trim());if(!match)return 0;
 return Number(match[1]??0)*86400+Number(match[2]??0)*3600+Number(match[3])*60+Number(match[4]);
}

/** 🗒️ Parses `ps -axo pid=,ppid=,etime=,rss=,pcpu=,command=`. */
export function parsePosixProcessListingV1(text:string):readonly ProcessFactsV1[]{
 const rows:ProcessFactsV1[]=[];
 for(const line of text.split("\n")){
  const match=/^\s*(\d+)\s+(\d+)\s+(\S+)\s+(\d+)\s+([\d.,]+)\s+(.*)$/u.exec(line);if(!match)continue;
  rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),ageSeconds:parseElapsedSecondsV1(match[3]!),rssMegabytes:Math.round(Number(match[4])/1024),cpuPercent:Number(match[5]!.replace(",","."))||0,command:match[6]!.trim()}));
 }
 return rows.filter(row=>row.pid>0);
}

/** 🪟️ Parses the JSON of `Get-CimInstance Win32_Process` (processes without a command line keep their image name). */
export function parseWindowsProcessListingV1(text:string,nowMs:number):readonly ProcessFactsV1[]{
 let value:unknown;try{value=JSON.parse(text);}catch{return [];}
 const rows:ProcessFactsV1[]=[];
 for(const row of Array.isArray(value)?value:[value]){
  const item=row as {ProcessId?:number;ParentProcessId?:number;CreationDate?:string;WorkingSetSize?:number;CommandLine?:string|null;Name?:string};
  if(!Number.isSafeInteger(item?.ProcessId)||item.ProcessId!<1)continue;
  const created=/\/Date\((\d+)\)\//u.exec(String(item.CreationDate??""))?.[1],started=created!==undefined?Number(created):Date.parse(String(item.CreationDate??""));
  rows.push(Object.freeze({pid:item.ProcessId!,parent:Number.isSafeInteger(item.ParentProcessId)?item.ParentProcessId!:0,ageSeconds:Number.isFinite(started)?Math.max(0,Math.floor((nowMs-started)/1000)):0,rssMegabytes:Math.round((item.WorkingSetSize??0)/1048576),cpuPercent:0,command:String(item.CommandLine??item.Name??"")}));
 }
 return rows;
}

const run=(program:string,args:readonly string[],environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,timeout:number):Promise<string>=>new Promise((resolve,reject)=>{
 execFile(program,[...args],{env:{...environment,LC_ALL:process.platform==="darwin"?"en_US.UTF-8":"C.UTF-8"} as NodeJS.ProcessEnv,maxBuffer:64*1048576,timeout,signal,windowsHide:true,encoding:"utf8"},(error,stdout)=>error?reject(error):resolve(stdout));
});

/** 🏭️ Completes the ports with the portable defaults: the listing of processes comes from `ps` (POSIX) or CIM (Windows), optional modules from their files. */
export function doctorPortsV1(root:string,overrides:Partial<DoctorPortsV1>&Pick<DoctorPortsV1,"signal">):DoctorPortsV1{
 const environment=overrides.environment??process.env,platform=overrides.platform??(process.platform==="win32"?"win32":process.platform==="darwin"?"darwin":"linux"),now=overrides.now??Date.now,signal=overrides.signal;
 const command=overrides.command??((program,args)=>run(program,args,environment,signal,15000));
 return Object.freeze({
  root,environment,platform,now,alive:doctorPidAliveV1,progress:()=>{},command,
  processes:async()=>platform==="win32"?parseWindowsProcessListingV1(await command("powershell",["-NoProfile","-NonInteractive","-Command","Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,CreationDate,WorkingSetSize,CommandLine,Name | ConvertTo-Json -Compress"]),now()):parsePosixProcessListingV1(await command("ps",["-axo","pid=,ppid=,etime=,rss=,pcpu=,command="])),
  optional:async name=>{
   const url=new URL(OPTIONAL[name],import.meta.url);
   if(!existsSync(fileURLToPath(url)))return undefined;
   return await import(url.href) as Readonly<Record<string,unknown>>;
  },
  ...overrides,
 });
}
