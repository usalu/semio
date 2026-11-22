import os
os.chdir("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool")
def sub(p,o,n,c=1):
    t=open(p,encoding="utf8").read()
    if t.count(o)==0 and n in t:return
    assert t.count(o)==c,(p,o[:70],t.count(o));open(p,"w",encoding="utf8").write(t.replace(o,n))
sub("🧬️schema/🔣️.json",'"swapActiveMegabytesPerSecond": 1,\n        "swapOutMegabytesPerSecond": 64,','"swapActiveBasisPointsPerSecond": 0.6,\n        "swapOutBasisPointsPerSecond": 40,')
sub("🟦️.ts","const swapLow=policy.swapActiveMegabytesPerSecond,swapHigh=policy.swapOutMegabytesPerSecond,","const swapLow=host.totalMegabytes*policy.swapActiveBasisPointsPerSecond/10000,swapHigh=host.totalMegabytes*policy.swapOutBasisPointsPerSecond/10000,")
sub("🧪️tests/🧫️fixtures/🔣️.json",'"swapActiveMegabytesPerSecond": 1,\n  "swapOutMegabytesPerSecond": 64','"swapActiveBasisPointsPerSecond": 0.6,\n  "swapOutBasisPointsPerSecond": 40')
sub("🧪️tests/🧫️fixtures/🐍️oracle.py",'lo, hi = p["swapActiveMegabytesPerSecond"], p["swapOutMegabytesPerSecond"]','lo, hi = h["totalMegabytes"] * p["swapActiveBasisPointsPerSecond"] / 10000, h["totalMegabytes"] * p["swapOutBasisPointsPerSecond"] / 10000')
T="🧪️tests/🧮️shared-budget/🟦️.ts"
sub(T,"""  expect(P.swapActiveMegabytesPerSecond).toBe(policy.hostCapacity.swap.activeMegabytesPerSecond);
  expect(P.swapOutMegabytesPerSecond).toBe(policy.hostCapacity.swap.outMegabytesPerSecond);
  let before=h.cores;
  for(let swap=0;swap<=P.swapOutMegabytesPerSecond;swap++){""","""  expect(P.swapActiveBasisPointsPerSecond).toBe(policy.hostCapacity.swap.activeBasisPointsPerSecond);
  expect(P.swapOutBasisPointsPerSecond).toBe(policy.hostCapacity.swap.outBasisPointsPerSecond);
  const lowSwap=h.total*P.swapActiveBasisPointsPerSecond/10000,highSwap=h.total*P.swapOutBasisPointsPerSecond/10000;
  let before=h.cores;
  for(let swap=0;swap<=Math.ceil(highSwap);swap++){""")
sub(T,"if(swap>P.swapActiveMegabytesPerSecond&&swap<P.swapOutMegabytesPerSecond)","if(swap>lowSwap&&swap<highSwap)")
sub(T,"swapOutMegabytesPerSecond:P.swapActiveMegabytesPerSecond+1}","swapOutMegabytesPerSecond:lowSwap+1}")
print("ok")
