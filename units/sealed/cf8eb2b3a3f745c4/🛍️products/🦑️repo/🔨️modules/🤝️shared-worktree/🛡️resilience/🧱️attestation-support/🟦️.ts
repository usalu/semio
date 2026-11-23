import {createHash} from "node:crypto";
import {readdir,readFile,stat} from "node:fs/promises";
import {dirname,join,relative,resolve,sep} from "node:path";

/** 🧾️ One failure kind of the health record, counted with the time of its last occurrence. */
export type FailureSummaryV1=Readonly<{kind:string;count:number;lastMs:number}>;

const sourceNames=new Set(["🟦️.ts","📜️script.ts"]),testsDirectory="🧪️tests";
const specifier=/\bfrom\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']|\bimport\s*["']([^"']+)["']/gu;
const bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const posix=(path:string):string=>path.split(sep).join("/");
const gone=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";

/** 🚶️ Collects the files below `directory` that `keep` accepts, as `/` paths relative to it, without tests and installed packages. */
export async function walkSourcesV1(directory:string,prefix:string,into:string[],keep:(name:string,isDirectory:boolean)=>boolean):Promise<void>{
 for(const entry of await readdir(directory,{withFileTypes:true})){
  if(entry.name===testsDirectory||entry.name==="node_modules")continue;
  const path=prefix===""?entry.name:`${prefix}/${entry.name}`;
  if(entry.isDirectory())await walkSourcesV1(join(directory,entry.name),path,into,keep);
  else if(entry.isFile()&&keep(entry.name,false))into.push(path);
 }
}

/** 🧬️ The source files of the module that make up its code: every `🟦️.ts` and `📜️script.ts` below `root`, tests excluded, as sorted `/` paths relative to `root`. */
export async function moduleSourcesV1(root:string):Promise<string[]>{
 const found:string[]=[];
 await walkSourcesV1(root,"",found,name=>sourceNames.has(name));
 return found.sort(bytewise);
}

/** 🕸️ The files the module sources import beyond themselves, directly or through each other, as sorted `/` paths relative to `root` (outside the module they start with `..`): TypeScript sources, JSON schemas and catalogues. Only relative specifiers are followed, because the module has no other kind of runtime import. */
export async function importClosureV1(root:string,sources:readonly string[]):Promise<string[]>{
 const own=new Set(sources),extra:string[]=[],scheduled=new Set(sources.map(path=>resolve(root,...path.split("/")))),queue=[...scheduled];
 for(let index=0;index<queue.length;index++){
  const file=queue[index]!;
  const path=posix(relative(root,file));
  if(!own.has(path))extra.push(path);
  if(!file.endsWith(".ts")&&!file.endsWith(".js")&&!file.endsWith(".mjs"))continue;
  let text:string;
  try{text=await readFile(file,"utf8");}catch(error){if(gone(error)){if(!own.has(path))extra.pop();continue;}throw error;}
  for(const match of text.matchAll(specifier)){
   const name=match[1]??match[2]??match[3]!;
   if(!name.startsWith("."))continue;
   const target=resolve(dirname(file),name);
   if(scheduled.has(target))continue;
   const info=await stat(target).catch(()=>undefined);
   if(info?.isFile()){scheduled.add(target);queue.push(target);}
  }
 }
 return extra.sort(bytewise);
}

/** 🎲️ The deterministic restart delay of one origin for one code digest in `[0, span)` milliseconds: the first four bytes of `sha256(originId \0 digest)` as an unsigned big-endian integer modulo `span`, so that a whole fleet never restarts in the same seconds and a machine always waits the same time for the same change. */
export function restartJitterMillisecondsV1(originId:string,digest:string,span:number=60_000):number{
 if(span<=0)return 0;
 return createHash("sha256").update(`${originId}\0${digest}`).digest().readUInt32BE(0)%span;
}

/** 🚧️ Raised instead of issuing a store call while an earlier store call is still stuck or has not returned within its deadline. */
export class StoreStalledV1 extends Error{constructor(){super("Store call refused: an earlier store call did not return within its deadline");this.name="StoreStalledV1";}}
