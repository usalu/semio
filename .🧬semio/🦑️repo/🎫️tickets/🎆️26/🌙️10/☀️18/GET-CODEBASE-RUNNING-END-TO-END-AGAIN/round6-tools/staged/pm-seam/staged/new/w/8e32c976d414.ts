import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

/** 🐹️ Reads the checksum list: a module version is a package, its zip line wins over its go.mod line. */
export const readGoSumV1:LockReaderV1=text=>{
 const rows=new Map<string,LockPackageV1>();
 for(const line of text.split(/\r?\n/u)){
  if(line.trim()==="")continue;
  const [module,version,hash,...rest]=line.trim().split(/\s+/u);
  if(!module||!version||!hash||rest.length||!hash.startsWith("h1:"))throw Error("Closed checksum line required: "+line);
  const modOnly=version.endsWith("/go.mod"),pure=modOnly?version.slice(0,-7):version,key=module+"@"+pure,prior=rows.get(key);
  if(modOnly&&prior)continue;
  rows.set(key,{name:module,version:pure,key,source:modOnly?"go.mod":"zip",integrity:hash});
 }
 return {packages:[...rows.values()],members:[]};
};
/** 🐹️ The adapter: descriptor data plus the lock reader. */
export const goPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readGoSumV1);
