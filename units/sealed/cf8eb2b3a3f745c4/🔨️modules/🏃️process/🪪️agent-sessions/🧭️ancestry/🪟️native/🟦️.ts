import {runtimeSupportsV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {runtimeNativeSystemV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {refuseProcessV1} from "../../../📖️texts/🟦️.ts";

const windows=schema.definitions.Policy.const.identity.ancestry.windows,record=windows.entry,line=windows.commandLine,time=windows.fileTime;

/** 🧵️ The exact kernel calls the process listing needs, as one replaceable call table (the foreign-function port): a process snapshot, its two cursors, one process handle with its creation time and command line, and the release of every handle. */
export type WindowsProcessBindingV1=Readonly<{
 snapshot():bigint;
 first(snapshot:bigint,entry:Buffer):number;
 next(snapshot:bigint,entry:Buffer):number;
 open(pid:number):bigint;
 creation(process:bigint,filetime:Buffer):number;
 commandLine(process:bigint,buffer:Buffer,returned:Buffer):number;
 close(handle:bigint):number;
 release():void;
}>;
/** 📸️ One row of the process snapshot: id, parent id and the image name without a directory (`Win32_Process.Name`). */
export type WindowsProcessEntryV1=Readonly<{pid:number;parent:number;name:string}>;
/** 🔎️ What one opened process adds to its snapshot row: the UTC start as ISO text with 7 fractional digits (the spelling the CIM listing carries) and the command line, empty when the process does not tell it. */
export type WindowsProcessDetailV1=Readonly<{started:string;commandLine:string}>;
/** 🖥️ The semantic Windows process listing: the snapshot once, the detail of a process on demand (undefined when it is gone or refuses a query handle). */
export type WindowsProcessHostV1=Readonly<{entries():readonly WindowsProcessEntryV1[];detail(pid:number):WindowsProcessDetailV1|undefined}>;
/** 🔌️ What a caller can replace to run the listing without a Windows kernel: the opener, the platform and architecture, and the foreign-function check. */
export type WindowsProcessOptionsV1=Readonly<{open?:()=>WindowsProcessBindingV1;platform?:string;architecture?:string;ffi?:()=>boolean}>;

/** 🕰️ Spells a FILETIME (100 ns ticks since 1601-01-01 UTC) the way the CIM listing spells `CreationDate` after `ToUniversalTime().ToString('o')`: `yyyy-MM-ddTHH:mm:ss.fffffffZ`, floored to the microsecond a WMI date can carry; undefined for an instant outside 1970..9999. See https://learn.microsoft.com/windows/win32/api/minwinbase/ns-minwinbase-filetime */
export function windowsFileTimeIsoV1(ticks:bigint):string|undefined{
 const second=BigInt(time.ticksPerSecond),since=ticks-BigInt(time.unixEpochTicks);
 if(since<=0n||since/second>BigInt(time.maximumUnixSeconds))return undefined;
 const fraction=Number(since%second);
 return new Date(Number(since/second)*1000).toISOString().slice(0,19)+"."+String(fraction-fraction%time.microsecondTicks).padStart(7,"0")+"Z";
}

/** 🧩️ Builds the semantic listing over a call table; no handle leaves this function, every snapshot and process handle opened is closed before it returns. */
export function createWindowsProcessHostV1(binding:WindowsProcessBindingV1):WindowsProcessHostV1{
 const invalid=BigInt(windows.invalidHandle),buffer=Buffer.alloc(line.capacityBytes),returned=Buffer.alloc(4),filetime=Buffer.alloc(8);
 let table:readonly WindowsProcessEntryV1[]|undefined;
 const name=(entry:Buffer)=>{const text=entry.toString("utf16le",record.name,record.name+record.nameUnits*2),end=text.indexOf("\0");return end<0?text:text.slice(0,end);};
 const entries=():readonly WindowsProcessEntryV1[]=>{
  if(table)return table;
  const snapshot=binding.snapshot();if(snapshot===invalid||snapshot===0n)throw refuseProcessV1("refused","agentSessions.process-snapshot-refused");
  const rows:WindowsProcessEntryV1[]=[];
  try{
   const entry=Buffer.alloc(record.bytes);entry.writeUInt32LE(record.bytes,0);
   for(let more=binding.first(snapshot,entry);more!==0;more=binding.next(snapshot,entry))rows.push(Object.freeze({pid:entry.readUInt32LE(record.processId),parent:entry.readUInt32LE(record.parentId),name:name(entry)}));
  }finally{binding.close(snapshot);}
  if(!rows.length)throw refuseProcessV1("invalid","agentSessions.process-snapshot-empty");
  return table=Object.freeze(rows);
 };
 const commandLine=(handle:bigint):string=>{
  if(binding.commandLine(handle,buffer,returned)<0)return"";
  const bytes=buffer.readUInt16LE(0);
  return bytes===0||bytes%2!==0||line.headerBytes+bytes>buffer.length?"":buffer.toString("utf16le",line.headerBytes,line.headerBytes+bytes);
 };
 return Object.freeze({entries,detail:(pid:number)=>{
  const handle=binding.open(pid);if(handle===0n)return undefined;
  try{
   if(binding.creation(handle,filetime)!==1)return undefined;
   const started=windowsFileTimeIsoV1(filetime.readBigUInt64LE(0));
   return started===undefined?undefined:Object.freeze({started,commandLine:commandLine(handle)});
  }finally{binding.close(handle);}
 }});
}

/** 🔓️ Loads kernel32 and ntdll through the runtime's foreign-function port; the six calls bind by name, every handle is a 64-bit unsigned value, and the library is closed by `release`. See https://learn.microsoft.com/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot and https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes */
export function openWindowsProcessBindingV1(system:Pick<ReturnType<typeof runtimeNativeSystemV1>,"dlopen"|"ptr">=runtimeNativeSystemV1()):WindowsProcessBindingV1{
 const{dlopen,ptr}=system,kernel=dlopen(windows.libraries.kernel,{
  CreateToolhelp32Snapshot:{args:["u32","u32"],returns:"u64"},Process32FirstW:{args:["u64","ptr"],returns:"i32"},Process32NextW:{args:["u64","ptr"],returns:"i32"},
  OpenProcess:{args:["u32","i32","u32"],returns:"u64"},GetProcessTimes:{args:["u64","ptr","ptr","ptr","ptr"],returns:"i32"},CloseHandle:{args:["u64"],returns:"i32"},
 }),nt=(()=>{try{return dlopen(windows.libraries.nt,{NtQueryInformationProcess:{args:["u64","u32","ptr","u32","ptr"],returns:"i32"}});}catch(error){kernel.close();throw error;}})();
 const k=kernel.symbols as unknown as Record<string,(...args:unknown[])=>unknown>,n=nt.symbols as unknown as Record<string,(...args:unknown[])=>unknown>,exit=Buffer.alloc(8),kernelTime=Buffer.alloc(8),user=Buffer.alloc(8);
 return Object.freeze({
  snapshot:()=>BigInt(k.CreateToolhelp32Snapshot!(windows.snapshotProcesses,0)as bigint),
  first:(snapshot,entry)=>Number(k.Process32FirstW!(snapshot,ptr(entry))),
  next:(snapshot,entry)=>Number(k.Process32NextW!(snapshot,ptr(entry))),
  open:pid=>BigInt(k.OpenProcess!(windows.queryLimitedInformation,0,pid)as bigint),
  creation:(process,filetime)=>Number(k.GetProcessTimes!(process,ptr(filetime),ptr(exit),ptr(kernelTime),ptr(user))),
  commandLine:(process,buffer,returned)=>Number(n.NtQueryInformationProcess!(process,line.informationClass,ptr(buffer),buffer.length,ptr(returned))),
  close:handle=>Number(k.CloseHandle!(handle)),
  release:()=>{let failure:unknown,failed=false;for(const library of[kernel,nt])try{library.close();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;},
 });
}

/** 🪟️ Runs one read of the process listing against the real kernel and releases it: the value of `use`, or null when this is not Windows on x64 or arm64, the runtime has no foreign-function interface, a library or call is refused, or `use` throws (the caller then asks the shell). */
export function withWindowsProcessHostV1<T>(use:(host:WindowsProcessHostV1)=>T,options:WindowsProcessOptionsV1={}):T|null{
 const{open=()=>openWindowsProcessBindingV1(),platform=process.platform,architecture=process.arch,ffi=()=>runtimeSupportsV1("ffi")}=options;
 if(platform!=="win32"||!(windows.architectures as readonly string[]).includes(architecture)||!ffi())return null;
 let binding:WindowsProcessBindingV1|undefined;
 try{binding=open();return use(createWindowsProcessHostV1(binding));}catch{return null;}
 finally{try{binding?.release();}catch{}}
}
