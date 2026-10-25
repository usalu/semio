/** Runs every listed test file in its own `bun test` process under an authored envelope; usage: bun census-run.ts <list> <outdir> [wallSeconds] [skipDone] */
import {spawn} from "node:child_process";
import {readFileSync,writeFileSync,appendFileSync,existsSync,mkdirSync,rmSync,openSync,closeSync} from "node:fs";
import {createBuildBootstrapCapabilitiesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {pinnedRustToolchainV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🟦️.ts";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const root="/Users/ueli/Documents/semio",mod=root+"/🧰️framework/🔨️modules/🏃️process/";
const [list,out,wallArg]=process.argv.slice(2),wall=Number(wallArg??420)*1000;
mkdirSync(out,{recursive:true});
const files=readFileSync(list,"utf8").split("\n").filter(Boolean),summary=out+"/summary.txt";
const done=new Set(existsSync(summary)?readFileSync(summary,"utf8").split("\n").map(l=>l.split("\t")[0]):[]);
for(const [n,f] of files.entries()){
 if(done.has(f))continue;
 const art=`/private/tmp/claude-501/pd/${process.env.CENSUS_TAG??"a"}${n}`;try{rmSync(art,{recursive:true,force:true});}catch{}mkdirSync(art,{recursive:true});
 const base:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:"prototype",SEMIO_BUILD_TOOL:process.env.SEMIO_BUILD_TOOL??"semio",SEMIO_TEST_ARTIFACT_DIR:art};Object.assign(base,pinnedRustToolchainV1(undefined,base).environment);issueRepositoryScriptProvidersV1(base);delete base.SEMIO_SCRIPT_STALL_MS;
 const authored=process.env.CENSUS_AUTHOR==="0"?{}:{artifacts:art,...(f.includes("🎟️writer")?{command:"writer"}:{})},capabilities={...await createBuildBootstrapCapabilitiesV1(root,["run","@semio-tech/framework-process:test-census"],base),...authored};
 const envelope=createScriptProcessEnvelope({version:1,owner:"proc-d:census",maximumElapsedMilliseconds:wall},capabilities,Date.now());
 const env=scriptProcessEnvironment(envelope,base) as NodeJS.ProcessEnv;
 const log=out+"/f"+String(n).padStart(3,"0")+".log",fd=openSync(log,"w"),started=Date.now();
 const rc=await new Promise<string>(resolve=>{
  const child=spawn("nice",["-n","10","bun","test","--timeout",process.env.CENSUS_TIMEOUT??"120000",...(process.env.CENSUS_PATTERN?["-t",process.env.CENSUS_PATTERN]:[]),mod+f],{cwd:root,env,stdio:["ignore",fd,fd],detached:true});
  const timer=setTimeout(()=>{try{process.kill(-child.pid!,"SIGTERM");}catch{}setTimeout(()=>{try{process.kill(-child.pid!,"SIGKILL");}catch{}},3000);resolve("WALL");},wall);
  child.on("close",code=>{clearTimeout(timer);resolve(String(code));});
 });
 closeSync(fd);
 const text=readFileSync(log,"utf8"),counts=[...text.matchAll(/^\s*(\d+) (pass|fail)$/gmu)].map(m=>`${m[2]}=${m[1]}`).join(" ");
 appendFileSync(summary,`${f}\trc=${rc}\t${counts}\t${Math.round((Date.now()-started)/1000)}s\tlog=${log.split("/").pop()}\n`);
 try{rmSync(art,{recursive:true,force:true});}catch{}
}
appendFileSync(summary,"DONE\n");
