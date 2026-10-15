import {createReadStream} from "node:fs";
import {writeFile} from "node:fs/promises";
import {createInterface} from "node:readline";
import {join,resolve} from "node:path";

const ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated"),name=process.argv[2];
if(process.argv.length!==3||!/^final-all-current[1-5]?\.log$/u.test(name??""))throw Error("Expected an owned retained complete-producer log");
const phases=new Map<string,{count:number,milliseconds:number,maximum:number}>(),longest:{phase:string,milliseconds:number,elapsed:number}[]=[],signal=AbortSignal.timeout(30000),input=createReadStream(join(output,name!),{encoding:"utf8",signal}),lines=createInterface({input,crlfDelay:Infinity});
let records=0,last:any,deadline=false;
for await(const line of lines){
 if(signal.aborted)throw signal.reason;
 if(line.includes("Original Script process deadline exhausted"))deadline=true;
 if(!line.startsWith("[DEBUG] graph-qualification "))continue;
 const row=JSON.parse(line.slice("[DEBUG] graph-qualification ".length));records++;last=row;
 if(row.reason!=="phase-complete")continue;
 const value=phases.get(row.phase)??{count:0,milliseconds:0,maximum:0};value.count++;value.milliseconds+=row.phaseMilliseconds;value.maximum=Math.max(value.maximum,row.phaseMilliseconds);phases.set(row.phase,value);longest.push({phase:row.phase,milliseconds:row.phaseMilliseconds,elapsed:row.elapsedMilliseconds});
}
const aggregate=[...phases].map(([phase,value])=>({phase,...value})).sort((a,b)=>b.milliseconds-a.milliseconds),top=longest.sort((a,b)=>b.milliseconds-a.milliseconds).slice(0,15),result={scope:"retained-producer-phase-intervals",log:name,records,deadline,aggregate,top,last};
await writeFile(join(output,"all-phase-audit.json"),JSON.stringify(result,null,2)+"\n");
const rows=aggregate.slice(0,15).map(row=>`| ${row.phase} | ${row.count} | ${row.milliseconds} | ${row.maximum} |`).join("\n"),report=`# Retained Complete Producer Phase Intervals\n\nRead-only analysis of ${name}, ${records} recorded phase events. The original deadline marker is ${deadline?"present":"absent"}. These are elapsed intervals between progress phase transitions, not CPU profiling or per-owner attribution. Active waits, asynchronous work, source settlement and progress emission can share an interval. Final incomplete phase time is excluded from aggregate completed intervals. No Source, leases, caches or peer processes were changed.\n\n| Phase | Completed Intervals | Total ms | Maximum ms |\n| --- | --- | --- | --- |\n${rows}\n\nLast recorded phase: ${last?.phase}; elapsed ${last?.elapsedMilliseconds}ms; duration ${last?.phaseMilliseconds}ms. Recorded observations: ${JSON.stringify(last?.observations)}. Counts: ${JSON.stringify(last?.counts)}. The exact inner source-conflict identity is unavailable because authority tracing was omitted. No accepted complete publication follows from these diagnostic records.\n`;
await writeFile(join(ticket,"all-phase-audit.md"),report);console.log("[DEBUG] Retained producer phase intervals "+JSON.stringify({records,deadline,top:aggregate.slice(0,5),lastElapsed:last?.elapsedMilliseconds}));
