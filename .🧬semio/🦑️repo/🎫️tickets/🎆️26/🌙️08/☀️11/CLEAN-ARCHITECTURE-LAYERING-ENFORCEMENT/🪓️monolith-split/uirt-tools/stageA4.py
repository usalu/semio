"""♻️ Stage A4: re-mounts the two retained-authority unit suites that stay with the reconcile crate."""
from prim_names import REPO,RT
path=f"{REPO}/{RT}/♻️reconcile/🦀️.rs"
t=open(path,encoding="utf-8").read()
a="impl SurfaceReconcileReadyPatch {\n    pub fn generation(&self) -> u64 {"
assert t.count(a)==1
mount1='#[cfg(test)]\n#[path = "../🩹️patch/🧪️tests/🩹️patch/🦀️.rs"]\nmod patch_handoff_tests;\n\n'
t=t.replace(a,mount1+a)
b="\n//#endregion 🎟️RetainedAuthority\n"
assert t.count(b)==1
mount2='\n#[cfg(test)]\n#[path = "../🚪️handback/🧪️tests/🚪️handback/🦀️.rs"]\nmod handback_entry_tests;\n'
t=t.replace(b,mount2+b)
open(path,"w",encoding="utf-8").write(t)
print("ok")
