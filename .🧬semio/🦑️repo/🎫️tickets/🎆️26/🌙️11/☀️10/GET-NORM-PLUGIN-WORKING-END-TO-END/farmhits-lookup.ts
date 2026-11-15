import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import{readdirSync,statSync,readFileSync,mkdtempSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";import{tmpdir}from"node:os";
import{startSamplingProfiler,samplingProfilerStackTraces}from"bun:jsc";
const W=R,B=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",CE=process.argv[2],OWNERS=Number(process.argv[3]??30),PER=Number(process.argv[4]??8),PROFILE=process.env.PROFILE==="1";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs") as any,nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs") as any,si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const reg=JSON.parse(readFileSync(W+"/📋️build-owners.json","utf8")),projectsByRoot=reg.owners.flatMap((o:any)=>[[o.root,o.name],...(o.nativeRoots??[]).map((r:string)=>[r,o.name])]);
const contracts=nodes.projectGeneratorContractsV1(W);
const env=await obs.observeInferenceSnapshotV1(()=>nodes.createProjectInferenceEnvironmentV1({workspaceRoot:W,registry:null,contracts,projectsByRoot,componentRoots:[],commandInputs:null,lateClosures:true}));
si.bindClosureStoreV1(W,CE,false);
const names=readdirSync(CE+"/e").filter((_,i)=>i%Number(process.env.STEP??1)===0).map(n=>[n,statSync(CE+"/e/"+n).mtimeMs] as const).sort((a,b)=>b[1]-a[1]).map(x=>x[0]);
const recipes:any[]=[];const seen=new Set<string>();
for(const n of names){const r=JSON.parse(brotliDecompressSync(readFileSync(CE+"/e/"+n)).toString("utf8"));const k=si.closureRecipeKeyV1(r.recipe);if(seen.has(k))continue;seen.add(k);if(process.env.BIG==="1"&&r.layers.length<1000)continue;if(["native-target","generic-target","target-sources","closure-of"].includes(r.recipe.fn))recipes.push([k,r.recipe]);if(recipes.length>=OWNERS*PER)break;}
console.log("recipes",recipes.length);
const ballast:any[]=[];if(process.env.BALLAST){for(let i=0;i<Number(process.env.BALLAST)*1e6/100;i++)ballast.push({a:i,b:"x"+i,c:[i]});console.log("ballast rss",Math.round(process.memoryUsage().rss/1e6));}
if(PROFILE)startSamplingProfiler();
const passRound=new Map();const times:number[]=[];const counts0=si.closureEntryCountsV1();
await si.withClosurePassV1(async()=>{
 const t0=performance.now();
 for(let i=0;i<OWNERS;i++){
  const mine=recipes.slice(i*PER,(i+1)*PER);if(!mine.length)break;
  const value={closureRecipes:Object.fromEntries(mine),closureInputs:{}};
  const rows=mine.map(([k])=>({kind:"closure",path:JSON.stringify([W,k,["digest"]]),fact:"\"x\""}));
  const a=performance.now();
  await (process.env.PASSROUND==='1'?obs.withObservationPassRoundV1(passRound,async()=>{await obs.settleClosureRowsV1(rows,value);}):obs.withObservationRoundV1(async()=>{await obs.settleClosureRowsV1(rows,value);}));
  const b=performance.now();
  await si.closureApiV1(env.scripts,W).resolve(mine);
  const c=performance.now();times.push(b-a);
  console.log("owner",i,"settle",Math.round(b-a),"resolve",Math.round(c-b),"rss",Math.round(process.memoryUsage().rss/1e6));
  if(process.memoryUsage().rss>5e9){console.log("rss abort");break;}
 }
 const cpu=process.cpuUsage();console.log("cpu ms",Math.round((cpu.user+cpu.system)/1000));console.log("total ms",Math.round(performance.now()-t0),"settle mean ms",Math.round(times.reduce((x,y)=>x+y,0)/times.length));
});
console.log(JSON.stringify({before:counts0,after:si.closureEntryCountsV1()}));
if(PROFILE){const traces=(samplingProfilerStackTraces() as any).traces,self=new Map<string,number>(),incl=new Map<string,number>(),key=(x:any)=>x.name+" "+String(x.sourceURL).replace(/^.*🏗️build\//,"")+":"+x.line;for(const t of traces){if(!t.frames.length)continue;const s=key(t.frames[0]);self.set(s,(self.get(s)??0)+1);for(const e of new Set<string>(t.frames.map(key)))incl.set(e,(incl.get(e)??0)+1);}const top=(m:Map<string,number>)=>[...m].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>v+" "+k).join("\n");console.log("SELF\n"+top(self)+"\nINCL\n"+top(incl));}
