/** 📏 Shared helpers of the prototype benchmarks. */
import {spawn} from "node:child_process";
import {readFileSync,readdirSync,rmSync,mkdirSync} from "node:fs";
import {loadavg} from "node:os";

export const epochOffsetUs=BigInt(Date.now())*1000n-process.hrtime.bigint()/1000n;
/** 🕒 Microseconds since the epoch from a monotonic clock, comparable with the fake tool's stamps. */
export const nowUs=()=>Number(process.hrtime.bigint()/1000n+epochOffsetUs);

export function percentiles(values){
 const sorted=[...values].sort((a,b)=>a-b),at=p=>sorted[Math.min(sorted.length-1,Math.floor(p*sorted.length))];
 return{n:sorted.length,min:round(sorted[0]),p50:round(at(0.5)),p90:round(at(0.9)),p99:round(at(0.99)),max:round(sorted[sorted.length-1]),mean:round(sorted.reduce((a,b)=>a+b,0)/sorted.length)};
}
export const round=value=>Math.round(value*100)/100;
export const load=()=>round(loadavg()[0]);

/** ▶ Runs a command, resolves wall milliseconds, exit code, stdout and stderr. */
export function timed(command,args,options={}){
 return new Promise(resolve=>{
  const started=performance.now(),child=spawn(command,args,{stdio:["ignore","pipe","pipe"],...options});
  const out=[],err=[];
  child.stdout.on("data",c=>out.push(c));child.stderr.on("data",c=>err.push(c));
  child.on("exit",code=>resolve({ms:performance.now()-started,code,stdout:Buffer.concat(out).toString(),stderr:Buffer.concat(err).toString()}));
 });
}

export function readStamps(directory){
 const out={};
 try{for(const name of readdirSync(directory))if(name.endsWith(".start"))out[name.slice(0,-6)]=Number(readFileSync(directory+"/"+name,"utf8"));}catch{}
 return out;
}
export function clearStamps(directory){rmSync(directory,{recursive:true,force:true});mkdirSync(directory,{recursive:true});}

export function closureOf(graph,name){
 const byName=new Map(graph.owners.map(owner=>[owner.name,owner])),seen=new Set([name]),order=[name];
 for(let at=0;at<order.length;at++)for(const dep of byName.get(order[at]).deps)if(!seen.has(dep)){seen.add(dep);order.push(dep);}
 return order;
}
