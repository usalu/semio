"""🧪 Neural engine carve: test-file rewrites (observing allocator through the model test kit, cache retirement tests to the budgeted crate). Idempotent from the snapshot."""
import os,re
SNAP="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/snap/neural/"
DEST="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/"
KIT="semio_framework_os_kernel_neural_engine_model::testkit"
ALLOC=f"#[global_allocator]\nstatic OBSERVED_SYSTEM:{KIT}::ObservedSystem={KIT}::ObservedSystem;\n"
def rd(rel):return open(SNAP+rel,encoding="utf-8").read()
def wr(rel,t):open(DEST+rel,"w",encoding="utf-8").write(t)
def rep(t,old,new,count=None):
    n=t.count(old)
    assert n>0 and (count is None or n==count),(old[:60],n)
    return t.replace(old,new)
OBS_OLD="crate::registry::tests::observe_ownership"
# T1 retirement tests stay in the model crate
t=rd("🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs")
a=t.index("//#region 🧠️CacheRetirement");b=t.index("//#endregion 🧠️CacheRetirement")+len("//#endregion 🧠️CacheRetirement")
cache_region=t[a:b]
t=t[:a]+t[b:]
t=t.replace("super::super::registry::tests::observe_ownership","crate::testkit::observe_ownership")
t=t.replace("super::super::super::","crate::")
assert "super::super::" not in t,[m.start() for m in re.finditer("super::super::",t)][:5]
t=rep(t,"use super::*;\nuse std::mem::size_of;\n","use super::*;\nuse std::mem::size_of;\n\n#[global_allocator]\nstatic OBSERVED_SYSTEM:crate::testkit::ObservedSystem=crate::testkit::ObservedSystem;\n",1)
wr("🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs",t)
# T2 registry tests
t=open("/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osnd/reg-tests-new.rs",encoding="utf-8").read()
wr("📔️registry/🧪️tests/📔️registry/🦀️.rs",t)
# T3 output tests
t=rd("📔️registry/✅️output/🧪️tests/🦀️.rs");wr("📔️registry/✅️output/🧪️tests/🦀️.rs",rep(t,OBS_OLD,KIT+"::observe_ownership"))
# T4 budgeted tests
for rel in ["📥️input/🧪️tests/🦀️.rs","🌳️tree/📋️copy/🧪️tests/🦀️.rs","🧭️topology/🧪️tests/🦀️.rs"]:
    t=rd(rel);wr(rel,rep(t,OBS_OLD,KIT+"::observe_ownership"))
t=rd("⏱️evaluation/🌊️wave/🧪️tests/🦀️.rs");t=rep(t,OBS_OLD,KIT+"::observe_ownership");t=t.replace("super::super::super::","super::super::");wr("⏱️evaluation/🌊️wave/🧪️tests/🦀️.rs",t)
t=rd("⏱️evaluation/🧪️tests/🦀️.rs");t=rep(t,OBS_OLD,KIT+"::observe_ownership")
cache=cache_region.replace("super::super::NeuralCacheRetirement","crate::NeuralCacheRetirement").replace("super::super::NeuralCache","crate::NeuralCache").replace("super::super::registry::tests::observe_ownership",KIT+"::observe_ownership")
assert "super::super" not in cache
cache=cache.replace('include_str!("../🧫️fixtures/','include_str!("../../🧵️retirement/🧪️tests/🧫️fixtures/')
t=t.rstrip("\n")+"\n\n"+ALLOC+"\n"+cache+"\n"
wr("⏱️evaluation/🧪️tests/🦀️.rs",t)
# T5 evaluator tests
t=rd("🧪️tests/🔬️unit/🦀️.rs");t=rep(t,OBS_OLD,KIT+"::observe_ownership");
assert t.startswith("\nuse super::*;\n") or t.startswith("use super::*;\n"),repr(t[:40])
t=t.replace("use super::*;\n","use super::*;\n\n"+ALLOC,1)
wr("🧪️tests/🔬️unit/🦀️.rs",t)
t=rd("🧪️tests/🚦️owned-controls/🦀️.rs");wr("🧪️tests/🚦️owned-controls/🦀️.rs",rep(t,"crate::registry::tests::observe_ownership",KIT+"::observe_ownership"))
print("tests rewritten")
