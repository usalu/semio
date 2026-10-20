import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{captureOwnedCommandV1}from"../../../../../../../🎛️owned-execution/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔒️leases/🟦️.ts";
import{readNativeBootSessionV1}from"../../🟦️.ts";
import{parseNativeBootEpochV1}from"../📋️plan/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
import type{NativeBootClockV1}from"../📡️observe/🟦️.ts";
const epoch=schema.definitions.Protocol.const.bootEpoch;
let primed:NativeBootClockV1|undefined,priming:Promise<NativeBootClockV1>|undefined;

/** 🕰️ Reads which boot is running and when it began; both are memoized for the process because neither can change inside it. */
export function readNativeBootClockV1(control:ResourceLeaseControl,cwd:string):Promise<NativeBootClockV1>{
 if(primed)return Promise.resolve(primed);
 return priming??=(async()=>{
  try{
   const platform=process.platform;if(platform!=="darwin"&&platform!=="linux"&&platform!=="win32")throw Error("Boot clock unavailable on this platform");
   const text=platform==="darwin"?await captureOwnedCommandV1(epoch.darwin.program,epoch.darwin.arguments,cwd,{control,output:{maximumBytes:epoch.maximumBytes,maximumLines:4}}):platform==="win32"?await captureOwnedCommandV1(epoch.win32.program,epoch.win32.arguments,cwd,{control,output:{maximumBytes:epoch.maximumBytes,maximumLines:4}}):readFileSync(epoch.linux.file,"utf8");
   const bootEpochMs=parseNativeBootEpochV1(platform,text);if(bootEpochMs===null)throw Error("Exact boot epoch text required");
   const hex=createHash("sha256").update("boot:"+bootEpochMs).digest("hex").slice(0,32).toUpperCase(),bootSession=platform==="win32"?`${hex.slice(0,8)}-${hex.slice(8,12)}-${hex.slice(12,16)}-${hex.slice(16,20)}-${hex.slice(20,32)}`:await readNativeBootSessionV1(control,cwd);
   return primed=Object.freeze({bootSession,bootEpochMs});
  }finally{priming=undefined;}
 })();
}
