import {parseTomlV1} from "../📖️formats/🟦️.ts";
import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

/** 🦀️ Reads the TOML lock: rows without a source are workspace crates. */
export const readCargoLockV1:LockReaderV1=text=>{
 const rows=(parseTomlV1(text).package??[]) as {name?:unknown;version?:unknown;source?:unknown;checksum?:unknown}[],members:LockMemberV1[]=[],packages:LockPackageV1[]=[];
 for(const row of rows){
  if(typeof row.name!=="string"||typeof row.version!=="string")throw Error("Named versioned lock package required");
  if(row.source===undefined){members.push({name:row.name,path:null});continue;}
  packages.push({name:row.name,version:row.version,key:row.name+"@"+row.version,source:String(row.source),integrity:typeof row.checksum==="string"?row.checksum:null});
 }
 return {packages,members};
};
/** 🦀️ The adapter: descriptor data plus the lock reader. */
export const cargoPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readCargoLockV1);
