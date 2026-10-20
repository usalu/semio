// 🔬 Cross-checks `affected` on the statically reconstructed 1,466-project graph: neutral TS rule vs Bazel rdeps vs the installed Nx's own locators, for frozen change sets.
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync,realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {createRequire} from "node:module";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {affectedBuildProjectsV1,affectedTouchesV1} from "./run/selection.ts";
import {nx as nxOracle} from "./affected-xcheck-lib.ts";
import {bazelAffectedBuildV1,bazelAffectedExpressionV1,bazelAffectedProjectsFromOutputV1,bazelAffectedWorkspaceV1} from "./staged/affected.ts";

const root="/Users/ueli/Documents/semio",dir=join(import.meta.dir,"..","🗑️generated","bzl-lazy"),graph=JSON.parse(readFileSync(join(dir,"graph.json"),"utf8")),installed=createRequire(join(root,"package.json"));
const BZ=root+"/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel",ws="/private/tmp/bzlazy/ws-affected",ob="/private/tmp/bzlazy/ob-affected",fixed=bazelAffectedWorkspaceV1();
const sets:Record<string,string[]>=JSON.parse(readFileSync(join(dir,"change-sets.json"),"utf8"));
mkdirSync(ws,{recursive:true});writeFileSync(join(ws,"MODULE.bazel"),fixed.module);
const env={...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"} as NodeJS.ProcessEnv;
const bazel=(...args:string[])=>spawnSync(BZ,[`--output_base=${ob}`,"--install_base=/private/tmp/bzlazy/install","--nosystem_rc","--nohome_rc",`--bazelrc=${root}/.bazelrc`,"--max_idle_secs=300","--host_jvm_args=-Xmx2048m",...args],{cwd:ws,encoding:"utf8",env,maxBuffer:256*1024*1024});

const report:any[]=[];
for(const[name,files]of Object.entries(sets)){
 const t0=performance.now(),neutral=affectedBuildProjectsV1(graph,files),neutralMs=Math.round(performance.now()-t0);
 const touches=affectedTouchesV1(graph,files),t1=performance.now(),build=bazelAffectedBuildV1(graph,touches);writeFileSync(join(ws,"BUILD.bazel"),build);const generateMs=Math.round(performance.now()-t1);
 let bazelProjects:string[]=[],queryMs=0,coldMs=0;
 if(touches.size){
  const expression=bazelAffectedExpressionV1(touches),queryFile=join(dir,"affected-query.txt");writeFileSync(queryFile,expression);
  const t2=performance.now(),first=bazel("query",...fixed.options,`--query_file=${queryFile}`);coldMs=Math.round(performance.now()-t2);
  if(first.status!==0)throw Error("bazel: "+first.stderr.slice(0,2000));
  const t3=performance.now(),second=bazel("query",...fixed.options,`--query_file=${queryFile}`);queryMs=Math.round(performance.now()-t3);
  bazelProjects=bazelAffectedProjectsFromOutputV1(graph,second.stdout);
 }
 const t4=performance.now(),nxProjects=nxOracle(graph,files),nxMs=Math.round(performance.now()-t4);
 const same=(a:string[],b:string[])=>JSON.stringify(a)===JSON.stringify(b),only=(a:string[],b:string[])=>a.filter(x=>!b.includes(x));
 report.push({set:name,files:files.length,touchedDirectly:touches.size,neutral:neutral.length,bazel:bazelProjects.length,nx:nxProjects.length,neutralEqNx:same(neutral,nxProjects),bazelEqNx:same(bazelProjects,nxProjects),bazelEqNeutral:same(bazelProjects,neutral),neutralOnly:only(neutral,nxProjects).slice(0,10),nxOnly:only(nxProjects,neutral).slice(0,10),buildBytes:build.length,ms:{neutral:neutralMs,generate:generateMs,bazelColdServer:coldMs,bazelWarmQuery:queryMs,nx:nxMs}});
 console.log(JSON.stringify(report.at(-1)));
}
bazel("shutdown");
writeFileSync(join(dir,"affected-xcheck.json"),JSON.stringify(report,null,1));
