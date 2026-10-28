/** 🔎 Read-only: which fact rows of the course slice name a path containing a needle, and which sets of the relevance hold it: `bun storefacts.ts <code-root> <data-root> <plan-file> <needle> <relative dir>`. Aborts above 2.3 GB. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [code,data,planFile,needle,probe]=process.argv.slice(2) as [string,string,string,string,string];
const L=join(code,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const generation=await import(join(L,"🕸️graph/🕰️generations/🟨️.mjs")),cutoff=await import(join(L,"🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),relevance=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts")),opening=await import(join(L,"../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")),changes=await import(join(code,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts")),journal=await import(join(L,"🏡️resident/📜️journal/🟦️.ts")),endpoint=await import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs"));
const plan=JSON.parse(readFileSync(planFile,"utf8")),owners=new Set<string>([...Object.keys(plan.projects),...(plan.unavailable??[]).map((row:{project:string})=>row.project)]);
const generations=join(opening.repositoryCachePartitionRootV1(data,"graph"),"generations"),engine=generation.createGenerationStoreV1({root:generations,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(),manifest=sealed.manifest;
const frontier=await engine.readFrontier(manifest.frontier),index=await engine.readIndexView(manifest.factIndex);
const selection=generation.selectFactClassesV1(index,cutoff.scopeWithDerivedV1(index,new Set([...owners,generation.GENERATION_POLICY_V1.envOwner]))),ids=cutoff.verifiableRowIdsV1(index,selection.ids);
const hits:Record<string,number>={},samples:string[]=[];
for(const id of ids){const row=index.rows[id];const text=row.path+"\u0000"+(typeof row.fact==="string"?row.fact:"");if(text.includes(needle)){hits[row.kind]=(hits[row.kind]??0)+1;if(String(row.path).includes(probe)||String(row.fact).includes(probe))samples.push(row.kind+" "+JSON.stringify(row.fact).slice(0,80)+" "+String(row.path).slice(-130));}}
console.log(JSON.stringify({generation:sealed.pointer.generation,rowsInSlice:ids.length,hitsByKind:hits,samples},null,1));
const state=changes.readChangeStateV1(journal.changeJournalDirectoryV1(endpoint.canonicalRootV1(data),process.env)),insensitive=process.platform==="darwin",excluded=memo.memoExcluderV1(state.exclude,insensitive);
console.log(JSON.stringify({journalExclude:state.exclude}));
const built=await relevance.buildGraphMemoRelevanceV1({root:data,rows:function*(){for(const id of ids)yield index.rows[id];},frontier,excluded,isLink:async()=>false});
const rel=memo.unpackMemoRelevanceV1(memo.packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped));
const parent=probe;const file=probe+"/build.json";
console.log(JSON.stringify({probe,names:{dir:rel.holds(parent),file:rel.holds(file)},strictDir:rel.strict(parent),stampedDir:rel.stamped(parent),parentOfDir:{dir:rel.holds(parent.slice(0,parent.lastIndexOf("/"))),strict:rel.strict(parent.slice(0,parent.lastIndexOf("/"))),stamped:rel.stamped(parent.slice(0,parent.lastIndexOf("/")))},inFrontier:[...frontier.directories].filter((r:any)=>r[0]===parent).length,excludedProbe:excluded(parent)}));
process.exit(0);
