import{lstatSync,readFileSync}from"node:fs";
import{dirname}from"node:path";
import{captureOwnedCommandV1}from"../../../../../🎛️owned-execution/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json" with{type:"json"};
const protocol=schema.definitions.ImageProtocol.const.bootSession,pattern=new RegExp(schema.definitions.BootSession.pattern,"u");
let primed:string|undefined,priming:Promise<string>|undefined;

/** 🥾️ Parses one operating-system boot session identifier into its canonical uppercase form. */
export function parseNativeBootSessionV1(text:string):string{const value=text.trim().toUpperCase();if(Buffer.byteLength(text)>protocol.maximumBytes||!pattern.test(value))throw Error("Exact boot session identifier required");return value;}

/** 📖️ Reads the identifier that is stable for one boot and changes at every restart; the identifier is publication metadata. */
export async function readNativeBootSessionV1(control:ResourceLeaseControl,cwd:string):Promise<string>{
 checkResourceLeaseControl(control);
 if(primed!==undefined)return primed;
 const value=await(priming??=(async()=>{try{const text=process.platform==="darwin"?await captureOwnedCommandV1(protocol.program,protocol.arguments,cwd,{control,output:{maximumBytes:protocol.maximumBytes,maximumLines:2}}):process.platform==="linux"?readFileSync(protocol.linuxFile,"utf8"):undefined;if(text===undefined)throw Error("Boot session identifier unavailable on this platform");primed=parseNativeBootSessionV1(text);return primed;}finally{priming=undefined;}})());
 checkResourceLeaseControl(control);return value;
}

/** 🧲️ Returns the identifier already read by the running operation; a writer must have awaited the reader first. */
export function nativeBootSessionV1():string{if(primed===undefined)throw Error("Boot session identifier must be read before a receipt is written");return primed;}

/** 🧭️ Lists the inode of every ancestor directory from the image directory's parent to the file system root; inodes survive restarts, device numbers do not. */
export function readNativeImageAncestryV1(directory:string):readonly string[]{
 const rows:string[]=[];for(let path=dirname(directory);;path=dirname(path)){const row=lstatSync(path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||rows.length>=protocol.maximumAncestors)throw Error("Exact bounded image ancestry required");rows.push(String(row.ino));if(dirname(path)===path)break;}
 return Object.freeze(rows);
}

/** 🧷️ Requires the exact published ancestor inode chain without rewriting publication DATA. */
export function requireNativeImageAncestryBindingV1(value:unknown):void{if(validateJsonSchemaSubset(schema.definitions.AncestryBinding,value,schema).length)throw Error("Closed Native ancestry binding required");const row=value as Readonly<{published:readonly string[];current:readonly string[]}>;if(row.published.length!==row.current.length||row.published.some((ino,index)=>ino!==row.current[index]))throw Error("Original Native image ancestry changed");}
