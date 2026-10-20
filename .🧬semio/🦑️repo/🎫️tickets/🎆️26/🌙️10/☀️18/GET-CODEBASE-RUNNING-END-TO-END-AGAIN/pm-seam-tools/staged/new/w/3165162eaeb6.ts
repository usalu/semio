import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

/** 📦️ Reads the v2/v3 package-lock: the empty key is the root, keys outside `node_modules` are workspace members, `link` rows are aliases. */
export const readPackageLockV1:LockReaderV1=text=>{
 const lock=JSON.parse(text) as {name?:string;lockfileVersion?:number;packages?:Record<string,{name?:string;version?:string;resolved?:string;integrity?:string;link?:boolean}>};
 if(typeof lock.lockfileVersion!=="number"||lock.lockfileVersion<2||!lock.packages)throw Error("Supported package-lock required");
 const members:LockMemberV1[]=[],packages:LockPackageV1[]=[];
 for(const [key,row] of Object.entries(lock.packages)){
  if(key===""){members.push({name:row.name??lock.name??"",path:"."});continue;}
  if(row.link===true)continue;
  const marker=key.lastIndexOf("node_modules/");
  if(marker<0){members.push({name:row.name??key.slice(key.lastIndexOf("/")+1),path:key});continue;}
  packages.push({name:row.name??key.slice(marker+13),version:row.version??"",key,source:row.resolved??null,integrity:row.integrity??null});
 }
 return {packages,members};
};
/** 📦️ The adapter: descriptor data plus the lock reader. */
export const npmPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readPackageLockV1);
