import json,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125/"
def sub(path,a,b):
    t=open(path,encoding="utf8").read()
    if a not in t: sys.exit("anchor missing: "+a[:70])
    open(path,"w",encoding="utf8").write(t.replace(a,b,1))
d=S+"stage/decision.ts"
sub(d,"rampIdleKeepBasisPoints?:number;","rampIdleKeepBasisPoints?:number;childStartMegabytes?:number;")
sub(d,'''export function keptHostMegabytesV1(policy:HostCapacityPolicyV1,grant:Pick<HostGrantV1,"reservedMegabytes"|"observedMegabytes"|"admittedAtMs"|"resumedAtMs">,nowMs:number):number{
 const rules=policy.memory,''','''export function keptHostMegabytesV1(policy:HostCapacityPolicyV1,grant:Pick<HostGrantV1,"reservedMegabytes"|"observedMegabytes"|"admittedAtMs"|"resumedAtMs"|"jobs"|"holder"|"label">,nowMs:number):number{
 const rules=policy.memory;
 if(rules.childStartMegabytes!==undefined&&(grant.holder!==undefined||grant.label==="graph-farm"))return Math.min(grant.reservedMegabytes,Math.max(grant.observedMegabytes??0,grant.jobs*rules.childStartMegabytes));
 const''')
sub(d,"/** 🪶️ What a ramping grant is still charged for memory it does not hold: its reservation, except that","/** 🪶️ What a ramping grant is still charged for memory it does not hold: its reservation, except that a farm child (a grant that names a holder or carries the label `graph-farm`, whose memory sits inside the tree of the read and so inside what the host already reports used) is charged `childStartMegabytes` per job from its first second, or what it measurably holds when that is more, never more than it reserved, so an idle or booting child holds only its resident memory; and that")
p=S+"stage/phases.ts"
sub(p,"const floor=Math.max(assembly?.megabytes??0,plan?.megabytes??0),","const declared=hostClassCeilingV1(policy,lineage[lineage.length-1]!),floor=Math.min(declared??Infinity,Math.max(assembly?.megabytes??0,plan?.megabytes??0)),")
sub(p,"import {estimateHostRatioV1,hostClassBoundV1,","import {estimateHostRatioV1,hostClassBoundV1,hostClassCeilingV1,")
q=S+"stage/schema.json";j=json.load(open(q,encoding="utf8"));j["definitions"]["Policy"]["properties"]["memory"]["properties"]["childStartMegabytes"]={"type":"integer","minimum":0,"description":"What a farm child (a grant of a run's worker) is charged per job while it boots or idles, or what it measurably holds when that is more: its memory sits inside the tree of the run and is already out of the available memory."}
open(q,"w",encoding="utf8").write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
r=S+"stage/policy.json";pj=json.load(open(r,encoding="utf8"));pj["hostCapacity"]["memory"]["childStartMegabytes"]=512
open(r,"w",encoding="utf8").write(json.dumps(pj,ensure_ascii=False,indent=2)+"\n")
t=S+"stage/farm-width.test.ts"
open(t,"a",encoding="utf8").write('''
test("the live read of 12:05: 18290 MB available, nine farm children resident about 4 GB in all, parent 2.2 GB: every child is admitted with more than 4 GB spare",()=>{
 const farm=(id:string,jobs:number,age:number):HostGrantV1=>({id,operation:"exec",jobs,cpu:jobs,reservedMegabytes:jobs*1536,admittedAtMs:now-age,observedMegabytes:0,pid:4300,bootAtMs:1_700_000_000_000,label:"graph-farm"}) as HostGrantV1;
 const read={...parent(6200,3072),reservedMegabytes:3072};
 const ledger=[read,farm("farmgrant01",5,20_000),farm("farmgrant02",3,15_000),{...farm("farmgrant03",1,30_000),holder:"showholder1"} as HostGrantV1];
 const host=sample(32,10,17.86,30),evaluation=evaluateHostCapacityV1(policy,host,ledger,[ask(10,{preferredJobs:1,essential:false})],now);
 expect(evaluation.unrealizedMegabytes).toBeLessThanOrEqual(9*512+3072);
 expect(evaluation.headroomMegabytes).toBeGreaterThan(4096);
 expect(evaluation.decisions[0]).toMatchObject({verdict:"admit"});
 expect(jobs(host,ledger,ask(10))).toBeGreaterThanOrEqual(2);
});
''')
