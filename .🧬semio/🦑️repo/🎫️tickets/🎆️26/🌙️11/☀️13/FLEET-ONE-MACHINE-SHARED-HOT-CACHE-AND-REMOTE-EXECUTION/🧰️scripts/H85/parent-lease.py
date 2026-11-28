import json,os,sys
d=sys.argv[1]
js=d+"/🔣️.json";ts=d+"/🟦️.ts"
raw=open(js,encoding="utf8").read();c=json.loads(raw)
assert json.dumps(c,indent=2,ensure_ascii=False)+"\n"==raw or json.dumps(c,indent=2,ensure_ascii=False)==raw,"json not canonical"
old=["lease-reopen","lease-busy","lease-maintenance-undefined","lease-first-close-frozen","lease-reopen-undefined","lease-maintenance-close","lease-second-recheck-close"]
rows=[
 ("lease-single-open",True,0,False,["open1","recheck1","close1"],0),
 ("lease-recheck-after-sweep",True,0,False,["open1","recheck1","recheck1","close1"],0),
 ("lease-first-recheck-changed",False,1,False,["open1","recheck1","close1"],1),
 ("lease-second-recheck-changed",False,2,False,["open1","recheck1","recheck1","close1"],1),
 ("lease-second-recheck-changed-close",False,2,True,["open1","recheck1","recheck1","close1"],2),
 ("lease-close-frozen",False,0,True,["open1","recheck1","close1"],1),
]
new=[r[0] for r in rows]
def swap(lst):
    i=lst.index(old[0]);assert lst[i:i+7]==old;lst[i:i+7]=new
a=c["artifactMaintenanceCaseSchema"]["properties"]["mode"]["enum"];swap(a)
swap(c["leaseReopenCaseSchema"]["properties"]["mode"]["enum"])
swap(c["artifactSourceReceiptSchema"]["properties"]["mode"]["enum"])
cases=c["artifactMaintenanceCases"];i=next(k for k,r in enumerate(cases) if r["mode"]==old[0]);assert [r["mode"] for r in cases[i:i+7]]==old
cases[i:i+7]=[{"mode":r[0],"accepted":r[1],"reads":int(r[1]),"failed":not r[1]} for r in rows]
c["leaseReopenCases"]=[{"mode":r[0],"accepted":r[1],"opened":1,"closed":1,"failures":r[5],"sweeps":r[0]!="lease-single-open" and "close-frozen" not in r[0] and "first-recheck" not in r[0],"changedAt":r[2],"closeFails":r[3],"trace":r[4]} for r in rows]
s=c["leaseReopenCaseSchema"];s["required"]=["mode","accepted","opened","closed","failures","sweeps","changedAt","closeFails","trace"]
p=s["properties"];p["opened"]={"type":"integer","minimum":1,"maximum":1};p["closed"]={"type":"integer","minimum":1,"maximum":1}
p["sweeps"]={"type":"boolean"};p["changedAt"]={"type":"integer","minimum":0,"maximum":2};p["closeFails"]={"type":"boolean"};p["trace"]={"type":"array","minItems":3,"maxItems":4,"items":{"enum":["open1","recheck1","close1"]}}
out=json.dumps(c,indent=2,ensure_ascii=False)+("\n" if raw.endswith("\n") else "")
open(js+".h85.tmp","w",encoding="utf8").write(out);os.replace(js+".h85.tmp",js)
lines=open(ts,encoding="utf8").read().split("\n")
k=next(i for i,l in enumerate(lines) if l.startswith("async function runLeaseReopenSourceCaseV1"));l=lines[k]
def rep(a,b):
    global l
    assert l.count(a)==1,a;l=l.replace(a,b)
rep("leaseRetirementFreshV1:()=>true","leaseRetirementFreshV1:()=>!row.sweeps")
rep('attempts++;trace.push("open"+attempts);if(attempts===2){expect(readers[0]!.physicalClosed,"Previous actual lease-image lifetime must close before replacement").toBe(true);expect(readers[0]!.fd.fd).toBe(-1);if(mode==="lease-reopen-undefined")throw primary;}','attempts++;trace.push("open"+attempts);')
rep('if(mode==="lease-first-close-frozen"&&index===1||mode==="lease-maintenance-close"&&index===1||mode==="lease-second-recheck-close"&&index===2)throw closing;','if(row.closeFails)throw closing;')
rep('async()=>{trace.push("recheck"+index);if(mode==="lease-second-recheck-close"&&index===2)throw primary;})','async()=>{rechecks++;trace.push("recheck"+index);if(row.changedAt===rechecks)throw primary;})')
i0=l.index(",withExclusiveMaintenance:");i1=l.index("};});}},run=new Function")
l=l[:i0]+l[i1:]
rep("let attempts=0,closed=0","let attempts=0,rechecks=0,closed=0")
rep('Object.is(flat[0],mode==="lease-first-close-frozen"?closing:primary)','Object.is(flat[0],row.closeFails&&row.failures===1?closing:primary)')
j0=l.index('const db=new Database(":memory:");try{const events=');j1=l.index("}finally{for(const reader of readers)")
l=l[:j0]+'expect(trace,mode).toEqual(row.trace);expect(trace.some(step=>step.startsWith("maintenance")),"a single open never takes exclusive image maintenance").toBe(false);'+l[j1:]
lines[k]=l
t="\n".join(lines)
a='/** 🪝️ Requires the first actual reader lifetime to settle before a replacement may be opened. */\ntest("protected artifact maintenance schema lease reopening",()=>withBuildTestInvocationV1(original=>runLeaseReopenSourceCaseV1(original,"lease-reopen")));'
assert t.count(a)==1
b='/** 🪝️ The lease image opens once (H43) and is re-validated by `recheck` before and after the sweep: an image that changed under the opener fails it and closes the one actual lifetime, with the close failure kept next to it. */\nfor(const row of corpus.leaseReopenCases)test("protected artifact maintenance schema lease single open "+row.mode,()=>withBuildTestInvocationV1(original=>runLeaseReopenSourceCaseV1(original,row.mode)));'
t=t.replace(a,b)
open(ts+".h85.tmp","w",encoding="utf8").write(t);os.replace(ts+".h85.tmp",ts)
print("ok")
