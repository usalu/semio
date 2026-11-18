import{readFileSync}from"node:fs";import{createHash}from"node:crypto";import{brotliDecompressSync}from"node:zlib";
import{load,R}from"./common.ts";
const[waveDir]=process.argv.slice(2);const m=await load(waveDir),{SI,OB,pn,env,recordPath}=m;
const headerOf=(path:string)=>{const text=brotliDecompressSync(readFileSync(path)).toString("utf8");return createHash("sha256").update(text.slice(0,text.indexOf("\n"))).digest("hex");};
process.stdout.write("READY\n");
for await(const line of console){
 if(line.trim()==="")continue;const{root,dir}=JSON.parse(line);const path=recordPath(root);const c0=process.cpuUsage();let refs:any=null,err="";
 try{refs=await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(()=>SI.runClosureTakeV1(()=>pn.resolveOwnerRefsV1(env,{cachePath:path},headerOf(path)))),{verdicts:dir})??null;}catch(e:any){err=String(e?.code??e?.message).slice(0,80);}
 const u=process.cpuUsage(c0);SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});
 process.stdout.write(JSON.stringify({root,refs,err,cpu:(u.user+u.system)/1000,pid:process.pid,rss:Math.round(process.resourceUsage().maxRSS/1048576)})+"\n");
}
