import {spawn} from "node:child_process";
import {createWriteStream} from "node:fs";
import {createInterface} from "node:readline";

/** ⏱️ Prefixes every output line of a command with elapsed milliseconds and prints a per-phase table of its `[script]` progress events.
 * usage: bun graphtime.ts <out.log> <command...>      (a sibling of x-ingress-tools/timed.ts keyed on the JSON events, not the human lines)
 */
const [output,...command]=process.argv.slice(2);
if(!output||command.length===0){console.error("usage: bun graphtime.ts <out.log> <command...>");process.exit(2);}
const started=performance.now(),sink=createWriteStream(output),rows:{at:number;line:string}[]=[];
const child=spawn(command[0]!,command.slice(1),{stdio:["ignore","pipe","pipe"],env:process.env});
const take=(stream:NodeJS.ReadableStream)=>createInterface({input:stream}).on("line",line=>{const at=Math.round(performance.now()-started);rows.push({at,line});sink.write(String(at).padStart(8)+" "+line+"\n");});
take(child.stdout);take(child.stderr);
child.on("close",code=>{
 const total=Math.round(performance.now()-started),phases=new Map<string,{first:number;last:number;count:number}>(),waits:string[]=[];
 for(const {at,line} of rows){
  const event=/"command":"([^"]+)","stage":"([^"]+)"/u.exec(line);
  if(event){const row=phases.get(event[1]!)??{first:at,last:at,count:0};row.last=at;row.count++;phases.set(event[1]!,row);}
  const wait=/Waiting for access to (\S+) \((\w+)\)/u.exec(line);if(wait)waits.push(at+" ms "+wait[1]+" "+wait[2]);
 }
 const observations=phases.get("build:selected-declaration")?.count??0,records=phases.get("build:selected-records")?.count??0;
 const table=["phase\tfirst_ms\tlast_ms\tspan_ms\tevents",...[...phases].map(([name,row])=>[name,row.first,row.last,row.last-row.first,row.count].join("\t")),...waits.slice(0,12).map(value=>"WAIT "+value),"owner observations computed (selected-declaration)\t"+observations,"owner records consulted (selected-records)\t"+records,"TOTAL\t0\t"+total+"\t"+total+"\t"+rows.length].join("\n");
 sink.write("\n"+table+"\nexit "+code+"\n");sink.end();process.stdout.write(table+"\nexit "+code+"\n");process.exit(code??1);
});
process.on("SIGINT",()=>child.kill("SIGINT"));process.on("SIGTERM",()=>child.kill("SIGTERM"));
