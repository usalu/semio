"""✂️ Stage E2: component-line edits of the split suites (shadow, cursor and owner access)."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
def edit(rel,pairs,count=None):
    p=f"{R}/{rel}"; t=open(p,encoding="utf-8").read()
    for old,new,n in pairs:
        c=t.count(old)
        assert c==n,(rel,old[:70],c,n)
        t=t.replace(old,new)
    open(p,"w",encoding="utf-8").write(t)
A="🧪️tests/🔬️authority-unit/🦀️.rs"
edit(A,[
 ("use super::*;\nuse semio_framework_ui_runtime_reconcile_cursor::testkit::*;\n","use super::*;\nuse semio_framework_ui_runtime_reconcile_cursor::{testkit::*, *};\nuse semio_framework_ui_runtime_reconcile_primitives::*;\n",1),
 ("SurfaceReconcileCursor::new(component_tree, current)","SurfaceReconcileCursor::new(component_tree, &current.shadow)",2),
 ("cursor.step(current)","cursor.step(&current.shadow)",2),
 ("SurfaceReconcileStep::Complete { mut reconciler, patch } => {\n                let mut output = None;\n                while !reconciler.seal_step(cursor.usage,","SurfaceReconcileStep::Complete { shadow, patch } => {\n                let mut reconciler = SurfaceReconciler::from_shadow(shadow);\n                let mut output = None;\n                while !reconciler.seal_step(cursor.usage(),",1),
 ("SurfaceReconcileStep::Complete { mut reconciler, patch } => {\n                let usage = cursor.usage;","SurfaceReconcileStep::Complete { shadow, patch } => {\n                let mut reconciler = SurfaceReconciler::from_shadow(shadow);\n                let usage = cursor.usage();",1),
 ("SurfaceReconcileStep::Complete { mut reconciler, patch } => {\n                let mut output = None;\n                loop {","SurfaceReconcileStep::Complete { shadow, patch } => {\n                let mut reconciler = SurfaceReconciler::from_shadow(shadow);\n                let mut output = None;\n                loop {",1),
 ("reconciler.seal_step(cursor.usage, &mut output, patch.is_some()).unwrap();","reconciler.seal_step(cursor.usage(), &mut output, patch.is_some()).unwrap();",1),
 ("cursor.assembly_generation","cursor.assembly_generation()",6),
 ("SurfaceReconcileCursor::new(tree(container(\"root\", children)), &current)","SurfaceReconcileCursor::new(tree(container(\"root\", children)), &current.shadow)",1),
 ("let step = cursor.step(&current);","let step = cursor.step(&current.shadow);",1),
 ("cursor.pending_root.is_some()","cursor.pending_root().is_some()",2),
 ("cursor.held_node.is_none()","cursor.held_node().is_none()",2),
 ("reconciler.document.as_ref().expect(","reconciler.shadow.document().expect(",1),
])
edit("📃️document/🧪️tests/🔒️authority/🦀️.rs",[
 ("owner.document.as_ref().unwrap()","owner.shadow.document().unwrap()",1),
 ("replacement.document.as_ref().unwrap()","replacement.shadow.document().unwrap()",1),
])
edit("♻️retirement/🌲️tree/🧪️tests/🔒️authority/🦀️.rs",[("SurfaceReconciler::new(","SurfaceReconciler::from_text(",2)])
print("ok")
