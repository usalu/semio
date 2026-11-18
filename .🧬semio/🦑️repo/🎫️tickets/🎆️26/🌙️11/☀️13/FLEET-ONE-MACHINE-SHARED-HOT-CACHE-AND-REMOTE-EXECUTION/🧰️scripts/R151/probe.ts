import {writeFileSync,readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";
const L="../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,buildInvocationGraphSelectionV1}=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const {buildGraphSourceFactsHoldV1}=await import(L+"🕸️graph/📁️workspace/🟦️.ts");
const owner:any=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
const observation:any=await import(L+"👁️observation/🟨️.mjs");
const root=process.env.SEMIO_WORKSPACE_ROOT!,out=process.env.R151_OUT!,projects=process.env.R151_PROJECTS??"@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp";
const loadSource=process.env.R151_LOAD_SOURCE!,limits=JSON.parse(readFileSync(process.env.R151_SCHEMA!,"utf8")).definitions.Limits.const;
const tree=ts.createSourceFile("load.ts",readFileSync(loadSource,"utf8"),ts.ScriptTarget.Latest,true);
const extract=(name:string)=>{const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text===name)!;return ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,node,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;};
const observeMemo:any=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1",extract("rememberOwnerFactsV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(new Map(),limits);
const registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
const selection=buildInvocationGraphSelectionV1(["run-many","--projects="+projects,"--targets=build"]);
let currentKey="";const rowsByKey=new Map<string,any[]>();let recording=false;
let edited:string|undefined,observedCount=new Map<string,number>(),rowsOf=new Map<string,any[]>(),memoOn=true;
const hold=async(rows:readonly any[])=>{if(recording)rowsByKey.set(currentKey,rows as any[]);return holdRows(rows);};
let hits=0,calls=0;const holdRows=async(rows:readonly any[])=>{calls++;const touched=edited!==undefined&&rows.some(row=>row.path===edited);if(touched){hits++;console.log("DBG touched",currentKey.split("\0")[1]);}return !touched&&await buildGraphSourceFactsHoldV1(rows);};
const pass=async(label:string)=>{
 observedCount=new Map();const started=performance.now(),cpu=process.cpuUsage(),nativeCache=new Map(),nativeClosures=new Map();
 const context={workspaceRoot:root,invocation:undefined,selection,nativeCache,nativeClosures};
 const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>closeBuildOwnerAdmissionV1(selection,registry,async(o:any,targets:readonly string[],inputs:readonly string[])=>{
  const observe=async()=>{observedCount.set(o.name,(observedCount.get(o.name)??0)+1);try{return await owner.discoverBuildOwnerAdmissionFactsV1(o,targets,registry,context,inputs);}catch{return owner.discoverBuildOwnerAdmissionFactsV1(o,[],registry,context,[]);}};
  if(!memoOn)return observe();
  const key="p\0"+o.name+"\0"+JSON.stringify([targets,inputs]);
  currentKey=key;return observeMemo({key,hold,observation,observe});
 })));
 const usage=process.cpuUsage(cpu),row={label,edited:edited?edited.slice(-90):null,ownersObserved:observedCount.size,observations:[...observedCount.values()].reduce((a,b)=>a+b,0),wallMs:Math.round(performance.now()-started),cpuMs:Math.round((usage.user+usage.system)/1000),closure:(captured.value as any).owners.length,rows:captured.observations.length};
 console.log(JSON.stringify({...row,holdCalls:calls,touchedKeys:hits}));calls=0;hits=0;return{row,captured};
};
const results:any[]=[];
const cold=await pass("cold, attempt 1 (the old per-attempt cost)");results.push(cold.row);
recording=true;const again=await pass("no edit (attempt 2)");results.push(again.row);recording=false;
const ownersOf=new Map<string,Set<string>>();for(const [key,rows] of rowsByKey){const name=key.split("\0")[1]!;for(const row of rows){if(!row.path)continue;if(!ownersOf.has(row.path))ownersOf.set(row.path,new Set());ownersOf.get(row.path)!.add(name);}}
const ranked=[...ownersOf].map(([path,set])=>[path,set.size] as [string,number]).sort((a,b)=>b[1]-a[1]);
const closureNames=new Set((again.captured.value as any).owners as string[]);
const outside=registry.owners.find((o:any)=>!closureNames.has(o.name)&&o.manifest&&!ownersOf.has(root+"/"+o.manifest));
const single=ranked.filter(([path,count])=>count===1&&/(📋️project\.json|Cargo\.toml)$/.test(path)&&path.includes("repo-mcp")||count===1&&/(📋️project\.json|Cargo\.toml)$/.test(path));
const shared=ranked.find(([path])=>/\.(ts|mjs)$/.test(path));
const unrelated=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔄️reload/🟨️.mjs";
const kinds:[string,string|undefined][]=[["(a) source file of an owner outside the closure",outside?root+"/"+outside.manifest:undefined],["(b) source file of one owner in the closure",single[0]?.[0]],["(c) build-tooling file read by the most owners (engine closure)",shared?.[0]],["(d) build-tooling file no owner read",unrelated]];
for(const [label,path] of kinds){if(path)console.log("DBG",label.slice(0,12),[...rowsByKey].filter(([k,rows])=>rows.some((r:any)=>r.path===path)).map(([k])=>k.split("\0").slice(1).join("|").slice(0,60)));edited=path;const r=await pass(label);results.push({...r.row,ownersReading:path===undefined?null:ownersOf.get(path)?.size??0});}
memoOn=false;edited=undefined;const old=await pass("without the memo (old code): every attempt observes every owner again");results.push(old.row);
writeFileSync(join(out,"probe2.json"),JSON.stringify({projects,registryOwners:registry.owners.length,topShared:ranked.slice(0,6).map(([p,c])=>[p.slice(-110),c]),singleCount:single.length,results},null,1));
