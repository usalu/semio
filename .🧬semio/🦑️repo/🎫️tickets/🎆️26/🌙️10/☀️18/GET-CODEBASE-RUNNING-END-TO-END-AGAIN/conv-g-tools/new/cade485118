import {lstat,readdir} from "node:fs/promises";
import type {BigIntStats} from "node:fs";
import {join} from "node:path";
import type {ChangeRowV1} from "../🟦️.ts";

/** 🌳️ The identity of every path at or under some roots: directories `d:<inode>`, files `f:<size>:<mtimeNs>:<inode>`, links `l:<size>:<mtimeNs>`. Directory times are left out on purpose: they move with every entry change and never decide anything alone. */
export type TreeStampsV1=ReadonlyMap<string,string>;
/** 🆚️ What differs between two sets of stamps. */
export type TreeDeltaV1=Readonly<{added:readonly string[];removed:readonly string[];changed:readonly string[]}>;

const stampOf=(row:BigIntStats):string=>row.isDirectory()?"d:"+row.ino:row.isSymbolicLink()?"l:"+row.size+":"+row.mtimeNs:"f:"+row.size+":"+row.mtimeNs+":"+row.ino;
const absolute=(root:string,path:string):string=>path===""?root:join(root,...path.split("/"));
const child=(path:string,name:string):string=>path===""?name:path+"/"+name;
const lstatOrNothing=async(file:string):Promise<BigIntStats|undefined>=>{try{return await lstat(file,{bigint:true});}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT"||code==="ENOTDIR")return undefined;throw error;}};

async function walkInto(root:string,path:string,into:Map<string,string>,skip:(path:string)=>boolean):Promise<void>{
 const row=await lstatOrNothing(absolute(root,path));
 if(!row)return;
 if(path!=="")into.set(path,stampOf(row));
 if(!row.isDirectory())return;
 let names:string[];
 try{names=await readdir(absolute(root,path));}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT"||code==="ENOTDIR")return;throw error;}
 await Promise.all(names.map(async name=>{const next=child(path,name);if(!skip(next))await walkInto(root,next,into,skip);}));
}

/** 🚶️ Walks the roots completely: the reference every journal-derived answer must equal. */
export async function walkTreeStampsV1(root:string,roots:readonly string[],skip:(path:string)=>boolean=()=>false):Promise<Map<string,string>>{
 const stamps=new Map<string,string>();
 await Promise.all(roots.map(path=>walkInto(root,path,stamps,skip)));
 return stamps;
}

/** ➖️ The difference of two stamp sets. */
export function diffTreeStampsV1(before:TreeStampsV1,after:TreeStampsV1):TreeDeltaV1{
 const added:string[]=[],removed:string[]=[],changed:string[]=[];
 for(const[path,stamp]of after){const old=before.get(path);if(old===undefined)added.push(path);else if(old!==stamp)changed.push(path);}
 for(const path of before.keys())if(!after.has(path))removed.push(path);
 return{added:added.sort(),removed:removed.sort(),changed:changed.sort()};
}

const within=(path:string,root:string):boolean=>root===""||path===root||path.startsWith(root+"/");

/** 🔁️ Brings stamps of the roots up to date from journal rows alone: each row's path is looked at once, a directory that is new, replaced or reported as added, removed or renamed is listed again below, a root below a structural directory row is walked again. The result equals a complete walk of the roots, which is the law the journal is held to. */
export async function applyChangeRowsV1(root:string,previous:TreeStampsV1,rows:readonly ChangeRowV1[],roots:readonly string[],skip:(path:string)=>boolean=()=>false):Promise<Readonly<{stamps:Map<string,string>;delta:TreeDeltaV1}>>{
 const next=new Map(previous);
 const dropBelow=(path:string):void=>{const prefix=path+"/";for(const key of next.keys())if(key.startsWith(prefix))next.delete(key);};
 const rewalk=roots.filter(base=>base!==""&&rows.some(row=>base.startsWith(row[2]+"/")&&!(row[1]===1&&row[0]==="m")));
 for(const base of rewalk){next.delete(base);dropBelow(base);const fresh=new Map<string,string>();await walkInto(root,base,fresh,skip);for(const[path,stamp]of fresh)next.set(path,stamp);}
 for(const row of[...rows].sort((a,b)=>a[2]<b[2]?-1:a[2]>b[2]?1:0)){
  const path=row[2];
  if(skip(path)||!roots.some(base=>within(path,base))||rewalk.some(base=>within(path,base)))continue;
  const status=await lstatOrNothing(absolute(root,path)),old=next.get(path);
  if(!status){if(old?.startsWith("d:"))dropBelow(path);next.delete(path);continue;}
  const stamp=stampOf(status);
  if(status.isDirectory()){
   const structural=row[1]===1&&row[0]!=="m";
   if(structural||old!==stamp){
    dropBelow(path);
    const fresh=new Map<string,string>();await walkInto(root,path,fresh,skip);
    for(const[key,value]of fresh)next.set(key,value);
   }
   next.set(path,stamp);
  }else{
   if(old?.startsWith("d:"))dropBelow(path);
   next.set(path,stamp);
  }
 }
 return{stamps:next,delta:diffTreeStampsV1(previous,next)};
}
