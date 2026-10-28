"""usage: patch-ledger.py <sandbox/tests/fixtures/ledger.json> — follows two peer moves of the ratcheted debt: the Go root-selection test now lives in 🏠️workspace/🧪️tests/🧭️root-selection/🐹️.go, and the vite store test names the real dependencies partition at two sites (the second site comes with the isolated concurrent-listener test)."""
import json,sys
p=sys.argv[1];raw=open(p).read();d=json.loads(raw)
n=0
for e in d["entries"]:
    if e["file"].endswith("root-selection_test.go") and e["rule"]=="go-unsandboxed-writer":
        e["file"]="🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧪️tests/🧭️root-selection/🐹️.go";n+=1
    if e["file"].endswith("⚡️caching/🌐️vite/💾️storage/🧪️tests/🟦️.ts") and e["rule"]=="real-graph-image":
        e["count"]=2;n+=1
assert n==2,n
open(p,"w").write(json.dumps(d,ensure_ascii=False,indent=2)+("\n" if raw.endswith("\n") else ""))
