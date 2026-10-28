/** ⏱️ Graph-free bench of batch planfp: a synthetic checkout with <files> inputs; times the witnessed planner, the witness check, and (with the batch) the publisher refresh after reuse, before and after a one-line edit; prints one JSON line. usage: bun bench.ts <repo> <files> <label> */
import {appendFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const[repo,count,label]=process.argv.slice(2) as [string,string,string];process.env.SEMIO_LANGUAGE="en";
const library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),witnessApi=await import(join(library,"🏗️build/🔭️witness/🟨️.mjs")),child=join(import.meta.dirname,"bench-child.ts");
const base=realpathSync(mkdtempSync(join(tmpdir(),"pfb-"))),root=join(base,"c"),workspace=join(base,"ws");
const git=(...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","commit.gpgsign=false",...args],{cwd:root,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(run.stderr.toString());};
try{
 for(let index=0;index<Number(count);index++){const directory=join(root,"gen/src","d"+String(index%100).padStart(2,"0"));mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"f"+index+".ts"),"export const v"+index+"="+index+";\n");}
 writeFileSync(join(root,"gen/script.ts"),"//\n");writeFileSync(join(root,"package.json"),"{}\n");git("init","-q");git("add","-A");git("commit","-q","-m","b");await Bun.sleep(2500);
 const old=new Date(Date.now()-60_000);for(const name of["gen/src/d01/f1.ts"])Bun.spawnSync(["touch","-t",old.toISOString().slice(0,16).replace(/[-T:]/g,"").slice(0,12),join(root,name)]);
 writeFileSync(join(base,"preload.mjs"),witnessApi.worldWitnessPreloadV1());
 const run=(mode:string,witnessed:boolean)=>{const started=performance.now(),result=Bun.spawnSync([process.execPath,...(witnessed?["--preload",join(base,"preload.mjs")]:[]),child,repo,root,workspace,mode],{cwd:repo,env:{...process.env,SEMIO_LANGUAGE:"en",...(witnessed?witnessApi.worldWitnessEnvironmentV1({out:join(base,"w.json"),root,workspace,writable:[tmpdir()],racyMilliseconds:0}):{})},stdout:"pipe",stderr:"pipe"});if(result.exitCode!==0)throw Error(mode+": "+result.stderr.toString().slice(-2000));const inner=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!);return{wall:Math.round(performance.now()-started),...inner};};
 const signal=new AbortController().signal,ports={git:async()=>true,ignore:async()=>true};
 const planned=run("defer",true),witness=witnessApi.readWorldWitnessV1(readFileSync(join(base,"w.json"),"utf8")),kinds=witnessApi.summarizeWorldWitnessV1(witness);
 const hasRefresh=readFileSync(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts"),"utf8").includes("refreshBazelWorkspaceSourcesV1");
 const first=hasRefresh?run("refresh",false):undefined,noop=hasRefresh?run("refresh",false):undefined;
 let started=performance.now();const holdsBefore=(await witnessApi.verifyWorldWitnessV1(witness,{signal,ports})).holds,verifyMs=Math.round(performance.now()-started);
 appendFileSync(join(root,"gen/src/d07/f7.ts"),"// one line\n");
 started=performance.now();const after=await witnessApi.verifyWorldWitnessV1(witness,{signal,ports}),verifyAfterMs=Math.round(performance.now()-started);
 const edited=hasRefresh&&after.holds?run("refresh",false):undefined,input=hasRefresh?readFileSync(join(workspace,Object.keys(JSON.parse(readFileSync(join(workspace,"snapshot.plan.json"),"utf8")).files).length?"snapshot.plan.json":"snapshot.plan.json"),"utf8").length:0;
 const copy=hasRefresh?readFileSync(join(workspace,"a",readdirAlias(workspace,"gen/src/d07"),"f7.ts"),"utf8"):"";
 console.log(JSON.stringify({label,files:Number(count),planner:planned,rows:witness.observations.length,kinds,holdsBefore,verifyMs,editHolds:after.holds,broken:after.holds?undefined:after.kind+" "+after.subject.slice(-40),verifyAfterMs,refreshFirst:first,refreshNoop:noop,refreshAfterEdit:edited,editedCopyHasLine:copy.includes("one line"),planBytes:input}));
}finally{rmSync(base,{recursive:true,force:true});}
function readdirAlias(workspace:string,directory:string):string{const map=JSON.parse(readFileSync(join(workspace,"sources.json"),"utf8")).directories as Record<string,string>;return Object.entries(map).find(([,folder])=>folder===directory)![0];}
