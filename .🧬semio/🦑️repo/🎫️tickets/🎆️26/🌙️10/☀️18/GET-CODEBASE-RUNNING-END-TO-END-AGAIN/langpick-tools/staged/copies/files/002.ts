import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{accessSync,constants,statSync}from"node:fs";
import{isAbsolute,win32}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
const protocol=schema.definitions.Protocol.const;
/** 🧰️ Names the Windows system programs whose absolute path derives from the system directory. */
export type NativeStorageWindowsProgramIdV1=keyof typeof protocol.windows;
/** 🔌️ Supplies the one host read pinning needs, so the laws replace the machine by a table. */
export type NativeStorageProgramHostV1=Readonly<{executable(path:string):boolean}>;
const say=(id:keyof typeof protocol.messages,values:Readonly<Record<string,string>>)=>{const row=protocol.messages[id],fill=(text:string)=>text.replace(/\{(\w+)\}/gu,(_,key:string)=>values[key]??"");return noticeLanguagesV1(["en","de"] as const).map(language=>fill(row[language])).join(" / ");};
/** 📍️ Builds the absolute path of one Windows system program from the validated SystemRoot, never from PATH. */
export function nativeStorageWindowsProgramPathV1(id:NativeStorageWindowsProgramIdV1,environment:Readonly<Record<string,string|undefined>>):string{
 const root=environment["SystemRoot"];if(typeof root!=="string"||!new RegExp(protocol.systemRootPattern,"u").test(root)||win32.normalize(root)!==root)throw Error(say("systemRoot",{id}));
 return protocol.windows[id].replace(protocol.systemRootToken,root);
}
const real:NativeStorageProgramHostV1=Object.freeze({executable:path=>{try{if(!statSync(path).isFile())return false;if(process.platform!=="win32")accessSync(path,constants.X_OK);return true;}catch{return false;}}});
const pinned=new Set<string>();
/** 📌️ Proves once per process that a system program named by its absolute path is an executable regular file and returns that path; a relative name or a missing program is refused in English and German. See https://man7.org/linux/man-pages/man7/environ.7.html */
export function pinNativeStorageProgramV1(program:string,host:NativeStorageProgramHostV1=real):string{
 if(!isAbsolute(program)&&!win32.isAbsolute(program))throw Error(say("absolute",{program}));
 if(host===real&&pinned.has(program))return program;
 if(!host.executable(program))throw Error(say("missing",{program}));
 if(host===real)pinned.add(program);return program;
}
