import json
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:70],s.count(a))
    return s.replace(a,b)

# lifecycle: the delay until the next start
s=rd(S+"base/lifecycle.ts")
s+='''
/** ⏳️ How long until the pool may start another child although the target asks for one: the longest of the spacing since the last start, the minimum age of a child that still has no owner and, while the allowed number of children is loading, the shortest warm-up left; `0` when a start is allowed now. The number the pool states when fewer children run than the target.
 * @see ../🧪️tests/🧮️lifecycle/🟦️.ts */
export function ownerPoolStartDelayV1(input:Readonly<{children:readonly OwnerPoolLifeV1[];lastStartMs:number;now:number;pollMilliseconds:number}>,policy:typeof P=P):number{
 const spacing=policy.startSpacingPolls*input.pollMilliseconds-(input.now-input.lastStartMs),minimumAge=policy.childMinimumAgePolls*input.pollMilliseconds,warmup=policy.startWarmupPolls*input.pollMilliseconds;
 const young=input.children.filter(child=>!child.drained&&child.running===0&&input.now-child.startedMs<minimumAge).map(child=>minimumAge-(input.now-child.startedMs));
 const loading=input.children.filter(child=>!child.ready&&!child.drained&&input.now-child.startedMs<warmup).map(child=>warmup-(input.now-child.startedMs));
 return Math.max(0,spacing,young.length===0?0:Math.max(...young),loading.length<policy.startingMaximum?0:Math.min(...loading));
}
'''
wr(S+"stage/lifecycle.ts",s)
t=rd(S+"base/lifecycle.test.ts")
t=sub(t,'import{ownerPoolDrainPickV1,ownerPoolMayStartV1}from"../../🧮️lifecycle/🟦️.ts";','import{ownerPoolDrainPickV1,ownerPoolMayStartV1,ownerPoolStartDelayV1}from"../../🧮️lifecycle/🟦️.ts";')
t+='''
test("the delay to the next start is the number that holds it: the spacing, the age of an idle child, the warm-up of a loading child, and zero exactly when a start is allowed",()=>{
 const at=(children:ReturnType<typeof child>[],lastStartMs:number,now:number)=>ownerPoolStartDelayV1({children,lastStartMs,now,pollMilliseconds:poll});
 expect(at([],-Infinity,1000)).toBe(0);
 expect(at([],1000,1000+3000)).toBe(P.startSpacingPolls*poll-3000);
 expect(at([child({startedMs:5000})],-Infinity,10000)).toBe(P.childMinimumAgePolls*poll-5000);
 const loading=Array.from({length:P.startingMaximum},(_all,index)=>child({ready:false,startedMs:10000-index*1000}));
 expect(at(loading,-Infinity,20000)).toBe(Math.min(...loading.map(row=>P.startWarmupPolls*poll-(20000-row.startedMs))));
 for(const now of[0,999,20000,100000])for(const children of[[],[child({startedMs:now-4000})],loading])for(const lastStartMs of[-Infinity,now-1000,now-P.startSpacingPolls*poll]){
  const allowed=ownerPoolMayStartV1({children,survivors:0,target:99,lastStartMs,now,pollMilliseconds:poll});
  expect(at(children,lastStartMs,now)===0,JSON.stringify({now,children,lastStartMs})).toBe(allowed);
 }
});
'''
wr(S+"stage/lifecycle.test.ts",t)

# schema texts
sc=rd(S+"stage/schema.json")
sc=sub(sc,'''          "reasonYielding": "waiting: the machine's own work holds the capacity; the pool keeps one child and takes what is left",
''','''          "reasonYielding": "waiting: the machine's own work holds the capacity; the pool keeps one child and takes what is left",
          "reasonMemory": "waiting: the next child needs {need} MB but only {free} MB are free above the {floor} MB floor ({held} MB are held for the machine's own work, {children} children run)",
          "reasonCores": "waiting: no idle core for the next child ({idle} of {cores} cores idle, {booked} booked by the machine's own work, {children} children run)",
          "reasonStarting": "starting: {children} of {target} children run, the next one may start in {seconds} s",
          "reasonWaiting": "waiting: the machine's own work has waited {seconds} s for memory or cores, the children stop until it runs",
          "reasonPressure": "waiting: the machine is under memory pressure or swaps ({swap} MB/s out), no child runs",
''')
sc=sub(sc,'''          "reasonYielding": "wartet: die eigene Arbeit der Maschine hält die Kapazität; der Pool behält einen Kindprozess und nimmt, was übrig ist",
''','''          "reasonYielding": "wartet: die eigene Arbeit der Maschine hält die Kapazität; der Pool behält einen Kindprozess und nimmt, was übrig ist",
          "reasonMemory": "wartet: der nächste Kindprozess braucht {need} MB, aber nur {free} MB sind über der Grenze von {floor} MB frei ({held} MB hält die eigene Arbeit der Maschine zurück, {children} Kindprozesse laufen)",
          "reasonCores": "wartet: kein freier Kern für den nächsten Kindprozess ({idle} von {cores} Kernen frei, {booked} von der eigenen Arbeit der Maschine gebucht, {children} Kindprozesse laufen)",
          "reasonStarting": "startet: {children} von {target} Kindprozessen laufen, der nächste darf in {seconds} s starten",
          "reasonWaiting": "wartet: die eigene Arbeit der Maschine wartet seit {seconds} s auf Speicher oder Kerne, die Kindprozesse enden, bis sie läuft",
          "reasonPressure": "wartet: die Maschine steht unter Speicherdruck oder lagert aus ({swap} MB/s), kein Kindprozess läuft",
''')
sc=sub(sc,'''            "yielding",
            "no-current-wave",''','''            "yielding",
            "pressure",
            "waiting",
            "starting",
            "no-current-wave",''')
wr(S+"stage/schema.json",sc)

# runner
r=rd(S+"base/runner.mjs")
r=sub(r,'import { ownerPoolDrainPickV1, ownerPoolMayStartV1 } from','import { ownerPoolDrainPickV1, ownerPoolMayStartV1, ownerPoolStartDelayV1 } from')
r=sub(r,'import { OWNER_POOL_POLICY_V1 as P, ownerPoolChildLimitsV1, ownerPoolAcceptsRevisionV1, ownerPoolChildMegabytesV1, ownerPoolKeepWaveV1, ownerPoolLiveChildrenV1, ownerPoolReopenMillisecondsV1, ownerPoolTargetV1, ownerPoolWhyIdleV1,','import { OWNER_POOL_POLICY_V1 as P, ownerPoolChildLimitsV1, ownerPoolAcceptsRevisionV1, ownerPoolChildMegabytesV1, ownerPoolKeepWaveV1, ownerPoolLiveChildrenV1, ownerPoolReopenMillisecondsV1, ownerPoolSizingV1, ownerPoolWhyIdleV1,')
r=sub(r,'''export const defaultOwnerPoolCapacityV1 = async ({ children, childMegabytes }) => ownerPoolTargetV1(''','''export const defaultOwnerPoolCapacityV1 = async ({ children, childMegabytes }) => ownerPoolSizingV1(''')
r=sub(r,'const REASONS = { yielding: "reasonYielding", revision','const REASONS = { yielding: "reasonYielding", starting: "reasonStarting", memory: "reasonMemory", cores: "reasonCores", load: "reasonCores", waiting: "reasonWaiting", revision') if False else r
wr(S+"stage/runner.mjs",r)
