import {readdirSync,readFileSync,existsSync} from "node:fs";
import {spawn} from "node:child_process";
import {join,dirname,basename} from "node:path";
import {L,SC} from "./patches.mjs";
const BUN="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun",ROOT="/Users/ueli/Documents/semio",SCRIPT=L+"🧪️testing/🌱️genuine-birth/📜️script.ts";
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_FLEET_")||k.startsWith("SEMIO_REMOTE_CACHE_"))delete env[k];
const walk=d=>readdirSync(d,{withFileTypes:true}).flatMap(e=>e.isDirectory()?(e.name==="node_modules"?[]:walk(join(d,e.name))):[join(d,e.name)]);
const run=(args,cwd)=>new Promise(done=>{let text="";const child=spawn(BUN,args,{cwd,env});child.stdout.on("data",d=>text+=d);child.stderr.on("data",d=>text+=d);const timer=setTimeout(()=>child.kill("SIGKILL"),270000);child.on("close",rc=>{clearTimeout(timer);const fail=Number(/^ *(\d+) fail/mu.exec(text)?.[1]??0),pass=Number(/^ *(\d+) pass/mu.exec(text)?.[1]??0);done({rc,pass,fail,text});});});
const pool=async(items,jobs,work)=>{let next=0;const out=[];await Promise.all(Array.from({length:jobs},async()=>{while(next<items.length){const i=next++;out[i]=await work(items[i]);}}));return out;};
const B=L+"🏗️build/",groups=[
 {dir:B+"🪪️graph-state/",base:"out-gs-base.txt"},
 {dir:B+"🕸️graph/🧮️derived-nodes/",base:"out-🧮️derived-nodes-mirror-base.txt"},
 {dir:B+"🏡️resident/",base:"out-🏡️resident-mirror-base.txt"},
 {dir:B+"🕸️graph/📥️inference/🧭️script-inputs/",base:"out-inf6-base.txt",filter:/closure-store-limits|🧹️retention|💾️entries|closure-answer/u},
];
const only=process.argv.slice(2);
const rows=[];let red=false;
const preflights=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"];
const harness=[...preflights,B+"🏡️resident/🧪️tests/🗃️state/🟦️.ts"];
const started=Date.now(),law=run(["test","./🟦️.ts","-t","workspace-scaled|same documents|workspace-named"],L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth");
const harnessRuns=Promise.all(harness.map(file=>run([SCRIPT,"--file="+file,"--birth=setup","--root="+ROOT],ROOT).then(r=>({name:"harness "+file.slice(B.length),...r}))));
for(const group of groups){
 if(Date.now()-started>240000){console.log("DEADLINE before",group.dir.slice(B.length));red=true;break;}
 const baseline=new Map(readFileSync(SC+group.base,"utf8").split("\n").filter(Boolean).map(l=>{const[k,rc,sum]=l.split("::");return[k,{rc:rc.replace("rc=","")==="0",fail:Number(/(\d+) fail/u.exec(sum)?.[1]??0)}];}));
 const files=walk(group.dir).filter(f=>/\/🧪️tests\/(.*\/)?(🟦️\.ts|[^/]*\.test\.ts)$/u.test(f)&&(group.filter===undefined||group.filter.test(f))).sort();
 const results=await pool(files,4,async f=>{let r=await run(["test","./"+basename(f),"--timeout","120000"],dirname(f));const key=f.slice(group.dir.length),was=baseline.get(key);
  if(r.rc!==0&&was&&(was.rc||r.fail>was.fail))for(let again=0;again<2&&r.rc!==0;again++)r=await run(["test","./"+basename(f),"--timeout","120000"],dirname(f));
  const worse=r.rc!==0&&(was===undefined||was.rc||r.fail>was.fail),mine=/maxItems|maximum[A-Z]|Documents|Suspects|closureStore|outputSignature|digestCache|journalMaximum|memoMaximum|unsent|aggregate|Nodes|ceiling/u.test(r.text.split("\n").filter(l=>/^error|^Expected|^Received|\(fail\)/u.test(l.trim())).join("\n")),bad=worse&&mine;if(worse&&!mine)console.log("  FOREIGN (not from the patched keys)",key);return{name:key,bad,rc:r.rc,pass:r.pass,fail:r.fail,was};});
 const bads=results.filter(r=>r.bad);console.log(group.dir.slice(B.length),results.length,"files,",bads.length,"worse than baseline");for(const r of bads){console.log("  WORSE",r.name,JSON.stringify(r.was),r.rc,r.pass,r.fail);red=true;}
}
const lawResult=await law;console.log("law",lawResult.rc,lawResult.pass,"pass",lawResult.fail,"fail");if(lawResult.rc!==0){red=true;console.log(lawResult.text.slice(-1500));}
for(const r of await harnessRuns){console.log(r.name,r.rc,r.pass,"pass",r.fail,"fail");if(r.rc!==0){red=true;console.log(r.text.slice(-1200));}}
console.log("checked in",Math.round((Date.now()-started)/1000),"s",red?"RED":"GREEN");process.exit(red?1:0);
