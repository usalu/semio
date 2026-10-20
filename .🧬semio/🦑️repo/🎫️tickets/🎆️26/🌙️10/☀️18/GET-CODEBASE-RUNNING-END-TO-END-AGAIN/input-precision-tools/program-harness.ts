// 🧪 Runs the real program analysis twice over a real tsconfig: cold (receipt written) and warm (nothing derived), compares both with each other and with `tsc --listFilesOnly`, and reports the time and receipt size.
import {readFileSync,rmSync,statSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {programInputsV1,readInputPrecisionV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {realHost,W} from "./real-host.ts";
import {programHost} from "./program-host.ts";
const tsconfig=process.argv[2]!,data=join("/private/tmp/ip/data","h-"+tsconfig.replaceAll("/","_").slice(-30));
if(!process.env.IP_WARM_ONLY)rmSync(data,{recursive:true,force:true});
const document=readInputPrecisionV1(JSON.parse(readFileSync(join(import.meta.dir,"staged2","rules.json"),"utf8")));
const {host}=await realHost(""),events:string[]=[];
const run=async(label:string)=>{
 let derived=0;const program=await programHost({data,document,report:event=>events.push(`${label}:${event.phase}`)});
 const counted={...program,imports:async(file:string,config:string)=>{derived++;return program.imports(file,config);}} as typeof program;
 Object.defineProperty(counted,"compiler",{get:()=>program.compiler});
 const t0=performance.now(),result=await programInputsV1(tsconfig,{...host,program:counted});
 return {label,derived,seconds:Math.round((performance.now()-t0)/100)/10,result};
};
const cold=process.env.IP_WARM_ONLY?await run("warm"):await run("cold"),warm=await run("warm");
const files=(r:any)=>JSON.stringify(r.result);
const run2=Bun.spawnSync([process.execPath,join(W,"node_modules/typescript/bin/tsc"),"-p",join(W,tsconfig),"--listFilesOnly"],{cwd:W,maxBuffer:512*1024*1024});
const listed=run2.stdout.toString().split("\n").filter(Boolean).filter(p=>p.startsWith(W+"/")&&!p.includes("/node_modules/")).map(p=>p.slice(W.length+1));
const declared=new Set([...cold.result.roots,...cold.result.files]),patterns=cold.result.patterns.filter(p=>!p.startsWith("!")),missing=listed.filter(p=>!declared.has(p)&&!patterns.length);
const size=readdirSync(join(data,"ip")).map(n=>statSync(join(data,"ip",n)).size);
console.log(JSON.stringify({tsconfig,cold:{derived:cold.derived,seconds:cold.seconds},warm:{derived:warm.derived,seconds:warm.seconds},equal:files(cold)===files(warm),tscFiles:listed.length,engineFiles:cold.result.files.length,receiptBytes:size,events}));
process.exit(0);
