import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;
/** 🧩️ One schema-declared storage backend: pure data, no behavior. */
export type NativeStorageBackendDeclarationV1=Readonly<{id:string;platforms:readonly string[];capabilities:readonly string[];backingFilename:string;dispatch:Readonly<Record<string,string>>;modules:readonly string[]}>;
/** 🎚️ Facts and policy that choose a backend; every field is optional and the host platform is only the default input. */
export type NativeStorageSelectionV1=Readonly<{platform?:string;backend?:string;require?:readonly string[];prefer?:readonly string[]}>;
/** 📜️ The closed list of declared backends in preference order. */
export const nativeStorageBackendsV1:readonly NativeStorageBackendDeclarationV1[]=Object.freeze(protocol.backends.map(row=>Object.freeze({...row}) as NativeStorageBackendDeclarationV1));
/** 🔣️ Names the environment variables that carry a selection. */
export const nativeStorageEnvironmentV1=Object.freeze({...protocol.environment});

const refuse=(key:keyof typeof protocol.refusals,detail?:string):never=>{throw Error(protocol.refusals[key]+(detail?": "+detail:""));};
const declared=(values:readonly string[],kind:string)=>{const known=schema.definitions.Capability.enum as readonly string[];for(const value of values)if(!known.includes(value))throw Error("Declared native storage "+kind+" required: "+value);return values;};

/** 🧭️ Chooses one declared backend: explicit id first, otherwise the candidate that meets every required capability and the most preferred ones, ties by declaration order. Pure and platform-independent; the host platform is just `selection.platform`. */
export function selectNativeStorageBackendV1(selection:NativeStorageSelectionV1,backends:readonly NativeStorageBackendDeclarationV1[]=nativeStorageBackendsV1):NativeStorageBackendDeclarationV1{
 const platform=selection.platform;if(typeof platform!=="string"||!(schema.definitions.Platform.enum as readonly string[]).includes(platform))return refuse("platform");
 const require=declared(selection.require??[],"capability"),prefer=declared(selection.prefer??protocol.preferred,"capability");
 const hosted=backends.filter(row=>row.platforms.includes(platform));
 if(selection.backend!==undefined){const row=backends.find(item=>item.id===selection.backend);if(!row||!hosted.includes(row))return refuse("unavailable",selection.backend);if(!require.every(name=>row.capabilities.includes(name)))return refuse("unsatisfied",selection.backend);return row;}
 const candidates=hosted.filter(row=>require.every(name=>row.capabilities.includes(name)));
 if(!candidates.length)return refuse("unsatisfied");
 const score=(row:NativeStorageBackendDeclarationV1)=>prefer.filter(name=>row.capabilities.includes(name)).length;
 return candidates.reduce((best,row)=>score(row)>score(best)?row:best);
}
