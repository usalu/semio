// 📏 Bazel-side cost of closure expansion on persisted real-graph project records: markers per plan, manifest load, expansion, expanded input volume.
import {readFileSync,readdirSync} from "node:fs";import {join} from "node:path";import {brotliDecompressSync} from "node:zlib";
const M=process.env.INCLOS_BASE??"/private/tmp/inclos/m",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",B=join(M,L,"🏗️build/🔧️tools/🏗️bazel");
const api:any=await import(join(B,"📜️script.ts")),closure:any=await import(join(B,"🔗️closure/🟦️.ts"));
const G="/Users/ueli/Documents/semio/.cache/b/g/m",NP=join(G,"inference/np"),sample=Number(process.argv[2]??"300"),stride=Number(process.argv[3]??"12");
const all=readdirSync(NP).filter(f=>f.endsWith(".json")).sort();
const projects:Record<string,any>={},tasks:any[]=[];
all.forEach((f,i)=>{const raw=readFileSync(join(NP,f));let t:string;try{t=brotliDecompressSync(raw).toString();}catch{t=raw.toString();}const p=JSON.parse(t.split("\n")[1]!);if(!p.targets)return;projects[p.name]={root:p.root,namedInputs:p.namedInputs??{},targets:p.targets};
 if(i%stride===0&&tasks.length<sample*10)for(const[n,tg]of Object.entries<any>(p.targets))if(tg.executor==="repo:script")tasks.push({id:p.name+":"+n,project:p.name,target:n,label:n,dependencies:[],definition:tg});});
const graph:any={namedInputs:{},projects,dependencies:[]};
const tPrep=performance.now(),t0=performance.now();
const markers=closure.bazelClosureMarkersForPlanV1(graph,tasks);
const loaded=await closure.loadBazelClosuresV1(markers,closure.filesystemBazelClosureFetchV1(G),closure.workspaceBazelLockFilesV1("/Users/ueli/Documents/semio"));
const t1=performance.now();
const perTask:unknown[][]=[];let expanded=0,files=new Set<string>(),failures=0;const errors:Record<string,number>={};
for(const t of tasks){try{const inputs=api.bazelTaskInputsV1(graph,t,loaded);expanded+=inputs.length;if(perTask.length<40)perTask.push(inputs);for(const x of inputs)if(typeof x==="string")files.add(x);}catch(e:any){failures++;errors[String(e.message).slice(0,90)]=(errors[String(e.message).slice(0,90)]??0)+1;}}
const t2=performance.now();
let indexMs=0,selected=0,oneShotMs=0;
if(typeof api.bazelSourceIndexV1==="function"){const inventory=[...files].filter(p=>!/[*?{[]/.test(p)).sort();const si=performance.now();const index=api.bazelSourceIndexV1(inventory);for(const inputs of perTask){const r=index.selectAll(inputs);selected+=r.marks.reduce((a:number,b:number)=>a+b,0);}indexMs=performance.now()-si;
 const sj=performance.now();for(const inputs of perTask.slice(0,3))api.bazelTaskSourcePathsV1(inputs,inventory);oneShotMs=(performance.now()-sj)/3;}
console.log(JSON.stringify({indexSelectMsFor40Tasks:Math.round(indexMs),oneShotSelectMsPerTask:Math.round(oneShotMs),selectedFiles:selected,projects:Object.keys(projects).length,tasks:tasks.length,markers:markers.length,manifestLoadMs:Math.round(t1-t0),expansionMs:Math.round(t2-t1),expandedEntries:expanded,perTaskEntries:Math.round(expanded/Math.max(1,tasks.length)),distinctPaths:files.size,failures,errors}));
