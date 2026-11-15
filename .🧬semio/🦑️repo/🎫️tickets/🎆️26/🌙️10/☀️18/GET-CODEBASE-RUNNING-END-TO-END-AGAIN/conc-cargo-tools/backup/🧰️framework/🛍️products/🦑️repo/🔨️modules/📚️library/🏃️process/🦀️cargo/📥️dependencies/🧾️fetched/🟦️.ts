import {createHash} from "node:crypto";
import {posix,resolve,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=Object.freeze({...schema.definitions.Protocol.const});
const digestOf=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const underHome=(home:string,path:string)=>{const row=resolve(home,path);if(row!==resolve(home)&&!row.startsWith(resolve(home)+sep))throw Error("Cargo registry stamp escapes its home");return row;};

/** 🧾️ Proves that every package of the named lockfiles was fetched while the registry directories still carried the recorded stamps. */
export type FetchedReceiptV1=Readonly<{version:1;request:string;locks:readonly Readonly<{path:string;digest:string;bytes:number}>[];registry:readonly Readonly<{path:string;dev:string;ino:string;mtimeNs:string}>[]}>;
/** 🔌️ Observes the repository Source bytes and the registry directories without granting any write. */
export type FetchedPortV1=Readonly<{read(path:string):Promise<Uint8Array|undefined>;stamp(path:string):Readonly<{dev:string;ino:string;mtimeNs:string}>|undefined;list(path:string):readonly string[]|undefined}>;

/** 📜️ Classifies a lockfile: only registry and path packages can be proven fetched by directory stamps; git sources keep the unconditional fetch. */
export function classifyFetchedLockV1(text:string):"registry"|"unsupported"{
 let document:{package?:Array<{source?:unknown}>};try{document=Bun.TOML.parse(text) as typeof document;}catch{return"unsupported";}
 if(!Array.isArray(document.package))return"unsupported";
 let registry=false;
 for(const row of document.package){if(row.source===undefined)continue;if(typeof row.source!=="string"||!/^(registry|sparse)\+/u.test(row.source))return"unsupported";registry=true;}
 return registry?"registry":"unsupported";
}

/** 🔎️ Names the lockfile cargo writes for one selected manifest: beside the nearest ancestor workspace root, else beside the manifest. */
export async function fetchedLockPathV1(manifest:string,read:FetchedPortV1["read"]):Promise<string|undefined>{
 let directory=posix.dirname(manifest);
 for(let depth=0;depth<64;depth++){
  const bytes=await read(posix.join(directory,"Cargo.toml"));
  if(bytes){let document:{workspace?:unknown;package?:{workspace?:unknown}};try{document=Bun.TOML.parse(Buffer.from(bytes).toString("utf8")) as typeof document;}catch{return undefined;}
   if(document.workspace!==undefined)return posix.join(directory,"Cargo.lock");
   if(typeof document.package?.workspace==="string"){const explicit=posix.normalize(posix.join(directory,document.package.workspace));return explicit.startsWith("..")?undefined:posix.join(explicit,"Cargo.lock");}}
  if(directory==="."||directory==="")break;
  directory=posix.dirname(directory);
 }
 return posix.join(posix.dirname(manifest),"Cargo.lock");
}

/** 📸️ Stamps the registry cache and extraction directories and every directory directly below them. */
export function fetchedRegistryStampsV1(home:string,port:FetchedPortV1):FetchedReceiptV1["registry"]|undefined{
 const rows:Array<FetchedReceiptV1["registry"][number]>=[];
 for(const parent of protocol.registryDirectories){
  const listed=port.list(underHome(home,parent));if(listed===undefined)continue;const names=[...listed].sort();
  for(const path of[parent,...names.map(name=>posix.join(parent,name))]){const stamp=port.stamp(underHome(home,path));if(!stamp)return undefined;rows.push(Object.freeze({path,...stamp}));}
 }
 return rows.length>protocol.maximumRegistryEntries?undefined:Object.freeze(rows);
}

/** 🧾️ Builds the receipt after a successful locked fetch; refuses any lockfile that is not purely registry-sourced. */
export async function captureFetchedV1(input:Readonly<{request:string;manifests:readonly string[];home:string}>,port:FetchedPortV1):Promise<FetchedReceiptV1|undefined>{
 const paths=new Set<string>();
 for(const manifest of input.manifests){const lock=await fetchedLockPathV1(manifest,port.read);if(!lock)return undefined;paths.add(lock);}
 if(!paths.size||paths.size>protocol.maximumLocks)return undefined;
 const locks:Array<FetchedReceiptV1["locks"][number]>=[];
 for(const path of[...paths].sort()){const bytes=await port.read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes||classifyFetchedLockV1(Buffer.from(bytes).toString("utf8"))!=="registry")return undefined;locks.push(Object.freeze({path,digest:digestOf(bytes),bytes:bytes.byteLength}));}
 const registry=fetchedRegistryStampsV1(input.home,port);if(!registry?.length)return undefined;
 const receipt:FetchedReceiptV1=Object.freeze({version:1,request:input.request,locks:Object.freeze(locks),registry});
 return validateJsonSchemaSubset(schema.definitions.Receipt,receipt,schema).length||Buffer.byteLength(JSON.stringify(receipt))>protocol.maximumReceiptBytes?undefined:receipt;
}

/** ✅️ Admits stored receipt text only when it is schema-valid and names this exact request. */
export function readFetchedReceiptV1(text:string,request:string):FetchedReceiptV1|undefined{
 let value:unknown;try{value=JSON.parse(text);}catch{return undefined;}
 return validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length||(value as FetchedReceiptV1).request!==request?undefined:value as FetchedReceiptV1;
}

/** 🛡️ A receipt holds while every lockfile digest and every registry directory stamp is unchanged. */
export async function fetchedHoldsV1(receipt:FetchedReceiptV1,home:string,port:FetchedPortV1):Promise<boolean>{
 for(const lock of receipt.locks){const bytes=await port.read(lock.path);if(!bytes||bytes.byteLength!==lock.bytes||digestOf(bytes)!==lock.digest)return false;}
 const now=fetchedRegistryStampsV1(home,port);
 return now!==undefined&&now.length===receipt.registry.length&&now.every((row,index)=>{const kept=receipt.registry[index]!;return row.path===kept.path&&row.dev===kept.dev&&row.ino===kept.ino&&row.mtimeNs===kept.mtimeNs;});
}

/** 🏷️ Where one request's receipt lives inside the registry-ready namespace. */
export function fetchedEntryNameV1(slot:number):string{return "f"+slot+protocol.entrySuffix;}
