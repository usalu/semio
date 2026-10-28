import {renameSync,unlinkSync} from "node:fs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const;

/** 🪟️ The file operations the lease store performs on its own files, as ports: a test (or a host with other rules) replaces one, the platform decides which errors are only busy. */
export type LeaseFilePortsV1=Readonly<{platform:string;unlink(path:string):void;rename(from:string,to:string):void;sleep(milliseconds:number,signal?:AbortSignal):Promise<void>}>;

/** 🪟️ The real operations of this host. */
export const leaseFilePortsV1:LeaseFilePortsV1=Object.freeze({
 platform:process.platform,
 unlink:path=>unlinkSync(path),
 rename:(from,to)=>renameSync(from,to),
 sleep:(milliseconds,signal)=>new Promise<void>((accept,reject)=>{if(signal?.aborted){reject(signal.reason);return;}const timer=setTimeout(accept,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);reject(signal.reason);},{once:true});}),
});

/** 🚧️ Whether a file error is only a sharing violation: on Windows `EBUSY`, `EPERM` and `EACCES` while another process holds the file open, never on POSIX hosts (there the same codes are real refusals). Mirrors the `busy` kind of the churn race table. See https://learn.microsoft.com/windows/win32/fileio/sharing-and-locking */
export function leaseFileBusyV1(error:unknown,platform:string=process.platform):boolean{
 const code=(error as {code?:unknown}|null)?.code,codes=(policy.busyCodes as Readonly<Record<string,readonly string[]>>)[platform];
 return typeof code==="string"&&codes!==undefined&&codes.includes(code);
}

/** 🚧️ Whether SQLite failed to open a lease file only transiently: on Windows `SQLITE_CANTOPEN` (primary result 14) while the file is delete-pending or still held by a scanner after SQLite's own retries; elsewhere it is a real refusal. See https://www.sqlite.org/rescode.html#cantopen */
export function leaseDatabaseUnopenableBusyV1(error:unknown,platform:string=process.platform):boolean{
 const rule=(policy.unopenable as Readonly<Record<string,{code:string;primary:number}>>)[platform],value=error!==null&&typeof error==="object"?error as{code?:string;errcode?:number;errno?:number}:undefined;
 return rule!==undefined&&(value?.code===rule.code||((value?.errcode??value?.errno??-1)&255)===rule.primary);
}

/** ⏱️ The pauses before a transiently unopenable lease database is given up on, in milliseconds. */
export const leaseUnopenablePausesV1:readonly number[]=Object.freeze([...policy.pausesMilliseconds]);

/** 🗑️ The `rmSync` options that survive a transient sharing violation on this platform (Node's own bounded retry), `force` everywhere. */
export function leaseRemovalOptionsV1(platform:string=process.platform):Readonly<{force:true;maxRetries?:number;retryDelay?:number}>{
 const rule=(policy.removal as Readonly<Record<string,{maxRetries:number;retryDelay:number}>>)[platform];
 return rule===undefined?{force:true}:{force:true,...rule};
}

/** 🔁️ The verdict of {@link settleLeaseFileV1}: the value, or the busy error that stayed through every pause. */
export type LeaseFileSettleV1<T>=Readonly<{settled:true;value:T}>|Readonly<{settled:false;error:unknown}>;

/** 🔁️ Runs one lease file operation: a sharing violation is retried on the declared pauses (cancellable), a persistent one comes back as `settled:false` so the caller can keep the file for the next sweep, and any other error is thrown at once. */
export async function settleLeaseFileV1<T>(operation:()=>T,ports:Pick<LeaseFilePortsV1,"platform"|"sleep">=leaseFilePortsV1,signal?:AbortSignal):Promise<LeaseFileSettleV1<T>>{
 for(let attempt=0;;attempt++){
  try{return{settled:true,value:operation()};}
  catch(error){
   if(!leaseFileBusyV1(error,ports.platform))throw error;
   const pause=policy.pausesMilliseconds[attempt];if(pause===undefined)return{settled:false,error};
   await ports.sleep(pause,signal);
  }
 }
}
