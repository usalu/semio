/** 🔁️ [incfacts] Replay of real projects on a root (a mirror), run as a bun test beneath a genuine birth (it needs the original invocation and the graph writer custody): the root `📋️project.json` (the `workspace` precision rules, `kinds` reads included) and every REPLAY_EVERY-th other authored `📋️project.json` are inferred in one process through the production project body (`createProjectInferenceEnvironmentV1` + `inferProjectBodyV1`: precision rules, command walks, presentation resources), each in its own detached observation scope as a project record would be. Writes per project the canonical JSON of the node (or the refusal) and the library `file` facts its rows hold, so a base run and a batch run compare node for node. env: REPLAY_ROOT, REPLAY_OUT, REPLAY_EVERY (default 6). */
import {test} from "bun:test";
import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,relative,sep} from "node:path";
const root=process.env.REPLAY_ROOT!,out=process.env.REPLAY_OUT!,every=Number(process.env.REPLAY_EVERY??6),L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),P=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript");

test("replay",async()=>{
 const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs")) as any;await inference.libraryBootstrap;
 const observation=await import(join(L,"🏗️build/👁️observation/🟨️.mjs")) as any,nodes=await import(join(L,"🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs")) as any,storage=await import(join(L,"🏗️build/📥️inputs/💾️storage/🟨️.mjs")) as any;
 const {withBuildTestInvocationV1}=await import(join(P,"🧪️testing/📥️invocation/🟦️.ts")) as any,{scriptCompilerAdmissionControlV1}=await import(join(P,"🏗️build/🎟️capacity/🟦️.ts")) as any;
 const skip=new Set(["node_modules",".git",".cache",".nx",".tmp",".🧬semio","target","dist"]),all:string[]=[];
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(skip.has(entry.name))continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path);else if(entry.name==="📋️project.json")all.push(relative(root,path).split(sep).join("/"));}};
 walk(root);all.sort();
 const configs=all.filter((config,index)=>config==="📋️project.json"||index%every===0);
 const sort=(value:any):any=>Array.isArray(value)?value.map(sort):value&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sort(value[key])])):value;
 const result:Record<string,{node:string;libraryFacts:string[]}>={},started=performance.now();
 await withBuildTestInvocationV1((original:any)=>storage.withRepositoryBuildGraphFactStoreV1(root,original,scriptCompilerAdmissionControlV1(original,"incfacts:replay",0),()=>observation.observeInferenceSnapshotV1(async()=>{
  const environment=nodes.createProjectInferenceEnvironmentV1({workspaceRoot:root,registry:undefined,projectsByRoot:[],componentRoots:[],contracts:nodes.projectGeneratorContractsV1(root),commandInputs:null,lateClosures:false});
  for(const[index,config]of configs.entries()){
   if(index%10===0)console.error(`[DEBUG] replay ${index}/${configs.length} ${((performance.now()-started)/1000).toFixed(0)} s`);
   const directory=config==="📋️project.json"?".":config.slice(0,-"/📋️project.json".length),raw=JSON.parse(readFileSync(join(root,config),"utf8")),json={...raw,root:directory,name:raw.name??directory};
   let node:string,rows:any[]=[];
   try{const detached=await observation.observeDetachedV1(()=>nodes.inferProjectBodyV1(environment,{name:json.name,root:directory,json,native:false,pipeline:false},undefined));node=JSON.stringify(sort(detached.value));rows=[...detached.rows,...detached.layers.flat()];}
   catch(error){node="refused: "+String((error as Error)?.message??error).split("\n")[0]!.split(root).join("{root}").slice(0,300);}
   result[config]={node,libraryFacts:[...new Set(rows.filter(row=>row?.kind==="file"&&typeof row.path==="string"&&row.path.startsWith(L)).map(row=>relative(L,row.path).split(sep).join("/")))].sort()};
  }
 })));
 writeFileSync(out,JSON.stringify(result,null,1));
 const facts=Object.values(result).filter(row=>row.libraryFacts.length),files=new Map<string,number>();for(const row of facts)for(const file of row.libraryFacts)files.set(file,(files.get(file)??0)+1);
 console.error(`[DEBUG] replay ${configs.length} of ${all.length} projects in ${((performance.now()-started)/1000).toFixed(1)} s, refused ${Object.values(result).filter(row=>row.node.startsWith("refused")).length}, with library file facts ${facts.length}`);
 for(const[file,count]of[...files].sort((a,b)=>b[1]-a[1]).slice(0,12))console.error(`[DEBUG]   ${count} x ${file}`);
},3_600_000);
