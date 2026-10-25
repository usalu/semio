import{createHash}from"node:crypto";
import{mkdirSync,writeFileSync,readFileSync,readdirSync,rmSync,copyFileSync}from"node:fs";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
const [loaderFile,work]=process.argv.slice(2) as [string,string];
rmSync(work,{recursive:true,force:true});mkdirSync(join(work,"root/sub"),{recursive:true});
const root=join(work,"root"),hash=(b:string|Uint8Array)=>createHash("sha256").update(b).digest("hex");
const files:Record<string,string>={
 "entry.ts":`import{helper}from"./sub/helper.ts";import data from"./data.json";import{readFileSync}from"node:fs";export async function runCapturedBuildSourceV1(args:string[]){const trace=(globalThis as any)[Symbol.for("semio.sourceTrace.v1")];if(process.env.TRACE_DIR)trace?.enable(process.env.TRACE_DIR);trace?.record("a",JSON.stringify({id:"smoke"}));console.log("ran",helper(),data.v,readFileSync(new URL("./sub/note.txt",import.meta.url),"utf8"),readFileSync(new URL("./live.txt",import.meta.url),"utf8").trim(),args.join(","));const lazy=await import("./sub/lazy.ts");console.log(lazy.lazy);}`,
 "sub/helper.ts":`export const helper=()=>"helped";`,
 "sub/lazy.ts":`export const lazy="lazy-loaded";`,
 "sub/note.txt":"note",
 "data.json":`{"v":7}`,
};
writeFileSync(join(root,"live.txt"),"live\n");
const rows=Object.entries(files).sort(([a],[b])=>a<b?-1:1).map(([path,text])=>{writeFileSync(join(root,path),text);return{path,sha256:hash(text),base64:Buffer.from(text).toString("base64"),imports:path==="entry.ts"?{"./sub/helper.ts":"sub/helper.ts","./data.json":"data.json","./sub/lazy.ts":"sub/lazy.ts"}:{}};});
const snapshot=JSON.stringify({version:1,root,entry:"entry.ts",files:rows}),snap=join(work,"implementation.json");writeFileSync(snap,snapshot);
const runner=join(work,"📜️script.ts");copyFileSync(loaderFile,runner);
const trace=join(work,"trace");
const run=spawnSync(process.execPath,[runner,snap,hash(snapshot),"a1","a2"],{encoding:"utf8",env:{...process.env,TRACE_DIR:process.env.WITH_TRACE?trace:""}});
console.log("rc",run.status);console.log(run.stdout.trim());console.log(run.stderr.trim().slice(0,1500));
try{for(const f of readdirSync(trace))console.log("TRACE",f,"\n"+readFileSync(join(trace,f),"utf8"));}catch{console.log("no trace dir");}
