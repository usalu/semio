import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {listOriginsV1,replaceOriginFileV1,requireOriginIdV1,storePathsV1} from "../../📒️journal/🟦️.ts";

/** 🙋️ One blob an origin cannot read (missing, or not hashing to its name) and the path whose entry needs it. */
export type BlobWantV1=Readonly<{sha256:string;path:string}>;

/** 📣️ The record `origins/<origin>/wants.json` of the store, shaped by `🧬️schema/🔣️.json`: the blobs that origin asks the fleet to put back, at most `wantsLimitV1`, an empty list when it wants none. */
export type BlobWantsV1=Readonly<{version:1;originId:string;wallMs:number;wants:readonly BlobWantV1[]}>;

/** 🔢️ How many wants one record carries. */
export const wantsLimitV1=64;

const hex=/^[0-9a-f]{64}$/u,origin=/^[a-z0-9][a-z0-9-]{0,39}$/u,fileName="wants.json";

/** 🔎️ Tells whether a value is a closed wants record. */
export function validWantsV1(value:unknown):value is BlobWantsV1{
 if(typeof value!=="object"||value===null||Array.isArray(value))return false;
 const record=value as Record<string,unknown>,keys=Object.keys(record).sort();
 if(keys.join()!=="originId,version,wallMs,wants"||record.version!==1||typeof record.originId!=="string"||!origin.test(record.originId))return false;
 if(!Number.isInteger(record.wallMs)||(record.wallMs as number)<0||!Array.isArray(record.wants)||record.wants.length>wantsLimitV1)return false;
 return record.wants.every(item=>typeof item==="object"&&item!==null&&Object.keys(item).sort().join()==="path,sha256"&&hex.test((item as BlobWantV1).sha256)&&typeof (item as BlobWantV1).path==="string"&&(item as BlobWantV1).path.length>=1&&(item as BlobWantV1).path.length<=4096);
}

/** ✍️ Writes the wants of the own origin atomically (the one small file of the own origin directory that other origins read to repair blobs). */
export async function writeWantsV1(storeRoot:string,record:BlobWantsV1):Promise<void>{
 requireOriginIdV1(record.originId);
 if(!validWantsV1(record))throw Error("Closed blob wants required");
 await replaceOriginFileV1(storeRoot,record.originId,fileName,JSON.stringify(record));
}

/** 📖️ Reads the wants of one origin; `undefined` when there are none or the file is not a closed record of that origin. */
export async function readWantsV1(storeRoot:string,originId:string):Promise<BlobWantsV1|undefined>{
 try{
  const value=JSON.parse(await readFile(join(storePathsV1(storeRoot).origins,originId,fileName),"utf8")) as unknown;
  return validWantsV1(value)&&value.originId===originId?value:undefined;
 }catch{return undefined;}
}

/** 🧺️ The non-empty wants of every origin except `except`, in the order of the origin ids. */
export async function readOthersWantsV1(storeRoot:string,except:string):Promise<readonly BlobWantsV1[]>{
 const found:BlobWantsV1[]=[];
 for(const originId of [...await listOriginsV1(storeRoot)].sort()){
  if(originId===except)continue;
  const record=await readWantsV1(storeRoot,originId);
  if(record!==undefined&&record.wants.length>0)found.push(record);
 }
 return found;
}
