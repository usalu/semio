"""🧱️ Stage A2: retargets the tree-retirement file and the reconcile root header after stage A."""
import re,sys
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
tree=open(f"{S}/{RT}/♻️retirement/🌲️tree/🦀️.rs",encoding="utf-8").read()
assert tree.count("pub(super)")==8
t=tree.replace("pub(super) ","pub ")
t=t.replace("crate::UiNativeResources","UiNativeResources").replace("crate::TreeNode","TreeNode").replace("crate::ComponentTree","ComponentTree")
tail='\n#[cfg(test)]\n#[path = "🧪️tests/🌲️tree/🦀️.rs"]\nmod tests;\n'
assert t.endswith(tail),repr(t[-120:])
t=t[:-len(tail)]+"\n" if False else t[:-len(tail)]
t=t.rstrip("\n")+"\n"
head="//! 🌲️ Single retained runtime tree owner with concrete, non-overwriting source transfers.\n\nuse semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode, UiNativeResources};\n"
assert t.startswith("//! 🌲️ Single retained runtime tree owner with concrete, non-overwriting source transfers.\n")
t=head+t[len("//! 🌲️ Single retained runtime tree owner with concrete, non-overwriting source transfers.\n"):]
t=t.replace("pub struct SurfaceTreeRetireCursor","pub struct SurfaceTreeRetireCursor")
open(f"{REPO}/{RT}/♻️retirement/🌲️tree/🦀️.rs","w",encoding="utf-8").write(t)
print("tree ok")
