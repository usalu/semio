import {writeEntryV1,type UnitEntryV1} from "../../💾️store/🟦️.ts";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {putCasObjectSyncV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import type {FleetClientV1} from "../📡️client/🟦️.ts";
import {FLEET_POLICY_V1,localTextV1,portableRootsV1,type PlatformV1,type PortableRootV1} from "../🪪️identity/🟦️.ts";
import {decodeActionResultV1,parseRemoteUnitV1} from "../📦️wire/🟦️.ts";
import type {SpoolJobV1} from "../📤️uploader/🟦️.ts";
import {uncoveredOutputsV1} from "../../✅️complete/🟦️.ts";

const fleet=FLEET_POLICY_V1,decoder=new TextDecoder("utf-8",{fatal:true}),encoder=new TextEncoder();

/** 📥️ What installing a fleet unit into the local store needs of a unit. */
export type InstallTargetV1=Readonly<{client:FleetClientV1;root:string;localKey:string;key:string;slot:string;platform:PlatformV1;roots:readonly PortableRootV1[];expected?:readonly string[]}>;

/** ⚡️ Downloads every blob at once, the largest first so the longest transfer overlaps the others, `fleet.limits.maximumDownloadConcurrency` at a time; `undefined` as soon as one is missing, short or corrupt. The digests come from the action result, so the manifest travels with the outputs and a unit costs one round trip for its result and one for its blobs. */
export async function downloadBlobsV1(client:FleetClientV1,wanted:ReadonlyMap<string,number>):Promise<Map<string,Uint8Array>|undefined>{
 const downloads=[...wanted.entries()].sort((left,right)=>right[1]-left[1]),bytes=new Map<string,Uint8Array>();let next=0,failed=false;
 await Promise.all(Array.from({length:Math.min(fleet.limits.maximumDownloadConcurrency,downloads.length)},async()=>{
  for(;;){const index=next++;if(index>=downloads.length||failed)return;const [sha,size]=downloads[index]!,blob=await client.getBlob(sha,size);if(!blob){failed=true;return;}bytes.set(sha,blob);}
 }));
 return failed?undefined:bytes;
}

/** 📥️ Fetches the unit from the fleet cache and installs it into the local store, as if it had been compiled here: the manifest and every blob are downloaded and verified against their digests (a blob that is missing, short or corrupt, or an output that the unit's command line promises and the manifest lacks, makes the whole entry a miss), paths are put back to this machine's roots and the entry is written as the newest variant of the local key. `hit` means the caller may now serve from the local store, whose second-level check still compares every recorded input file and environment value. */
export async function installFleetUnitV1(unit:InstallTargetV1):Promise<"hit"|"miss"|"fault">{
 try{
  const answer=await unit.client.getResult(unit.key);
  if(answer==="miss")return "miss";
  if(typeof answer==="string")return "fault";
  const result=decodeActionResultV1(answer);
  if(result.slot!==undefined&&result.slot!==unit.slot)return "miss";
  const manifestFile=result.files.find(file=>file.path===fleet.manifestName);if(!manifestFile)return "miss";
  const declared=new Map(result.files.map(file=>[file.path,file.digest])),wanted=new Map<string,number>();
  for(const digest of[manifestFile.digest,...result.files.filter(file=>file.path.startsWith("out/")).map(file=>file.digest),...[result.stdout,result.stderr].filter((stream):stream is NonNullable<typeof stream>=>stream!==undefined)])wanted.set(digest.hash,digest.size);
  if(manifestFile.digest.size>fleet.limits.maximumManifestBytes||wanted.size>fleet.limits.maximumBlobs+3||[...wanted.values()].some(size=>size>fleet.limits.maximumBlobBytes))return "miss";
  const bytes=await downloadBlobsV1(unit.client,wanted);if(bytes===undefined)return "miss";
  const manifest=parseRemoteUnitV1(bytes.get(manifestFile.digest.hash)!,{key:unit.key});
  if(JSON.stringify(manifest.platform)!==JSON.stringify(unit.platform)||manifest.slot!==unit.slot)return "miss";
  if(unit.expected&&uncoveredOutputsV1(manifest.outputs.map(output=>output.name),unit.expected).length>0)return "miss";
  for(const output of manifest.outputs){const file=declared.get("out/"+output.name);if(file?.hash!==output.sha256||file.size!==output.bytes)return "miss";}
  for(const [sha,stream] of [[manifest.stdout,result.stdout],[manifest.stderr,result.stderr]] as const){if(sha!==null&&stream?.hash!==sha)return "miss";}
  const root=unit.root,cas=casRootV1(),local=(sha:string,text:boolean):Readonly<{sha:string;bytes:number}>=>{
   const blob=bytes.get(sha)!,stored=text?encoder.encode(localTextV1(decoder.decode(blob),unit.roots)):blob;
   return {sha:putCasObjectSyncV1(cas,stored),bytes:stored.length};
  };
  const outputs=manifest.outputs.map(output=>{const stored=local(output.sha256,output.template===true);return {name:output.name,sha256:stored.sha,bytes:stored.bytes,executable:output.executable,...(output.template?{template:true}:{})};});
  const stream=(sha:string|null):string|null=>sha===null?null:local(sha,true).sha;
  const entry:UnitEntryV1={version:1,key:unit.localKey,slot:unit.slot,crate:manifest.crate,dependencies:manifest.dependencies.map(row=>({path:localTextV1(row.path,unit.roots),sha256:row.sha256})),environment:manifest.environment.map(row=>({name:row.name,value:row.value===null?null:localTextV1(row.value,unit.roots)})),outputs,stderr:stream(manifest.stderr),stdout:stream(manifest.stdout),createdMs:Date.now(),provenance:null};
  writeEntryV1(root,entry);
  return "hit";
 }catch{return "fault";}
}

/** 🤝️ The fleet and this machine disagree about the bytes of one key (a second upload was refused): the fleet's copy is authoritative, because every other machine already keys its dependants on it. Installs it as the newest local variant of the unit, so a later local hit restores the fleet's bytes and no dependant here is keyed on a copy nobody else has. `true` when the fleet's copy is now the local one. */
export async function adoptFleetUnitV1(client:FleetClientV1,root:string,job:SpoolJobV1):Promise<boolean>{
 try{return await installFleetUnitV1({client,root,localKey:job.localKey,key:job.key,slot:job.slot,platform:job.platform,roots:portableRootsV1({cwd:job.cwd,workspace:job.workspace??undefined})})==="hit";}catch{return false;}
}
