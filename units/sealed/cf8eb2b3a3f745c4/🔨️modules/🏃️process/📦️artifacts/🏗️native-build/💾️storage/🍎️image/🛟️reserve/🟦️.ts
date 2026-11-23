import{closeSync,openSync,renameSync,rmSync,statSync,statfsSync,writeSync}from"node:fs";
import{randomBytes}from"node:crypto";
import{join}from"node:path";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
const policy=imageSchema.definitions.ImageProtocol.const.reclaim.availability.refresh.reserve;
/** 📏️ Reports the bytes a writer can still take at a path; `undefined` when unknown. */
export type NativeImageReserveSpaceV1=(path:string)=>Readonly<{availableBytes:number}>|undefined;
const measure:NativeImageReserveSpaceV1=path=>{try{const row=statfsSync(path);return{availableBytes:Number(row.bavail)*row.bsize};}catch{return undefined;}};
/** 🛟️ Names the reserve file of a mounted image root: a file of real bytes the volume can give back when it is full or latched at zero, so a flush, a claim or a delete still finds room. */
export function nativeImageReservePathV1(root:string):string{return join(root,policy.file);}
/** 🆘️ Gives the reserve file at a path back; `true` when bytes were freed. Never throws. */
export function releaseNativeImageReserveFileV1(path:string):boolean{
 try{const bytes=statSync(path).size;rmSync(path);return bytes>0;}catch{return false;}
}
/** 🧱️ Keeps a reserve file of `bytes` whole while the volume has `multiplier` times its size free, so it never takes the last bytes; `present`, `created` or `skipped` when the volume is too full or the write failed. Never throws. */
export function ensureNativeImageReserveFileV1(path:string,bytes:number,multiplier:number,space:NativeImageReserveSpaceV1=measure):"present"|"created"|"skipped"{
 try{if(statSync(path).size>=bytes)return"present";}catch{}
 let temporary="";
 try{
  const free=space(path);if(!free||free.availableBytes<bytes*multiplier)return"skipped";
  temporary=path+"."+randomBytes(3).toString("hex")+".tmp";const fd=openSync(temporary,"w"),block=Buffer.alloc(policy.chunkBytes,0x5a);
  try{for(let written=0;written<bytes;written+=policy.chunkBytes)writeSync(fd,block,0,Math.min(policy.chunkBytes,bytes-written));}finally{closeSync(fd);}
  renameSync(temporary,path);return"created";
 }catch{if(temporary)try{rmSync(temporary,{force:true});}catch{}return"skipped";}
}
/** 🆘️ Gives the reserve of a mounted image root back; `true` when bytes were freed. Never throws. */
export function releaseNativeImageReserveV1(root:string):boolean{return releaseNativeImageReserveFileV1(nativeImageReservePathV1(root));}
/** 🧱️ Keeps the reserve of a mounted image root whole while the volume has room for it. Never throws. */
export function ensureNativeImageReserveV1(root:string,space:NativeImageReserveSpaceV1=measure):"present"|"created"|"skipped"{return ensureNativeImageReserveFileV1(nativeImageReservePathV1(root),policy.bytes,policy.creationFreeMultiplier,space);}
/** 🚪️ Runs a write and, when the volume refuses it for lack of space, gives the reserve back and runs it once more. */
export function withNativeImageReserveV1<T>(root:string,body:()=>T):T{
 try{return body();}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOSPC"||!releaseNativeImageReserveV1(root))throw error;return body();}
}
