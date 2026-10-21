import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ⚙️ The authored constants of the in-house engine. */
export const SEMIO_POLICY_V1=schema.definitions.Policy.const;
/** 🏷️ What happened to one task in a run. */
export type SemioStatusV1=typeof schema.definitions.Status.enum[number];
/** 🗒️ One task of a run record. */
export type SemioRecordTaskV1=Readonly<{id:string;status:SemioStatusV1;source?:"local"|"remote";key?:string;startedMs:number;endedMs:number;exitCode?:number;reason?:string}>;
/** 🧾️ What a run did: the exit code, whether the caller cancelled it, and every task. */
export type SemioRecordV1=Readonly<{version:1;exitCode:number;cancelled:boolean;tasks:readonly SemioRecordTaskV1[]}>;
/** 📣️ Progress of a run: a task started or ended, with the count of ended tasks. */
export type SemioEventV1=Readonly<{kind:"start"|"end";task:string;status?:SemioStatusV1;done:number;total:number}>;
/** 📦️ One stored file, directory or link of a task's outputs. */
export type SemioFileV1=Readonly<{path:string;kind:"file"|"directory"|"symlink";mode:number;sha256?:string;size?:number;link?:string}>;
/** 🗃️ The outputs of one action key as the cache stores them. */
export type SemioEntryV1=Readonly<{version:1;key:string;task:string;files:readonly SemioFileV1[];digest:string}>;

/** 🔏️ Hashes text or bytes with SHA-256. */
export const semioHashV1=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");

/** 🔤️ Serialises a value with sorted object keys, so equal values hash equally everywhere. */
export function semioCanonicalV1(value:unknown):string{
 if(Array.isArray(value))return "["+value.map(semioCanonicalV1).join(",")+"]";
 if(value&&typeof value==="object")return "{"+Object.keys(value).sort().filter(key=>(value as Record<string,unknown>)[key]!==undefined).map(key=>JSON.stringify(key)+":"+semioCanonicalV1((value as Record<string,unknown>)[key])).join(",")+"}";
 return JSON.stringify(value)??"null";
}

/** 🔑️ The digest of a set of outputs: path, kind, executable bit and content, never timestamps or full modes. */
export function semioOutputsDigestV1(files:readonly SemioFileV1[]):string{
 return semioHashV1(semioCanonicalV1([...files].sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0).map(file=>({path:file.path,kind:file.kind,x:(file.mode&0o111)!==0,sha:file.sha256,link:file.link}))));
}

/** 🔐️ The action key of a task: its identity, resolved execution, forwarded arguments, declared outputs, input contents, environment inputs and the output digests of its dependencies. */
export function semioActionKeyV1(parts:Readonly<{task:string;execution:unknown;args:readonly string[];outputs:unknown;files:readonly (readonly [string,string])[];environment:readonly (readonly [string,string])[];dependencies:readonly (readonly [string,string])[]}>):string{
 const rows=(values:readonly (readonly [string,string])[])=>[...values].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:left[1]<right[1]?-1:left[1]>right[1]?1:0);
 return semioHashV1(semioCanonicalV1({v:1,...parts,files:rows(parts.files),environment:rows(parts.environment),dependencies:rows(parts.dependencies)}));
}

/** 🔎️ The first index in a sorted list whose value is not below the prefix, so prefix ranges cost a binary search. */
export function semioLowerBoundV1(sorted:readonly string[],value:string):number{
 let low=0,high=sorted.length;
 while(low<high){const middle=(low+high)>>1;if(sorted[middle]!<value)low=middle+1;else high=middle;}
 return low;
}

/** 🌳️ The literal directory prefix of a glob pattern, which bounds the files that can match it. */
export function semioPatternPrefixV1(pattern:string):string{
 const at=pattern.search(/[?*{[(]/u);
 if(at<0)return pattern;
 const head=pattern.slice(0,at);
 return head.slice(0,head.lastIndexOf("/")+1);
}

/** 📚️ Selects the inventory files that match include patterns and none of the exclude patterns. */
export function semioSelectFilesV1(inventory:readonly string[],includes:readonly string[],excludes:readonly string[]):string[]{
 const chosen=new Set<string>(),excluded=excludes.map(pattern=>new Bun.Glob(pattern));
 for(const pattern of includes){
  const glob=/[?*{[(]/u.test(pattern)?new Bun.Glob(pattern):undefined,prefix=semioPatternPrefixV1(pattern);
  if(!glob){if(inventory[semioLowerBoundV1(inventory,pattern)]===pattern)chosen.add(pattern);continue;}
  for(let at=semioLowerBoundV1(inventory,prefix);at<inventory.length&&inventory[at]!.startsWith(prefix);at++)if(glob.match(inventory[at]!))chosen.add(inventory[at]!);
 }
 return [...chosen].filter(path=>!excluded.some(glob=>glob.match(path))).sort();
}
