import {lstat} from "node:fs/promises";
import {dirname,isAbsolute,relative,resolve,sep} from "node:path";
import {cargoRepositoryPackages,type CargoDiscoveryOperation} from "../../../📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import {CargoController} from "../../../📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts";

/** 🏭️ Declares one artifact-owned inventory contribution. */
export type MutationInventoryProvider=Readonly<{script:string;roots:readonly string[]}>;
const declaredPath=(value:unknown):value is string=>typeof value==="string"&&value.length>0&&!isAbsolute(value)&&!/^[A-Za-z]:/u.test(value)&&!value.includes("\\");

/** 🧬️ Admits the closed contributed provider with caller-owned partial records. */
export async function admitMutationInventoryProvider(value:unknown,operation:CargoDiscoveryOperation):Promise<MutationInventoryProvider>{
 const control=new CargoController(operation,operation.workspace);if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Invalid mutation inventory provider");let fields=0;
 for(const key in value)if(Object.hasOwn(value,key)){await control.step("inventory-declaration-field",key,key.length+1);if(key!=="script"&&key!=="roots")throw Error("Invalid closed mutation inventory provider");fields++;}
 if(fields!==2||!("script"in value)||!("roots"in value)||!declaredPath(value.script)||!Array.isArray(value.roots)||!value.roots.length)throw Error("Invalid closed mutation inventory provider");
 await control.step("inventory-declaration-owner",value.script,value.script.length+1,128+value.script.length*2);const roots:string[]=[],seen=new Set<string>(),provider={script:value.script,roots};operation.workspace.partialRecords.push(provider,seen);
 for(const root of value.roots){if(!declaredPath(root))throw Error("Invalid mutation inventory provider root");await control.step("inventory-declaration-root",root,root.length+1,64+root.length*2);if(seen.has(root))throw Error("Repeated mutation inventory provider root");seen.add(root);roots.push(root);}
 await control.finish("inventory-declaration-admitted",provider.script);return provider;
}

/** 📁️ Checks every physical ancestor under the same admitted metadata operation. */
async function physical(root:string,path:string,optional:boolean,control:CargoController):Promise<Readonly<{present:boolean;kind:"file"|"directory"|"other";size:number}>>{
 await control.step("inventory-provider-path",path,path.length+1,32+path.length*2);const local=relative(root,path);if(isAbsolute(local)||local===".."||local.startsWith(".."+sep))throw Error("Mutation inventory provider escapes its repository");
 const parts=local.split(sep).filter(Boolean);await control.step("inventory-provider-ancestors",path,parts.length,32+local.length*2);let current=root;
 for(const part of parts){current=resolve(current,part);await control.step("inventory-provider-stat",current);try{const info=await lstat(current);if(info.isSymbolicLink())throw Error("Mutation inventory provider follows a symlink");}catch(error){if(optional&&error&&typeof error==="object"&&"code"in error&&error.code==="ENOENT")return{present:false,kind:"other",size:0};throw error;}}
 await control.step("inventory-provider-kind",path);const info=await lstat(path);return{present:true,kind:info.isFile()?"file":info.isDirectory()?"directory":"other",size:info.size};
}

/** 📇️ Resolves declarations from metadata retained by the original manifest read. */
export async function discoverMutationInventoryProviders(repoRoot:string,operation:CargoDiscoveryOperation):Promise<MutationInventoryProvider[]>{
 const control=new CargoController(operation,operation.workspace);await control.step("inventory-providers-owner",repoRoot,1,64);const providers:MutationInventoryProvider[]=[];operation.workspace.partialRecords.push(providers);
 for(const pkg of await cargoRepositoryPackages(repoRoot,operation)){
  await control.step("inventory-provider-metadata",pkg.manifest);const metadata=pkg.metadata;if(!metadata||typeof metadata!=="object"||Array.isArray(metadata)||!("semio"in metadata))continue;const semio=metadata.semio;if(!semio||typeof semio!=="object"||Array.isArray(semio)||!("mutation-inventory"in semio))continue;
  const provider=await admitMutationInventoryProvider(semio["mutation-inventory"],operation),owner=dirname(resolve(repoRoot,pkg.manifest));await control.step("inventory-provider-script-owner",pkg.manifest,provider.script.length+1,32+provider.script.length*2);const script=resolve(owner,provider.script),info=await physical(repoRoot,script,false,control);if(info.kind!=="file"||info.size>1048576)throw Error("Mutation inventory provider script is not bounded and regular");
  await control.step("inventory-provider-roots-owner",pkg.manifest,1,64);const roots:string[]=[];operation.workspace.partialRecords.push(roots);for(const path of provider.roots){await control.step("inventory-provider-root-owner",path,path.length+1,32+path.length*2);const full=resolve(owner,path);roots.push(full);const info=await physical(repoRoot,full,true,control);if(info.present&&info.kind!=="directory")throw Error("Mutation inventory scope is not a directory");}
  await control.step("inventory-provider-record",script,1,64);providers.push({script,roots});
 }
 await control.finish("inventory-providers-admitted",repoRoot);return providers;
}

/** 🎯️ Selects exactly one contributed owner without any ancestor-directory search. */
export async function selectMutationInventoryProvider(providers:readonly MutationInventoryProvider[],owner:string,operation:CargoDiscoveryOperation):Promise<MutationInventoryProvider|undefined>{
 const control=new CargoController(operation,operation.workspace);let selected:MutationInventoryProvider|undefined;
 for(const provider of providers){await control.step("inventory-provider-selection",provider.script,provider.script.length+1);let matches=false;for(const root of provider.roots){await control.step("inventory-provider-root-selection",root,root.length+owner.length+1,32+root.length*2+owner.length*2);const local=relative(root,owner);if(!isAbsolute(local)&&local!==".."&&!local.startsWith(".."+sep)){matches=true;break;}}if(matches){if(selected)throw Error("Mutation inventory owner has ambiguous providers");selected=provider;}}
 await control.finish("inventory-provider-selected",owner);return selected;
}
