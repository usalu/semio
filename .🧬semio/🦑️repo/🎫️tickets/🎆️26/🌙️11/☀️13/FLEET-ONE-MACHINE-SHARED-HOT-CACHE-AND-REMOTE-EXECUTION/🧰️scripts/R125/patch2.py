import json,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125/"
def sub(path,a,b):
    t=open(path,encoding="utf8").read()
    if a not in t: sys.exit("anchor missing: "+a[:70])
    open(path,"w",encoding="utf8").write(t.replace(a,b,1))
d=S+"stage/decision.ts"
sub(d,"rampIdleObservedBasisPoints?:number;","rampIdleObservedBasisPoints?:number;rampIdleKeepMegabytes?:number;rampIdleKeepBasisPoints?:number;")
sub(d,"const unrealizedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(0,grant.reservedMegabytes-(grant.observedMegabytes??0)):0,committedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;",
 "const unrealizedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(0,keptHostMegabytesV1(policy,grant,nowMs)-(grant.observedMegabytes??0)):0,committedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(keptHostMegabytesV1(policy,grant,nowMs),grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;")
sub(d,"/** 🧮️ Decides, for a host sample, the live grants and the queue,",'''/** 🪶️ What a ramping grant is still charged for memory it does not hold: its reservation, except that a grant which held its ramp slot `rampIdleAfterMilliseconds` while using less than `rampIdleObservedBasisPoints` of it is idle, not growing, and keeps only the larger of `rampIdleKeepMegabytes` and `rampIdleKeepBasisPoints` of what it really holds (never more than it reserved): a reservation is what a job needs, so the unused rest of a worst-case estimate is released to the others; without those two numbers the whole reservation stays charged. A pure function of its arguments.
 * @see ../🧪️tests/🟦️.ts */
export function keptHostMegabytesV1(policy:HostCapacityPolicyV1,grant:Pick<HostGrantV1,"reservedMegabytes"|"observedMegabytes"|"admittedAtMs"|"resumedAtMs">,nowMs:number):number{
 const rules=policy.memory,idle=rules.rampIdleAfterMilliseconds!==undefined&&rules.rampIdleObservedBasisPoints!==undefined&&rules.rampIdleKeepMegabytes!==undefined&&rules.rampIdleKeepBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=rules.rampIdleAfterMilliseconds&&typeof grant.observedMegabytes==="number"&&grant.observedMegabytes*10000<grant.reservedMegabytes*rules.rampIdleObservedBasisPoints;
 return idle?Math.min(grant.reservedMegabytes,Math.max(rules.rampIdleKeepMegabytes!,Math.ceil(grant.observedMegabytes!*rules.rampIdleKeepBasisPoints!/10000))):grant.reservedMegabytes;
}

/** 🧮️ Decides, for a host sample, the live grants and the queue,''')
sub(d,"ramping=(grant:HostGrantV1)=>grant.quiet!==true&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),committed=(grant:HostGrantV1)=>ramping(grant)?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0)","ramping=(grant:HostGrantV1)=>grant.quiet!==true&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),committed=(grant:HostGrantV1)=>ramping(grant)?Math.max(keptHostMegabytesV1(policy,grant,nowMs),grant.observedMegabytes??0)")
p=S+"stage/schema.json";j=json.load(open(p,encoding="utf8"));props=j["definitions"]["Policy"]["properties"]["memory"]["properties"]
props["rampIdleKeepMegabytes"]={"type":"integer","minimum":0,"description":"With rampIdleKeepBasisPoints: what an idle ramping grant (see rampIdleAfterMilliseconds) is still charged beside what it holds, so the unused rest of its reservation is released to others."}
props["rampIdleKeepBasisPoints"]={"type":"integer","minimum":10000,"maximum":100000}
open(p,"w",encoding="utf8").write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
q=S+"stage/policy.json";pj=json.load(open(q,encoding="utf8"));m=pj["hostCapacity"]["memory"];m["rampIdleKeepMegabytes"]=3072;m["rampIdleKeepBasisPoints"]=15000
open(q,"w",encoding="utf8").write(json.dumps(pj,ensure_ascii=False,indent=2)+"\n")
t=S+"stage/farm-width.test.ts"
sub(t,'import {evaluateHostCapacityV1,hostClassBoundV1,','import {evaluateHostCapacityV1,hostClassBoundV1,keptHostMegabytesV1,')
open(t,"a",encoding="utf8").write('''
test("a reservation is released to what the job uses: a read that reserved 14336 MB and holds 1875 MB for minutes is charged at three gigabytes, a young or busy one in full",()=>{
 const grant=(observed:number,age:number)=>({...parent(observed,14336),admittedAtMs:now-age});
 expect(keptHostMegabytesV1(policy,grant(1875,600_000),now)).toBe(3072);
 expect(keptHostMegabytesV1(policy,grant(1875,5_000),now)).toBe(14336);
 expect(keptHostMegabytesV1(policy,grant(9000,600_000),now)).toBe(14336);
 expect(keptHostMegabytesV1(policy,grant(2800,600_000),now)).toBe(4200);
 expect(jobs(sample(32,10,17),[grant(1875,600_000)],ask(10,{essential:true}))).toBeGreaterThanOrEqual(7);
});
''')
sub(d,"unrealized=Math.max(0,holder.reservedMegabytes-(holder.observedMegabytes??0));","unrealized=Math.max(0,keptHostMegabytesV1(policy,holder,nowMs)-(holder.observedMegabytes??0));")
