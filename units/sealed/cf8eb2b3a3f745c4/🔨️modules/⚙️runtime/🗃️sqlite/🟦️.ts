import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeSqliteDatabaseV1, RuntimeSqliteOptionsV1 } from "../📋️contract/🟦️.ts";

/** 🔌️ The filesystem operations the open explanation uses; a caller may inject its own to simulate a full disk or an exhausted quota. */
export type SqliteFilesystemV1={mkdirSync(path:string,options:{recursive:boolean}):unknown;openSync(path:string,flags:string):number;writeSync(descriptor:number,data:Uint8Array):number;closeSync(descriptor:number):void;unlinkSync(path:string):void;lstatSync(path:string):{size:number;nlink:number};statfsSync(path:string):{bavail:number;bsize:number;ffree:number}};
function sqliteFilesystemV1():SqliteFilesystemV1|undefined{const host=(globalThis as {process?:{getBuiltinModule?:(name:string)=>unknown}}).process;return host?.getBuiltinModule?.("node:fs") as SqliteFilesystemV1|undefined;}
function sqliteDirectoryOfV1(path:string):string{const cut=Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\"));return cut>0?path.slice(0,cut):".";}
/** 🩺️ Names why a SQLite file cannot be opened, in English and German: the path, the errno of a sibling create-and-write probe (one 4 KiB block, so a block quota that still allows an empty file shows too), the database state and the free space of its directory; when the probe fails with `EDQUOT` or `ENOSPC` the message starts with the directory that is full (`No space left (EDQUOT) in <directory>`) and the error carries `space: true`, `directory` and `probe`. The original code, errno and result code stay on the error so every classifier of the callers keeps working.
 * @see ./🧪️tests/🩺️open-failure/🟦️.ts */
export function explainSqliteOpenFailureV1(error:unknown,path:string|undefined,filesystem:SqliteFilesystemV1|undefined=sqliteFilesystemV1()):unknown{
  const text=error instanceof Error?error.message:String(error);
  if(path===undefined||path===""||path===":memory:"||!/unable to open database file/u.test(text))return error;
  const fs=filesystem,facts:string[]=[],directory=sqliteDirectoryOfV1(path),code=(cause:unknown)=>(cause as {code?:string}).code??String(cause);let probeCode="unknown";
  if(fs){
    try{const probe=path+".probe-"+String(Date.now()),descriptor=fs.openSync(probe,"wx");try{fs.writeSync(descriptor,new Uint8Array(4096));}finally{fs.closeSync(descriptor);try{fs.unlinkSync(probe);}catch{}}probeCode="ok";}catch(cause){probeCode=code(cause);}
    facts.push("create probe "+probeCode);
    try{const stat=fs.lstatSync(path);facts.push(`database ${stat.size} bytes nlink ${stat.nlink}`);}catch(cause){facts.push("database "+code(cause));}
    try{const stat=fs.statfsSync(directory);facts.push(`free ${stat.bavail*stat.bsize} bytes ${stat.ffree} files`);}catch(cause){facts.push("statfs "+code(cause));}
  }
  const space=probeCode==="EDQUOT"||probeCode==="ENOSPC",detail=`${path} (${facts.join(", ")||"no filesystem probe"})`;
  const head=space?`No space left (${probeCode}) in ${directory}: the disk or the quota of the user holding it is full; free space there. / Kein Platz mehr (${probeCode}) in ${directory}: der Datenträger oder die Quote des Benutzers ist voll; dort Platz freigeben. `:"";
  const source=error as {code?:unknown;errno?:unknown;errcode?:unknown},explained=new Error(`${head}${text}: ${detail}. / Die SQLite-Datenbank kann nicht geöffnet werden: ${detail}.`,{cause:error});
  return Object.assign(explained,{...(source.code!==undefined?{code:source.code}:{}),...(source.errno!==undefined?{errno:source.errno}:{}),...(source.errcode!==undefined?{errcode:source.errcode}:{}),path,directory,probe:probeCode,space});
}
/** 📂️ Creates the missing parent directories of a database that is opened for creation, once. */
function createSqliteParentsV1(path:string|undefined,options:RuntimeSqliteOptionsV1|undefined):boolean{
  if(path===undefined||path===""||path===":memory:"||options?.readonly===true||options?.create===false)return false;
  const fs=sqliteFilesystemV1();if(!fs)return false;
  try{fs.lstatSync(sqliteDirectoryOfV1(path));return false;}catch{try{fs.mkdirSync(sqliteDirectoryOfV1(path),{recursive:true});return true;}catch{return false;}}
}
/** 🗃️ A SQLite connection whose implementation is chosen by the hosting runtime. */
export interface SqliteDatabaseV1 extends RuntimeSqliteDatabaseV1 {}
export class SqliteDatabaseV1 {
  constructor(path?: string, options?: RuntimeSqliteOptionsV1) {
    const Member=requireRuntimeMemberV1("sqlite")();
    try{return new Member(path, options) as SqliteDatabaseV1;}
    catch(error){
      if(createSqliteParentsV1(path,options))try{return new Member(path, options) as SqliteDatabaseV1;}catch(second){throw explainSqliteOpenFailureV1(second,path);}
      throw explainSqliteOpenFailureV1(error,path);
    }
  }
}
