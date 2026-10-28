import {spawn,spawnSync} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
const BAZEL="/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel";
export type Flags={layout:Record<string,string>;startup:string[];command:string[];environment:Record<string,string>};
export function flagsFor(tools:string,root:string,slot:string,heap:number):Flags{
 const out=spawnSync("bun",["run",join(tools,"flags.ts"),root,slot,String(heap)],{encoding:"utf8"});if(out.status!==0)throw Error(out.stderr);return JSON.parse(out.stdout);
}
export function table():Map<number,{ppid:number;rss:number;command:string}>{
 const text=spawnSync("ps",["-axo","pid=,ppid=,rss=,command="],{encoding:"utf8",maxBuffer:1<<26}).stdout,rows=new Map<number,{ppid:number;rss:number;command:string}>();
 for(const line of text.split("\n")){const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/.exec(line);if(match)rows.set(Number(match[1]),{ppid:Number(match[2]),rss:Number(match[3]),command:match[4]!});}
 return rows;
}
export function subtreeRss(root:number,rows=table()):number{
 let total=0;const stack=[root];while(stack.length){const pid=stack.pop()!,row=rows.get(pid);if(!row)continue;total+=row.rss;for(const[child,value]of rows)if(value.ppid===pid)stack.push(child);}return total;
}
export function serverPid(outputBase:string):number|undefined{
 const file=join(outputBase,"server","server.pid.txt");try{return existsSync(file)?Number(readFileSync(file,"utf8").trim()):undefined;}catch{return undefined;}
}
export type Run={wall:number;code:number;peakRssMb:number;endRssMb:number;stdout:string;log:string};
export async function bazel(flags:Flags,extraStartup:string[],cwd:string,args:string[],options:{env?:Record<string,string>;sample?:number}={}):Promise<Run>{
 const outputBase=flags.startup.find(value=>value.startsWith("--output_base="))!.slice(14),started=Date.now();
 const child=spawn(BAZEL,[...flags.startup,...extraStartup,...args],{cwd,env:{...process.env,...flags.environment,...options.env},stdio:["ignore","pipe","pipe"]});
 let stdout="",log="",peak=0,end=0;child.stdout.on("data",chunk=>{stdout+=chunk;log+=chunk;});child.stderr.on("data",chunk=>log+=chunk);
 const timer=setInterval(()=>{const pid=serverPid(outputBase);if(pid){const rss=subtreeRss(pid);peak=Math.max(peak,rss);end=rss;}},options.sample??200);
 const code=await new Promise<number>(resolve=>child.on("close",value=>resolve(value??-1)));clearInterval(timer);
 const pid=serverPid(outputBase);if(pid)end=subtreeRss(pid);
 return {wall:Date.now()-started,code,peakRssMb:Math.round(peak/1024),endRssMb:Math.round(end/1024),stdout,log};
}
export async function idleSamples(flags:Flags,seconds:number,step:number):Promise<number[]>{
 const outputBase=flags.startup.find(value=>value.startsWith("--output_base="))!.slice(14),rows:number[]=[];
 for(let waited=0;waited<=seconds;waited+=step){const pid=serverPid(outputBase);rows.push(pid?Math.round(subtreeRss(pid)/1024):0);await Bun.sleep(step*1000);}return rows;
}
export function shutdown(flags:Flags,cwd:string):void{spawnSync(BAZEL,[...flags.startup,"shutdown"],{cwd,encoding:"utf8"});}
