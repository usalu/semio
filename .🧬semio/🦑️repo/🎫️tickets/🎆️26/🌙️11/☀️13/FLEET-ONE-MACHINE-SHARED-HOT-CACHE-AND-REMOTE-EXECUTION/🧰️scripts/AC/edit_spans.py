import sys
def edit(path,pairs,out):
    t=open(path).read()
    for old,new in pairs:
        assert t.count(old)==1,(path,t.count(old),old[:70])
        t=t.replace(old,new)
    open(out,'w').write(t)
P='profilePhaseV1'
gens=[
('import{selectDoubtfulRowsV1}from"./🔎️doubt/🟨️.mjs";','import{selectDoubtfulRowsV1}from"./🔎️doubt/🟨️.mjs";\nimport{profilePhaseV1}from"../../⏱️profile/🟨️.mjs";'),
('const index=await readIndexView(manifest.factIndex);if(!index)return cold("index");','const index=await profilePhaseV1("graph.plan.index",()=>readIndexView(manifest.factIndex));if(!index)return cold("index");'),
('candidates=doubt?await selectDoubtfulRowsV1(index,verifiable,doubt,{signal,progress,chunk:P.doubtChunkRows}):verifiable;','candidates=doubt?await profilePhaseV1("graph.plan.doubt",()=>selectDoubtfulRowsV1(index,verifiable,doubt,{signal,progress,chunk:P.doubtChunkRows})):verifiable;'),
('const{stale}=await staleFactRowsV1(index,candidates,hold,{signal,progress,...doubt?{budget:Infinity}:{}});','const{stale}=await profilePhaseV1("graph.plan.facts",()=>staleFactRowsV1(index,candidates,hold,{signal,progress,...doubt?{budget:Infinity}:{}}));'),
('settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{}}),','settled=await profilePhaseV1("graph.plan.settle",()=>settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{}})),'),
('  await prime?.(signal);\n  const current=await read(signal)','  await profilePhaseV1("graph.build.prime",async()=>prime?.(signal));\n  const current=await read(signal)'),
('live=reuse?hint.live:await membership(previous?.frontier?previous:undefined,signal);\n  const found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,...previous?.frontier&&live.dirty?{journalDirty:live.dirty}:{},only,skipEnvironment:coverage!==undefined,signal,progress});',
 'live=reuse?hint.live:await profilePhaseV1("graph.build.membership",()=>membership(previous?.frontier?previous:undefined,signal));\n  const found=reuse?hint.plan:await profilePhaseV1("graph.build.plan",()=>plan({current,identity,changes:previous?.frontier?live.changes:undefined,...previous?.frontier&&live.dirty?{journalDirty:live.dirty}:{},only,skipEnvironment:coverage!==undefined,signal,progress}));'),
('const result=await compute(spec,signal);own(result);','const result=await profilePhaseV1("graph.build.compute",()=>compute(spec,signal));own(result);'),
('    await mapLimit(stored,P.readConcurrency,job=>job(),signal);','    await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));'),
('    let index=await patchView(cold?emptyFactIndexViewV1():await readIndexView(usable.factIndex),{drop:[...dropped],upserts,append},signal);','    let index=await profilePhaseV1("graph.build.patch",async()=>patchView(cold?emptyFactIndexViewV1():await readIndexView(usable.factIndex),{drop:[...dropped],upserts,append},signal));'),
('  const packedIndex=packFactIndexBytesV1(index.document.buffer,P.maximumIndexBytes),indexStored=await keep(P.indexes,packedIndex.bytes,".facts.br");','  const packedIndex=await profilePhaseV1("graph.build.pack",()=>packFactIndexBytesV1(index.document.buffer,P.maximumIndexBytes)),indexStored=await profilePhaseV1("graph.build.keep",()=>keep(P.indexes,packedIndex.bytes,".facts.br"));'),
('  let outcome=await publish({manifestDigest,expectedGeneration,verified,signal}),done=','  let outcome=await profilePhaseV1("graph.build.publish",()=>publish({manifestDigest,expectedGeneration,verified,signal})),done='),
('  await settler.markIndexed({held:found.refresh,dropped:(result.derived??[]).map(fact=>derivedOwnerNameV1(fact.row))}).catch(()=>undefined);','  await profilePhaseV1("graph.build.indexed",()=>settler.markIndexed({held:found.refresh,dropped:(result.derived??[]).map(fact=>derivedOwnerNameV1(fact.row))}).catch(()=>undefined));'),
('  if(!rest.length)await doubt?.published?.({rows:done.index.rows,observedAt:observedAtMs}).catch(()=>false);','  if(!rest.length)await profilePhaseV1("graph.build.stamps",async()=>doubt?.published?.({rows:done.index.rows,observedAt:observedAtMs}).catch(()=>false));'),
('await retire({signal}).catch(error=>{if(signal?.aborted)throw error;});','await profilePhaseV1("graph.build.retire",()=>retire({signal}).catch(error=>{if(signal?.aborted)throw error;}));'),
('   const live=reuse?planned.live:await membership(previous.frontier?previous:undefined,signal);\n   const verdict=reuse?planned.plan:await plan({current:first,identity,changes:previous.frontier?live.changes:undefined,...previous.frontier&&live.dirty?{journalDirty:live.dirty}:{},only:scope,skipEnvironment:coverage!==undefined,signal,progress});',
 '   const live=reuse?planned.live:await profilePhaseV1("graph.refresh.membership",()=>membership(previous.frontier?previous:undefined,signal));\n   const verdict=reuse?planned.plan:await profilePhaseV1("graph.refresh.plan",()=>plan({current:first,identity,changes:previous.frontier?live.changes:undefined,...previous.frontier&&live.dirty?{journalDirty:live.dirty}:{},only:scope,skipEnvironment:coverage!==undefined,signal,progress}));'),
]
host=[
('import {journalGraphSeamsV1} from "../../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";','import {journalGraphSeamsV1} from "../../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";\nimport {profileMarkV1,profilePhaseV1} from "../../⏱️profile/🟨️.mjs";'),
('    const live=await seams.membership(previous,signal);\n    state.stable','    const live=await profilePhaseV1("graph.host.membership",()=>seams.membership(previous,signal));\n    state.stable'),
('      settled:async(input:{rows:readonly unknown[];ids:readonly number[];stale:ReadonlySet<number>})=>{if(epochUsed)await epoch.settled(input as never);},','      settled:async(input:{rows:readonly unknown[];ids:readonly number[];stale:ReadonlySet<number>})=>{if(epochUsed)await profilePhaseV1("graph.host.stamps",()=>epoch.settled(input as never));journalPort.manifest.flushVerdicts();},'),
('      await upgradeBuildCapacityBeforeRebuildV1();\n      let image:string|undefined;\n      const result=await withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);});',
 '      await profilePhaseV1("graph.host.capacity",()=>upgradeBuildCapacityBeforeRebuildV1());\n      let image:string|undefined;\n      const result=await profilePhaseV1("graph.host.custody",()=>withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;profileMarkV1("graph.host.custody-held");return body(store,control,claim);}));'),
]
exec(open(__import__('os').path.dirname(__import__('os').path.abspath(__file__))+'/exch_pairs.py').read())
if __name__=="__main__":
    edit('gens.mjs',gens,'gens.new.mjs');edit('host.ts',host,'host.new.ts');edit('exch.ts',exch,'exch.new.ts');print("ok")
