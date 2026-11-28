import {closeSync,existsSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,renameSync,statSync,writeFileSync} from "node:fs";
import {basename,dirname,join,relative} from "node:path";
import policy from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {brokenTextV1} from "./📖️texts/🟦️.ts";
import {type JournalCursorV1,type JournalEditorV1,type JournalEventV1} from "../📜️change-journal/🟦️.ts";

/** 📐️ The closed policy of the broken crates board: where boards live, how the replica journal is read, which files make a crate dirty and how the check runs.
 * @see ./🧬️schema/🔣️.json */
export type BrokenPolicyV1=Readonly<typeof policy>;
/** 🚦️ One crate: `ok` (checked, compiles), `broken` (a compile error in this crate), `blocked` (not reached because a dependency is broken), `unchecked` (known, not yet swept). */
export type BrokenStatusV1="ok"|"broken"|"blocked"|"unchecked";
/** 🧾️ The first compile error of a crate. */
export type BrokenErrorV1=Readonly<{code?:string;message:string;file?:string;line?:number}>;
/** 📋️ One row of the board. */
export type BrokenRowV1=Readonly<{crate:string;dir?:string;status:BrokenStatusV1;firstError?:BrokenErrorV1;lastEditor?:JournalEditorV1;checkedAtMs:number;brokenSinceMs?:number;digest?:string;origin?:string}>;
/** 🗂️ The board one machine publishes (`origins/<origin>/broken.json`): the rows, the journal cursor, the crates waiting for quiet and the last sweep. */
export type BrokenBoardV1=Readonly<{version:1;origin:string;atMs:number;cursor:JournalCursorV1;rows:readonly BrokenRowV1[];dirty:Readonly<Record<string,number>>;lastSweep?:Readonly<{atMs:number;mode:"workspace"|"crates"|"idle";seconds:number;crates:number;code:number;digest?:string}>}>;
/** 🌍️ The board of the whole fleet as `repo fleet status --section broken` shows it: the freshest row of every crate over all machines. */
export type BrokenSectionV1=Readonly<{atMs:number;origins:readonly Readonly<{origin:string;ageSeconds:number;crates?:number}>[];counts:Readonly<Record<BrokenStatusV1,number>>;rows:readonly BrokenRowV1[]}>;
/** 🗺️ The crates of the checkout: directory to name, and the names every crate depends on. */
export type BrokenIndexV1=Readonly<{byDirectory:ReadonlyMap<string,string>;dependencies:ReadonlyMap<string,readonly string[]>}>;

const rules=policy as BrokenPolicyV1;
const STATUS_ORDER:Record<BrokenStatusV1,number>={broken:0,blocked:1,unchecked:2,ok:3};

/** 🛡️ Reads the policy under the closed shape of the board schema. */
export function readBrokenPolicyV1(value:unknown=policy):BrokenPolicyV1{
 if(typeof value!=="object"||value===null||(value as {version?:number}).version!==1)throw Error("broken-policy-invalid");
 return value as BrokenPolicyV1;
}

/** ✅️ Checks a board against its closed schema; the empty list means valid. */
export function validateBrokenBoardV1(value:unknown):readonly string[]{
 return validateJsonSchemaSubset({$ref:"#/definitions/Board"},value,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

/** ✅️ Checks the merged section against its closed schema; the empty list means valid. */
export function validateBrokenSectionV1(value:unknown):readonly string[]{
 return validateJsonSchemaSubset({$ref:"#/definitions/Section"},value,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

const tomlName=(text:string):string|undefined=>{
 let section="";
 for(const raw of text.split("\n")){
  const line=raw.trim(),head=/^\[([^\]]+)\]$/u.exec(line);
  if(head!==null){section=head[1]!.trim();continue;}
  if(section==="package"){const match=/^name\s*=\s*"([^"]+)"/u.exec(line);if(match!==null)return match[1];}
 }
 return undefined;
};

const tomlDependencies=(text:string):readonly string[]=>{
 const names=new Set<string>();let section="";
 for(const raw of text.split("\n")){
  const line=raw.trim(),head=/^\[([^\]]+)\]$/u.exec(line);
  if(head!==null){
   section=head[1]!.trim();
   const nested=/^(?:target\.[^\]]+\.)?(?:dev-|build-)?dependencies\.([A-Za-z0-9_-]+)$/u.exec(section);
   if(nested!==null)names.add(nested[1]!);
   continue;
  }
  if(/^(?:target\.[^\]]+\.)?(?:dev-|build-)?dependencies$/u.test(section)){
   const match=/^([A-Za-z0-9_-]+)(?:\.[A-Za-z]+)?\s*=/u.exec(line);
   if(match!==null)names.add(match[1]!);
   const renamed=/package\s*=\s*"([^"]+)"/u.exec(line);
   if(renamed!==null)names.add(renamed[1]!);
  }
 }
 return [...names];
};

/** 🗺️ Indexes the crates of a checkout from their manifests: the directory of every `[package]`, and which crates each one depends on (all dependency tables, renamed ones by their package name); the workspace manifest and the excluded folders are skipped. */
export function brokenIndexCratesV1(root:string,limits:BrokenPolicyV1=rules):BrokenIndexV1{
 const byDirectory=new Map<string,string>(),dependencies=new Map<string,readonly string[]>(),excluded=new Set(limits.excludedSegments);
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(join(root,directory),{withFileTypes:true});}catch{return;}
  for(const entry of entries){
   if(excluded.has(entry.name))continue;
   if(entry.isDirectory())walk(directory===""?entry.name:directory+"/"+entry.name);
   else if(entry.name==="Cargo.toml"){
    let text:string;try{text=readFileSync(join(root,directory,entry.name),"utf8");}catch{continue;}
    const name=tomlName(text);
    if(name!==undefined){byDirectory.set(directory,name);dependencies.set(name,tomlDependencies(text));}
   }
  }
 };
 for(const name of limits.manifestRoots)walk(name);
 return {byDirectory,dependencies};
}

/** 🎯️ The crate a path belongs to: the nearest ancestor directory that holds a package manifest. */
export function brokenCrateOfV1(path:string,index:BrokenIndexV1):Readonly<{crate:string;dir:string}>|undefined{
 let directory=dirname(path);
 while(directory!=="."&&directory!=="/"&&directory!==""){
  const name=index.byDirectory.get(directory);
  if(name!==undefined)return {crate:name,dir:directory};
  const parent=dirname(directory);
  if(parent===directory)break;
  directory=parent;
 }
 return undefined;
}

/** 🔁️ The seed crates and every crate that depends on one of them, transitively. */
export function brokenDependentsV1(seed:Iterable<string>,index:BrokenIndexV1):ReadonlySet<string>{
 const reverse=new Map<string,string[]>(),crates=new Set(seed),queue=[...crates];
 for(const [crate,needs] of index.dependencies)for(const need of needs){const list=reverse.get(need)??[];list.push(crate);reverse.set(need,list);}
 while(queue.length){const next=queue.pop()!;for(const user of reverse.get(next)??[])if(!crates.has(user)){crates.add(user);queue.push(user);}}
 return crates;
}

/** 🔁️ The crates to check after a set of changes: the crates that own a changed Rust file or manifest and every crate that depends on them, transitively; `workspace` when a file that affects every crate changed (lock file, toolchain, cargo config) or a manifest of an unknown crate appeared. */
export function brokenDirtyCratesV1(events:readonly JournalEventV1[],index:BrokenIndexV1,limits:BrokenPolicyV1=rules):Readonly<{workspace:boolean;direct:ReadonlySet<string>;crates:ReadonlySet<string>}>{
 const direct=new Set<string>();let workspace=false;
 for(const event of events){
  const path=event.path,name=basename(path);
  if(limits.workspaceTriggers.some(trigger=>path===trigger||path.endsWith("/"+trigger))){workspace=true;continue;}
  if(!limits.rustExtensions.some(extension=>path.endsWith(extension)))continue;
  const owner=brokenCrateOfV1(path,index);
  if(owner===undefined){if(name==="Cargo.toml"&&event.operation!=="delete")workspace=true;continue;}
  direct.add(owner.crate);
 }
 const crates=brokenDependentsV1(direct,index);
 return {workspace,direct,crates};
}

/** 🏷️ The crate name of a cargo package id in the old (`name 0.1.0 (path+file://…)`) and the new (`path+file:///dir/name#0.1.0`, `…#name@0.1.0`) spelling. */
export function brokenPackageNameV1(id:string):string{
 const hash=id.indexOf("#");
 if(hash<0)return id.split(" ")[0]!;
 const tail=id.slice(hash+1),at=tail.indexOf("@");
 if(at>0)return tail.slice(0,at);
 const url=id.slice(0,hash).replace(/\/+$/u,"");
 return url.slice(url.lastIndexOf("/")+1);
}

/** 🧮️ What one check run said per crate: the first error of every crate that failed, the crates that compiled, in the order cargo reported them. */
export type BrokenCheckV1=Readonly<{errors:ReadonlyMap<string,BrokenErrorV1>;built:ReadonlySet<string>;finished:boolean|undefined}>;

/** 🔬️ Parses the JSON message lines of `cargo check --message-format=json` (lines that are not JSON are ignored): the first compile error of each crate with code, message and the primary span, the crates that produced an artifact, and whether the build finished. `strip` removes a root prefix from span paths so they read relative to the checkout. */
export function brokenParseCargoV1(text:string,strip:readonly string[]=[],limits:BrokenPolicyV1=rules):BrokenCheckV1{
 const errors=new Map<string,BrokenErrorV1>(),built=new Set<string>();let finished:boolean|undefined;
 for(const line of text.split("\n")){
  if(!line.startsWith("{"))continue;
  let row:{reason?:string;package_id?:string;message?:{level?:string;message?:string;code?:{code?:string}|null;spans?:{is_primary?:boolean;file_name?:string;line_start?:number}[]};success?:boolean};
  try{row=JSON.parse(line);}catch{continue;}
  if(row.reason==="compiler-artifact"&&row.package_id!==undefined)built.add(brokenPackageNameV1(row.package_id));
  else if(row.reason==="build-finished")finished=row.success;
  else if(row.reason==="compiler-message"&&row.package_id!==undefined&&row.message?.level==="error"){
   const crate=brokenPackageNameV1(row.package_id);
   if(errors.has(crate)||(row.message.message??"").startsWith("aborting due to"))continue;
   const span=row.message.spans?.find(item=>item.is_primary===true)??row.message.spans?.[0];
   let file=span?.file_name;
   if(file!==undefined)for(const prefix of strip)if(file.startsWith(prefix+"/")){file=file.slice(prefix.length+1);break;}
   errors.set(crate,{...row.message.code?.code===undefined?{}:{code:row.message.code.code},message:(row.message.message??"").slice(0,limits.limits.messageCharacters),...file===undefined?{}:{file},...span?.line_start===undefined?{}:{line:span.line_start}});
  }
 }
 return {errors,built,finished};
}

/** 🧩️ Merges one check run into the rows of a board: a crate with an error is `broken` (its first error and the moment it first broke are kept while it stays broken), one that produced an artifact `ok`, a checked crate without either `blocked` (cargo skipped it because a dependency failed); crates the run did not check keep their row, new crates start `unchecked`. The editor comes from the port, asked only for broken crates. */
export function brokenMergeV1(rows:readonly BrokenRowV1[],checked:ReadonlySet<string>,check:BrokenCheckV1,index:BrokenIndexV1,context:Readonly<{nowMs:number;digest?:string;origin:string;editor:(file:string|undefined,dir:string|undefined)=>JournalEditorV1|undefined}>,limits:BrokenPolicyV1=rules):readonly BrokenRowV1[]{
 const previous=new Map(rows.map(row=>[row.crate,row])),directories=new Map([...index.byDirectory].map(([dir,name])=>[name,dir])),result=new Map<string,BrokenRowV1>();
 for(const crate of directories.keys())result.set(crate,previous.get(crate)??{crate,dir:directories.get(crate)!,status:"unchecked",checkedAtMs:0});
 for(const crate of checked){
  const old=previous.get(crate),dir=directories.get(crate),error=check.errors.get(crate);
  const base={crate,...dir===undefined?{}:{dir},checkedAtMs:context.nowMs,...context.digest===undefined?{}:{digest:context.digest},origin:context.origin};
  if(error!==undefined){
   const editor=context.editor(error.file,dir);
   result.set(crate,{...base,status:"broken",firstError:error,...editor===undefined?{}:{lastEditor:editor},brokenSinceMs:old?.status==="broken"&&old.brokenSinceMs!==undefined?old.brokenSinceMs:context.nowMs});
  }else if(check.built.has(crate))result.set(crate,{...base,status:"ok"});
  else result.set(crate,{...base,status:"blocked"});
 }
 return [...result.values()].sort((a,b)=>STATUS_ORDER[a.status]-STATUS_ORDER[b.status]||(a.crate<b.crate?-1:a.crate>b.crate?1:0)).slice(0,limits.limits.maximumRows);
}

/** 🔀️ Merges the boards of all machines into the section of the fleet status: per crate the row of the machine that checked it last, the counts, and the age of every board; boards older than the stale limit stay listed in `origins` but do not decide a crate that a fresher board knows. */
export function brokenSectionV1(boards:readonly BrokenBoardV1[],nowMs:number,limits:BrokenPolicyV1=rules):BrokenSectionV1{
 const best=new Map<string,BrokenRowV1>();
 for(const board of boards)for(const row of board.rows){
  const known=best.get(row.crate);
  if(known===undefined||(row.status!=="unchecked"&&(known.status==="unchecked"||row.checkedAtMs>known.checkedAtMs)))best.set(row.crate,{...row,origin:row.origin??board.origin});
 }
 const rows=[...best.values()].sort((a,b)=>STATUS_ORDER[a.status]-STATUS_ORDER[b.status]||(a.crate<b.crate?-1:a.crate>b.crate?1:0)).slice(0,limits.limits.maximumRows);
 const counts={ok:0,broken:0,blocked:0,unchecked:0};
 for(const row of best.values())counts[row.status]++;
 return {atMs:nowMs,origins:boards.map(board=>({origin:board.origin,ageSeconds:Math.max(0,Math.round((nowMs-board.atMs)/1000)),crates:board.rows.length})).sort((a,b)=>a.origin<b.origin?-1:1),counts,rows};
}

/** 📥️ Reads the boards of every origin from the fleet store (`<store>/v1/origins/<origin>/broken.json`); an unreadable, oversized or invalid board is skipped, never an error. */
export function brokenReadBoardsV1(store:string,limits:BrokenPolicyV1=rules):readonly BrokenBoardV1[]{
 const origins=join(store,...limits.store.directory),boards:BrokenBoardV1[]=[];let names:string[];
 try{names=readdirSync(origins);}catch{return boards;}
 for(const origin of names.sort()){
  const file=join(origins,origin,limits.store.boardFile);
  try{
   if(statSync(file).size>limits.limits.maximumBoardBytes)continue;
   const value=JSON.parse(readFileSync(file,"utf8"));
   if(validateBrokenBoardV1(value).length===0)boards.push(value as BrokenBoardV1);
  }catch{}
 }
 return boards;
}

/** 💾️ Publishes the board of this machine atomically (hidden temporary file, then rename) so a reader never sees a half board. */
export function brokenWriteBoardV1(store:string,board:BrokenBoardV1,limits:BrokenPolicyV1=rules):string{
 const problems=validateBrokenBoardV1(board);
 if(problems.length)throw Error("broken-board-invalid:"+problems[0]);
 const file=join(store,...limits.store.directory,board.origin,limits.store.boardFile),temporary=`${file}.${process.pid}.tmp`;
 mkdirSync(dirname(file),{recursive:true});
 writeFileSync(temporary,JSON.stringify(board));
 renameSync(temporary,file);
 return file;
}

/** 🖨️ Renders the section as text lines in the chosen language: the head with the counts, one line per machine board and one per broken or blocked crate with its first error, place, age and last editor. */
export function brokenFormatV1(section:BrokenSectionV1|undefined,language:"en"|"de",limit:number=40):readonly string[]{
 if(section===undefined||section.origins.length===0)return [brokenTextV1(language,"section.none")];
 const age=(ms:number):string=>{const seconds=Math.max(0,Math.round((section.atMs-ms)/1000));return seconds<90?`${seconds} s`:seconds<5400?`${Math.round(seconds/60)} min`:`${Math.round(seconds/3600)} h`;};
 const lines=[brokenTextV1(language,"section.head",{broken:section.counts.broken,blocked:section.counts.blocked,ok:section.counts.ok,unchecked:section.counts.unchecked,origins:section.origins.length})];
 for(const origin of section.origins)lines.push(brokenTextV1(language,"section.origin",{origin:origin.origin,crates:origin.crates??0,age:origin.ageSeconds}));
 let shown=0;
 for(const row of section.rows){
  if(row.status!=="broken"&&row.status!=="blocked")continue;
  if(shown++>=limit)break;
  if(row.status==="blocked"){lines.push(brokenTextV1(language,"row.blocked",{crate:row.crate}));continue;}
  const error=row.firstError,where=error?.file===undefined?"-":error.file+(error.line===undefined?"":":"+error.line),editor=row.lastEditor===undefined?brokenTextV1(language,"editor.unknown"):brokenTextV1(language,"editor.known",{origin:row.lastEditor.origin,age:age(row.lastEditor.atMs)});
  lines.push(brokenTextV1(language,"row.broken",{crate:row.crate,error:(error?.code===undefined?"":error.code+" ")+(error?.message??"?").split("\n")[0]!,where,age:row.brokenSinceMs===undefined?"?":age(row.brokenSinceMs),editor}));
 }
 return lines;
}

/** 📏️ The path of a file relative to a root when it lies below it, else unchanged. */
export const brokenRelativeV1=(root:string,path:string):string=>{const inside=relative(root,path);return inside.startsWith("..")||inside===path?path:inside;};
