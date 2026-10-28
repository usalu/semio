// hasher-proof.mjs <probe-file> : real Nx native hasher, REAL engine runtime inputs (the entry script runs in the real tree),
// before / with / after a one-line probe in a scheduling-domain file of the engine closure. Old rule = digest over the whole closure.
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
import {readFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
const root=process.cwd(),lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",probe=process.argv[2];
const m=await import(pathToFileURL(lib+"🟨️.mjs").href),sdk=createRequire(resolve(root,".nx/installation/package.json")),native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust;
const task={id:"fixture:build",target:{project:"fixture",target:"build"},overrides:{},outputs:[],projectRoot:"fixture",cache:true,parallelism:true},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}};
const hashOf=family=>{
 const target=m.projectBuildTargetForNxV1({execution:{kind:"command",command:"repo ./📜️script.ts",...(family==="native"?{ownerArguments:true,nativeOwnerCommand:"cargo"}:{})},cache:true,inputs:["{projectRoot}/index.ts"]});
 const name=target.inputs.at(-1),graph={nodes:{fixture:{name:"fixture",type:"lib",data:{root:"fixture",targets:{build:target},namedInputs:{default:["{projectRoot}/**/*"],[name]:m.nxBuildEngineInputsV1(family)}}}},externalNodes:{"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:"nx-fixed"}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},dependencies:{fixture:[]}},graphRef=native.transferProjectGraph(transform(graph)),filemap=native.testOnlyTransferFileMap({fixture:[{file:"fixture/index.ts",hash:"index-fixed"}]},[]),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(root,graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});
 return hasher.hashPlans(plans,{[task.id]:{}},root,true)[task.id].value;
};
const oldDigest=()=>m.nxBuildEngineFilesDigestV1(root,m.nxBuildEngineClosureV1({version:1,domains:[{id:"x",contract:{en:"x",de:"x"},prefixes:["zz/"]}]}).files);
const SUFFIX="\n// nxperf-g probe (temporary)\n",snap=()=>({generic:hashOf("generic"),native:hashOf("native")});
const before=snap(),oldBefore=oldDigest();let during,oldDuring,after;
const original=readFileSync(probe,"utf8");writeFileSync(probe,original+SUFFIX);
try{during=snap();oldDuring=oldDigest();}finally{const now=readFileSync(probe,"utf8");writeFileSync(probe,now.endsWith(SUFFIX)?now.slice(0,-SUFFIX.length):now);}
after=snap();
const same=(a,b)=>a.generic===b.generic&&a.native===b.native;
console.log(JSON.stringify({probe:probe.replace(root+"/",""),taskHash:{before,during,after},unchangedWithProbe:same(before,during),unchangedAfterRevert:same(before,after),oldRuleDigestMovedWithProbe:oldBefore!==oldDuring,rssMB:Math.round(process.memoryUsage().rss/1e6)},null,1));
