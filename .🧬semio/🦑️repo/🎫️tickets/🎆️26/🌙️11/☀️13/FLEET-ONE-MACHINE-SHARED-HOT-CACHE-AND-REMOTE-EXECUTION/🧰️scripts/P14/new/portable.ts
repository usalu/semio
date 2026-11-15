import{realpathSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧳️ Which observations of a record name the producer's machine: the kinds of the location rows, the dependency generation directory, its token and the probe that finds the taker's generation. See `./🧬️schema/🔣️.json`. */
export const portableRowsPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type PortableRowsPolicyV1=typeof portableRowsPolicyV1;
export type ObservationRowV1=Readonly<{kind:string;path:string;fact?:unknown}>;
const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),slashed=(text:string)=>text.replaceAll("\\","/");
const generationPattern=(policy:PortableRowsPolicyV1)=>new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])([0-9a-f]{16,64})(?=[\\\\/]|$)`,"gu");
const mapStrings=(value:unknown,change:(text:string)=>string):unknown=>typeof value==="string"?change(value):Array.isArray(value)?value.map(item=>mapStrings(item,change)):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,mapStrings(item,change)])):value;
const rewrite=(row:ObservationRowV1,change:(text:string)=>string):ObservationRowV1=>({...row,path:change(row.path),...row.fact===undefined?{}:{fact:mapStrings(row.fact,change)}});

/** 📍️ Whether `path` is a directory above `root` (every spelling of the separators, any drive). */
export function aboveRootV1(path:string,root:string):boolean{
 const below=slashed(root).replace(/\/+$/u,"")+"/",above=slashed(path).endsWith("/")?slashed(path):slashed(path)+"/";
 return below!==above&&below.startsWith(above);
}

/** 📤️ The observations of a record as every machine can replay them: the `lstat` of the directories above `root` is dropped, and when the rows name exactly one dependency generation its id is replaced by the token. */
export function portableRowsV1(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const kept=rows.filter(row=>!(policy.locationKinds.includes(row.kind)&&row.fact==="directory"&&aboveRootV1(row.path,root))),pattern=generationPattern(policy),ids=new Set<string>();
 for(const row of kept){for(const match of JSON.stringify([row.path,row.fact??null]).matchAll(pattern))ids.add(match[2]!);}
 if(ids.size!==1)return kept;
 const[id]=[...ids];
 return kept.map(row=>rewrite(row,text=>text.replaceAll(new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])${id}(?=[\\\\/]|$)`,"gu"),`$1${policy.token}`)));
}

/** 📥️ The observations of a portable record for this machine: the token becomes the machine's own dependency generation, or a name no directory has when it has none, so the project is inferred here. */
export function localRowsV1(rows:readonly ObservationRowV1[],generation:string|undefined,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const put=generation??policy.unavailable;
 return rows.map(row=>JSON.stringify([row.path,row.fact??null]).includes(policy.token)?rewrite(row,text=>text.replaceAll(policy.token,put)):row);
}

/** 🔎️ The id of the dependency generation this machine runs its inference tooling from: the real path of the `node_modules` of `root`, undefined where it is no generation directory. */
export function dependencyGenerationV1(root:string,resolve:(path:string)=>string=realpathSync,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string|undefined{
 try{const found=generationPattern(policy).exec(slashed(resolve(join(root,policy.probe))));return found?.[2];}catch{return undefined;}
}
