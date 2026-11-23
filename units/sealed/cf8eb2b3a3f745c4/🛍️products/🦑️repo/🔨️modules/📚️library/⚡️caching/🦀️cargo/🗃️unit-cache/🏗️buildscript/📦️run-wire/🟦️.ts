import {createHash} from "node:crypto";
import {FLEET_POLICY_V1,localTextV1,portableTextV1,type PlatformV1,type PortableRootV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import type {RemoteUnitV1} from "../../🛰️fleet/📦️wire/🟦️.ts";
import type {BuildEntryV1} from "../../💾️store/🟦️.ts";

const fleet=FLEET_POLICY_V1,limits=fleet.limits,encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true}),HEX=/^[a-f0-9]{64}$/u;

/** 🧾️ The output of a run's manifest that carries its facts (the files and environment that make Cargo rerun the script, the package name). */
export const RUN_FACTS_NAME_V1="run.json";
const FILE_PREFIX="f:";

/** 🏷️ The manifest name of one OUT_DIR file: a manifest name holds no slash, so `/` is written `%2F` and `%` is written `%25`. */
export const runFileNameV1=(path:string):string=>FILE_PREFIX+path.replaceAll("%","%25").replaceAll("/","%2F");

/** 🏷️ The OUT_DIR path of a manifest name made by {@link runFileNameV1}. */
export const runFilePathV1=(name:string):string=>name.slice(FILE_PREFIX.length).replaceAll("%2F","/").replaceAll("%25","%");

const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");
const textOf=(bytes:Uint8Array):string|undefined=>{if(bytes.includes(0))return undefined;try{return decoder.decode(bytes);}catch{return undefined;}};

/** 🚧️ The real paths of a machine that no portable text may still hold: every spelling of every root, and the extra directories given (the home directory, the lane). */
export function runLeakRootsV1(roots:readonly PortableRootV1[],extra:readonly (string|undefined)[]=[]):readonly string[]{
 return [...new Set([...roots.map(root=>root.path),...extra.filter((path):path is string=>typeof path==="string")])].filter(path=>path.length>=4);
}

/** 🚧️ The first of `leaks` that a blob still names (text after its roots became tokens, binary files as they are), `undefined` when none. */
export function runLeakV1(bytes:Uint8Array,leaks:readonly string[]):string|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.length);
 for(const leak of leaks)if(buffer.includes(leak))return leak;
 return undefined;
}

/** 🧾️ The facts document of a run: its package, the facts and the environment values that make Cargo rerun the script, with every root of the machine as its token. */
type RunFactsV1={version:1;package:string;facts:BuildEntryV1["facts"];environment:BuildEntryV1["environment"]};

/** 📤️ Packs the run of a build script for the wire: every OUT_DIR file and the two streams with the roots of the machine as tokens, the facts as `run.json`. A string is the reason the run cannot be shared (`path-dependent: <file>`, `name-too-long`, `too-many-files`, `too-large`, `unreadable`): a text or binary that still names a real directory of the machine after the roots became tokens, a path that does not fit a manifest name, more files than a manifest holds. */
export function packRunV1(entry:BuildEntryV1,header:Readonly<{key:string;localKey:string;slot:string;platform:PlatformV1;costMs:number}>,roots:readonly PortableRootV1[],read:(sha256:string)=>Uint8Array|undefined,extraLeaks:readonly (string|undefined)[]=[]):Readonly<{unit:RemoteUnitV1;contents:ReadonlyMap<string,Uint8Array>}>|string{
 const contents=new Map<string,Uint8Array>(),leaks=runLeakRootsV1(roots,extraLeaks);let total=0;
 const refuse=(reason:string):never=>{throw Error(reason);};
 const add=(bytes:Uint8Array,label:string):string=>{
  const leak=runLeakV1(bytes,leaks);if(leak!==undefined)refuse("path-dependent: "+label+" holds "+leak);
  total+=bytes.length;if(bytes.length>limits.maximumBlobBytes||total>limits.maximumEntryBytes)refuse("too-large");
  const hash=sha(bytes);contents.set(hash,bytes);return hash;
 };
 const portable=(bytes:Uint8Array):{bytes:Uint8Array;template:boolean}=>{const text=textOf(bytes);if(text===undefined)return{bytes,template:false};const converted=portableTextV1(text,roots);return{bytes:encoder.encode(converted),template:converted!==text};};
 try{
  if(entry.outputs.length+1>limits.maximumBlobs)refuse("too-many-files");
  const outputs:RemoteUnitV1["outputs"]=[];
  const facts:RunFactsV1={version:1,package:entry.package,facts:entry.facts.map(row=>({...row,path:portableTextV1(row.path,roots)})),environment:entry.environment.map(row=>({name:row.name,value:row.value===null?null:portableTextV1(row.value,roots)}))};
  const factsBytes=encoder.encode(JSON.stringify(facts)),factsHash=add(factsBytes,RUN_FACTS_NAME_V1);
  outputs.push({name:RUN_FACTS_NAME_V1,sha256:factsHash,bytes:factsBytes.length,executable:false});
  for(const row of entry.outputs){
   const name=runFileNameV1(row.path);if(name.length>200)refuse("name-too-long");
   const stored=read(row.sha256);if(stored===undefined)refuse("unreadable");
   const converted=portable(stored!),hash=add(converted.bytes,row.path);
   outputs.push({name,sha256:hash,bytes:converted.bytes.length,executable:row.executable,...(row.template||converted.template?{template:true}:{})});
  }
  const stream=(value:string|null,label:string):string|null=>{
   if(value===null)return null;
   const stored=read(value);if(stored===undefined)return refuse("unreadable");
   return add(portable(stored).bytes,label);
  };
  const stdout=stream(entry.stdout,"stdout"),stderr=stream(entry.stderr,"stderr");
  return Object.freeze({unit:{version:1,key:header.key,localKey:header.localKey,slot:header.slot,crate:entry.package,platform:header.platform,createdMs:entry.createdMs,costMs:header.costMs,dependencies:[],environment:[],outputs,stdout,stderr,...(entry.retention===undefined?{}:{retention:entry.retention})},contents});
 }catch(error){return String((error as Error).message);}
}

/** 📥️ Turns a manifest and its verified blobs into the run the local store records: tokens become this machine's roots again, the run is keyed by `localKey`. Throws on a manifest that is no run (no `run.json`, a facts document of another shape). `put` stores a blob in the local object store and answers its digest. */
export function unpackRunV1(input:Readonly<{localKey:string;roots:readonly PortableRootV1[];manifest:RemoteUnitV1;blobs:ReadonlyMap<string,Uint8Array>;put:(bytes:Uint8Array)=>string}>):BuildEntryV1{
 const {manifest,blobs,roots}=input,row=manifest.outputs.find(output=>output.name===RUN_FACTS_NAME_V1);
 if(row===undefined)throw Error("run facts missing");
 const document=JSON.parse(decoder.decode(blobs.get(row.sha256)!)) as RunFactsV1;
 if(document.version!==1||typeof document.package!=="string"||!Array.isArray(document.facts)||!Array.isArray(document.environment))throw Error("run facts invalid");
 for(const fact of document.facts)if(typeof fact.path!=="string"||!["file","tree","package"].includes(fact.kind)||!HEX.test(fact.sha256))throw Error("run fact invalid");
 for(const variable of document.environment)if(typeof variable.name!=="string"||!(variable.value===null||typeof variable.value==="string"))throw Error("run environment invalid");
 const local=(text:string)=>localTextV1(text,roots);
 const outputs=manifest.outputs.filter(output=>output.name.startsWith(FILE_PREFIX)).map(output=>{
  const blob=blobs.get(output.sha256)!,stored=output.template===true?encoder.encode(local(decoder.decode(blob))):blob;
  return{path:runFilePathV1(output.name),sha256:input.put(stored),bytes:stored.length,executable:output.executable,template:output.template===true};
 });
 const stream=(digest:string|null):string|null=>digest===null?null:input.put(encoder.encode(local(decoder.decode(blobs.get(digest)!))));
 return{version:1,key:input.localKey,...(manifest.retention===undefined?{}:{retention:manifest.retention}),package:document.package,facts:document.facts.map(fact=>({...fact,path:local(fact.path)})),environment:document.environment.map(variable=>({name:variable.name,value:variable.value===null?null:local(variable.value)})),outputs,stdout:stream(manifest.stdout),stderr:stream(manifest.stderr),createdMs:Date.now()};
}
