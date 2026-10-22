import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/"
old_start=" /** 🧩️ Assembles the graph of exactly the owners in `names` (every owner without it): nodes and edges come from fragments only, so unrelated owners are never read. */\n async function assemble("
src=open(G+"🟨️.mjs",encoding="utf8").read()
i=src.index(old_start); j=src.index(" /** 📸️ Reads one complete graph without any lease")
new=''' /**
  * 🌊️ Streams the graph of exactly the owners in `names` (every owner without it) fragment by fragment: `nodes()` yields `[name, node]` in code point order while at most `readConcurrency` fragments are resident, so a consumer that drops what it was given keeps the resident state independent of the number of owners; `finish()` answers the rest (dependencies, external nodes, unavailable projects, generation, membership) once the nodes were consumed. Nodes and edges come from fragments only, so unrelated owners are never read.
  */
 function assembleStream(current,{names,compare=(a,b)=>a<b?-1:a>b?1:0,signal,progress}={}){
  const manifest=current.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const wanted=(names?[...new Set(names)]:Object.keys(manifest.owners)).sort(compareCodePointsV1),keep=names?new Set(wanted):undefined;
  for(const name of wanted)if(!manifest.owners[name])throw fail("Unknown project in the closure: "+name,"GENERATION_UNKNOWN_OWNER");
  const unavailable=[],edges=new Map(),roots=[];let global,drained=false,started=false;
  async function*nodes(){
   if(started)throw fail("The node stream of an assembly is read once","GENERATION_STREAM_USED");started=true;
   const globalBytes=await fs.read(fragmentPath(manifest.global));if(!globalBytes||sha256(globalBytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");
   global=unpack(globalBytes,P.maximumFragmentBytes).value??{};
   await progress?.({phase:"assembly",done:0,total:wanted.length});
   const pending=[];let next=0,done=0;
   const launch=()=>{while(pending.length<P.readConcurrency&&next<wanted.length){const name=wanted[next++],job=readFragment(manifest.owners[name]).then(fragment=>{if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment;});job.catch(()=>{});pending.push(job);}};
   launch();
   while(pending.length){
    aborted(signal);const fragment=await pending.shift();launch();
    roots.push(fragment.root);
    for(const edge of fragment.edges)if(!keep||keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)edges.set(JSON.stringify(edge),edge);
    if(fragment.kind==="project")yield[fragment.name,fragment.node];else unavailable.push(fragment.node);
    await progress?.({phase:"assembly",done:++done,total:wanted.length});
   }
   drained=true;
  }
  async function finish(){
   if(!drained)throw fail("The node stream must be consumed before the assembly is finished","GENERATION_STREAM_OPEN");
   const external=new Set();
   if(keep){
    for(const edge of edges.values())if(global.externalNodes?.[edge.target]!==undefined)external.add(edge.target);
    let grown=true;
    while(grown){grown=false;for(const edge of global.edges??[])if((keep.has(edge.source)||external.has(edge.source))&&(keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)){edges.set(JSON.stringify(edge),edge);if(global.externalNodes?.[edge.target]!==undefined&&!external.has(edge.target)){external.add(edge.target);grown=true;}}}
   }else for(const edge of global.edges??[])edges.set(JSON.stringify(edge),edge);
   const sorted=[...edges.entries()].sort(([a],[b])=>compare(a,b)).map(([,edge])=>edge);
   const externalNodes=Object.fromEntries(Object.entries(global.externalNodes??{}).filter(([key])=>!keep||external.has(key)));
   const rows=[...(global.unavailable??[]),...unavailable].filter(row=>!keep||roots.some(root=>row.root===root||row.root.startsWith(root+"/")));
   return{dependencies:sorted,externalNodes,unavailable:rows.sort((a,b)=>compareCodePointsV1(a.root,b.root)),generation:manifest.generation,membership:manifest.membership};
  }
  return Object.freeze({nodes,finish});
 }

 /** 🧩️ Assembles the graph of exactly the owners in `names` (every owner without it) into one object: the streamed nodes collected in order, then the rest. */
 async function assemble(current,options){
  const stream=assembleStream(current,options),projects={};
  for await(const[name,node]of stream.nodes())projects[name]=node;
  return{projects,...await stream.finish()};
 }

'''
src=src[:i]+new+src[j:]
src=src.replace("return Object.freeze({read,readIndex,readFrontier,readFragment,plan,build,refresh,refreshClosure,closureOf,assemble,snapshot,","return Object.freeze({read,readIndex,readFrontier,readFragment,plan,build,refresh,refreshClosure,closureOf,assemble,assembleStream,snapshot,")
open(G+"🟨️.mjs",'w',encoding="utf8").write(src)
apply(G+"🟨️.d.mts",[
 (' assemble(current:GenerationSealedV1,options?:',' assembleStream(current:GenerationSealedV1,options?:{names?:readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):{nodes():AsyncGenerator<[string,unknown],void,undefined>;finish():Promise<Omit<GenerationAssembledV1,"projects">>};\n assemble(current:GenerationSealedV1,options?:'),
])
