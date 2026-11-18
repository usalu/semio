import{expect,test}from"bun:test";
import{createHeapSamplerV1,releaseIfGrownV1,useFootprintGateV1,FOOTPRINT_POLICY_V1}from"../🟨️.mjs";
const MB=1048576,clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};

test("R57 law: the heap census takes at most censusDutyPercent of the wall time however long the loop stays starved, every starved row is still written and carries the last counts",()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let counted=0,spent=0;
 const costMilliseconds=2000,sampler=createHeapSamplerV1({meter:{bytes:()=>1},statsSync:()=>{counted++;spent+=costMilliseconds;time.advance(costMilliseconds);return{heapSize:300*MB,extraMemorySize:50*MB,objectCount:1000+counted,objectTypeCounts:{Object:counted}};},stack:()=>"s",usage:()=>({rss:2*MB}),now:time.now,emit:fields=>rows.push(fields)});
 const began=time.now();let calls=0;
 for(;time.now()-began<600_000;calls++){time.advance(P.pulseMilliseconds);sampler.starved(time.now());}
 expect(rows.length).toBe(calls);
 expect(counted).toBeGreaterThanOrEqual(2);
 expect(spent/(time.now()-began)).toBeLessThanOrEqual(P.censusDutyPercent/100+0.005);
 expect(rows[0]!.objects).toBe(1001);expect(rows[1]!.objects).toBe(1001);expect(rows[rows.length-1]!.objects).toBe(1000+counted);
 expect(new Set(rows.map(row=>row.objects)).size).toBe(counted);
});

test("R57 law: a regular sample waits for the census quiet time instead of walking the heap again, and speaks once it passed",async()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let counted=0,rss=100*MB;
 const sampler=createHeapSamplerV1({meter:{bytes:()=>1},stats:async()=>{counted++;time.advance(1000);return{heapSize:MB,objectCount:counted};},usage:()=>({rss}),now:time.now,emit:fields=>rows.push(fields)});
 await sampler.tick();expect(rows.length).toBe(1);expect(counted).toBe(1);
 rss+=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();
 expect(rows.length).toBe(1);expect(counted).toBe(1);
 time.advance(1000*(100-P.censusDutyPercent)/P.censusDutyPercent);await sampler.tick();
 expect(rows.length).toBe(2);expect(counted).toBe(2);
});

test("R57 law: a forced release is bounded to releaseDutyPercent of the wall time, never runs below the soft limit of the grant, and runs from it",()=>{
 const P=FOOTPRINT_POLICY_V1,state={quietUntil:-Infinity};let at=1_000_000,used=0,released=0,spent=0;
 const call=(more:Record<string,unknown>={})=>releaseIfGrownV1({usage:()=>used,release:()=>{released++;spent+=3000;at+=3000;},threshold:100*MB,now:()=>at,decision:()=>"settle",state,...more});
 const began=at;
 for(;at-began<600_000;at+=1000){used+=200*MB;call();}
 expect(released).toBeGreaterThanOrEqual(2);
 expect(spent/(at-began)).toBeLessThanOrEqual(P.releaseDutyPercent/100+0.01);
 const before=released;used+=500*MB;at+=1_000_000;expect(call({decision:()=>"run"})).toBe(false);expect(released).toBe(before);
 expect(call({decision:()=>"settle"})).toBe(true);expect(released).toBe(before+1);
 used+=500*MB;at+=1_000_000;expect(call({decision:()=>"wait"})).toBe(true);
});

test("R57 law: the release decision of a process reads the footprint gate it holds: below soft nothing, from soft a release",()=>{
 let footprint=100*MB;const state={quietUntil:-Infinity,releasedAt:0},gate={grantBytes:()=>1000*MB,footprintBytes:()=>footprint,stop:()=>undefined};let released=0,used=0,at=1_000_000;
 const call=()=>releaseIfGrownV1({usage:()=>used,release:()=>{released++;at+=5;},threshold:50*MB,state,now:()=>at});
 try{
  useFootprintGateV1(gate as never);
  used+=300*MB;expect(call()).toBe(false);expect(released).toBe(0);
  footprint=FOOTPRINT_POLICY_V1.softPercent*10*MB+MB;expect(call()).toBe(true);expect(released).toBe(1);
 }finally{useFootprintGateV1(undefined);}
 used+=300*MB;at+=1_000_000;expect(call()).toBe(true);
});
