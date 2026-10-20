import {spawn} from "node:child_process";
import {createWriteStream} from "node:fs";
import {createInterface} from "node:readline";

const [output,...command]=process.argv.slice(2);
if(!output||command.length===0){console.error("usage: bun timed.ts <out.log> <command...>");process.exit(2);}

/** ⏱️ Prefixes every output line of a command with elapsed milliseconds and prints a per-phase table. */
const started=performance.now(),sink=createWriteStream(output),rows:{at:number;line:string}[]=[];
const child=spawn(command[0]!,command.slice(1),{stdio:["ignore","pipe","pipe"],env:process.env});
const take=(stream:NodeJS.ReadableStream)=>createInterface({input:stream}).on("line",line=>{
  const at=Math.round(performance.now()-started);rows.push({at,line});
  const text=String(at).padStart(8)+" "+line;sink.write(text+"\n");process.stdout.write(text.slice(0,240)+"\n");
});
take(child.stdout);take(child.stderr);

child.on("close",code=>{
  const total=Math.round(performance.now()-started);
  const phases=new Map<string,{first:number;last:number;count:number;max:number}>();
  const wait=new Map<string,{first:number;last:number}>();
  for(const {at,line} of rows){
    const progress=/repository:command · ([^:]+): (\d+) (operations|edges)/u.exec(line);
    if(progress){
      const row=phases.get(progress[1]!)??{first:at,last:at,count:0,max:0};
      row.last=at;row.count++;row.max=Math.max(row.max,Number(progress[2]));phases.set(progress[1]!,row);
    }
    const queued=/Waiting for access to (\S+) \((\w+)\)/u.exec(line);
    if(queued){const key=queued[1]+" "+queued[2],row=wait.get(key)??{first:at,last:at};row.last=at;wait.set(key,row);}
  }
  const table=["phase\tfirst_ms\tlast_ms\tspan_ms\tlines\tmax_counter",...[...phases].map(([name,row])=>[name,row.first,row.last,row.last-row.first,row.count,row.max].join("\t")),...[...wait].map(([name,row])=>["WAIT "+name,row.first,row.last,row.last-row.first,"",""].join("\t")),["TOTAL",0,total,total,rows.length,""].join("\t")].join("\n");
  sink.write("\n"+table+"\nexit "+code+"\n");sink.end();process.stdout.write("\n"+table+"\nexit "+code+"\n");
  process.exit(code??1);
});
process.on("SIGINT",()=>child.kill("SIGINT"));
process.on("SIGTERM",()=>child.kill("SIGTERM"));
