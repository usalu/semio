import{expect,test}from"bun:test";
import{GRAPH_MEMORY_POLICY_V1 as policy,createParentCollectorV1}from"../🟨️.mjs";
const MB=1048576;

test("R57 law: the forced collections of the parent take at most collectDutyPercent of the wall time however long a collection takes and however fast the garbage returns",()=>{
 let clock=0,held=4096*MB,collections=0,spent=0;const costMilliseconds=8000;
 const collector=createParentCollectorV1({ports:{sample:()=>held,collect:()=>{collections++;spent+=costMilliseconds;clock+=costMilliseconds;held=1000*MB;}},now:()=>clock});
 const began=clock;
 for(;clock-began<1_200_000;clock+=100){held+=3000*MB;collector.afterProject();collector.afterProject();}
 expect(collections).toBeGreaterThanOrEqual(2);
 expect(spent/(clock-began)).toBeLessThanOrEqual(policy.collectDutyPercent/100+0.02);
});

test("R57 law: a cheap collection keeps the old pace: not twice within the minimum interval, again once it passed",()=>{
 let clock=0,held=4096*MB,collections=0;
 const collector=createParentCollectorV1({ports:{sample:()=>held,collect:()=>{collections++;held=1000*MB;}},now:()=>clock});
 held=4096*MB;collector.afterProject();expect(collector.afterProject()).toBe(true);
 held=4096*MB;clock+=policy.collectMinimumIntervalMilliseconds-1;collector.afterProject();expect(collector.afterProject()).toBe(false);
 clock+=1;collector.afterProject();expect(collector.afterProject()).toBe(true);expect(collections).toBe(2);
});
