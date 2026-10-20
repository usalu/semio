import {appendFileSync} from "node:fs";
import {createRequire} from "node:module";
const log=process.env.LAW_GUARD_LOG??"/dev/null",self=import.meta.path,subcommands=new Set(["test","run","x","build","install","pm","add","remove","repl","create","init","upgrade"]);
const flat=(a:unknown):string=>Array.isArray(a)?a.map(String).join(" "):String(a);
const heavy=/^(cargo|rustc|rustup|nx|bazel|bazelisk|playwright|vitest|nextest|docker|npm|npx|pnpm|yarn)$/;
const check=(kind:string,words:string[])=>{
 const names=words.map(w=>w.split("/").pop()!);
 const text=words.join(" ");
 if(names.some(w=>heavy.test(w))||words.some(w=>w.includes("🦀️cargo"))||names.includes("relay")||(names.includes("repo")&&names.some(w=>/^(run|cargo|run-many|dependencies|show|affected)$/.test(w)))){appendFileSync(log,`${kind}\t${text.slice(0,300)}\n`);throw new Error(`[GUARD] blocked ${kind}: ${text.slice(0,200)}`);}
};
const isBun=(c:string)=>c===process.execPath||c==="bun"||c.endsWith("/bun");
const inject=(words:string[]):string[]=>{
 if(!words.length||!isBun(words[0]!))return words;
 const next=words[1];
 if(next===undefined||next.startsWith("--preload"))return words;
 if(subcommands.has(next))return [words[0]!,next,"--preload",self,...words.slice(2)];
 return [words[0]!,"--preload",self,...words.slice(1)];
};
const bunSpawn=Bun.spawn.bind(Bun),bunSpawnSync=Bun.spawnSync.bind(Bun);
const wrapBun=(kind:string,original:any)=>(cmd:any,...r:any[])=>{
 if(Array.isArray(cmd)){const words=cmd.map(String);check(kind,words);return original(inject(words),...r);}
 const words=(cmd?.cmd??[]).map(String);check(kind,words);return original({...cmd,cmd:inject(words)},...r);
};
(Bun as any).spawn=wrapBun("Bun.spawn",bunSpawn);
(Bun as any).spawnSync=wrapBun("Bun.spawnSync",bunSpawnSync);
const cp=createRequire(import.meta.url)("node:child_process");
for(const name of ["spawn","spawnSync","exec","execSync","execFile","execFileSync","fork"]){
 const original=cp[name];
 cp[name]=function(command:any,args?:any,...rest:any[]){
  const shell=name==="exec"||name==="execSync";
  const words=shell?String(command).split(/\s+/):[String(command),...(Array.isArray(args)?args.map(String):[])];
  check("cp."+name,words);
  if(!shell&&name!=="fork"&&Array.isArray(args)){const w=inject(words);return original.call(this,w[0],w.slice(1),...rest);}
  return original.call(this,command,args,...rest);
 };
}
