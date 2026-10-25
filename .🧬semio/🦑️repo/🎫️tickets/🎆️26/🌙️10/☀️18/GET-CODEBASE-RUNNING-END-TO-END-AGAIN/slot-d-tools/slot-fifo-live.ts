import {spawn,type ChildProcess} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const root=process.cwd(),out=process.argv[2]!,tools=join(root,process.argv[3]??".");
const bazel=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"),port_=await import(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"));
const record=(row:Record<string,unknown>)=>writeFileSync(out,JSON.stringify({t:Date.now(),...row})+"\n",{flag:"a"});
const sleep=(ms:number)=>new Promise(done=>setTimeout(done,ms));
const base=mkdtempSync(join(realpathSync(tmpdir()),"slot-fifo-")),directories={leases:join(base,"leases"),server:join(base,"p/server"),workspaces:join(base,"p/workspaces")};
for(const dir of Object.values(directories))mkdirSync(dir,{recursive:true});
const children=new Map<string,ChildProcess>(),waiter=join(tools,"slot-fifo-waiter.ts"),keyOf=(index:number)=>String(index).padStart(16,"0");
const launch=(name:string,mode:string,key:string)=>{
 const child=spawn(process.execPath,[waiter,JSON.stringify(directories),name,mode,key],{cwd:root,stdio:["pipe","pipe","inherit"]});children.set(name,child);child.stdout.setEncoding("utf8");
 child.stdout.on("data",(text:string)=>{for(const line of text.split("\n").filter(Boolean))writeFileSync(out,line+"\n",{flag:"a"});});
 child.on("exit",(code,signal)=>record({name,type:"exit",code,signal}));return child;
};
const holder=(slot:string)=>new Promise<ChildProcess>((resolve,reject)=>{
 const script=join(base,`holder-${slot}.ts`);
 writeFileSync(script,`import {bazelSlotPortV1} from ${JSON.stringify(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim(${JSON.stringify(slot)},new AbortController().signal);console.log(claim?"held":"busy");if(claim){await new Promise(done=>process.stdin.on("close",done).resume());claim();}`);
 const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]});child.stdout.setEncoding("utf8");child.stdout.on("data",(text:string)=>text.includes("held")?resolve(child):reject(Error("busy")));child.on("error",reject);children.set("holder-"+slot,child);
});
const port=port_.bazelSlotPortV1(directories,{write:()=>undefined});
const probe=async(phase:string)=>{const started=Date.now();let ok=false;try{const release=await port.gate(AbortSignal.timeout(1000),"build");release();ok=true;}catch{}record({type:"probe",phase,ok,latencyMs:Date.now()-started,waiterFiles:readdirSync(directories.server).filter(name=>name.startsWith("waiter-")).length});};
const probing=async(phase:string,ms:number)=>{const end=Date.now()+ms;while(Date.now()<end){await probe(phase);await sleep(300);}};
let exit=1;
try{
 record({type:"phase",phase:"control",note:"old behaviour: a run that waits for a slot holds the picker exclusively"});
 launch("old","old","");await sleep(1500);await probing("control",5000);children.get("old")!.stdin!.end();await sleep(800);
 record({type:"phase",phase:"fifo",note:"production port: s0 and s1 held by two real processes, four real waiters arrive 1.5 s apart"});
 await holder("s0");await holder("s1");
 launch("W1","real",keyOf(1));
 const stepped=async(ms:number)=>{await probing("fifo",ms);};
 await stepped(1500);launch("W4","real",keyOf(4));
 await stepped(1500);launch("W2","real",keyOf(2));
 await stepped(1500);launch("W3","real",keyOf(3));
 await stepped(1500);record({type:"action",action:"kill W4 (a waiter that has not claimed anything)"});children.get("W4")!.kill("SIGKILL");
 await stepped(2500);record({type:"action",action:"SIGKILL the holder of s0 (the slot must be freed by the kernel)"});children.get("holder-s0")!.kill("SIGKILL");
 await stepped(3000);record({type:"action",action:"release s1 gracefully"});children.get("holder-s1")!.stdin!.end();
 await stepped(3000);record({type:"action",action:"W1 finishes its session"});children.get("W1")!.stdin!.end();
 await stepped(3000);for(const name of ["W2","W3"])children.get(name)!.stdin!.end();await sleep(1500);
 record({type:"end",waiterFilesLeft:readdirSync(directories.server).filter(name=>name.startsWith("waiter-")).length});exit=0;
}finally{
 for(const child of children.values())try{child.kill("SIGKILL");}catch{}
 await sleep(500);if(existsSync(base))rmSync(base,{recursive:true,force:true});
}
process.exit(exit);
