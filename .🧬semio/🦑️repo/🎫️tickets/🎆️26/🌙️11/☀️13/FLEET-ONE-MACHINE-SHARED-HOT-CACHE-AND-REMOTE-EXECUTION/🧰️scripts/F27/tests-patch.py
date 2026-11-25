import sys
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
files=["🧪️tests/🕸️graph/🔄️candidate/🟦️.ts","🧪️tests/🕸️graph/🧊️stale-facts/🟦️.ts","🧪️tests/🕸️graph/🌊️seal-under-churn/🟦️.ts","🧪️tests/🕸️graph/🩹️repair-rounds/🟦️.ts","🕸️graph/📥️load/🧪️tests/📁️inventory/🟦️.ts"]
add="readForgottenPathsV1:()=>({paths:new Map(),listings:new Map()}),rowsForgottenV1:()=>false,forgetPersistedPathsV1:async()=>true,"
for f in files:
    t=open(B+f,encoding="utf8").read()
    if "forgetPersistedPathsV1" in t: continue
    n=t.count("readAdmissionMemoV1:")
    if n!=1: sys.exit("%s: %d anchors"%(f,n))
    open(B+f,"w",encoding="utf8").write(t.replace("readAdmissionMemoV1:",add+"readAdmissionMemoV1:"))
print("ok")
