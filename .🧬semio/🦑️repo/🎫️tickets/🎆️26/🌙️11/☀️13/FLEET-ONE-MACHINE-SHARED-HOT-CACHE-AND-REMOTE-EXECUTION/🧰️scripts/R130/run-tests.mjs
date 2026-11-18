import {readFileSync,readdirSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join,basename} from "node:path";
const BUN="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun";
const [stage,out]=process.argv.slice(2),tests=JSON.parse(readFileSync(join(stage,"tests.json"),"utf8")),lines=[];
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_FLEET_")||k.startsWith("SEMIO_REMOTE_CACHE_"))delete env[k];
for(const{dir,target}of tests){
 for(const f of readdirSync(target).filter(n=>n.endsWith(".ts"))){
  const r=spawnSync(BUN,["test","./"+f,"--timeout","180000"],{cwd:target,env,encoding:"utf8",maxBuffer:1<<27}),text=(r.stdout??"")+(r.stderr??"");
  const sum=text.split("\n").filter(l=>/^ *\d+ (pass|fail)/u.test(l)).map(l=>l.trim()).join(" ");
  lines.push(dir.split("/🏗️build/")[1]+"/"+f+"::rc="+r.status+"::"+sum);console.log(lines.at(-1));
  if(r.status!==0)writeFileSync(join(stage,"fail-"+lines.length+".txt"),text);
 }
}
writeFileSync(out,lines.join("\n")+"\n");
