import {mkdirSync,mkdtempSync,readdirSync,lstatSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";

export const HERE=import.meta.dir;
export const ORACLE=join(HERE,"oracle.py");
export const NATIVE=process.env.SEMIO_REPO_NATIVE_DIR??"";
export const WATCHER=process.env.SEMIO_REPO_WATCH_BINARY??(NATIVE?NATIVE+"/semio-repo-watch":"");
export const FAKETOOL=process.env.SEMIO_REPO_FAKETOOL_BINARY??(NATIVE?NATIVE+"/faketool":"");
export const haveBinary=(path:string)=>{try{lstatSync(path);return true;}catch{return false;}};

export function rng(seed:number){let s=seed>>>0;return()=>{s=(s*1664525+1013904223)>>>0;return s/4294967296;};}
export const pick=<T>(random:()=>number,items:readonly T[]):T=>items[Math.floor(random()*items.length)]!;
export const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));

export async function waitFor(check:()=>boolean|Promise<boolean>,timeoutMs=5000,stepMs=5){
 const end=performance.now()+timeoutMs;
 while(performance.now()<end){if(await check())return true;await sleep(stepMs);}
 return false;
}

export function tempRoot(prefix:string){return realpathSync(mkdtempSync(join(tmpdir(),prefix)));}

/** 🧷 A temporary directory with a path short enough for a unix socket beneath it: the platform's own temporary directory, or the POSIX root `/tmp` where that one is long. */
export function shortTempDirectory(prefix:string){return mkdtempSync(join(process.platform==="win32"||tmpdir().length<=40?tmpdir():"/tmp",prefix));}
export const cleanup=(path:string)=>rmSync(path,{recursive:true,force:true});

/** 📸 Naive complete snapshot of a tree: files with modes, directories with stamps, honouring the same exclusions. */
export function snapshot(root:string,excluded:(rel:string)=>boolean){
 const files=new Map<string,number>(),directories=new Map<string,[number,number]>();
 const walk=(rel:string)=>{
  const status=lstatSync(rel===""?root:root+"/"+rel);directories.set(rel,[status.mtimeMs,status.ino]);
  for(const name of readdirSync(rel===""?root:root+"/"+rel)){
   const child=rel===""?name:rel+"/"+name;if(excluded(child))continue;
   const s=lstatSync(root+"/"+child);
   if(s.isDirectory())walk(child);else if(s.isFile())files.set(child,s.mode);
  }
 };
 walk("");
 return{files,directories};
}

export const sha=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex");

/** 🧮 Owner digest from the independent Python oracle (hashlib). */
export function pythonDigest(root:string,paths:string[]){
 const out=spawnSync("python3",["-I",ORACLE,"digest",root,...paths],{encoding:"utf8"});
 if(out.status!==0)throw new Error(out.stderr);
 return out.stdout.trim();
}

export function writeTree(root:string,files:Record<string,string>){
 for(const[path,content]of Object.entries(files)){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),content);}
}
export const read=(path:string)=>readFileSync(path,"utf8");
