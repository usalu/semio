import {parseJsoncV1} from "../📖️formats/🟦️.ts";
import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockProjectionV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

const linkedV1=/^(workspace|file|link|portal):/u;

/** 🍞️ Reads the JSONC lock of this manager: members come from the workspaces table, packages from the packages table. */
export const readBunLockV1:LockReaderV1=text=>{
 const lock=parseJsoncV1(text) as {lockfileVersion?:number;workspaces?:Record<string,{name?:string}>;packages?:Record<string,unknown[]>};
 if(![1,2,3].includes(lock.lockfileVersion as number))throw Error("Supported lock version required: "+String(lock.lockfileVersion));
 const members:LockMemberV1[]=Object.entries(lock.workspaces??{}).map(([path,row])=>({name:row.name??path,path:path===""?".":path})),packages:LockPackageV1[]=[];
 for(const [key,row] of Object.entries(lock.packages??{})){
  const spec=row[0];if(typeof spec!=="string")throw Error("Named lock package required: "+key);
  const at=spec.lastIndexOf("@");if(at<=0)throw Error("Versioned lock package required: "+key);
  const version=spec.slice(at+1);if(linkedV1.test(version))continue;
  packages.push({name:spec.slice(0,at),version,key,source:typeof row[1]==="string"&&row[1]!==""?row[1]:null,integrity:(row.find(item=>typeof item==="string"&&/^sha\d+-/u.test(item)) as string|undefined)??null});
 }
 return {packages,members};
};
/** 🔁️ Rewrites the workspace paths of the lock between the original tree and the staged generation, and the patch identities with them. */
export function projectBunLockV1(text:string,input:LockProjectionV1):string{
 const lock=parseJsoncV1(text) as any,links=input.reverse?new Map([...input.mapping].map(([source,target])=>[target,source])):input.mapping;
 if(lock.patchedDependencies)lock.patchedDependencies=Object.fromEntries(input.patches.map(row=>[row.name,input.reverse?row.reverse:row.forward]));
 lock.workspaces=Object.fromEntries(Object.entries(lock.workspaces??{}).filter(([path])=>input.reverse||path===""||links.has(path)).map(([path,value])=>[links.get(path)??path,value]));
 for(const [key,row] of Object.entries(lock.packages??{}) as [string,any][]){
  if(!Array.isArray(row)||typeof row[0]!=="string")continue;
  const marker=row[0].indexOf("@workspace:");if(marker<0)continue;
  const path=row[0].slice(marker+11),target=links.get(path);
  if(!target){if(input.reverse)throw Error("Locked workspace package lacks original admitted Source");delete lock.packages[key];continue;}
  row[0]=row[0].slice(0,marker+11)+target;
 }
 return JSON.stringify(lock,null,2)+"\n";
}
/** 🍞️ The adapter: descriptor data plus the lock reader and its capabilities. */
export const bunPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readBunLockV1,{document:parseJsoncV1,project:projectBunLockV1});
