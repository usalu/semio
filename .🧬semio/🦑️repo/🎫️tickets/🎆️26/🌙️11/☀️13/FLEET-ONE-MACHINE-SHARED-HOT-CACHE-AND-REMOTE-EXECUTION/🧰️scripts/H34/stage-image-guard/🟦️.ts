import{readdirSync,statSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const MB=1048576;
/** 🖼️ The free-space rule of the per-purpose cache images: the guard floor and the start need as shares of the image (clamped), the partitions each stage writes into, the bilingual name of each purpose and how soon a full image is looked at again.
 * @see ./🧬️schema/🔣️.json */
export const READY_IMAGE_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
/** 📋️ What one mounted image says: its partition id, its mount path, its size and its free bytes. */
export type ReadyImageFactV1=Readonly<{partition:string;path:string;freeBytes:number;totalBytes:number;hostFreeBytes?:number|null}>;
/** 🚫️ The image that is short before a stage starts: the same row shape as a short disk (megabytes) with the partition and its size. */
export type ReadyImageShortV1=Readonly<{partition:string;path:string;resource:"space";needed:number;available:number;total:number;host:number|null;limitedByHost:boolean}>;
/** 🛑️ The image that crossed the guard floor while a stage runs (megabytes). */
export type ReadyImageGuardV1=Readonly<{partition:string;path:string;resource:"space";available:number;floor:number}>;
/** 🔎️ What a filesystem says about a path: free and total bytes, `null` when it cannot be measured. */
export type ReadyImageFactsPortV1=(path:string)=>Readonly<{freeBytes:number|null;totalBytes?:number|null}>|null;

const clamp=(value:number,minimum:number,maximum:number):number=>Math.max(minimum,Math.min(maximum,value));

/** 🛑️ The free bytes below which a running stage is stopped: `floorBasisPoints` of the image clamped to the declared minimum and maximum, never more than `floorShareBasisPoints` of the image (a small image keeps a floor it can meet). Pure and language-agnostic. */
export function readyImageFloorBytesV1(totalBytes:number,policy=READY_IMAGE_POLICY_V1):number{
 const share=Math.floor(totalBytes*policy.floorShareBasisPoints/10000);
 return Math.min(share,clamp(Math.floor(totalBytes*policy.floorBasisPoints/10000),policy.floorMinimumMegabytes*MB,policy.floorMaximumMegabytes*MB));
}
/** 📐️ The free bytes a stage needs in an image before it starts: `needBasisPoints` of the image clamped to the declared minimum and maximum, never below the guard floor, so a started stage is never stopped at once. Pure and language-agnostic. */
export function readyImageNeedBytesV1(totalBytes:number,policy=READY_IMAGE_POLICY_V1):number{
 return Math.max(readyImageFloorBytesV1(totalBytes,policy),clamp(Math.floor(totalBytes*policy.needBasisPoints/10000),policy.needMinimumMegabytes*MB,policy.needMaximumMegabytes*MB));
}

/** 🗂️ The images a stage writes into, as the mounts below `.cache/b`: only an own volume (a folder that is no mount is left out), only partitions the policy names for the stage, never an excluded control partition, never an image smaller than `minimumTotalMegabytes`, a volume that cannot be measured is skipped (unknown is not full). */
export function readyImageFactsV1(workspace:string,stage:string,facts:ReadyImageFactsPortV1,policy=READY_IMAGE_POLICY_V1,ownVolume:(path:string,base:string)=>boolean=(path,base)=>statSync(path).dev!==statSync(base).dev):readonly ReadyImageFactV1[]{
 const wanted=(policy.stages as Readonly<Record<string,readonly string[]>>)[stage]??[],base=join(workspace,...policy.directory),rows:ReadyImageFactV1[]=[];
 let hostFreeBytes:number|null=null;
 try{const host=facts(join(workspace,policy.directory[0]!));hostFreeBytes=typeof host?.freeBytes==="number"?host.freeBytes:null;}catch{}
 let names:string[];try{names=readdirSync(base).sort();}catch{return[];}
 for(const name of names){
  if(!wanted.includes(name)||policy.excluded.includes(name))continue;
  const path=join(base,name,policy.mount);
  try{if(!ownVolume(path,base))continue;}catch{continue;}
  const row=facts(path);
  if(row===null||typeof row.freeBytes!=="number"||typeof row.totalBytes!=="number"||row.totalBytes<policy.minimumTotalMegabytes*MB)continue;
  rows.push(Object.freeze({partition:name,path,freeBytes:row.freeBytes,totalBytes:row.totalBytes,hostFreeBytes}));
 }
 return Object.freeze(rows);
}

/** 🌡️ Whether the free bytes of an image are those of the host volume: a thin image reports what its sparse backing file can still grow by, which is the free space of the host, so an image that shows at least the free bytes of the host (less a small tolerance) is only as roomy as the host. Pure and language-agnostic. */
export function readyImageHostLimitedV1(image:Pick<ReadyImageFactV1,"freeBytes"|"hostFreeBytes">,policy=READY_IMAGE_POLICY_V1):boolean{
 const host=image.hostFreeBytes;
 return typeof host==="number"&&image.freeBytes>=host-Math.max(policy.hostLimitedBytes,Math.floor(host*policy.hostLimitedBasisPoints/10000));
}

/** 🚫️ The image that lacks the most of its need before a stage starts (the smallest share of its need free), `null` when every image holds its need. Pure and language-agnostic. */
export function readyImageShortV1(images:readonly ReadyImageFactV1[],policy=READY_IMAGE_POLICY_V1):ReadyImageShortV1|null{
 let worst:{image:ReadyImageFactV1;need:number}|null=null;
 for(const image of images){
  const need=readyImageNeedBytesV1(image.totalBytes,policy);
  if(image.freeBytes>=need)continue;
  if(worst===null||image.freeBytes/need<worst.image.freeBytes/worst.need)worst={image,need};
 }
 return worst===null?null:Object.freeze({partition:worst.image.partition,path:worst.image.path,resource:"space" as const,needed:Math.ceil(worst.need/MB),available:Math.floor(worst.image.freeBytes/MB),total:Math.floor(worst.image.totalBytes/MB),host:typeof worst.image.hostFreeBytes==="number"?Math.floor(worst.image.hostFreeBytes/MB):null,limitedByHost:readyImageHostLimitedV1(worst.image,policy)});
}
/** 🛑️ The image whose free bytes fell below its guard floor while a stage runs, `null` when none did. Pure and language-agnostic. */
export function readyImageGuardV1(images:readonly ReadyImageFactV1[],policy=READY_IMAGE_POLICY_V1):ReadyImageGuardV1|null{
 for(const image of images){
  const floor=readyImageFloorBytesV1(image.totalBytes,policy);
  if(image.freeBytes<floor)return Object.freeze({partition:image.partition,path:image.path,resource:"space" as const,available:Math.floor(image.freeBytes/MB),floor:Math.ceil(floor/MB)});
 }
 return null;
}

/** 🔍️ The partition a failed write names: the last line that says no space was left, or the line before it (Cargo says `failed to write <path>` and then `Caused by: No space left`), is searched for a path below `.cache/b/<partition>/m`; `null` when no such path is named. Pure and language-agnostic. */
export function readyImagePartitionOfLinesV1(lines:readonly string[],noSpace:readonly string[],policy=READY_IMAGE_POLICY_V1):string|null{
 const own=new RegExp(`[\\\\/]${policy.directory.map(part=>part.replaceAll(".","\\.")).join("[\\\\/]")}[\\\\/]([A-Za-z0-9]+)[\\\\/]${policy.mount}(?=[\\\\/\`'"\\s:]|$)`,"u");
 for(let index=lines.length-1;index>=0;index--){
  if(!noSpace.some(pattern=>lines[index]!.includes(pattern)))continue;
  for(const at of[index,index-1,index-2]){const found=at<0?null:own.exec(lines[at]!);if(found!==null&&!policy.excluded.includes(found[1]!))return found[1]!;}
 }
 return null;
}

/** 🏷️ The bilingual name of the purpose of a partition (the partition id for one the policy does not know). */
export function readyImagePurposeV1(partition:string,policy=READY_IMAGE_POLICY_V1):Readonly<{en:string;de:string}>{
 return(policy.purposes as Readonly<Record<string,{en:string;de:string}>>)[partition]??{en:partition,de:partition};
}

/** 🔁️ Whether a full image is looked at again: `recheckMilliseconds` passed or it gained `recheckGrowthMegabytes` since it was said. Pure and language-agnostic. */
export function readyImageRecheckDueV1(said:Readonly<{atMs:number;availableMegabytes:number}>,availableMegabytes:number,nowMs:number,policy=READY_IMAGE_POLICY_V1):boolean{
 return nowMs-said.atMs>=policy.recheckMilliseconds||availableMegabytes-said.availableMegabytes>=policy.recheckGrowthMegabytes;
}

/** 🧹️ The bytes to free inside a full image: what it lacks to its need, never less than `retainMinimumMegabytes` so that one collection makes room for more than the next write. Pure and language-agnostic. */
export function readyImageRetainBytesV1(image:Pick<ReadyImageFactV1,"freeBytes"|"totalBytes">,policy=READY_IMAGE_POLICY_V1):number{
 return Math.max(policy.retainMinimumMegabytes*MB,readyImageNeedBytesV1(image.totalBytes,policy)-image.freeBytes);
}

/** 🔌️ Keeps the collectors that work inside one image: a writer whose root lies below the mount is collected for that image, and its relief to the image counts as relief for the volume asked about (the volume IS the image). Pure. */
export function readyImageWritersV1<W extends Readonly<{root:string;row:Readonly<{relief:string}>}>>(writers:readonly W[],mount:string,separator:string="/"):readonly W[]{
 const inside=(root:string)=>root===mount||root.startsWith(mount.endsWith(separator)?mount:mount+separator);
 return writers.filter(writer=>inside(writer.root)).map(writer=>({...writer,row:{...writer.row,relief:"host"}}));
}
