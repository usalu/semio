import {parseTomlV1} from "../📖️formats/🟦️.ts";
import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

const localV1=["editable","virtual","directory","path"];

/** 🐍️ Reads the TOML lock: local sources are workspace members, everything else a package. */
export const readUvLockV1:LockReaderV1=text=>{
 const rows=(parseTomlV1(text).package??[]) as {name?:unknown;version?:unknown;source?:Record<string,string>;sdist?:{hash?:string};wheels?:{hash?:string}[]}[],members:LockMemberV1[]=[],packages:LockPackageV1[]=[];
 for(const row of rows){
  if(typeof row.name!=="string")throw Error("Named lock package required");
  const kind=Object.keys(row.source??{})[0];
  if(kind===undefined)throw Error("Sourced lock package required: "+row.name);
  if(localV1.includes(kind)){members.push({name:row.name,path:row.source![kind]!});continue;}
  const version=typeof row.version==="string"?row.version:"";
  packages.push({name:row.name,version,key:row.name+"@"+version,source:kind+":"+row.source![kind],integrity:row.sdist?.hash??row.wheels?.[0]?.hash??null});
 }
 return {packages,members};
};
/** 🐍️ The adapter: descriptor data plus the lock reader. */
export const uvPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readUvLockV1);
