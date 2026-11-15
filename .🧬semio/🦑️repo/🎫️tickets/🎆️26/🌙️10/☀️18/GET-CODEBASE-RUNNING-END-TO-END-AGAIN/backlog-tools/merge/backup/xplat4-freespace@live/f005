import {existsSync,lstatSync,statSync,statfsSync} from "node:fs";
import {dirname,join} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import type {PartitionFactsV1} from "../../🟦️.ts";
import {readJsonFileV1,type DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";

const BACKING="fixed.dmg",MOUNT="m",PUBLICATION=".publication";
const entry=(path:string)=>{try{return lstatSync(path);}catch{return undefined;}};

/** 💾️ Reads one partition from its receipt, its backing file, its mount and its publication residue, without opening it. */
function partition(ports:DoctorPortsV1,id:string,directory:string):PartitionFactsV1{
 const outer=entry(directory);
 if(!outer)return {id,state:"absent",capacityBytes:null,freeBytes:null,imageBytes:null,backing:"none",publication:"none",publicationAgeSeconds:null,image:"none"};
 const receiptPath=join(directory,"image.json"),receiptEntry=entry(receiptPath),receipt=receiptEntry?readJsonFileV1(receiptPath,65536) as {version?:unknown}|undefined:undefined;
 const image:PartitionFactsV1["image"]=receiptEntry===undefined?"none":receipt&&typeof receipt==="object"&&receipt.version===1?"ok":"damaged";
 const mount=join(directory,MOUNT),inner=entry(mount),directoryHost=ports.platform!=="darwin",backingPath=join(directory,BACKING),backingEntry=entry(backingPath);
 let state:PartitionFactsV1["state"],capacityBytes:number|null=null,freeBytes:number|null=null;
 if(directoryHost)state=inner?.isDirectory()?"directory":"absent";
 else if(inner?.isDirectory()&&inner.dev!==outer.dev){
  state="mounted";
  try{const row=statfsSync(mount);capacityBytes=row.blocks*row.bsize;freeBytes=row.bavail*row.bsize;}catch{state="detached";}
 }else state=backingEntry?"detached":"absent";
 const residue=["intent","receipt"].find(name=>existsSync(join(directory,PUBLICATION,name+".json"))) as "intent"|"receipt"|undefined;
 let age:number|null=null;
 if(residue)try{age=Math.max(0,Math.floor((ports.now()-statSync(join(directory,PUBLICATION,residue+".json")).mtimeMs)/1000));}catch{age=0;}
 return {id,state,capacityBytes,freeBytes,imageBytes:backingEntry?backingEntry.size:null,backing:directoryHost?"none":backingEntry?"present":"missing",publication:residue??"none",publicationAgeSeconds:age,image};
}

/** 💽️ The disk-level facts: image state, free space against budget, unsettled publications and the free space of the disk that holds the caches. */
export const storageProbeV1:DoctorProbeV1=Object.freeze({
 area:"storage",phase:1,
 async read(ports){
  const rows=Object.entries(budget.partitions).map(([id,row])=>partition(ports,id,join(ports.root,...row.directory.split("/"))));
  let host=join(ports.root,".cache");while(!existsSync(host)&&dirname(host)!==host)host=dirname(host);
  let hostFreeBytes:number|null=null,hostTotalBytes:number|null=null;
  try{const row=statfsSync(host);hostFreeBytes=row.bavail*row.bsize;hostTotalBytes=row.blocks*row.bsize;}catch{}
  return {sections:{storage:{hostFreeBytes,hostTotalBytes,partitions:rows}}};
 },
});
