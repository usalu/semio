import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@VALID@@";
import {SEMIO_POLICY_V1 as policy,semioHashV1,semioOutputsDigestV1,type SemioEntryV1} from "../@@TS@@";

/** 🌐️ The shared cache a build may read from and write to besides its local store; every failure is a miss, never a build failure. */
export interface SemioRemoteV1{
 get(key:string,signal?:AbortSignal):Promise<Uint8Array|undefined>;
 put(key:string,bytes:Uint8Array,signal?:AbortSignal):Promise<void>;
}

const encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});

/** 📦️ Packs an entry and the blobs it names into one archive: magic, header length, header JSON, then the blobs in header order. */
export async function packSemioEntryV1(entry:SemioEntryV1,readBlob:(sha:string)=>Promise<Uint8Array>):Promise<Uint8Array>{
 const blobs:Uint8Array[]=[],shas=[...new Set(entry.files.filter(file=>file.kind==="file").map(file=>file.sha256!))].sort();
 for(const sha of shas)blobs.push(await readBlob(sha));
 const header=encoder.encode(JSON.stringify({entry,blobs:shas.map((sha,at)=>({sha256:sha,length:blobs[at]!.length}))})),magic=encoder.encode(policy.archiveMagic),size=magic.length+4+header.length+blobs.reduce((sum,blob)=>sum+blob.length,0);
 if(size>policy.maximumEntryBytes)throw Error("Bounded cache entry size required");
 const output=new Uint8Array(size),view=new DataView(output.buffer);
 output.set(magic,0);view.setUint32(magic.length,header.length);output.set(header,magic.length+4);
 let at=magic.length+4+header.length;
 for(const blob of blobs){output.set(blob,at);at+=blob.length;}
 return output;
}

/** 📭️ Unpacks an archive, verifies every blob digest, the entry digest and the entry key, hands the blobs to `writeBlob` and returns the entry; any defect refuses the whole archive. */
export async function unpackSemioEntryV1(bytes:Uint8Array,key:string,writeBlob:(sha:string,bytes:Uint8Array)=>Promise<void>):Promise<SemioEntryV1>{
 const magic=encoder.encode(policy.archiveMagic);
 if(bytes.length>policy.maximumEntryBytes||bytes.length<magic.length+4||!magic.every((value,at)=>bytes[at]===value))throw Error("Closed cache archive required");
 const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),length=view.getUint32(magic.length),start=magic.length+4;
 if(start+length>bytes.length)throw Error("Closed cache archive required");
 const header=JSON.parse(decoder.decode(bytes.subarray(start,start+length))) as {entry:SemioEntryV1;blobs:{sha256:string;length:number}[]};
 if(validateJsonSchemaSubset(schema.definitions.Entry,header.entry,schema).length||header.entry.key!==key||semioOutputsDigestV1(header.entry.files)!==header.entry.digest||!Array.isArray(header.blobs))throw Error("Closed cache archive required");
 let at=start+length;const wanted=new Set(header.entry.files.filter(file=>file.kind==="file").map(file=>file.sha256!)),seen=new Set<string>();
 for(const blob of header.blobs){
  if(!Number.isSafeInteger(blob.length)||blob.length<0||at+blob.length>bytes.length)throw Error("Closed cache archive required");
  const body=bytes.subarray(at,at+blob.length);at+=blob.length;
  if(semioHashV1(body)!==blob.sha256||!wanted.has(blob.sha256))throw Error("Closed cache archive required");
  seen.add(blob.sha256);await writeBlob(blob.sha256,body);
 }
 if(at!==bytes.length||seen.size!==wanted.size)throw Error("Closed cache archive required");
 return header.entry;
}

/** 🛰️ Speaks the write-once blob route of the in-house cache server (`/v1/cache/<key>` with an optional bearer token). A miss, an error and a timeout all answer "nothing"; an existing key (409) counts as stored. */
export function semioHttpRemoteV1(options:Readonly<{url:string;token?:string;fetch?:typeof fetch}>):SemioRemoteV1{
 const call=options.fetch??fetch,base=options.url.replace(/\/+$/u,"")+policy.remotePath+policy.remoteKeyPrefix,headers=options.token?{authorization:"Bearer "+options.token}:{} as Record<string,string>;
 const timeout=(signal?:AbortSignal)=>signal?AbortSignal.any([signal,AbortSignal.timeout(policy.remoteTimeoutMilliseconds)]):AbortSignal.timeout(policy.remoteTimeoutMilliseconds);
 const remote:SemioRemoteV1={
  async get(key,signal){try{const response=await call(base+key,{headers,signal:timeout(signal)});if(!response.ok){await response.arrayBuffer().catch(()=>undefined);return undefined;}return new Uint8Array(await response.arrayBuffer());}catch(error){if(signal?.aborted)throw error;return undefined;}},
  async put(key,bytes,signal){try{const response=await call(base+key,{method:"PUT",headers:{...headers,"content-type":"application/octet-stream"},body:bytes as BodyInit,signal:timeout(signal)});await response.arrayBuffer().catch(()=>undefined);}catch(error){if(signal?.aborted)throw error;}}
 };
 return Object.freeze(remote);
}
