# Restructures the architect editor root: base + behavior become child modules; consumers use explicit paths; window-ownership test moves to the root.
import os, re
S="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any"
E=S+"/✏️editor"
root=E+"/🦀️.rs"
s=open(root).read()
iC=s.index("//#region 🔖️Constants\n")
iB=s.index("//#region 🔧️Behavior\n")
eB_marker="pub use behavior::*;\n//#endregion 🔧️Behavior\n"
eB=s.index(eB_marker)
base_text=s[iC:iB]
beh_block=s[iB:eB]
# behavior: doc lines + body
mod_idx=beh_block.index("pub mod behavior {\n")
docs=beh_block[len("//#region 🔧️Behavior\n"):mod_idx]
body=beh_block[mod_idx+len("pub mod behavior {\n"):]
assert body.rstrip().endswith("}")
body=body.rstrip()[:-1].rstrip("\n")+"\n"
body="\n".join(l[4:] if l.startswith("    ") else l for l in body.split("\n"))
docs="".join(("//!"+l[3:] if l.startswith("///") else l)+"\n" for l in docs.rstrip("\n").split("\n"))
assert 'include!("🧪️tests/🔬️behavior-unit/🦀️.rs");' in body
body=body.replace('include!("🧪️tests/🔬️behavior-unit/🦀️.rs");','include!("../🧪️tests/🔬️behavior-unit/🦀️.rs");')
beh_file=docs+"\n"+body
os.makedirs(E+"/🔧️behavior",exist_ok=True)
open(E+"/🔧️behavior/🦀️.rs","w").write(beh_file)
base_header='''//! 🧱️ Architect editor base — the app id, the UI assembly helpers, the interaction constants and the document reset effect that every editor surface shares, so no surface reaches the editor root for them.

use crate::standards::v1::subsets::any::schema::snapshot::{ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA};

'''
render_support='''
//#region 🧪️RenderSupport
/// 🧪️ Projects a built UI node into its retired semantic tree, the one-line render check the window and panel unit tests share.
#[cfg(test)]
pub(crate) mod render_support {
    pub fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
    }
}
//#endregion 🧪️RenderSupport
'''
os.makedirs(E+"/🧱️base",exist_ok=True)
open(E+"/🧱️base/🦀️.rs","w").write(base_header+base_text.rstrip("\n")+"\n"+render_support)
mounts='''#[path = "🧱️base/🦀️.rs"]
pub mod base;
pub use base::*;

//#region 🔧️Behavior
#[path = "🔧️behavior/🦀️.rs"]
pub mod behavior;
'''
s=s[:iC]+mounts+s[eB:]
# window ownership mount at root
a='''#[cfg(test)]
#[path = "🧪️tests/⚖️declared-verbs/🦀️.rs"]
mod declared_verb_laws;
'''
assert s.count(a)==1
s=s.replace(a,a+'''
#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod window_ownership_tests;
''')
open(root,"w").write(s)
# consumers
names=["ARCHITECT_APP_ID","ui_label","ui_text","ui_node","ui_children","ui_capacity_error","architect_action","ui_value_text","ui_value_bool","ui_value_number","ui_value_list","ui_value_map","ARCHITECT_INTERACTION_PROGRAM","ARCHITECT_INTERACTION_GRANULARITY_ENTITY","reset_document_effect"]
alt="|".join(names)
files=["🎮️commands/📚️example","🎮️commands/🕸️graph","🎮️commands/📤️exchange","🎭️modes/✏️edit/🪟️windows/↔️adjacency","🎭️modes/✏️edit/🪟️windows/📋️register","🎭️modes/✏️edit/🪟️windows/📓️report","📌️panels/🔍️inspection","📌️panels/📚️catalogue","📌️panels/🗿️artifact"]
for f in files:
    p=E+"/"+f+"/🦀️.rs"
    t=open(p).read()
    def brace(m):
        items=[x.strip() for x in m.group(1).split(",") if x.strip()]
        assert all(i in names for i in items),(p,items)
        return "crate::editor::architect::base::{"+", ".join(items)+"}"
    t=re.sub(r'crate::editor::architect::\{([^}]*)\}',lambda m: brace(m) if all(x.strip() in names for x in m.group(1).split(",") if x.strip()) else m.group(0),t)
    t=re.sub(r'crate::editor::architect::('+alt+r')\b',r'crate::editor::architect::base::\1',t)
    open(p,"w").write(t)
# project_render users
old="crate::editor::architect::unit_tests::context::project_render"
new="crate::editor::architect::base::render_support::project_render"
n=0
for dirpath,dirs,fs in os.walk(E):
    if "node_modules" in dirpath: continue
    for fn in fs:
        if fn!="🦀️.rs": continue
        p=os.path.join(dirpath,fn)
        if p==E+"/🧪️tests/🔬️unit/🦀️.rs": continue
        t=open(p).read()
        if old in t:
            n+=t.count(old); open(p,"w").write(t.replace(old,new))
print("project_render rewrites",n)
# move window ownership test
src=E+"/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs"
dst=E+"/🧪️tests/🔬️window-ownership/🦀️.rs"
t=open(src).read()
assert t.startswith("use super::*;\n")
t=t.replace("use super::*;\n","use super::*;\nuse crate::editor::architect::modes::edit::windows::register::config::*;\n",1)
a='include_str!("../🧫️fixtures/🔬️window-ownership/🔣️.json")'
assert t.count(a)==1
t=t.replace(a,'include_str!("../../🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/🔣️.json")')
os.makedirs(os.path.dirname(dst),exist_ok=True)
open(dst,"w").write(t)
os.remove(src)
os.rmdir(os.path.dirname(src))
cfg=E+"/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🦀️.rs"
c=open(cfg).read()
a='''
#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod window_ownership_tests;
'''
assert c.count(a)==1
open(cfg,"w").write(c.replace(a,""))
