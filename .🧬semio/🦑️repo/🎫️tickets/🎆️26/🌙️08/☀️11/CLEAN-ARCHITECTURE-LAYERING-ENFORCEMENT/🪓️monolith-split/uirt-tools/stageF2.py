"""🧪️ Stage F2: primitives suites take the primitives-level ledger guard."""
REPO="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
for rel in ["🧪️tests/🔬️primitives-unit/🦀️.rs","♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]:
    p=f"{REPO}/{rel}"; t=open(p,encoding="utf-8").read()
    t=t.replace("crate::surface_reconcile_registry_test_guard()","testkit::registry_guard()")
    open(p,"w",encoding="utf-8").write(t)
