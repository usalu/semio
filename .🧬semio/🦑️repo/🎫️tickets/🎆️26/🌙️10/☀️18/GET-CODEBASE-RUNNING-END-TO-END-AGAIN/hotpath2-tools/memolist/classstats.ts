/** 📊 Read-only statistics of the listing refinement on the LIVE generation: `bun classstats.ts <code-root> <data-root> <plan-file> <journal-dir> <from-seq>`; code from <code-root> (a mirror), generation and journal from <data-root>. Aborts above 2.3 GB RSS. */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const [code,data,planFile,journalDirectory,fromText]=process.argv.slice(2) as [string,string,string,string,string];
const L=join(code,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const generation=await import(join(L,"🕸️graph/🕰️generations/🟨️.mjs")),cutoff=await import(join(L,"🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),relevance=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts")),opening=await import(join(L,"../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")),changes=await import(join(code,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts"));
const guard=()=>{const rss=process.memoryUsage().rss;if(rss>2.3*1024**3){console.log(JSON.stringify({abort:"rss",mb:Math.round(rss/1048576)}));process.exit(3);}};
const t0=performance.now(),mark=(step:string,detail:unknown="")=>{guard();console.log(JSON.stringify({step,ms:Math.round(performance.now()-t0),rssMb:Math.round(process.memoryUsage().rss/1048576),detail}));};
const plan=JSON.parse(readFileSync(planFile,"utf8")),owners=new Set<string>([...Object.keys(plan.projects),...(plan.unavailable??[]).map((row:{project:string})=>row.project)]);
const generations=join(opening.repositoryCachePartitionRootV1(data,"graph"),"generations"),engine=generation.createGenerationStoreV1({root:generations,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(),manifest=sealed.manifest;
const frontier=await engine.readFrontier(manifest.frontier),index=await engine.readIndexView(manifest.factIndex);
mark("index",{generation:sealed.pointer.generation,rows:index.rows.length,frontierDirectories:[...frontier.directories].length,frontierMembers:[...frontier.members].length});
const selection=generation.selectFactClassesV1(index,cutoff.scopeWithDerivedV1(index,new Set([...owners,generation.GENERATION_POLICY_V1.envOwner]))),ids=cutoff.verifiableRowIdsV1(index,selection.ids);
const state=changes.readChangeStateV1(journalDirectory),insensitive=process.platform==="darwin",excluded=memo.memoExcluderV1(state.exclude,insensitive);
const kinds:Record<string,number>={};for(const id of ids){const kind=index.rows[id].kind;kinds[kind]=(kinds[kind]??0)+1;}
mark("kinds",kinds);
const build=async(withFrontier:boolean)=>relevance.buildGraphMemoRelevanceV1({root:data,rows:function*(){for(const id of ids)yield index.rows[id];},frontier:withFrontier?frontier:{directories:[],members:[]},excluded,isLink:async()=>false});
const built=await build(true);
mark("built",{rows:built.rows,names:built.directories.size,strict:built.strict.size,stamped:built.stamped.size,stamps:built.stamps.size,recheck:built.recheck.rows.length,frontierDirectories:[...frontier.directories].length,capturedAgeS:Math.round((Date.now()-frontier.capturedAt)/1000)});
const bare=built;
const rel=memo.unpackMemoRelevanceV1(memo.packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped));
const relBare=memo.unpackMemoRelevanceV1(memo.packMemoRelevanceV1(bare.directories,insensitive,bare.strict,bare.stamped));
const oldRel=memo.unpackMemoRelevanceV1(memo.packMemoRelevanceV1(built.directories,insensitive));
const stampSizes={tableBytes:memo.packMemoStampsV1(built.stamps,insensitive).length,relBytes:memo.packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped).length};mark('sizes',stampSizes);
const out:Record<string,number>={},examples:Record<string,string[]>={};let lost=0,burst=0;
for(const name of readdirSync(journalDirectory).filter(name=>name.endsWith(".log")).sort())for(const line of readFileSync(join(journalDirectory,name),"utf8").split("\n")){
 if(!line)continue;let item:any;try{item=JSON.parse(line);}catch{continue;}if(item.s<=Number(fromText))continue;
 if(item.l){lost++;continue;}const rows=item.r??[];if(rows.filter((row:any)=>row[0]==="m"&&row[1]===0).length>1000){burst+=rows.length;continue;}
 for(const row of rows){
  const path=row[2];if(path.startsWith(".🧬semio/🦑️repo/🏡️resident-fence"))continue;
  const old=oldRel.holds(path)||(!(row[0]==="m"&&row[1]===1)&&oldRel.holds(path.lastIndexOf("/")<0?"":path.slice(0,path.lastIndexOf("/"))));
  const now=memo.classifyMemoChangesV1(rel,[row]),bareNow=memo.classifyMemoChangesV1(relBare,[row]);
  const key="old:"+(old?"relevant":"irrelevant")+" new:"+(now.changed?"relevant":now.pending.size?"pending":"irrelevant")+" newNoFrontier:"+(bareNow.changed?"relevant":bareNow.pending.size?"pending":"irrelevant");
  out[key]=(out[key]??0)+1;(examples[key]??=[]).length<3&&examples[key]!.push(row[0]+row[1]+" "+path.slice(-90));
 }
}
console.log(JSON.stringify({burstRowsSkipped:burst,lost,classes:out},null,1));console.log(JSON.stringify(examples,null,1));
process.exit(0);
