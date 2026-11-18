B="baseb/";W="workb/"
rd=lambda n:open(B+n,encoding="utf8").read()
def sub(t,old,new,cnt=1):
    assert t.count(old)==cnt,(old[:60],t.count(old));return t.replace(old,new)
t=rd("1")
t=sub(t,'beats.set("flush support",Date.now())','beats.set("supports",Date.now())')
t=sub(t,'await beat("flush support",built,chunk.length)','await beat("supports",built,chunk.length)')
t=sub(t,'await beat("flush parts",partStats.decided,partStats.asked,true)','await beat("decisions",partStats.decided,partStats.asked,true)')
t=sub(t,'await beat("flush parts",partStats.decided,partStats.asked);','await beat("decisions",partStats.decided,partStats.asked);')
open(W+"1","w",encoding="utf8").write(t)
t=rd("2")
t=sub(t,'reports `flush parts N of M`','reports `decisions N of M`')
t=sub(t,'/** 🧪️ The settle names its work: `flush parts N of M` rises','/** 🧪️ The settle names its work: `decisions N of M` rises')
t=sub(t,'test("the part decisions report flush parts counters"','test("the part decisions report decisions counters"')
t=sub(t,'row.phase==="flush parts"','row.phase==="decisions"')
t=sub(t,'toEqual({phase:"flush parts",','toEqual({phase:"decisions",')
t=sub(t,'import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";','import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";\nimport{createGraphPhaseClockV1}from"../../../../⏱️phases/🟨️.mjs";\nimport{createGenerationProgressV1}from"../../../../🕰️generations/🔌️load/🟨️.mjs";')
t+='''
/** 🧪️ Every phase a settle reports is accepted by the graph phase counters of a generation load (a word of lower-case letters, begun once with a finite total): a name the counters refuse ends the whole slice read with "Closed graph phase and finite total required" (g1-m10, `flush parts`, reference 3a9fba88). The events of a cold settle, a warm settle and a settle with every closure stale go through the real generation progress and a real clock. */
test("every phase of a settle is accepted by the graph phase counters",async()=>{
 const root=scratch();
 try{
  const cold=harness(root);const first=await cold.settle();
  await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
  const warm=harness(root);await warm.settle();
  const stale=harness(root,{stale:new Set([text(rowAt(2*STEP+7)),text(rowAt(10*STEP+3))])});await stale.settle();
  const events=[...cold.progress,...warm.progress,...stale.progress],phases=new Set(events.map(row=>row.phase));
  expect([...phases].every(phase=>/^[a-z]+$/u.test(phase)),[...phases].join(",")).toBe(true);
  expect(phases.has("decisions")).toBe(true);
  for(const run of [cold,warm,stale]){
   const published:string[]=[],report=createGenerationProgressV1(createGraphPhaseClockV1({publish:resource=>{published.push(resource);},minimumMilliseconds:0}) as never);
   for(const event of run.progress)await report(event);
   expect(published.length).toBeGreaterThan(0);
  }
 }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
},120000);
'''
open(W+"2","w",encoding="utf8").write(t)
