import {constants,openSync,closeSync,fstatSync,lstatSync} from "node:fs";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { runtimeSupportsV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeNativeSystemV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import { runtimeNativeLibcV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts";

/** 🔐️ Retains exact physical compiler lock descriptors under the original process lifetime. */
export interface PhysicalFileLocksV1 {readonly pid:number;readonly paths:readonly string[];check():void;close():void;}

/** 🧱️ One host's kernel lock mechanism: `retain` opens and identifies a file, `tryLock` takes the exclusive lock on the file retained last (false = another holder; the failure code is read by a second foreign call, so a failure that is not the contention code is repeated up to `Protocol.attempts` times before it is refused), `verify` proves every retained file is still the named one, `close` releases everything. */
export interface PhysicalLockBackendV1 {retain(path:string):void;tryLock():boolean;verify():void;close():void;}

type NativeSystemV1=ReturnType<typeof runtimeNativeSystemV1>;
const messages=schema.definitions.Messages.const;
type PhysicalLockMessageKeyV1=keyof typeof messages;

/** 🚨️ A refusal of the physical lock: `message` is the English text (logs and matchers), `key` and `texts` carry both languages for the boundary that renders it. */
export class PhysicalLockErrorV1 extends Error{
 readonly key:PhysicalLockMessageKeyV1;readonly texts:Readonly<{en:string;de:string}>;
 constructor(key:PhysicalLockMessageKeyV1,code?:number){const fill=(text:string)=>text.replace("{code}",String(code));const texts=Object.freeze({en:fill(messages[key].en),de:fill(messages[key].de)});super(texts.en);this.name="PhysicalLockError";this.key=key;this.texts=texts;}
}

function retainPhysicalDescriptorV1(path:string,held:{fd:number;path:string;dev:number;ino:number}[]):number{
 const fd=openSync(path,constants.O_RDWR|constants.O_NOFOLLOW);let retained=false;
 try{const actual=fstatSync(fd),named=lstatSync(path);if(!actual.isFile()||named.isSymbolicLink()||actual.dev!==named.dev||actual.ino!==named.ino)throw new PhysicalLockErrorV1("identity");held.push({fd,path,dev:actual.dev,ino:actual.ino});retained=true;return fd;}finally{if(!retained)closeSync(fd);}
}

/** 🍎️ System flock through the required Bun runtime (macOS and Linux); the C library is bound by its loader name. See https://man7.org/linux/man-pages/man2/flock.2.html */
export function createPosixPhysicalLockBackendV1(system:NativeSystemV1=runtimeNativeSystemV1(),platform:string=process.platform,libc:string=runtimeNativeLibcV1()):PhysicalLockBackendV1{
 const {dlopen,read}=system,errnoName=platform==="darwin"?"__error":"__errno_location",library=dlopen(libc,{flock:{args:["i32","i32"],returns:"i32"},[errnoName]:{args:[],returns:"ptr"}}),protocol=schema.definitions.Protocol.const,held:{fd:number;path:string;dev:number;ino:number}[]=[];let closed=false;
 return Object.freeze({
  retain:(path:string)=>{retainPhysicalDescriptorV1(path,held);},
  tryLock:()=>{for(let attempt=0;attempt<protocol.attempts;attempt++){if(library.symbols.flock!(held.at(-1)!.fd,protocol.exclusive|protocol.nonblocking)===0)return true;const pointer=library.symbols[errnoName]!();if(pointer!==null&&read.i32(pointer)===protocol.contentionErrno[platform as"darwin"|"linux"])return false;}throw new PhysicalLockErrorV1("admission");},
  verify:()=>{for(const row of held){const actual=fstatSync(row.fd),named=lstatSync(row.path);if(!actual.isFile()||named.isSymbolicLink()||actual.dev!==row.dev||actual.ino!==row.ino||named.dev!==row.dev||named.ino!==row.ino)throw new PhysicalLockErrorV1("originalIdentity");}},
  close:()=>{if(closed)return;closed=true;try{for(const row of held.reverse()){library.symbols.flock!(row.fd,protocol.unlock);closeSync(row.fd);}}finally{library.close();}},
 });
}

/** 🪟️ Names a path for the wide Win32 file API without the 260-character limit: drive paths gain `\\?\`, UNC paths `\\?\UNC\`; an extended path stays as it is. See https://learn.microsoft.com/windows/win32/fileio/maximum-file-path-limitation */
export function windowsExtendedPathV1(path:string):string{
 if(path.startsWith("\\\\?\\"))return path;
 if(path.startsWith("\\\\"))return "\\\\?\\UNC\\"+path.slice(2);
 if(/^[A-Za-z]:\\/u.test(path))return "\\\\?\\"+path;
 throw new PhysicalLockErrorV1("absolutePath");
}

/** 🪟️ Windows byte-range locks through `kernel32`: the same whole-file exclusive `LockFileEx` range Cargo's own build lock takes, so a Cargo process and this lock exclude each other; the lock file is opened by this module (the Node descriptor of the runtime is not a handle) and identified by volume serial and file index. See https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-lockfileex */
export function createWindowsPhysicalLockBackendV1(system:Pick<NativeSystemV1,"dlopen"|"ptr">=runtimeNativeSystemV1()):PhysicalLockBackendV1{
 const win=schema.definitions.Windows.const,layout=win.information,invalid=BigInt(win.invalidHandle),kernel=system.dlopen(win.library,{
  CreateFileW:{args:["ptr","u32","u32","ptr","u32","u32","i64"],returns:"i64"},
  LockFileEx:{args:["i64","u32","u32","u32","u32","ptr"],returns:"i32"},
  UnlockFileEx:{args:["i64","u32","u32","u32","ptr"],returns:"i32"},
  CloseHandle:{args:["i64"],returns:"i32"},
  GetLastError:{args:[],returns:"u32"},
  GetFileInformationByHandle:{args:["i64","ptr"],returns:"i32"},
 }),symbols=kernel.symbols,rows:{handle:bigint;path:string;identity:string;locked:boolean}[]=[],overlapped=Buffer.alloc(win.overlappedBytes);let closed=false;
 const fail=(key:"openFailed"|"unreadable"):never=>{throw new PhysicalLockErrorV1(key,Number(symbols.GetLastError!()));};
 const open=(path:string,access:number):bigint=>{const name=Buffer.from(windowsExtendedPathV1(path)+"\0","utf16le"),handle=symbols.CreateFileW!(system.ptr(name),access,win.share,null,win.creation,win.reparseFlag,0n) as bigint;return handle===invalid?fail("openFailed"):handle;};
 const identify=(handle:bigint):string=>{const bytes=Buffer.alloc(layout.bytes);if(symbols.GetFileInformationByHandle!(handle,system.ptr(bytes))===0)fail("unreadable");const attributes=bytes.readUInt32LE(layout.attributes);if(attributes&(win.reparseAttribute|win.directoryAttribute))throw new PhysicalLockErrorV1("identity");return bytes.readUInt32LE(layout.volume)+":"+bytes.readUInt32LE(layout.indexHigh)+":"+bytes.readUInt32LE(layout.indexLow);};
 const range=(handle:bigint,flags:number|null):number=>{overlapped.fill(0);return flags===null?symbols.UnlockFileEx!(handle,0,win.rangeLow,win.rangeHigh,system.ptr(overlapped)):symbols.LockFileEx!(handle,flags,0,win.rangeLow,win.rangeHigh,system.ptr(overlapped));};
 return Object.freeze({
  retain:(path:string)=>{const handle=open(path,win.access);try{rows.push({handle,path,identity:identify(handle),locked:false});}catch(error){symbols.CloseHandle!(handle);throw error;}},
  tryLock:()=>{const row=rows.at(-1)!;let code=0;for(let attempt=0;attempt<schema.definitions.Protocol.const.attempts;attempt++){if(range(row.handle,win.exclusive|win.failImmediately)!==0){row.locked=true;return true;}code=Number(symbols.GetLastError!());if(code===win.contentionError)return false;}throw new PhysicalLockErrorV1("admissionWindows",code);},
  verify:()=>{for(const row of rows){if(identify(row.handle)!==row.identity)throw new PhysicalLockErrorV1("originalIdentity");const named=open(row.path,win.metadataAccess);try{if(identify(named)!==row.identity)throw new PhysicalLockErrorV1("originalIdentity");}finally{symbols.CloseHandle!(named);}}},
  close:()=>{if(closed)return;closed=true;try{for(const row of rows.reverse()){if(row.locked)range(row.handle,null);symbols.CloseHandle!(row.handle);}}finally{kernel.close();}},
 });
}

/** 🧭️ Picks the lock mechanism of one platform; a host without a qualified one is refused before any file is touched. */
export function physicalLockBackendV1(platform:string=process.platform,system?:NativeSystemV1):PhysicalLockBackendV1{
 const backends=schema.definitions.Backends.const;
 if(backends.posix.includes(platform))return createPosixPhysicalLockBackendV1(system);
 if(backends.windows.includes(platform))return createWindowsPhysicalLockBackendV1(system);
 throw new PhysicalLockErrorV1("unsupported");
}

function admitPhysicalRequestV1(paths:readonly string[],control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean}>):string[]{
 checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema.definitions.Request,request,schema).length)throw new PhysicalLockErrorV1("request");
 const admitted=paths.map(requireMaterializedPathV1).sort();if(validateJsonSchemaSubset(schema,admitted).length)throw new PhysicalLockErrorV1("paths");return admitted;
}

/** 🏪️ Takes every named file in sorted order through one backend and holds all of them until `close`; a held file fails at once or waits cancellably, and any failure releases what was taken. */
export async function holdPhysicalFileLocksV1(admitted:readonly string[],control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean}>,backend:PhysicalLockBackendV1):Promise<PhysicalFileLocksV1>{
 const protocol=schema.definitions.Protocol.const;let closed=false;
 try{
  for(const path of admitted){checkResourceLeaseControl(control);backend.retain(path);
   while(!backend.tryLock()){if(!request.wait)throw new PhysicalLockErrorV1("held");checkResourceLeaseControl(control);await control.publish({resource:"physical-compiler-lock:"+path,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();const remaining=checkResourceLeaseControl(control);await new Promise<void>(accept=>setTimeout(accept,Math.min(remaining??protocol.retryMilliseconds,protocol.retryMilliseconds)));checkResourceLeaseControl(control);}
  }
  const check=():void=>{checkResourceLeaseControl(control);if(closed)throw new PhysicalLockErrorV1("closed");backend.verify();},close=():void=>{closed=true;backend.close();};
  check();return Object.freeze({pid:process.pid,paths:Object.freeze([...admitted]),check,close});
 }catch(error){closed=true;backend.close();throw error;}
}

/** 🏪️ Uses the system file lock of the host through the required Bun runtime: flock on macOS and Linux, `LockFileEx` on Windows; unsupported backends refuse before mutation. */
export async function acquirePhysicalFileLocksV1(paths:readonly string[],control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean}>):Promise<PhysicalFileLocksV1>{
 const admitted=admitPhysicalRequestV1(paths,control,request);if(!runtimeSupportsV1("ffi"))throw new PhysicalLockErrorV1("unsupported");
 return holdPhysicalFileLocksV1(admitted,control,request,physicalLockBackendV1());
}
