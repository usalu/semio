 /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. The verification view of the fact index (the rows the plan read stay materialized in it) is released before `compute` runs, so the inference never carries it; a later plan in this process opens it again. */
 async function build(request){
  let wave,total=0,last;
  for(;;){
   let computed;
   try{last=await buildOnce(wave?{...request,hint:wave}:request,result=>{computed=result;});}finally{await computed?.dispose?.();}
   total+=last.computed??0;
   if(last.published&&last.current){try{await request.published?.(last.current.manifest.generation,{done:total,total:total+(last.deferred?.plan?.dirty?.size??0),...(last.phases?{phases:last.phases}:{})});}catch{}}
   if(!last.more||!last.published)return{...last,computed:total};
   wave=last.deferred;
  }
 }
 async function buildOnce({identity,implementation,membership,compute,prime,signal,progress,hint,only,observedAtMs=now(),coverage,claim},own){
  const marks={};if(!processStartedV1){processStartedV1=true;marks.start=Math.round(process.uptime()*1000);}Object.assign(marks,carriedStagesV1);for(const key of Object.keys(carriedStagesV1))delete carriedStagesV1[key];let markedAt=now();const mark=name=>{const at=now();marks[name]=(marks[name]??0)+at-markedAt;markedAt=at;};
  await profilePhaseV1("graph.build.prime",async()=>prime?.(signal));mark("prime");
  const current=await read(signal),usable=current?.manifest&&current.manifest.identity===identity?current.manifest:undefined;
  const reuse=hint&&usable&&current.pointer.generation===hint.generation&&(hint.wave!==undefined||(hint.holds?await hint.holds().catch(()=>false):now()-hint.atMs<=P.hintMaximumAgeMilliseconds));
  let previous=usable&&!reuse?{manifest:usable,frontier:await readFrontier(usable.frontier)}:undefined,live=reuse?hint.live:await profilePhaseV1("graph.build.membership",async()=>{const began=now();try{return await membership(previous?.frontier?previous:undefined,signal);}finally{marks.membership=now()-began;}});
  const found=reuse?hint.plan:await profilePhaseV1("graph.build.plan",()=>plan({current,identity,changes:previous?.frontier?live.changes:undefined,...previous?.frontier&&live.dirty?{journalDirty:live.dirty}:{},only,skipEnvironment:coverage!==undefined,signal,progress}));
  const priority=readPriorityV1();mark("plan");for(const[name,ms]of Object.entries(found.times??{}))marks["plan."+name]=ms;
  if(found.kind==="clean"){
   writePendingV1(process.env,current?.pointer.generation??-1,[],priority);
   if(usable.membership===live.digest&&!found.refresh?.size)return{current,built:false,published:false,plan:found,computed:0};
   return{...await refreshMetadata({current,live,signal,held:found.refresh}),built:false,plan:found,computed:0};
  }
  const cold=found.kind==="cold",owners0=cold?{}:usable.owners,names=cold?[]:orderOwnersV1([...found.dirty].sort(compareCodePointsV1),priority),offset=hint?.wave?.done??0,wave=names.slice(0,hint?.wave?.size??scaledOwnerWaveV1(P.ownerWave,process.env.SEMIO_SLICE_WAVE_BASIS_POINTS)),rest=names.slice(wave.length),waveStartedMs=now();
  if(!cold)writePendingV1(process.env,current?.pointer.generation??-1,names,priority);
  const want=claim&&await claim.extend({owners:[...names,...found.removed],structural:cold||found.envStale||found.added.length>0,signal});
  if(want&&!want.held)return{current,built:false,published:false,computed:0,plan:found,needsBuild:true,pending:{generation:current?.pointer.generation??-1,roots:{},...want.request}};
  aborted(signal);
  const spec={cold,owners:wave,more:rest.length>0,total:offset+names.length,done:offset,discover:found.added,removed:found.removed,changed:cold?[]:(live.changes??[]).filter(change=>!change.directory).map(change=>change.path),previousEnv:cold?null:usable.env,previousParts:cold?undefined:usable.envParts,envStale:found.envStale,generation:(current?.pointer.generation??-1)+1,coldReason:cold?found.reason:undefined,reasons:found.reasons??[],staleRows:found.stale??0,staleKinds:found.staleKinds,
   stubs:Object.fromEntries(Object.entries(owners0).map(([name,row])=>[name,{root:row.root,kind:row.kind,targets:row.targets,...(row.nativeRoot?{nativeRoot:row.nativeRoot}:{}),...(row.config?{config:row.config}:{}),...(row.tag?{tag:row.tag}:{}),...(row.reads?{reads:row.reads}:{})}]))};
  previous=undefined;indexCache.clear();
    const result=await profilePhaseV1("graph.build.compute",()=>compute(spec,signal));own(result);mark("compute");
  stats.computed+=result.owners.length;aborted(signal);
  const envDigest=result.env.digest,owners={...owners0},dropped=new Set(found.removed),upserts=new Map(),append=new Map(),returned=new Set(),stored=[];
  for(const name of dropped)delete owners[name];
  for(const owner of result.owners){
   aborted(signal);
   if(owner.partial==="edges"){
    const old=owners[owner.name];if(!old)throw fail("Edge refresh requires a published owner","GENERATION_INCOMPLETE");
    stored.push(async()=>{const fragment=await readFragment(old);if(!fragment)throw fail("Edge refresh requires the published fragment","GENERATION_INCOMPLETE");const saved=await store(P.fragments,{...fragment,edges:owner.edges??await result.reEdge(owner.name,fragment),edgesEnv:envDigest},".json.br",P.maximumFragmentBytes),{reads:_,...base}=old,reads=owner.reads??result.readsOf?.(owner.name);owners[owner.name]={...base,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest,...(reads?{reads}:{})};});
    if(owner.rows?.length)append.set(owner.name,owner.rows);
    continue;
   }
   returned.add(owner.name);
   stored.push(async()=>{
    const body={version:1,name:owner.name,root:owner.root,kind:owner.kind,node:owner.node,edges:owner.edges,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{})};
    const saved=await store(P.fragments,body,".json.br",P.maximumFragmentBytes);
    owners[owner.name]={root:owner.root,kind:owner.kind,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),...(owner.reads?{reads:owner.reads}:{})};
   });
   upserts.set(owner.name,owner.rows??[]);
  }
    const noisy=returned.size>P.ownerWave;if(noisy)say?.("storing",{count:returned.size});
  let fragmentsDone=0;if(stored.length)await progress?.({phase:"fragments",done:0,total:stored.length});
  await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,async job=>{await job();if(stored.length)await progress?.({phase:"fragments",done:++fragmentsDone,total:stored.length});},signal));mark("fragments");
  for(const name of wave)if(!returned.has(name)&&!dropped.has(name)){delete owners[name];dropped.add(name);}
  for(const[name,row]of Object.entries(owners))if(row.tag&&result.replaceTags?.includes(row.tag)&&!returned.has(name)){delete owners[name];dropped.add(name);}
  for(const name of returned)dropped.delete(name);
  const renamed=new Map();
  for(const name of returned){const row=owners[name];if(row&&!row.tag&&row.kind==="project")renamed.set(row.root,name);}
  for(const[name,row]of Object.entries(owners))if(!returned.has(name)&&!row.tag&&row.kind==="project"&&renamed.has(row.root)){delete owners[name];dropped.add(name);}
  for(const name of result.carried??[])if(owners[name]&&!returned.has(name))owners[name]={...owners[name],edgesEnv:envDigest};
  for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==envDigest)throw fail("Every owner needs edges of the published environment: "+name+" ("+JSON.stringify({kind:row.kind,tag:row.tag??null,edges:String(row.edgesEnv??"none").slice(0,12),environment:String(envDigest).slice(0,12),root:row.root,dirty:wave.includes(name),returned:returned.has(name),dropped:dropped.has(name),carried:(result.carried??[]).includes(name),sameRootReturned:[...returned].filter(other=>owners[other]?.root===row.root)})+")","GENERATION_INCOMPLETE");
  upserts.set(P.envOwner,result.env.rows??[]);
  for(const[name,rows]of derivedUpsertsV1(result.derived??[],found.refresh))upserts.set(name,rows);
    if(noisy)say?.("indexing",{count:Object.keys(owners).length});
    let index=await profilePhaseV1("graph.build.patch",async()=>patchView(cold?emptyFactIndexViewV1():await readIndexView(usable.factIndex),{drop:[...dropped],upserts,append},signal));
  
  const orphans=orphanedDerivedOwnersV1(index);if(orphans.length)index=await patchView(index,{drop:orphans},signal);
  const unsupported=unsupportedDerivedV1(index);if(unsupported.length)throw fail("Derived facts need a supporting computation: "+unsupported.slice(0,3).map(row=>row.path.slice(0,96)).join(", "),"GENERATION_INCOMPLETE");

  mark("patch");
  if(noisy)say?.("sealing",{generation:(current?.pointer.generation??-1)+1});
  const packedIndex=await profilePhaseV1("graph.build.pack",()=>packFactIndexBytesV1(index.document.buffer,P.maximumIndexBytes)),indexStored=await profilePhaseV1("graph.build.keep",()=>keep(P.indexes,packedIndex.bytes,".facts.br"));
  
  mark("pack");
  const globalStored=await store(P.fragments,{version:1,name:"@global",value:result.global??null},".json.br",P.maximumFragmentBytes);
  const frontierStored=await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes);
  const expectedGeneration=current?.pointer.generation??-1;
  const{owners:_table,...envParts}=result.env.parts??{},next={version:1,generation:expectedGeneration+1,parent:current?.pointer.manifest??null,identity,implementation,env:envDigest,membership:live.digest,frontier:frontierStored.digest,factIndex:indexStored.digest,global:globalStored.digest,...(result.env.parts?{envParts}:{}),owners,built:{kind:cold?"cold":"delta",base:cold?null:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
  const manifestDigest=await writeManifest(next);aborted(signal);
  const verified=stamp&&!rest.length?mergeVerifiedStampV1({owners:Object.keys(owners).sort(compareCodePointsV1),scope:cold?undefined:only,observedAtMs,carried:cold?undefined:coverage}):undefined;
  let outcome=await profilePhaseV1("graph.build.publish",()=>publish({manifestDigest,expectedGeneration,verified,signal})),done=outcome.published?{next,index,packed:packedIndex,kept:indexStored}:undefined;
  mark("publish");
  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1({read,readIndex:readIndexView,writeManifest,publish,patch:(base,patch)=>patchView(base,patch,signal),pack:value=>packFactIndexBytesV1(value.document.buffer,P.maximumIndexBytes),keep:bytes=>keep(P.indexes,bytes,".facts.br"),key:rowKeyV1,stats},{base:current,mine:next,patch:{drop:[...dropped],upserts,append},verified,signal});if(merged.outcome){outcome=merged.outcome;done=merged;}}
  
  if(!done)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};
  indexCache.set("view:"+done.kept.digest,done.index);
  await profilePhaseV1("graph.build.indexed",()=>settler.markIndexed({held:found.refresh,dropped:(result.derived??[]).map(fact=>derivedOwnerNameV1(fact.row))}).catch(()=>undefined));
  if(!done.theirs)writePendingV1(process.env,outcome.pointer.generation,rest,priority);
  const deferred=!done.theirs?{generation:outcome.pointer.generation,live,atMs:now(),wave:{done:offset+wave.length,size:nextOwnerWaveV1({size:wave.length,elapsedMilliseconds:now()-waveStartedMs,inferred:result.inferred,basisPoints:process.env.SEMIO_SLICE_WAVE_BASIS_POINTS})},plan:{...found,times:{},kind:"delta",dirty:new Set(rest),added:[],removed:[],pending:[],envStale:false,reasons:[],refresh:new Map(),stale:0,staleKinds:{}}}:undefined;
  if(!rest.length)await profilePhaseV1("graph.build.stamps",async()=>doubt?.published?.({rows:done.index.rows,observedAt:observedAtMs}).catch(()=>false));
  if(!rest.length&&await(claim?claim.alone({signal}).catch(()=>false):true))await profilePhaseV1("graph.build.retire",()=>retire({signal}).catch(error=>{if(signal?.aborted)throw error;}));
  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:result.owners.length,phases:{...marks,...Object.fromEntries(Object.entries(result.phases??{}).map(([name,ms])=>["compute."+name,ms]))},...(rest.length?{more:true,...deferred?{deferred}:{}}:{}),carried:result.carried?.length??0,...(done.theirs?{merged:done.theirs.length}:{}),index:{rows:done.index.rows.length,owners:done.index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:done.packed.expandedBytes,packedBytes:done.kept.bytes}};
 }

