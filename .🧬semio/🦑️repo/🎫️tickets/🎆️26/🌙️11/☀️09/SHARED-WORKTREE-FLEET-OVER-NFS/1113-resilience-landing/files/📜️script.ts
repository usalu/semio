#!/usr/bin/env repo
import {existsSync,readdirSync,readFileSync,realpathSync} from "node:fs";
import {join,relative,resolve} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

type Check=(root:string,signal?:AbortSignal)=>Promise<{ok:boolean;error:string|null}>;
type Run=(args:readonly string[],context:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;write:(line:string)=>void;notice:(line:string)=>void;readStdin?:()=>Promise<string>}>)=>Promise<{exitCode:number}>;

const snapshotsOf=(root:string):string[]=>{
 const base=join(root,".cache","f","code"),found:{name:string;createdMs:number}[]=[];
 try{for(const entry of readdirSync(base,{withFileTypes:true}))if(entry.isDirectory()&&existsSync(join(base,entry.name,"sealed.json"))){try{found.push({name:entry.name,createdMs:Number((JSON.parse(readFileSync(join(base,entry.name,"sealed.json"),"utf8")) as {createdMs:number}).createdMs)||0});}catch{}}}catch{}
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.name<right.name?-1:1)).map(item=>join(base,item.name,"files"));
};

/** ⌨️ Reads standard input to the end; empty when standard input is a terminal. */
const readStandardInput=async():Promise<string>=>{
 if(process.stdin.isTTY)return "";
 const chunks:Buffer[]=[];
 for await(const chunk of process.stdin)chunks.push(Buffer.from(chunk as Uint8Array));
 return Buffer.concat(chunks).toString("utf8");
};

/** 📜️ Runs one replica command against a repository root with explicit sinks: stdout lines and stderr lines; returns the process exit code. This file is the stable entry the service managers run, so it imports nothing but Node built-ins and keeps every other module behind a dynamic import: the code of the worktree is loaded and checked first (all modules load, schemas compile, texts complete); when that fails, the newest sealed snapshot below `.cache/f/code/` that loads and checks is run instead and keeps the machine replicating, so the fix can arrive and the next restart picks it up. `self-check` never falls back: it judges the worktree code. */
export async function runReplicaScriptV1(args:readonly string[],root:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,write:(line:string)=>void,notice:(line:string)=>void,readStdin?:()=>Promise<string>):Promise<number>{
 const module=resolve(import.meta.dirname,".."),inside=relative(root,module),strict=args[0]==="self-check";
 let first=true;
 for(const base of strict?[root]:[root,...snapshotsOf(root)]){
  let run:Run;
  try{
   const directory=join(base,inside),load=async(...path:string[]):Promise<Record<string,unknown>>=>await import(pathToFileURL(join(directory,...path)).href) as Record<string,unknown>;
   run=(await load("🔁️replica","⌨️command","🟦️.ts")).runReplicaCommandV1 as Run;
   const report=await ((await load("🛡️resilience","🟦️.ts")).selfCheckV1 as Check)(directory,signal);
   if(!report.ok)throw Error(report.error??"self-check failed");
  }catch(error){
   if(!strict)notice(`${first?"The code of the worktree":`The sealed code ${base}`} does not start (${error instanceof Error?error.message:String(error)}) / Der Code ${first?"des Arbeitsverzeichnisses":`des Snapshots ${base}`} startet nicht (${error instanceof Error?error.message:String(error)})`);
   else if(first)notice(error instanceof Error?error.message:String(error));
   first=false;
   continue;
  }
  if(base!==root)notice(`Running the sealed code ${base} until the worktree code starts again / Der versiegelte Code ${base} läuft, bis der Code des Arbeitsverzeichnisses wieder startet`);
  try{return (await run(args,{root,environment:base===root?environment:{...environment,SEMIO_FLEET_CODE_WATCH:module},signal,write,notice,readStdin})).exitCode;}
  catch(error){notice(error instanceof Error?error.message:String(error));return 1;}
 }
 notice("No replica code starts: neither the worktree nor a sealed snapshot / Kein Replica-Code startet: weder das Arbeitsverzeichnis noch ein versiegelter Snapshot");
 return 1;
}

const entry=process.argv[1],canonical=(path:string):string=>{const real=realpathSync(path);return process.platform==="win32"?real.toLowerCase():real;};
if(entry!==undefined&&entry!==""&&(()=>{try{return canonical(entry)===canonical(fileURLToPath(import.meta.url));}catch{return false;}})()){
 const controller=new AbortController(),stop=():void=>{controller.abort();},signals:NodeJS.Signals[]=process.platform==="win32"?["SIGINT","SIGTERM","SIGBREAK","SIGHUP"]:["SIGINT","SIGTERM"];
 for(const name of signals)process.once(name,stop);
 process.exitCode=await runReplicaScriptV1(process.argv.slice(2),resolve(import.meta.dirname,"../../../../../.."),process.env,controller.signal,line=>{process.stdout.write(`${line}\n`);},line=>{process.stderr.write(`${line}\n`);});
 for(const name of signals)process.removeListener(name,stop);
}
