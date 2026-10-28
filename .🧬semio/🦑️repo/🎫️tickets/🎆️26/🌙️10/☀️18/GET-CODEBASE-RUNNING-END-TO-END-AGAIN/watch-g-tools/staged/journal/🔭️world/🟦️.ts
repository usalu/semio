import {lstat} from "node:fs/promises";
import {dirname,relative,sep} from "node:path";
import {changedWatchedPathsV1,type ChangeCursorV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";

/** 🧾️ One fact of a computation's world: kind letter, subject (an absolute path for the path kinds) and the value it had. */
export type WorldRowV1=readonly[kind:string,subject:string,value:string];
/** ✅️ The verdict of the stat-based check this module only narrows. */
export type WorldVerdictV1=Readonly<{holds:true;checked:number}>|Readonly<{holds:false;checked:number;kind:string;subject:string}>;
/** 🔭️ How a world check was answered and what it cost in file system questions. */
export type WorldJournalVerdictV1=Readonly<{verdict:WorldVerdictV1;via:"journal"|"stat";journalled:number;statted:number;cursor?:ChangeCursorV1;refusal?:string}>;

const PATH_KINDS=/^(?:[lLsSdfrp]|a\d*)$/u;
/** 🔎️ Whether the kind observes a path, so that a change event of that path decides it. */
export const worldRowIsPathV1=(kind:string):boolean=>PATH_KINDS.test(kind);

/** 🔗️ The directories above the subjects that are symbolic links (workspace-relative): changes made through a link are reported at the link's target, so nothing below such a directory may be vouched for by path. Computed when the world is recorded; one `lstat` per distinct directory. */
export async function symlinkedDirectoriesV1(workspace:string,subjects:Iterable<string>):Promise<readonly string[]>{
 const directories=new Set<string>();
 for(const subject of subjects){for(let at=dirname(subject);at.length>workspace.length&&!directories.has(at);at=dirname(at))directories.add(at);}
 const linked:string[]=[];
 await Promise.all([...directories].map(async directory=>{try{if((await lstat(directory)).isSymbolicLink())linked.push(relative(workspace,directory).split(sep).join("/"));}catch{}}));
 return linked.sort();
}

/** 🧭️ Checks a recorded world through the change journal: rows whose path the journal proves untouched since `cursor` hold without a file system question; rows it reports, rows outside the workspace, in a region the watcher does not report or below a symbolic link, and rows that do not observe a path go to `stat`. When the journal cannot vouch every row goes to `stat`. The answer is the one `stat` alone would give; the new cursor is offered when the world holds. */
export async function checkWorldViaJournalV1(options:Readonly<{
 client:ChangeJournalClientV1;cursor:ChangeCursorV1|undefined;workspace:string;rows:readonly WorldRowV1[];linked?:readonly string[];insensitive?:boolean;
 stat:(rows:readonly WorldRowV1[])=>Promise<WorldVerdictV1>;signal?:AbortSignal;
}>):Promise<WorldJournalVerdictV1>{
 const {client,cursor,workspace,rows,stat,signal}=options,insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32"),linked=options.linked??[];
 const fallback=async(refusal:string):Promise<WorldJournalVerdictV1>=>({verdict:await stat(rows),via:"stat",journalled:0,statted:rows.length,refusal});
 if(!cursor)return fallback("cursor");
 const excluded=client.excluder(),prefix=workspace.endsWith(sep)?workspace:workspace+sep,insides:(string|undefined)[]=new Array(rows.length),kinds=new Map<string,boolean>(),rest:WorldRowV1[]=[],watched=new Set<string>();
 const linkedDirectories=linked.map(link=>link+"/");
 for(let at=0;at<rows.length;at++){
  const row=rows[at]!,subject=row[1];
  let path=kinds.get(row[0]);if(path===undefined){path=worldRowIsPathV1(row[0]);kinds.set(row[0],path);}
  if(!path||!subject.startsWith(prefix)){rest.push(row);continue;}
  const inside=sep==="/"?subject.slice(prefix.length):subject.slice(prefix.length).split(sep).join("/");
  if(inside===""||excluded(inside)||linked.includes(inside)||linkedDirectories.some(link=>inside.startsWith(link))){rest.push(row);continue;}
  insides[at]=inside;watched.add(inside);
 }
 const answer=await client.since(cursor,[""],signal);
 if(!answer.exact)return fallback(answer.reason);
 const hits=changedWatchedPathsV1(watched,answer.rows,insensitive),hot:WorldRowV1[]=[];
 let journalled=0;
 for(let at=0;at<rows.length;at++){const inside=insides[at];if(inside===undefined)continue;if(hits.has(inside))hot.push(rows[at]!);else journalled++;}
 const check=[...rest,...hot],verdict=await stat(check);
 return{verdict:verdict.holds?{holds:true,checked:verdict.checked+journalled}:verdict,via:"journal",journalled,statted:check.length,cursor:verdict.holds?answer.cursor:undefined};
}
