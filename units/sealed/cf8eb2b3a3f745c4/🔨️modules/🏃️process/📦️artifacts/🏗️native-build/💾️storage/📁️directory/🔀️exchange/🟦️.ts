import { runtimeNativeSystemV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import { runtimeNativeLibcV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts";
import{openSync,closeSync,fsyncSync,constants}from"node:fs";
import{createRequire}from"node:module";
import{nativeDirectoryProtocolV1,type NativeDirectoryPlatformPortV1}from"../📋️contract/🟦️.ts";
const protocol=nativeDirectoryProtocolV1();

/** 🔌️ Isolates the one kernel call that atomically exchanges two directory names. */
export type NativeDirectoryExchangeCallV1=Readonly<{exchange(first:string,second:string):Readonly<{status:number;errno:number}>}>;

/** 💾️ Flushes one directory entry table through a no-follow directory descriptor. */
export function flushNativePosixDirectoryV1(path:string):void{const descriptor=openSync(path,constants.O_RDONLY|(constants.O_DIRECTORY??0)|(constants.O_NOFOLLOW??0));let failed=false,failure:unknown;try{fsyncSync(descriptor);}catch(error){failed=true;failure=error;}try{closeSync(descriptor);}catch(error){if(!failed)throw error;}if(failed)throw failure;}

const exchangeRow=()=>process.platform==="darwin"?protocol.exchange.darwin:protocol.exchange.linux;
/** 🧭️ Names how the kernel exchange is reached through one C library: the `renameat2` wrapper when the library has it, else the raw system call of the architecture. */
export type NativeDirectoryExchangeEntryV1=Readonly<{symbol:string;syscall:number|null}>;
/** 🔬️ Plans the ordered entries of one platform and architecture; pure, so every distribution layout is a table row of the laws. */
export function planNativeDirectoryExchangeEntriesV1(platform:string,arch:string):readonly NativeDirectoryExchangeEntryV1[]{
 if(platform==="darwin")return Object.freeze([Object.freeze({symbol:protocol.exchange.darwin.symbol,syscall:null})]);
 if(platform!=="linux")throw Error("Atomic directory exchange requires Linux or macOS");
 const linux=protocol.exchange.linux,number=(linux.syscalls as Readonly<Record<string,number>>)[arch];
 return Object.freeze([Object.freeze({symbol:linux.symbol,syscall:null}),...(number===undefined?[]:[Object.freeze({symbol:linux.fallbackSymbol,syscall:number})])]);
}
/** 🔗️ Binds the first entry of the plan that the given library and foreign function system offer; a library without any is a precise refusal naming what was tried. */
export function bindNativeDirectoryExchangeV1(system:Pick<ReturnType<typeof runtimeNativeSystemV1>,"dlopen"|"FFIType"|"read">,library:string,entries:readonly NativeDirectoryExchangeEntryV1[],platform:string,arch:string):NativeDirectoryExchangeCallV1{
 const{dlopen,FFIType,read}=system,row=platform==="darwin"?protocol.exchange.darwin:protocol.exchange.linux,error=row.error,failures:unknown[]=[];
 for(const entry of entries){
  const raw=entry.syscall!==null,args=raw?[FFIType.i64,FFIType.i32,FFIType.ptr,FFIType.i32,FFIType.ptr,FFIType.u32]:[FFIType.i32,FFIType.ptr,FFIType.i32,FFIType.ptr,FFIType.u32];
  let binding:ReturnType<typeof dlopen>;try{binding=dlopen(library,{[entry.symbol]:{args,returns:raw?FFIType.i64:FFIType.i32},[error]:{args:[],returns:FFIType.ptr}});}catch(cause){failures.push(cause);continue;}
  const call=binding.symbols[entry.symbol]as unknown as(...args:unknown[])=>number|bigint,locate=binding.symbols[error]as unknown as()=>number;
  return Object.freeze({exchange:(first:string,second:string)=>{const paths=[row.atFdcwd,Buffer.from(first+"\0"),row.atFdcwd,Buffer.from(second+"\0"),row.flag],status=Number(raw?call(entry.syscall,...paths):call(...paths));return Object.freeze({status,errno:status===0?0:read.i32(locate()as never)});}});
 }
 throw Error(protocol.refusals.unsupported+": "+library+" lacks "+entries.map(entry=>entry.symbol).join(" and ")+(platform==="linux"&&entries.length===1?"; no system call is declared for "+arch:""),{cause:new AggregateError(failures)});
}
/** 🧲️ Binds the C library the loader resolves through bun:ffi once. */
export function nativeDirectoryExchangeCallV1():NativeDirectoryExchangeCallV1{
 const platform=process.platform,entries=planNativeDirectoryExchangeEntriesV1(platform,process.arch);
 return bindNativeDirectoryExchangeV1(runtimeNativeSystemV1(),platform==="darwin"?protocol.exchange.darwin.library:runtimeNativeLibcV1(),entries,platform,process.arch);
}

/** 🔁️ Builds the atomic platform port: one exchange call, nothing journaled, nothing to recover. */
export function createNativeDirectoryExchangePortV1(call:NativeDirectoryExchangeCallV1|undefined=undefined,flush:(path:string)=>void=flushNativePosixDirectoryV1):NativeDirectoryPlatformPortV1{
 let selected=call;
 return Object.freeze({kind:"exchange"as const,
  swap:(first:string,second:string)=>{const outcome=(selected??=nativeDirectoryExchangeCallV1()).exchange(first,second);if(outcome.status===0)return;if((exchangeRow().unsupported as readonly number[]).includes(outcome.errno))throw Error(protocol.refusals.unsupported+" (errno "+outcome.errno+")");throw Error("Directory exchange failed (status "+outcome.status+", errno "+outcome.errno+")");},
  flush,pending:()=>false,recover:()=>"clean"as const,prepareVolume:()=>{}});
}
