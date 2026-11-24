import sys,os
lib=sys.argv[1]
live=sys.argv[2]
B="🏗️build/🌐️remote-execution/"
def read(p):return open(p,encoding="utf8").read()
def write(p,s):open(p,"w",encoding="utf8").write(s)
def rep(s,old,new,count=1):
    if new in s and old not in s:return s
    assert s.count(old)==count,(old[:70],s.count(old))
    return s.replace(old,new)
def edit(rel,pairs):
    p=os.path.join(lib,rel);s=read(p)
    for old,new in pairs:s=rep(s,old,new)
    write(p,s)
C=B+"🚀️offload/🔌️client/🟦️.ts"
p=os.path.join(lib,C);s=read(p);l=read(os.path.join(live,C))
a,b="/** 🔌️ Submits an action once","const wired="
if "executeOffloadRidingThroughV1" in s:
    s=s[:s.index(a)]+l[l.index(a):l.index(b)]+s[s.index(b):]
    i=s.index("/** 🛟️ Submits like `executeOffloadV1`");j=s.index("/** ⏱️ Runs one phase of an offload")
    s=s[:i]+s[j:]
    write(p,s)
O=B+"🚀️offload/🟦️.ts"
p=os.path.join(lib,O);s=read(p)
tail="signal:cancelled,onStage:staged},policy.timeouts.rideThroughMilliseconds);}"
if "executeOffloadRidingThroughV1" in s:
    assert s.count(tail)==2
    k=s.index(tail);s=s[:k]+"signal:cancelled,onStage:staged,rideThroughMilliseconds:Math.max(policy.timeouts.rideThroughMilliseconds,patience)});}"+s[k+len(tail):]
    k=s.index(tail);s=s[:k]+"signal:cancelled,onStage:staged,rideThroughMilliseconds:peak>0?Math.max(policy.timeouts.rideThroughMilliseconds,readRemoteExecutionPolicyV1().worker.outage.seconds*1000):policy.timeouts.rideThroughMilliseconds});}"+s[k+len(tail):]
    s=rep(s,"execution=await executeOffloadRidingThroughV1({","execution=await executeOffloadV1({",2)
    s=rep(s,"import {executeOffloadRidingThroughV1,offloadExecuteBudgetMillisecondsV1,","import {executeOffloadV1,offloadExecuteBudgetMillisecondsV1,")
s=rep(s,'"incomplete-inputs","worker-policy","path-dependent-output"','"incomplete-inputs","remote-failed-local-redo","worker-policy","path-dependent-output"')
s=rep(s,'''   const kind=classifyOffloadExitV1({exitCode:outputs.exitCode,stderr});
   if(kind==="incomplete-inputs")return settle(local("incomplete-inputs",offloadIncompleteDetailV1(stderr)));
   if(kind==="infrastructure"){''','''   const verdict=classifyOffloadExitV1({exitCode:outputs.exitCode,stderr}),kind=verdict.kind;
   if(kind==="incomplete-inputs")return settle(local("incomplete-inputs",offloadIncompleteDetailV1(stderr)));
   if(verdict.kind==="environmental")return settle(local("remote-failed-local-redo",`${verdict.class} on ${execution.origin}, exit ${outputs.exitCode}: ${offloadIncompleteDetailV1(stderr)}`));
   if(kind==="infrastructure"){''')
write(p,s)
edit(B+"🚀️offload/🧬️schema/🔣️.json",[
('        "incomplete-inputs",\n        "crate-type",','        "incomplete-inputs",\n        "remote-failed-local-redo",\n        "crate-type",'),
('      "reason.incomplete-inputs": "the other machine lacked an input",\n','      "reason.incomplete-inputs": "the other machine lacked an input",\n      "reason.remote-failed-local-redo": "the other machine failed in a way only it can explain, compiled again here",\n'),
('      "reason.incomplete-inputs": "der anderen Maschine fehlte eine Eingabe",\n','      "reason.incomplete-inputs": "der anderen Maschine fehlte eine Eingabe",\n      "reason.remote-failed-local-redo": "die andere Maschine scheiterte auf eine Weise, die nur sie erklärt, hier erneut kompiliert",\n'),
('"rideThroughMilliseconds": 20000','"rideThroughMilliseconds": 60000'),
])
edit(B+"🚀️offload/📐️policy/🟦️.ts",[('|"incomplete-inputs"|"crate-type"','|"incomplete-inputs"|"remote-failed-local-redo"|"crate-type"')])
