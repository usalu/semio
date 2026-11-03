"""🪚 EN 1992 scene (300075 B, 4533 lines of the 262144 B / 4000 line budget) splits into `-scene-foundation` (kit, layout, mechanics, reinforcement, tracks, SceneInput) and `-scene-diagrams` (the diagram set and every module that only feeds it); the scene crate keeps the root, members, punching, anchors, cases and handles. Run once."""
import os, sys, re
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
import lib
ART = NORM + "/🗿️artifacts/🏛️en1992"
SC = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
P = "semio-s-artifact-norm-en1992-scene"
F, D = P + "-foundation", P + "-diagrams"
FD = SC + "/🧱️foundation"
DD = SC + "/📈️diagrams"
lib.lock_files = lambda old: [ART + "/Cargo.lock"]
BASE = ["semio-s-artifact-norm-en1992-checks", "semio-s-artifact-norm-en1992-model", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene"]
FOUND = [("🧰️kit", "kit"), ("📍️layout", "layout"), ("🔬️mechanics", "mechanics"), ("🧱️reinforcement", "reinforcement"), ("🎞️tracks", "tracks")]
DIAG = [("🔗️bond", "bond"), ("🧷️ties", "ties"), ("🔐️integrity", "integrity"), ("📊️ledger", "ledger"), ("🧲️prestress-limit", "prestress_limit"), ("🧵️material-laws", "material_laws"), ("📏️crack-width-moment", "crack_width_moment"), ("🌀️creep-curve", "creep_curve"), ("📳️fatigue-range", "fatigue_range"), ("🔥️fire-capacity", "fire_capacity"), ("🛢️liquid-pressure", "liquid_pressure"), ("🧮️second-order", "second_order"), ("🌀️torsion-interaction", "torsion_interaction"), ("🕸️strut-and-tie", "strut_and_tie"), ("✂️shear", "shear"), ("🎨️grades", "grades"), ("🪨️crack-pattern", "crack_pattern"), ("📉️prestress-loss", "prestress_loss")]
assert not os.path.exists(FD), "already split"
for f, _ in FOUND:
    os.makedirs(FD, exist_ok=True)
    os.rename(SC + "/" + f, FD + "/" + f)
for f, _ in DIAG:
    os.rename(SC + "/" + f, DD + "/" + f)
VIS = re.compile(r"pub\((?:crate|super)\) ")
def widen(path):
    t = rd(path)
    wr(path, VIS.sub("pub ", t))
for folder, group in ((FD, FOUND), (DD, DIAG)):
    for f, _ in group:
        for dp, _, fs in os.walk(folder + "/" + f):
            for n in fs:
                if n.endswith(".rs"):
                    p = dp + "/" + n
                    if "/🧪️tests" in p:
                        continue
                    widen(p)
for f, _ in DIAG:
    p = DD + "/" + f + "/🦀️.rs"
    t = rd(p)
    t = t.replace("super::diagram_set::", "super::")
    wr(p, t)
k = FD + "/🧰️kit/🦀️.rs"
t = rd(k)
old_e = 'pub fn entities_of(document: &En1992Snapshot) -> Vec<EntityRef> {'
assert old_e in t
t = t.replace(old_e, 'pub fn entities_where(document: &En1992Snapshot, declared: impl Fn(&CaseRow) -> bool) -> Vec<EntityRef> {')
t = t.replace("filter(|e| super::cases::declared(e))", "filter(|e| declared(e))")
t = t.replace("    super::tracks::finish(scene);\n    super::prestress_loss::finish(scene);\n", "")
t = t.replace("use super::model::RcMember;", "use super::model::{CaseRow, RcMember};")
t = t.replace("/// 🆔️ Every document entity in outliner order: the members, then the anchors;", "/// 🆔️ Every document entity in outliner order: the members, then the anchors, then the cases `declared` accepts;")
t = t.replace("The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, the description and the field tracks.", "The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments and the description; the field tracks are added by the scene root.")
wr(k, t)
dr = DD + "/🦀️.rs"
widen(dr)
t = rd(dr)
head, body = t.split("\n\nuse super::kit::*;", 1)
body = "use kit::*;" + body
body = body.replace("use super::mechanics;\n", "").replace("use super::SceneInput;\n", "")
body = re.sub(r"\bsuper::", "", body)
body = body.replace("pub fn diagrams(input: &SceneInput", "pub fn diagrams(input: &SceneInput")
mods = "".join('#[path = "%s/🦀️.rs"]\npub mod %s;\n\n' % (f, m) for f, m in DIAG)
externs = "extern crate semio_s_artifact_norm_en1992_checks as checks;\nextern crate semio_s_artifact_norm_en1992_model as model;\nextern crate semio_s_artifact_norm_en1992_scene_foundation as foundation;\nextern crate semio_s_plugin_norm_compliance as norm;\nextern crate semio_s_plugin_norm_structural_scene as scene;\n\nuse foundation::{kit, layout, mechanics, reinforcement, tracks, SceneInput};\n\n"
wr(dr, head + "\n\n" + externs + mods + body)
fr = FD + "/🦀️.rs"
wr(fr, '''//! 🧱️ EN 1992 scene foundation: the entity builder and kit, the layout, the section mechanics, the bar positions and the field tracks every projection and diagram of the scene stands on.

extern crate semio_s_artifact_norm_en1992_checks as checks;
extern crate semio_s_artifact_norm_en1992_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;

#[path = "📍️layout/🦀️.rs"]
pub mod layout;

#[path = "🔬️mechanics/🦀️.rs"]
pub mod mechanics;

#[path = "🧱️reinforcement/🦀️.rs"]
pub mod reinforcement;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

use layout::Placement;
use model::En1992Snapshot;
use norm::CheckReport;
use scene::SceneRevision;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1992Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}
''')
def pkg(d, name, desc, deps, dev):
    p = d + "/📦️packages/🦀️rust"
    toml = '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % (relp(ART, p), name, desc)
    toml += "".join("%s = { workspace = true }\n" % x for x in sorted(deps))
    if dev:
        toml += '\n[dev-dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true, features = ["compliance-testing"] }\nserde_json = { workspace = true }\n'
    wr(p + "/Cargo.toml", toml)
    register_workspace(ART, name, p)
    script_register(ART, P, name)
    return p
pf = pkg(FD, F, "EN 1992 scene foundation: scene kit, layout, section mechanics, bar positions and field tracks", BASE, False)
pd = pkg(DD, D, "EN 1992 scene diagrams: the 2D diagram set with table twins and the projections that feed it", BASE + [F], True)
lock_register(P, F, new_deps=BASE)
lock_register(P, D, new_deps=BASE + [F])
add_dep(SC + "/📦️packages/🦀️rust/Cargo.toml", F)
add_dep(SC + "/📦️packages/🦀️rust/Cargo.toml", D)
root = SC + "/🦀️.rs"
t = rd(root)
for f, m in FOUND + DIAG + [("📈️diagrams", "diagram_set")]:
    t, n = re.subn(r'#\[path = "%s/🦀️\.rs"\]\n(?:pub )?mod %s;\n(?:pub use [^\n]*;\n)?\n' % (re.escape(f), m), "", t)
    assert n == 1, f
head, rest = t.split('#[path = "🏛️members/🦀️.rs"]', 1)
rest = '#[path = "🏛️members/🦀️.rs"]' + rest
new = '''pub use foundation::{layout, mechanics, reinforcement, tracks, SceneInput};
pub use foundation::kit::{entity_dependencies, EntityKindSpec, ENTITY_KINDS};
pub use foundation::layout::{Layout, Placement};
use foundation::kit;
pub use diagram_set::{bond, crack_width_moment, grades, integrity, ledger, material_laws, prestress_limit, second_order, shear, ties, torsion_interaction};
use diagram_set::{crack_pattern, creep_curve, fatigue_range, fire_capacity, liquid_pressure, prestress_loss, strut_and_tie};

'''
head = head.replace("pub use kit::{entities_of, entity_dependencies, EntityKindSpec, ENTITY_KINDS};\n", "").replace("pub use layout::{Layout, Placement};\n", "")
head = head.replace("extern crate semio_s_plugin_norm_structural_scene as scene;\n", "extern crate semio_s_artifact_norm_en1992_scene_diagrams as diagram_set;\nextern crate semio_s_artifact_norm_en1992_scene_foundation as foundation;\nextern crate semio_s_plugin_norm_structural_scene as scene;\n")
t = head + new + rest
i = t.index("/// 🎯️ Everything one projection reads")
j = t.index("/// 📦️ One complete projection")
t = t[:i] + t[j:]
t = t.replace("    kit::finish(&mut scene, summary(input));", "    finish(&mut scene, summary(input));").replace("        kit::finish(scene, summary(&input_of(doc, summary_only)));", "        finish(scene, summary(&input_of(doc, summary_only)));")
t = t.replace("fn run(input: &SceneInput", '''/// 🆔️ Every document entity in outliner order: the members, the anchors and the cases that declare geometry.
pub fn entities_of(document: &En1992Snapshot) -> Vec<EntityRef> {
    kit::entities_where(document, cases::declared)
}

/// 🏁️ The global parts of the merged scene: the kit's legends and layers, then the field tracks of the fire time, the age and the prestress.
fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    kit::finish(scene, description);
    tracks::finish(scene);
    prestress_loss::finish(scene);
}

fn run(input: &SceneInput''', 1)
t = t.replace("use scene::{live_scene, Diagram2d, EntityRef, Handle, LocalizedCopy, SceneRevision, StructuralScene};", "use scene::{live_scene, Diagram2d, EntityRef, Handle, LocalizedCopy, SceneRevision, StructuralScene};")
wr(root, t)
f = "/home/deck/semio/📋️build-owners.json"
b = rd(f)
anchor = '    {\n      "name": "%s-schema",' % "semio-s-artifact-norm-en1992"
assert anchor in b and D not in b
ent = ""
for n, d in ((D, pd), (F, pf)):
    rel = os.path.relpath(d, "/home/deck/semio")
    ent += '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/Cargo.toml",\n      "packages": []\n    },\n' % (n, rel, rel)
wr(f, b.replace(anchor, ent + anchor, 1))
print("done")
