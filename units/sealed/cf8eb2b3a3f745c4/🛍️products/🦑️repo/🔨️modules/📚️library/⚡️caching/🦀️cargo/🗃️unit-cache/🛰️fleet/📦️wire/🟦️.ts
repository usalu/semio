import {createHash} from "node:crypto";
import {FLEET_POLICY_V1,type PlatformV1} from "../🪪️identity/🟦️.ts";
import {requireSlotShapeV1,type DeclarationV1} from "../../../../../🏗️build/🪢️slots/🟦️.ts";

const fleet=FLEET_POLICY_V1,HEX=/^[a-f0-9]{64}$/u,EMPTY=createHash("sha256").digest("hex"),encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});

/** 🔢️ A blob address of Bazel's remote execution API. */
export type WireDigestV1=Readonly<{hash:string;size:number}>;
/** 📦️ One file of an action result. */
export type WireFileV1=Readonly<{path:string;digest:WireDigestV1;executable:boolean}>;
/** 📬️ What a unit's action result says: its files, the two streams, the slot and the worker name. */
export type WireResultV1=Readonly<{files:readonly WireFileV1[];stdout:WireDigestV1|undefined;stderr:WireDigestV1|undefined;slot:string|undefined;worker:string|undefined}>;

function varint(value:number):number[]{
 const bytes:number[]=[];let rest=value;
 while(rest>=128){bytes.push((rest%128)|128);rest=Math.floor(rest/128);}
 bytes.push(rest);return bytes;
}
const tag=(field:number,wire:number)=>varint(field*8+wire);
function concat(parts:readonly (readonly number[]|Uint8Array)[]):Uint8Array{
 let size=0;for(const part of parts)size+=part.length;
 const out=new Uint8Array(size);let offset=0;for(const part of parts){out.set(part,offset);offset+=part.length;}return out;
}
const delimited=(field:number,payload:Uint8Array)=>concat([tag(field,2),varint(payload.length),payload]);
const text=(field:number,value:string)=>delimited(field,encoder.encode(value));
const flag=(field:number,value:boolean)=>concat([tag(field,0),[value?1:0]]);
const integer=(field:number,value:number)=>concat([tag(field,0),varint(value)]);

/** 🔢️ The digest of bytes. */
export const wireDigestOfV1=(bytes:Uint8Array):WireDigestV1=>({hash:createHash("sha256").update(bytes).digest("hex"),size:bytes.length});
const digestMessage=(digest:WireDigestV1)=>concat([text(1,digest.hash),digest.size===0?new Uint8Array(0):integer(2,digest.size)]);

/** ✍️ Encodes Bazel's `ActionResult` for one unit: every output as an `OutputFile` (the manifest first), the streams as digests, and in `execution_metadata` the worker name and one `Any` in `auxiliary_metadata` that carries the slot id. */
export function encodeActionResultV1(input:Readonly<{files:readonly WireFileV1[];stdout?:WireDigestV1|undefined;stderr?:WireDigestV1|undefined;slot:string}>):Uint8Array{
 const parts:Uint8Array[]=[];
 for(const file of input.files)parts.push(delimited(2,concat([text(1,file.path),delimited(2,digestMessage(file.digest)),file.executable?flag(4,true):new Uint8Array(0)])));
 if(input.stdout)parts.push(delimited(6,digestMessage(input.stdout)));
 if(input.stderr)parts.push(delimited(8,digestMessage(input.stderr)));
 const any=concat([text(1,fleet.slotType),delimited(2,encoder.encode(input.slot))]);
 parts.push(delimited(9,concat([text(1,fleet.worker),delimited(11,any)])));
 return concat(parts);
}

type Field=Readonly<{number:number;wire:number;value:number;data:Uint8Array}>;
function* fields(bytes:Uint8Array):Generator<Field>{
 let offset=0;
 const read=():number=>{let result=0;for(let index=0;index<10;index++){const byte=bytes[offset+index];if(byte===undefined)throw Error("truncated varint");result+=(byte&0x7f)*2**(7*index);if((byte&0x80)===0){offset+=index+1;return result;}}throw Error("varint too long");};
 while(offset<bytes.length){
  const header=read(),number=Math.floor(header/8),wire=header%8;
  if(wire===0)yield {number,wire,value:read(),data:new Uint8Array(0)};
  else if(wire===2){const length=read();if(length>bytes.length-offset)throw Error("field exceeds the message");yield {number,wire,value:length,data:bytes.subarray(offset,offset+length)};offset+=length;}
  else if(wire===1||wire===5){const length=wire===1?8:4;if(offset+length>bytes.length)throw Error("truncated fixed field");offset+=length;}
  else throw Error("unsupported wire type");
 }
}
function digestOf(bytes:Uint8Array):WireDigestV1{
 let hash="",size=0;
 for(const field of fields(bytes)){if(field.number===1&&field.wire===2)hash=decoder.decode(field.data);else if(field.number===2&&field.wire===0)size=field.value;}
 if(!HEX.test(hash)||((size===0)!==(hash===EMPTY)))throw Error("digest invalid");
 return {hash,size};
}

/** 🔎️ Reads an `ActionResult` far enough to serve a unit; any malformed part throws, so the caller treats the entry as a miss. */
export function decodeActionResultV1(bytes:Uint8Array):WireResultV1{
 const files:WireFileV1[]=[];let stdout:WireDigestV1|undefined,stderr:WireDigestV1|undefined,slot:string|undefined,worker:string|undefined;
 for(const field of fields(bytes)){
  if(field.number===2&&field.wire===2){
   let path="",digest:WireDigestV1|undefined,executable=false;
   for(const part of fields(field.data)){if(part.number===1&&part.wire===2)path=decoder.decode(part.data);else if(part.number===2&&part.wire===2)digest=digestOf(part.data);else if(part.number===4&&part.wire===0)executable=part.value!==0;}
   if(path===""||digest===undefined)throw Error("output file invalid");
   files.push({path,digest,executable});
  }else if(field.number===6&&field.wire===2)stdout=digestOf(field.data);
  else if(field.number===8&&field.wire===2)stderr=digestOf(field.data);
  else if(field.number===9&&field.wire===2)for(const part of fields(field.data)){
   if(part.number===1&&part.wire===2)worker=decoder.decode(part.data);
   else if(part.number===11&&part.wire===2){
    let url="",value:Uint8Array|undefined;
    for(const any of fields(part.data)){if(any.number===1&&any.wire===2)url=decoder.decode(any.data);else if(any.number===2&&any.wire===2)value=any.data;}
    if(url===fleet.slotType&&value)slot=decoder.decode(value);
   }
  }
 }
 return {files,stdout,stderr,slot,worker};
}

/** 🪢️ The slot id an action result carries, `undefined` for a result without one or one that is not a unit's. */
export function readUnitSlotV1(bytes:Uint8Array):string|undefined{
 try{const slot=decodeActionResultV1(bytes).slot;return slot!==undefined&&HEX.test(slot)?slot:undefined;}catch{return undefined;}
}

/** 📜️ One unit as it travels: the portable form of a local cache entry. */
export type RemoteUnitV1={version:1;key:string;localKey:string;slot:string;crate:string;platform:PlatformV1;createdMs:number;costMs:number;dependencies:{path:string;sha256:string|null}[];environment:{name:string;value:string|null}[];outputs:{name:string;sha256:string;bytes:number;executable:boolean;template?:boolean}[];stdout:string|null;stderr:string|null;retention?:DeclarationV1};

const isDigest=(value:unknown):value is string=>typeof value==="string"&&HEX.test(value);
const isInteger=(value:unknown):value is number=>typeof value==="number"&&Number.isSafeInteger(value)&&value>=0;
const isObject=(value:unknown):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value);

/** 🛡️ Parses and strictly checks a manifest (`RemoteUnit` of the schema); throws on the first problem, so a broken entry is a miss. */
export function parseRemoteUnitV1(bytes:Uint8Array,expected:Readonly<{key:string}>):RemoteUnitV1{
 if(bytes.length>fleet.limits.maximumManifestBytes)throw Error("manifest too large");
 const value=JSON.parse(decoder.decode(bytes)) as unknown;
 if(!isObject(value)||value.version!==1||value.key!==expected.key||!isDigest(value.localKey)||!isDigest(value.slot)||typeof value.crate!=="string"||value.crate===""||!isObject(value.platform)||!isInteger(value.createdMs)||!isInteger(value.costMs))throw Error("manifest header invalid");
 if(!Array.isArray(value.dependencies)||!Array.isArray(value.environment)||!Array.isArray(value.outputs)||value.outputs.length<1||value.outputs.length>fleet.limits.maximumBlobs)throw Error("manifest lists invalid");
 for(const row of value.dependencies){if(!isObject(row)||typeof row.path!=="string"||row.path===""||!(row.sha256===null||isDigest(row.sha256)))throw Error("dependency invalid");}
 for(const row of value.environment){if(!isObject(row)||typeof row.name!=="string"||row.name===""||!(row.value===null||typeof row.value==="string"))throw Error("environment invalid");}
 const names=new Set<string>();
 for(const row of value.outputs){
  if(!isObject(row)||typeof row.name!=="string"||row.name===""||row.name.length>200||/[\\/]/u.test(row.name)||names.has(row.name)||!isDigest(row.sha256)||!isInteger(row.bytes)||row.bytes>fleet.limits.maximumBlobBytes||typeof row.executable!=="boolean"||!(row.template===undefined||typeof row.template==="boolean"))throw Error("output invalid");
  names.add(row.name);
 }
 if(!(value.stdout===null||isDigest(value.stdout))||!(value.stderr===null||isDigest(value.stderr)))throw Error("stream invalid");
 if(value.retention!==undefined){try{requireSlotShapeV1("Declaration",value.retention,"retention");}catch{throw Error("retention invalid");}}
 return value as unknown as RemoteUnitV1;
}

/** 🧾️ Everything one unit uploads: the manifest bytes, the blobs by digest (manifest, outputs, streams) and the encoded action result. */
export type UnitUploadV1=Readonly<{manifest:Uint8Array;blobs:ReadonlyMap<string,Uint8Array>;result:Uint8Array}>;

/** 📤️ Packs a unit for the wire from its manifest and the bytes of every blob the manifest names. */
export function packUnitV1(unit:RemoteUnitV1,contents:ReadonlyMap<string,Uint8Array>):UnitUploadV1{
 const manifest=encoder.encode(JSON.stringify(unit)),blobs=new Map<string,Uint8Array>(),manifestDigest=wireDigestOfV1(manifest);blobs.set(manifestDigest.hash,manifest);
 const files:WireFileV1[]=[{path:fleet.manifestName,digest:manifestDigest,executable:false}];
 for(const output of unit.outputs){
  const bytes=contents.get(output.sha256);if(bytes===undefined)throw Error("output bytes missing");
  blobs.set(output.sha256,bytes);files.push({path:"out/"+output.name,digest:{hash:output.sha256,size:bytes.length},executable:output.executable});
 }
 const stream=(sha:string|null):WireDigestV1|undefined=>{if(sha===null)return undefined;const bytes=contents.get(sha);if(bytes===undefined)throw Error("stream bytes missing");blobs.set(sha,bytes);return {hash:sha,size:bytes.length};};
 return {manifest,blobs,result:encodeActionResultV1({files,stdout:stream(unit.stdout),stderr:stream(unit.stderr),slot:unit.slot})};
}
