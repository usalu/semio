import {spawn} from "node:child_process";
import {chmod,lstat,mkdir,readdir,rename,rm,stat,unlink,writeFile} from "node:fs/promises";
import {readFileSync,statSync} from "node:fs";
import {join} from "node:path";
import {deflateSync} from "node:zlib";
import {GIT_PINS_V1,resolveGitV1} from "../../🌿️git/🟦️.ts";
import {readPackNamesV1} from "../🥾️bootstrap/🧺️packs/🟦️.ts";

/** 🧷️ The directory of packs below the worktree root, with the trailing slash. */
export const packDirectoryV1=".git/objects/pack/";

/** 🔏️ The decoded `.git/semio-packs.json`: stems the replica may publish and stems (or `multi-pack-index`) whose deletion it may publish. Schema: `./🧬️schema/🔣️.json#/definitions/PackAuthorisation`. */
export type PackAuthorisationV1=Readonly<{publish:ReadonlySet<string>;retire:ReadonlySet<string>}>;

/** 🧹️ What one rescue pass did. Schema: `./🧬️schema/🔣️.json#/definitions/Rescue`. */
export type RescueV1=Readonly<{examined:number;objects:number;retired:number;kept:number}>;

const extensions=new Set(["pack","idx","rev","mtimes","bitmap","keep","promisor"]),authorisationFile=(worktree:string):string=>join(worktree,".git","semio-packs.json"),twelveHours=12*3_600_000;
const none:PackAuthorisationV1={publish:new Set(),retire:new Set()};
const cache=new Map<string,Readonly<{stamp:string;value:PackAuthorisationV1}>>();

/** 🔬️ Splits a worktree-relative path into the stem and extension of a pack file of `.git/objects/pack`; `multi-pack-index` and its derived files have the stem `multi-pack-index`; `undefined` for other paths and unknown names. */
export function packPartsV1(path:string):Readonly<{stem:string;extension:string}>|undefined{
 if(!path.startsWith(packDirectoryV1))return undefined;
 const name=path.slice(packDirectoryV1.length);
 if(name===""||name.includes("/"))return undefined;
 if(name==="multi-pack-index"||name.startsWith("multi-pack-index-"))return {stem:"multi-pack-index",extension:name.slice(name.lastIndexOf(".")+1)};
 const dot=name.lastIndexOf(".");
 if(dot<=0)return undefined;
 const extension=name.slice(dot+1);
 return extensions.has(extension)&&!name.startsWith("tmp_")&&!name.startsWith(".")?{stem:name.slice(0,dot),extension}:undefined;
}

/** 🔏️ Reads the authorisation of a worktree (cached by size and modification time, so the synchronous ignore check stays cheap); absent or unreadable means nothing is authorised. */
export function readPackAuthorisationV1(worktree:string):PackAuthorisationV1{
 const file=authorisationFile(worktree);
 try{
  const info=statSync(file),stamp=`${info.size}:${info.mtimeMs}`,held=cache.get(file);
  if(held?.stamp===stamp)return held.value;
  const raw=JSON.parse(readFileSync(file,"utf8")) as {version?:unknown;publish?:Record<string,number>;retire?:Record<string,number>};
  const value=raw.version===1?{publish:new Set(Object.keys(raw.publish??{})),retire:new Set(Object.keys(raw.retire??{}))}:none;
  cache.set(file,{stamp,value});
  return value;
 }catch{return none;}
}

/** ✍️ Authorises stems for publishing and/or retiring (atomic write, entries older than 12 hours dropped). Called by the consolidator before it places a pack and before it retires packs. */
export async function authorisePacksV1(worktree:string,grant:Readonly<{publish?:readonly string[];retire?:readonly string[]}>,now:number=Date.now()):Promise<void>{
 const file=authorisationFile(worktree);
 let raw:{publish:Record<string,number>;retire:Record<string,number>}={publish:{},retire:{}};
 try{const parsed=JSON.parse(readFileSync(file,"utf8")) as typeof raw;raw={publish:parsed.publish??{},retire:parsed.retire??{}};}catch{}
 for(const table of [raw.publish,raw.retire])for(const [stem,at] of Object.entries(table))if(now-at>twelveHours)delete table[stem];
 for(const stem of grant.publish??[])raw.publish[stem]=now;
 for(const stem of grant.retire??[])raw.retire[stem]=now;
 const temporary=`${file}.${process.pid}.tmp`;
 await writeFile(temporary,JSON.stringify({version:1,...raw}));
 await rename(temporary,file);
}

/** 🚫️ Tells whether a path of the pack directory or of `.git/objects/info` (the dumb-transport pack list and the commit graphs, derived caches every git client rewrites) stays on this machine: unknown names, every `multi-pack-index`, and every pack whose `.pack` the fleet does not hold (`known`) unless the consolidator authorised its stem, or the fleet holds no pack at all (`fleetHasPacks` false: the first machine seeds its packs). Other paths are not decided here and answer false. */
export function packLocalV1(path:string,known:(packPath:string)=>boolean,fleetHasPacks:()=>boolean,worktree:string):boolean{
 if(path.startsWith(".git/objects/info/"))return true;
 if(!path.startsWith(packDirectoryV1))return false;
 const parts=packPartsV1(path);
 if(parts===undefined)return true;
 if(parts.stem==="multi-pack-index")return true;
 if(known(`${packDirectoryV1}${parts.stem}.pack`))return false;
 if(readPackAuthorisationV1(worktree).publish.has(parts.stem))return false;
 return fleetHasPacks();
}

async function remove(file:string):Promise<void>{
 try{await unlink(file);}
 catch(error){
  if((error as {code?:string}).code==="ENOENT")return;
  await chmod(file,0o666).catch(()=>{});
  await rm(file,{force:true});
 }
}

function holds(names:Buffer,fanout:Uint32Array,name:Buffer):boolean{
 const first=name[0]!;
 let low=first===0?0:fanout[first-1]!,high=fanout[first]!;
 while(low<high){
  const middle=(low+high)>>>1,order=names.compare(name,0,20,middle*20,middle*20+20);
  if(order===0)return true;
  if(order>0)high=middle;else low=middle+1;
 }
 return false;
}

function readObjects(executable:string,worktree:string,names:readonly string[],signal:AbortSignal|undefined):Promise<Map<string,Readonly<{type:string;content:Buffer}>>|undefined>{
 return new Promise(done=>{
  const pins=GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]),child=spawn(executable,["-C",worktree,...pins,"cat-file","--batch"],{stdio:["pipe","pipe","ignore"],env:{...process.env,GIT_OPTIONAL_LOCKS:"0"},...signal===undefined?{}:{signal}});
  const chunks:Buffer[]=[];
  child.stdout.on("data",chunk=>{chunks.push(chunk as Buffer);});
  child.stdin.on("error",()=>{});
  child.on("error",()=>done(undefined));
  child.on("close",code=>{
   if(code!==0){done(undefined);return;}
   const data=Buffer.concat(chunks),found=new Map<string,{type:string;content:Buffer}>();
   let offset=0;
   while(offset<data.length){
    const end=data.indexOf(0x0a,offset);
    if(end<0){done(undefined);return;}
    const [name,type,size]=data.subarray(offset,end).toString("latin1").split(" ");
    if(type===undefined||size===undefined||type==="missing"){done(undefined);return;}
    const length=Number(size);
    found.set(name!,{type,content:data.subarray(end+1,end+1+length)});
    offset=end+1+length+1;
   }
   done(found);
  });
  child.stdin.end(names.join("\n")+"\n");
 });
}

async function writeLoose(worktree:string,name:string,type:string,content:Buffer):Promise<boolean>{
 const directory=join(worktree,".git","objects",name.slice(0,2)),file=join(directory,name.slice(2));
 if(await lstat(file).then(()=>true,()=>false))return true;
 await mkdir(directory,{recursive:true});
 const temporary=`${file}.tmp-rescue-${process.pid}`;
 await writeFile(temporary,deflateSync(Buffer.concat([Buffer.from(`${type} ${content.length}\0`),content]),{level:1}),{mode:0o444});
 try{await rename(temporary,file);}catch{await unlink(temporary).catch(()=>{});}
 return lstat(file).then(()=>true,()=>false);
}

/** 🛟️ Deals with the complete packs (pack and index, older than `minAgeMilliseconds`) of this machine that the fleet does not hold and that were not authorised: every object they carry that no fleet pack holds and no loose file exists for is written as a loose object (the replica then publishes it like any other file), and once nothing is missing any more the local pack is deleted (index first). Dead fragments (a pack without index, an index without pack) older than an hour are deleted too. A pack with more than `limit` missing objects, or one whose objects git cannot read, stays and is counted in `kept`. Nothing of the fleet's packs is touched. */
export async function rescueLocalPacksV1(worktree:string,known:(packPath:string)=>boolean,options:Readonly<{now?:number;minAgeMilliseconds?:number;limit?:number;signal?:AbortSignal}>={}):Promise<RescueV1>{
 const directory=join(worktree,".git","objects","pack"),now=options.now??Date.now(),minAge=options.minAgeMilliseconds??120_000,limit=options.limit??200_000,auth=readPackAuthorisationV1(worktree);
 const groups=new Map<string,Map<string,number>>();
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  const parts=packPartsV1(`${packDirectoryV1}${name}`),info=await stat(join(directory,name)).catch(()=>undefined);
  if(parts===undefined||parts.stem==="multi-pack-index"||info===undefined)continue;
  const group=groups.get(parts.stem)??new Map<string,number>();
  group.set(parts.extension,info.mtimeMs);groups.set(parts.stem,group);
 }
 const locals:string[]=[],tables:Array<Readonly<{count:number;fanout:Uint32Array;names:Buffer}>>=[];
 let objects=0,retired=0,kept=0;
 for(const [stem,group] of groups){
  if(known(`${packDirectoryV1}${stem}.pack`)||auth.publish.has(stem)){
   if(group.has("pack")&&group.has("idx")){const table=await readPackNamesV1(join(directory,`${stem}.idx`));if(table!==undefined)tables.push(table);}
   continue;
  }
  const newest=Math.max(...group.values());
  if(group.has("pack")&&group.has("idx")){if(now-newest>=minAge)locals.push(stem);}
  else if(now-newest>=3_600_000){for(const extension of ["idx",...extensions])await remove(join(directory,`${stem}.${extension}`));await remove(join(directory,`${stem}.pack`));retired++;}
 }
 const git=await resolveGitV1();
 for(const stem of locals){
  options.signal?.throwIfAborted();
  const table=await readPackNamesV1(join(directory,`${stem}.idx`));
  if(table===undefined||git===null){kept++;continue;}
  const missing:string[]=[];
  for(let index=0;index<table.count&&missing.length<=limit;index++){
   const name=table.names.subarray(index*20,index*20+20);
   if(tables.some(other=>holds(other.names,other.fanout,name)))continue;
   const hex=name.toString("hex");
   if(await lstat(join(worktree,".git","objects",hex.slice(0,2),hex.slice(2))).then(()=>true,()=>false))continue;
   missing.push(hex);
  }
  if(missing.length>limit){kept++;continue;}
  let complete=true;
  if(missing.length>0){
   const found=await readObjects(git,worktree,missing,options.signal);
   if(found===undefined)complete=false;
   else for(const name of missing){const item=found.get(name);if(item===undefined||!await writeLoose(worktree,name,item.type,item.content)){complete=false;break;}objects++;}
  }
  if(!complete){kept++;continue;}
  for(const extension of ["idx","rev","bitmap","mtimes","keep","promisor","pack"])await remove(join(directory,`${stem}.${extension}`));
  retired++;
 }
 return {examined:locals.length,objects,retired,kept};
}
