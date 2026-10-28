/** Runs every listed build-module test file in its own `bun test` process under the official-default capability envelope; samples the process-group RSS and kills above the ceiling. usage: bun census-run.ts <list> <outdir> [wallSeconds]; env CENSUS_TAG, CENSUS_RSS_MB (default 3072), CENSUS_COMMAND, CENSUS_AUTHOR=1 for an artifacts capability, CENSUS_PATTERN */
import {spawn,execFileSync} from "node:child_process";
import {readFileSync,appendFileSync,existsSync,mkdirSync,rmSync,openSync,closeSync} from "node:fs";
import {createBuildBootstrapCapabilitiesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {pinnedRustToolchainV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🟦️.ts";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const root="/Users/ueli/Documents/semio",mod=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const [list,out,wallArg]=process.argv.slice(2),wall=Number(wallArg??300)*1000,ceilingKb=Number(process.env.CENSUS_RSS_MB??3072)*1024;
mkdirSync(out,{recursive:true});
const files=readFileSync(list,"utf8").split("\n").filter(Boolean),summary=out+"/summary.txt";
const done=new Set([summary,...(process.env.CENSUS_SKIP_DIRS?.split(",")??[]).map(d=>d+"/summary.txt")].flatMap(path=>existsSync(path)?readFileSync(path,"utf8").split("\n").map(l=>l.split("\t")[0]):[]));
const table=JSON.parse(readFileSync(mod+"🗂️suites/🔣️.json","utf8")) as {suites:{id:string;trees?:string[];files?:string[]}[]},suiteOf=(file:string):string=>{let best=-1,id="build-census";for(const suite of table.suites)for(const tree of [...(suite.trees??[]),...(suite.files??[])])if((file===tree||file.startsWith(tree.replace(/\/$/u,"")+"/"))&&tree.length>best){best=tree.length;id=suite.id;}return id;};
const tag=process.env.CENSUS_TAG??"a",artRoot=process.env.CENSUS_ARTROOT??"/private/tmp/claude-501/rg";
const groupRssKb=(pgid:number):number=>{try{return execFileSync("ps",["-o","rss=","-g",String(pgid)],{encoding:"utf8"}).split("\n").reduce((sum,line)=>sum+(Number(line.trim())||0),0);}catch{return 0;}};
for(const [n,f] of files.entries()){
 if(done.has(f))continue;
 const art=`${artRoot}/${tag}${n}`;try{rmSync(art,{recursive:true,force:true});}catch{}mkdirSync(art,{recursive:true});
 const base:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:"prototype",SEMIO_BUILD_TOOL:process.env.SEMIO_BUILD_TOOL??"semio",SEMIO_TEST_ARTIFACT_DIR:art};Object.assign(base,pinnedRustToolchainV1(undefined,base).environment);issueRepositoryScriptProvidersV1(base);delete base.SEMIO_SCRIPT_STALL_MS;mkdirSync(artRoot+"/tmp",{recursive:true});base.TMPDIR=artRoot+"/tmp";
 const authored=process.env.CENSUS_AUTHOR==="1"?{artifacts:art,...(process.env.CENSUS_COMMAND?{command:process.env.CENSUS_COMMAND}:{})}:process.env.CENSUS_COMMAND?{command:process.env.CENSUS_COMMAND}:{},capabilityBase={...base},capabilities=(delete capabilityBase.SEMIO_TEST_ARTIFACT_DIR,{...await createBuildBootstrapCapabilitiesV1(root,process.env.CENSUS_SELECTION?JSON.parse(process.env.CENSUS_SELECTION):["run","@semio-tech/repo:test-"+suiteOf(f)],capabilityBase),...authored});
 const envelope=createScriptProcessEnvelope({version:1,owner:"route-g:census",maximumElapsedMilliseconds:wall},capabilities,Date.now());
 const env=scriptProcessEnvironment(envelope,base) as NodeJS.ProcessEnv;
 const log=out+"/f"+String(n).padStart(3,"0")+".log",fd=openSync(log,"w"),started=Date.now();
 let peak=0;
 const rc=await new Promise<string>(resolve=>{
  const child=spawn("nice",["-n","10","bun","test","--timeout",process.env.CENSUS_TIMEOUT??"120000",...(process.env.CENSUS_PATTERN?["-t",process.env.CENSUS_PATTERN]:[]),(f.startsWith("/")?f:mod+f)],{cwd:root,env,stdio:["ignore",fd,fd],detached:true});
  const stop=(verdict:string)=>{try{process.kill(-child.pid!,"SIGTERM");}catch{}setTimeout(()=>{try{process.kill(-child.pid!,"SIGKILL");}catch{}},3000);resolve(verdict);};
  const timer=setTimeout(()=>stop("WALL"),wall),sampler=setInterval(()=>{const kb=groupRssKb(child.pid!);if(kb>peak)peak=kb;if(kb>ceilingKb){clearTimeout(timer);clearInterval(sampler);stop("RSS");}},1500);
  child.on("close",code=>{clearTimeout(timer);clearInterval(sampler);resolve(String(code));});
 });
 closeSync(fd);
 const text=readFileSync(log,"utf8"),counts=[...text.matchAll(/^\s*(\d+) (pass|fail|skip)$/gmu)].map(m=>`${m[2]}=${m[1]}`).join(" ");
 appendFileSync(summary,`${f}\trc=${rc}\t${counts}\t${Math.round((Date.now()-started)/1000)}s\tlog=${log.split("/").pop()}\trss=${Math.round(peak/1024)}MB\n`);
 try{rmSync(art,{recursive:true,force:true});}catch{}
}
appendFileSync(summary,"DONE\n");
