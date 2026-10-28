import {appendFileSync,fstatSync} from "node:fs";
import {spawnSync,spawn} from "node:child_process";
const rows:string[]=[];
for(let fd=0;fd<24;fd++){try{const s=fstatSync(fd);rows.push(fd+":"+(s.isFIFO()?"fifo":s.isSocket()?"sock":s.isCharacterDevice()?"chr":s.isFile()?"file":"other"));}catch{}}
const log=(text:string)=>appendFileSync(process.env.FDS_LOG!,text+"\n");
const inner=process.argv[2]!,args=process.argv.slice(3);
const variants:Record<string,Array<"inherit"|"pipe"|"ignore"|number>>={
 A:["inherit","inherit","inherit",3,"ignore",5],
 B:["inherit","inherit","inherit",3,4,5],
 C:["inherit","inherit","inherit"],
 D:["inherit","inherit","inherit","ignore","ignore","ignore"],
 E:["inherit","pipe","pipe",3,"ignore",5],
 F:["inherit","pipe","pipe"],
 G:["inherit","pipe","pipe",3,4,5],
 H:["inherit","pipe","pipe","ignore","ignore","ignore"],
};
if(args[0]!=="--version"){process.exit(spawnSync(inner,args,{stdio:"inherit"}).status??1);}
log(rows.join(" ")+" | "+(process.env.CARGO_MAKEFLAGS??""));
process.stdout.write("rustc 0.0.0-fake\n");
for(const [name,stdio] of Object.entries(variants)){
 try{const r=spawnSync(inner,args,{stdio,env:process.env as NodeJS.ProcessEnv});log(name+" status="+r.status+" error="+(r.error?.message??""));}catch(e){log(name+" threw "+(e as Error).message);}
}
