"""🪡 agents3 batch `narrow`: anchored edits that make an upgrade stop waiting for ALL its jobs once it has waited `wait.bypassMilliseconds`.
`edit_narrow.py <root>` edits the files below `<root>` in place (a mirror); `edits(rel, text)` is the pure function `make_narrow.py` reuses against the live bytes.
Cause (agents3 simulation on the live learned estimates, a real 16 GB laptop): a quiet run whose work has to run asks for all 8 jobs (`minimumJobs = preferredJobs`); beside a busy peer whose learned full reservation (p90 over cold and hot runs, 2-2.5x its real footprint) still counts in full while it ramps, it waits for the peer's whole run (hot-firstedit: course edit 266 s for a 50 s run, mixed-long: 46-69 s).
Fix: after `wait.bypassMilliseconds` the waiter's request is rewritten with `minimumJobs = protocol.upgradeNarrowedMinimumJobs` (1) and one spare job of memory (`protocol.growSpareJobs`, so two runs side by side stay within the footprint error of the estimates): the decision admits the largest job count whose memory fits (`reduced`), the grower of the holder adds jobs once memory frees, and no upgrade waits longer than the bypass window for memory that carries even one job."""
import os, re, sys

HC = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
SCHEMA = HC + "🧬️schema/🔣️.json"
UPGRADE = HC + "🔼️upgrade/🟦️.ts"
TEST = HC + "🧪️tests/🔼️upgrade/👥️agents/🟦️.ts"
FEATURE = HC + "🧪️tests/🔼️upgrade/👥️agents/🥒️.feature"
FILES = [SCHEMA, UPGRADE, TEST, FEATURE]

TEST_ADDED = r'''
test("An upgrade that waited for all its jobs past the bypass window takes the jobs that fit beside a busy peer instead of waiting for the peer's whole run, and says so in English and German",async()=>{
 const floor=hostFloorMegabytesV1(fast,{totalMegabytes:16384}),peerUnrealized=5486-3600,available=floor+peerUnrealized+4000,request=full("courseaaa9",404,700),held=200,spare=available-floor-peerUnrealized,expected=Math.min(7,Math.floor((spare+held-request.fixedMegabytes-request.perJobMegabytes)/request.perJobMegabytes)),reserved=request.fixedMegabytes+(expected+1)*request.perJobMegabytes;
 expect(expected,"the world is built so that the whole request does not fit, a part of it does, and one job of memory stays spare").toBe(4);
 for(const language of ["en","de"] as const)for(const narrowing of [true,false]){
  const peer={...running("printaaaa1",2,5486,3600),rampMilliseconds:10_000_000},other=quiet("dashaaaaa1",3,500_000),mine:HostGrantV1={...quiet("courseaaa9",1,500_000),reservedMegabytes:1502,observedMegabytes:held},lines:string[]=[],grants=[peer,other,mine];let state=mine,polls=0,clock=905_000;
  const ledger={takeLock:()=>true,releaseLock:()=>undefined,holdsLock:()=>true,writeJson:()=>undefined,remove:()=>undefined,queuePath:()=>join(fresh(),"waiter.json"),readGrants:()=>grants.map(grant=>grant.id===state.id?state:grant),readWaiters:()=>[] as HostWaiterV1[],sample:async()=>({...machine({availableMegabytes:available,load:2}),atMs:clock})};
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
  const tuned=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:10,bypassMilliseconds:narrowing?decisionFixture.policy.wait.bypassMilliseconds:3_600_000}});
  const outcome=await upgradeHostGrantV1({ledger:ledger as never,directory:fresh(),policy:tuned,request:{...request,minimumJobs:request.preferredJobs},control,state:()=>state,publish:next=>{state=next;},requested:()=>++polls<(narrowing?300:40),stopped:()=>false,now:()=>(clock+=1000),pid:mine.pid,language,write:line=>lines.push(line),session:{address:mine.session!,weight:1}});
  if(!narrowing){expect([outcome,state.jobs,state.quiet],`${language} without the bypass window: it keeps waiting for the peer`).toEqual(["withdrawn",1,true]);continue;}
  expect(outcome,language).toBe("upgraded");
  expect([state.jobs,state.reservedMegabytes,state.quiet],`${language}: the jobs that fit and the memory of those jobs`).toEqual([expected,reserved,undefined]);
  expect(lines.at(-1),language).toMatch(language==="en"?new RegExp("from 1502 MB to "+reserved+" MB with "+expected+" of 8 job\\(s\\) after \\d+s"):new RegExp("von 1502 MB auf "+reserved+" MB mit "+expected+" von 8 Job\\(s\\) vergrößert"));
 }
});
'''


def must(text, anchor, new, count=1):
    assert text.count(anchor) == count, (text.count(anchor), anchor[:80])
    return text.replace(anchor, new)


def edits(rel, text):
    if rel == SCHEMA:
        return must(text, '        "growSettleAttempts": 2\n', '        "growSettleAttempts": 2,\n        "upgradeNarrowedMinimumJobs": 1\n')
    if rel == UPGRADE:
        text = must(text, 'const record:HostWaiterV1=requireHostCapacityV1<HostWaiterV1>("Waiter",{request,arrivedAtMs,pid,upgrade:true,...context.session===undefined?{}:{session:context.session.address,weight:context.session.weight}});\n let lastProgress=-Infinity,done=false;',
                    'let record:HostWaiterV1=requireHostCapacityV1<HostWaiterV1>("Waiter",{request,arrivedAtMs,pid,upgrade:true,...context.session===undefined?{}:{session:context.session.address,weight:context.session.weight}});\n let asked=request,lastProgress=-Infinity,done=false;')
        text = must(text, '   checkResourceLeaseControl(control);\n   const host=await context.ledger.sample();',
                    '   checkResourceLeaseControl(control);\n   if(asked===request&&request.minimumJobs>protocol.upgradeNarrowedMinimumJobs&&now()-started>=policy.wait.bypassMilliseconds){asked=requireHostCapacityV1<HostRequestV1>("Request",{...request,minimumJobs:protocol.upgradeNarrowedMinimumJobs,fixedMegabytes:request.fixedMegabytes+protocol.growSpareJobs*request.perJobMegabytes});record=requireHostCapacityV1<HostWaiterV1>("Waiter",{...record,request:asked});writeJson(waiterPath,record);}\n   const host=await context.ledger.sample();')
        text = must(text, 'const wanted=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes));', 'const wanted=Math.min(asked.fixedMegabytes+asked.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes));')
        text = must(text, 'to:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes)),reason:describeHostReasonV1', 'to:Math.min(asked.fixedMegabytes+asked.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes)),reason:describeHostReasonV1')
        text = must(text, 'the wait ends when the asker cancelled (the request file vanished or the control fired), in which case the small grant is kept. */\nexport async function upgradeHostGrantV1',
                    'the wait ends when the asker cancelled (the request file vanished or the control fired), in which case the small grant is kept. A run that has waited `wait.bypassMilliseconds` for all its jobs stops insisting on them: its request is rewritten with `protocol.upgradeNarrowedMinimumJobs` and one spare job of memory (`protocol.growSpareJobs`, the margin a grow keeps for a footprint above its estimate), the decision admits the largest job count whose memory fits beside the busy peers (`upgradedReduced`) and the grower adds jobs once memory frees up, so no upgrade waits longer than that window for memory that carries even one job. */\nexport async function upgradeHostGrantV1')
        return text
    if rel == TEST:
        return text.rstrip("\n") + "\n" + TEST_ADDED
    if rel == FEATURE:
        return text.rstrip("\n") + "\n\n  Scenario: An Upgrade Stops Insisting On All Its Jobs After The Bypass Window\n    Given a quiet run whose work has to run and whose whole request does not fit beside a busy peer\n    When it has waited for the memory of all its jobs longer than the bypass window\n    Then it is admitted with the largest number of jobs whose memory fits and keeps waiting for nothing\n    And it says so in English or German, and its grower adds jobs once memory frees up\n"
    raise KeyError(rel)


def main():
    root = os.path.abspath(sys.argv[1])
    for rel in FILES:
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        new = edits(rel, text)
        if new != text:
            open(path, "w", encoding="utf8").write(new)
            print("edited", rel[-70:])
        else:
            print("unchanged", rel[-70:])


if __name__ == "__main__":
    main()
