import{lstatSync,openSync,closeSync,writeSync,fsyncSync,readFileSync,unlinkSync,constants}from"node:fs";
import{join,dirname,basename}from"node:path";
import{nativeDirectoryProtocolV1,nativeDirectorySchemaV1,observeNativeDirectoryIdentityV1,sameNativeDirectoryIdentityV1,type NativeDirectoryIdentityV1,type NativeDirectoryPlatformPortV1,type NativeDirectoryRecoveryV1}from"../📋️contract/🟦️.ts";
import{requireNativeWindowsCaseStatusV1,type NativeWindowsHostPortV1,type NativeWindowsResultV1}from"../🪟️host/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const protocol=nativeDirectoryProtocolV1(),schema=nativeDirectorySchemaV1,errors=protocol.windowsErrors;
type Intent=Readonly<{version:1;kind:"directory-swap";first:string;second:string;temporary:string;identities:Readonly<{first:NativeDirectoryIdentityV1;second:NativeDirectoryIdentityV1}>}>;

const exists=(path:string)=>{try{lstatSync(path);return true;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}};
const identityAt=(path:string):NativeDirectoryIdentityV1|undefined=>exists(path)?observeNativeDirectoryIdentityV1(path):undefined;
const same=(row:NativeDirectoryIdentityV1|undefined,expected:NativeDirectoryIdentityV1)=>!!row&&sameNativeDirectoryIdentityV1(row,expected);
/** 📜️ Serializes the canonical swap intent receipt. */
export function nativeDirectorySwapIntentBytesV1(input:Intent):Buffer{if(validateJsonSchemaSubset(schema.definitions.SwapIntent,input,schema).length)throw Error("Closed directory swap intent required");const bytes=Buffer.from(JSON.stringify(input)+"\n");if(bytes.length>protocol.swap.maximumIntentBytes)throw Error("Bounded directory swap intent required");return bytes;}
/** 📖️ Admits only canonical bounded intent bytes. */
export function readNativeDirectorySwapIntentV1(bytes:Uint8Array):Intent{if(bytes.byteLength>protocol.swap.maximumIntentBytes)throw Error("Bounded directory swap intent required");const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),row=JSON.parse(text);if(validateJsonSchemaSubset(schema.definitions.SwapIntent,row,schema).length||nativeDirectorySwapIntentBytesV1(row).toString()!==text)throw Error("Canonical directory swap intent required");return row as Intent;}
const pause=(milliseconds:number)=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};
/** ⏳️ Retries the first rename of a swap while an antivirus scanner or indexer holds a handle (sharing violation or access denied), with a bounded linear backoff; nothing has moved yet, so a retry cannot tear the swap. */
export function moveNativeDirectoryWithRetryV1(host:NativeWindowsHostPortV1,from:string,to:string,sleep:(milliseconds:number)=>void=pause):NativeWindowsResultV1{
 const retry=protocol.swap.retry;let result=host.moveFile(from,to);
 for(let attempt=1;attempt<retry.attempts&&!result.ok&&(result.error===errors.sharing||result.error===errors.accessDenied);attempt++){sleep(retry.backoffMilliseconds*attempt);result=host.moveFile(from,to);}
 return result;
}
const refuseMove=(label:string,result:NativeWindowsResultV1)=>{if(result.ok)return;throw Error(protocol.refusals.windowsRename+" ("+label+", Win32 error "+result.error+")"+(result.error===errors.accessDenied||result.error===errors.sharing?": "+protocol.refusals.windowsOpenHandle:""));};

/** 🪟️ Swaps two sibling directories with three durable write-through renames guarded by an intent receipt; only the lease holder may call it, so no reader exists while the name `m` is briefly absent. */
export function createNativeDirectoryJournaledPortV1(host:NativeWindowsHostPortV1):NativeDirectoryPlatformPortV1{
 const intentPath=(directory:string)=>join(directory,protocol.swap.intent),temporaryPath=(directory:string)=>join(directory,protocol.swap.temporary);
 const flushDirectory=(path:string)=>{const result=host.flushDirectory(path);if(result.ok||(protocol.exchange.windows.flush.tolerated as readonly number[]).includes(result.error))return;throw Error("Directory flush failed (Win32 error "+result.error+")");};
 const removeIntent=(directory:string)=>{unlinkSync(intentPath(directory));flushDirectory(directory);};
 const forward=(directory:string,intent:Intent,first:string,second:string,temporary:string):NativeDirectoryRecoveryV1=>{
  const state=()=>({first:identityAt(first),second:identityAt(second),temporary:identityAt(temporary)}),{first:a,second:b}=intent.identities;
  let now=state();
  if(same(now.first,a)&&same(now.second,b)&&!now.temporary){removeIntent(directory);return"rolled-back";}
  if(!now.first&&same(now.temporary,a)&&same(now.second,b)){refuseMove("second to first",host.moveFile(second,first));now=state();}
  if(same(now.first,b)&&same(now.temporary,a)&&!now.second){refuseMove("temporary to second",host.moveFile(temporary,second));now=state();}
  if(same(now.first,b)&&same(now.second,a)&&!now.temporary){removeIntent(directory);return"completed";}
  throw Error(protocol.refusals.swapState);
 };
 const recover=(directory:string):NativeDirectoryRecoveryV1=>{
  const path=intentPath(directory);if(!exists(path))return"clean";
  const first=join(directory,protocol.root),second=join(directory,protocol.volume);
  const row=lstatSync(path);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size>protocol.swap.maximumIntentBytes)throw Error(protocol.refusals.swapState);
  let intent:Intent;try{intent=readNativeDirectorySwapIntentV1(readFileSync(path));}
  catch(cause){if(exists(temporaryPath(directory))||!exists(first)||!exists(second))throw Error(protocol.refusals.swapState,{cause});removeIntent(directory);return"rolled-back";}
  const a=join(directory,intent.first),b=join(directory,intent.second),t=join(directory,intent.temporary);
  if(intent.temporary!==protocol.swap.temporary)throw Error(protocol.refusals.swapState);
  return forward(directory,intent,a,b,t);
 };
 return Object.freeze({kind:"journaled"as const,
  swap:(first:string,second:string)=>{
   const directory=dirname(first);if(dirname(second)!==directory||basename(first)===basename(second))throw Error(protocol.refusals.swapSiblings);
   const intent=intentPath(directory),temporary=temporaryPath(directory);if(exists(intent)||exists(temporary))throw Error(protocol.refusals.interrupted);
   const identities={first:observeNativeDirectoryIdentityV1(first),second:observeNativeDirectoryIdentityV1(second)},record:Intent={version:1,kind:"directory-swap",first:basename(first),second:basename(second),temporary:protocol.swap.temporary,identities},bytes=nativeDirectorySwapIntentBytesV1(record);
   const descriptor=openSync(intent,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);let failed=false,failure:unknown;
   try{if(writeSync(descriptor,bytes)!==bytes.length)throw Error("Complete directory swap intent write required");fsyncSync(descriptor);}catch(error){failed=true;failure=error;}
   try{closeSync(descriptor);}catch(error){if(!failed){failed=true;failure=error;}}
   if(failed){try{unlinkSync(intent);}catch{}throw failure;}
   flushDirectory(directory);
   const result=moveNativeDirectoryWithRetryV1(host,first,temporary);if(!result.ok){removeIntent(directory);refuseMove("first to temporary",result);}
   const entering=host.moveFile(second,first);if(!entering.ok){const back=host.moveFile(temporary,first);if(!back.ok)throw new AggregateError([Error(protocol.refusals.windowsRename+" (second to first, Win32 error "+entering.error+")"),Error(protocol.refusals.windowsRename+" (rollback, Win32 error "+back.error+")")],protocol.refusals.interrupted);removeIntent(directory);refuseMove("second to first",entering);}
   forward(directory,record,first,second,temporary);
  },
  flush:flushDirectory,
  pending:directory=>exists(intentPath(directory)),
  recover,
  prepareVolume:directory=>requireNativeWindowsCaseStatusV1(host.setCaseSensitive(directory))
 });
}
