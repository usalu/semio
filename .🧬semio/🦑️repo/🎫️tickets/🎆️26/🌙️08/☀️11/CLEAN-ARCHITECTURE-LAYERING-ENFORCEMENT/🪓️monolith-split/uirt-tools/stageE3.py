"""✂️ Stage E3: cursor-side suites read the shadow through its detached retire and the renamed completion field."""
import re
REPO="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
for rel in ["🧪️tests/🔬️cursor-unit/🦀️.rs","📏️ownership/🧪️tests/📏️ownership/🦀️.rs","📃️document/🧪️tests/📃️document/🦀️.rs"]:
    p=f"{REPO}/{rel}"; t=open(p,encoding="utf-8").read()
    t=re.sub(r"\b(current|candidate|reconciler)\.retire_one\(\)",r"\1.retire_one_detached()",t)
    t=t.replace("Complete { reconciler, patch }","Complete { shadow: reconciler, patch }")
    open(p,"w",encoding="utf-8").write(t)
print("ok")
