"""🪚 Crate-budget H: EN 1993 scene members: the view modules that nothing below reads and the `fragment` that composes every module become the crate `-scene-members-projection` (root `🧩️projection`); the scene facade imports them from there."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🔩️en1993"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = ANY + "/🎬️scene-members"
P = "semio-s-artifact-norm-en1993-scene-members"
N = P + "-projection"
MODS = [("🧩️detail-sketches", "detail_sketches"), ("🏹️loads-and-supports", "loads_and_supports"), ("🧱️plates-and-shells", "plates_and_shells"), ("🧵️fatigue-views", "fatigue_views"), ("🌀️torsion-views", "torsion_views"), ("🔬️plate-details", "plate_details"), ("🏛️annex-blocks", "annex_blocks"), ("🕸️frame-model", "frame_model"), ("🎨️material-and-case-views", "material_and_case_views")]
sc = rd(ANY + "/🎬️scene/🦀️.rs")
if N.replace("-", "_") in sc:
    sys.exit("done")
root = rd(M + "/🦀️.rs")
for d, m in MODS:
    pat = '#[path = "%s/🦀️.rs"]\npub mod %s;\n\n' % (d, m)
    assert pat in root, d
    root = root.replace(pat, "")
fa = root.index("/// 🧩️ The part of the scene of `input`")
fragment = root[fa:]
root = root[:fa].rstrip("\n") + "\n"
root = root.replace("use kit::Out;\n", "").replace("use scene::projection::{Fragment, Part};\nuse scene::projection_kit;\n\n", "").replace("use scene::{EntityRef, LocalizedCopy, SceneRevision, StructuralScene};\nuse std::collections::BTreeMap;\n", "use scene::SceneRevision;\n")
wr(M + "/🦀️.rs", root)
new = "//! 🧩️ EN 1993 scene projection: the view modules of details, loads, plates, fatigue, torsion, annex blocks and the frame model, and the `fragment` that composes every module of the scene members. See [`semio_s_artifact_norm_en1993_scene_members`].\n\n"
new += "".join("extern crate %s as %s;\n" % x for x in [("semio_s_artifact_norm_en1993_checks", "checks"), ("semio_s_artifact_norm_en1993_model", "model"), ("semio_s_plugin_norm_compliance", "norm"), ("semio_s_plugin_norm_mechanics", "mechanics"), ("semio_s_plugin_norm_structural_scene", "scene"), ("semio_s_artifact_norm_en1993_scene_members", "base")])
new += "\nuse base::kit::Out;\nuse base::layout::Layout;\nuse base::{eigen, handles, joint_rows, joints, kit, layout, members, tracks, SceneInput};\nuse scene::projection::{Fragment, Part};\n\n"
new += "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in MODS) + "\n" + fragment
wr(M + "/🧩️projection/🦀️.rs", new)
sc = sc.replace("use scene_members::{kit, tracks, detail_sketches, fatigue_views};", "use scene_members::{kit, tracks};\nuse projection::{detail_sketches, fatigue_views};")
sc = sc.replace("pub use scene_members::{fragment, SceneInput};", "pub use scene_members::SceneInput;\npub use projection::fragment;")
sc = sc.replace("extern crate semio_s_artifact_norm_en1993_scene_members as scene_members;\n", "extern crate semio_s_artifact_norm_en1993_scene_members as scene_members;\nextern crate %s as projection;\n" % N.replace("-", "_"), 1)
wr(ANY + "/🎬️scene/🦀️.rs", sc)
mt = M + "/📦️packages/🦀️rust/Cargo.toml"
t = rd(mt)
deps = re.findall(r"(?m)^(semio-[\w-]+) = ", t.split("[dependencies]\n")[1].split("\n[")[0])
new_crate(ART, M + "/🧩️projection", N, "EN 1993 scene projection: detail, load, plate, fatigue, torsion, annex and frame views and the composed fragment", deps + [P], P, mt)
add_dep(ANY + "/🎬️scene/📦️packages/🦀️rust/Cargo.toml", N)
