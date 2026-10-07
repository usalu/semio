export type CargoManifestSelection=Readonly<{manifest:string;name:string}>;

/** 🎯️ Admits one exact canonical package from its declared manifest and nearest physical workspace. */
export async function cargoPackageForManifest(root:string,selection:CargoManifestSelection,operation:CargoDiscoveryOperation):Promise<CargoWorkspacePackage>{
 await checkpoint(operation,"manifest-selection");
 if(!selection||typeof selection!=="object"||Array.isArray(selection)||!Object.hasOwn(selection,"manifest")||!Object.hasOwn(selection,"name"))throw Error("Cargo manifest selection requires an owned object");
 for(const key in selection){await checkpoint(operation,"manifest-selection");if(Object.hasOwn(selection,key)&&key!=="manifest"&&key!=="name")throw Error("Cargo manifest selection has an unknown field");}
 const {manifest,name}=selection;
 if(typeof manifest!=="string"||!manifest||manifest.length>4096||manifest.includes("\\")||manifest.includes("\0")||isAbsolute(manifest)||manifest.split("/").some(part=>!part||part==="."||part==="..")||manifest.split("/").at(-1)!=="Cargo.toml"||typeof name!=="string"||!name||name.length>4096||name.includes("\0"))throw Error("Cargo manifest selection is invalid");
 await checkpoint(operation,manifest,manifest.length+name.length+1);
 const document=await cargoManifestDocument(root,manifest,operation),packageRow=table(document.package);
 if(packageRow.name!==name)throw Error("Cargo manifest canonical identity does not match the selector");
 const selected=await cargoWorkspaceForManifest(root,manifest,operation),localManifest=slash(relative(resolve(root,selected.directory),resolve(root,manifest))),localDirectory=slash(dirname(localManifest));
 if(localManifest.startsWith("../")||isAbsolute(localManifest)||!await matches(selected.memberManifests,localManifest,operation)||!await matches(selected.contribution.members,localDirectory,operation))throw Error("Cargo manifest is outside its declared member authority");
 if(await matches(selected.contribution.exclude,localManifest,operation))throw Error("Cargo manifest is excluded by its declared owner");
 for(let directory=localDirectory;;directory=slash(dirname(directory))){await checkpoint(operation,directory);if(await matches(selected.contribution.exclude,directory,operation)||await matches(selected.contribution.exclude,directory+"/",operation))throw Error("Cargo manifest is excluded by its declared owner");for(const pattern of selected.contribution.exclude)if(pattern.endsWith("/**")&&await cargoPatternMatches(pattern.slice(0,-3),directory,operation))throw Error("Cargo manifest is excluded by its declared owner");if(directory===".")break;}
 return retain(operation,manifest,128+manifest.length*2+name.length*2,()=>({directory:slash(dirname(manifest)),manifest,name,workspace:selected.directory,metadata:packageRow.metadata??null}));
}
