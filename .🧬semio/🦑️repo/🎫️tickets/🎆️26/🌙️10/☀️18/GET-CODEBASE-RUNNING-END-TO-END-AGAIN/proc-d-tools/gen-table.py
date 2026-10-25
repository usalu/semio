"""Generates the initial suites table from the CURRENT script.ts suite literals (existing behaviour preserved) plus new facet suites given as a python dict file."""
import re,json,sys,os
mod="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process"
s=open(mod+"/📜️script.ts",encoding="utf8").read()
B={"quick":300000,"fundamental":15000,"long":900000,"exhaustive":1800000}
start=s.index("const suites:")
body=s[start:s.index("const suite = suites[")]
rows=[]
for m in re.finditer(r'(?:"([a-z0-9-]+)"|([a-z]+))\s*:\s*\{\s*source\s*:\s*"([^"]+)"\s*,\s*budgetMs\s*:\s*(TEST_LEVEL_BUDGET_MS\.(\w+)|\d[\d_]*)(?:\s*,\s*testNamePattern\s*:\s*"([^"]+)")?\s*\}',body):
    sid=m.group(1) or m.group(2)
    bud=B[m.group(5)] if m.group(5) else int(m.group(4).replace("_",""))
    rows.append(dict(id=sid,source=m.group(3),budgetMs=bud,testNamePattern=m.group(6)))
print(len(rows),"suite literals",file=sys.stderr)
json.dump(rows,open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)

EXTRAS={
 "cargo-driver":["🧪️testing/🦀️cargo/🧪️tests/🎚️mode/🟦️.ts"],
 "cargo-profile":["🧪️testing/🦀️cargo/🧪️tests/🎚️mode/🟦️.ts"],
 "go-build-mode":["../📁️filesystem/🛡️durability/🧪️tests/🟦️.ts"],
 "capture":["📥️capture/📥️invocation/🧪️tests/🟦️.ts"],
 "exact-cargo-laws":["🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧪️tests/🟦️.ts"],
 "native-artifacts":[f"📦️artifacts/🏗️native-build/{f}/🧪️tests/🟦️.ts" for f in ["🧾️receipt","🎰️jobserver","🎟️admission","📈️telemetry","🗃️stage-cache","🧩️component-pass"]],
 "routing":["🔌️providers/🧪️tests/🟦️.ts"],
 "workspace":["../../🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧪️tests/🌱️workspace-resolution/🟦️.ts"],
 "owned-execution":[],
 "exact-cargo-storage":["🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧪️tests/🟦️.ts","🧪️testing/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts","🧪️testing/🧰️toolchain/📌️pinned/🧪️tests/🟦️.ts","🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image/🟦️.ts"],
 "compiler-capacity-inheritance":["🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts","../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🧪️tests/🟦️.ts"],
 "compiler-units":["../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🔒️exclusive-writer/🟦️.ts","../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⛓️cpu-last/🟦️.ts","../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🎟️capacity/🧪️tests/🟦️.ts"],
 "compiler-capacity":["🔒️leases/📦️bounded/🧪️tests/🟦️.ts"],
 "native-storage-platform":["📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts","💾️state/🧪️tests/🟦️.ts","../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🟦️.ts","../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts"],
}
CAPS={"resource-lease-writer":{"artifacts":True,"command":"writer"}}
TYPECHECK={"routing":"strict","command":"strict","capture":"strict","cargo-driver":"strict","cargo-profile":"strict","exact-cargo-laws":"strict","exact-cargo-storage":"strict","compiler-state":"strict-indexed"}
NEW=json.load(open(sys.argv[2],encoding="utf8")) if len(sys.argv)>2 else []
suites=[]
for r in rows:
    sid=r["id"]
    if sid=="owned-execution":
        suites.append(dict(id=sid,budgetMs=r["budgetMs"],typecheck="none",trees=["🎛️owned-execution"]));continue
    files=[r["source"]]+EXTRAS.get(sid,[])
    row=dict(id=sid,budgetMs=r["budgetMs"],typecheck=TYPECHECK.get(sid,"none"),files=files)
    if r["testNamePattern"]: row["testNamePattern"]=r["testNamePattern"]
    if sid in CAPS: row["capabilities"]=CAPS[sid]
    if sid=="exact-cargo-storage": row["typecheckFiles"]=["🧪️testing/🦀️cargo/🎯️exact/🟦️.ts"]+EXTRAS[sid]
    suites.append(row)
suites+=NEW
code=["resource-leases","artifact-publication","artifact-files","cargo-build-lease","native-artifact-publication"]
table=dict(version=1,suites=suites,codeSuites=code,helpers=[])
out=mod+"/🧪️testing/🗂️suites/🔣️.json"
json.dump(table,open(out,"w",encoding="utf8"),ensure_ascii=False,indent=2);open(out,"a").write("\n")
print(len(suites),"suites written to",out,file=sys.stderr)
