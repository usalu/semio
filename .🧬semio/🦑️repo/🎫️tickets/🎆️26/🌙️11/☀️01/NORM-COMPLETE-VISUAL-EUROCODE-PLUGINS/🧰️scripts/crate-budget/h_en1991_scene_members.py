"""🪚 Crate-budget H: EN 1991 scene members: lane bands and the stack level move into `layout`; traffic, hazards and `fragment` become the crate `-scene-members-projection` (root `🧩️projection`); the scene facade imports them from there."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🏋️en1991"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = ANY + "/🎬️scene-members"
P = "semio-s-artifact-norm-en1991-scene-members"
N = P + "-projection"
mroot = rd(M + "/🦀️.rs")
if N.replace("-", "_") in rd(ANY + "/🎬️scene/🦀️.rs"):
    sys.exit("done")
traffic = rd(M + "/🌉️traffic/🦀️.rs")
a = traffic.index("/// 🛣️ One lane of the deck")
b = traffic.index("struct Deck<'a>")
band = traffic[a:b]
traffic = traffic[:a] + traffic[b:]
band = band.replace("    fn sub(", "    pub fn sub(").replace("    fn label_id(", "    pub fn label_id(").replace("    fn name(", "    pub fn name(").replace("part_2::notional_lanes", "super::checks::part_2::notional_lanes")
traffic = traffic.replace("use super::layout::{deck_width,", "use super::layout::{bands_of, deck_width, Band,", 1)
wr(M + "/🌉️traffic/🦀️.rs", traffic)
hz = rd(M + "/🚁hazards/🦀️.rs")
hz = hz.replace("const STRIP_Z: f64 = 0.15;\nconst STACK_STEP: f64 = 0.4;\n", "")
sl = re.search(r"/// 🧱️ Height of the construction load stack.*?\n}\n\n", hz, re.S)
stack = sl.group(0)
hz = hz.replace(stack, "")
hz = hz.replace("use super::collections::{stage_row_y, stage_slots};\n", "use super::collections::{stage_row_y, stage_slots};\nuse super::layout::stack_level;\n", 1)
hz = re.sub(r"\bSTRIP_Z\b", "super::layout::STRIP_Z", hz) if False else hz
wr(M + "/🚁hazards/🦀️.rs", hz)
lay = rd(M + "/📍️layout/🦀️.rs")
lay = lay.rstrip("\n") + "\n\npub const STRIP_Z: f64 = 0.15;\npub const STACK_STEP: f64 = 0.4;\n\n" + stack.replace("d: &En1991Snapshot", "d: &En1991Snapshot") + band.rstrip("\n") + "\n"
wr(M + "/📍️layout/🦀️.rs", lay)
h = rd(M + "/🕹️handles/🦀️.rs")
h = h.replace("super::traffic::bands_of", "super::layout::bands_of").replace("super::hazards::stack_level", "super::layout::stack_level")
wr(M + "/🕹️handles/🦀️.rs", h)
mods = re.search(r"(?s)fn_start", "") 
fa = mroot.index("/// 🧩️ The part of the scene of `input`")
fragment = mroot[fa:]
mroot = mroot[:fa].rstrip("\n") + "\n"
for d, m in [("🌉️traffic", "traffic"), ("🚁hazards", "hazards")]:
    mroot = mroot.replace('#[path = "%s/🦀️.rs"]\npub mod %s;\n' % (d, m), "")
mroot = mroot.replace("use kit::Out;\n", "").replace("use scene::projection::{Fragment, Part};\nuse scene::projection_kit;\n\n", "").replace("use scene::{EntityRef, LocalizedCopy, SceneRevision, StructuralScene};\nuse std::collections::BTreeMap;\n", "use scene::SceneRevision;\n")
wr(M + "/🦀️.rs", mroot)
root = "//! 🧩️ EN 1991 scene projection: the traffic and hazard modules and the `fragment` that composes every module of the scene members into the part of the scene a caller asks for. See [`semio_s_artifact_norm_en1991_scene_members`].\n\n"
root += "".join("extern crate %s as %s;\n" % x for x in [("semio_s_artifact_norm_en1991_checks", "checks"), ("semio_s_artifact_norm_en1991_model", "model"), ("semio_s_artifact_norm_en1991_scene_fields", "fields"), ("semio_s_artifact_norm_en1991_scene_members", "members"), ("semio_s_plugin_norm_compliance", "norm"), ("semio_s_plugin_norm_structural_scene", "scene")])
root += "\nuse members::kit::Out;\nuse members::layout::Layout;\nuse members::{accidental, cases, collections, deflection, dynamics, envelope, gravity, handles, kit, layout, silo, snow, thermal_fire, wave3, wind, Ctx, SceneInput};\nuse scene::projection::{Fragment, Part};\n\n"
root += '#[path = "../🌉️traffic/🦀️.rs"]\npub mod traffic;\n#[path = "../🚁hazards/🦀️.rs"]\npub mod hazards;\n\n' + fragment
wr(M + "/🧩️projection/🦀️.rs", root)
sc = rd(ANY + "/🎬️scene/🦀️.rs")
sc = sc.replace("use members::{kit, envelope, wind, snow, gravity, thermal_fire, accidental, traffic, collections,", "use projection::traffic;\nuse members::{kit, envelope, wind, snow, gravity, thermal_fire, accidental, collections,")
sc = sc.replace("pub use members::{fragment, SceneInput};", "pub use members::SceneInput;\npub use projection::fragment;")
sc = sc.replace("extern crate semio_s_artifact_norm_en1991_scene_members as members;\n", "extern crate semio_s_artifact_norm_en1991_scene_members as members;\nextern crate %s as projection;\n" % N.replace("-", "_"), 1)
wr(ANY + "/🎬️scene/🦀️.rs", sc)
tt = ANY + "/🎬️scene/🧪️tests/🔬️unit/🦀️.rs"
wr(tt, rd(tt).replace("use members::hazards::{block_side, exaggeration};", "use projection::hazards::{block_side, exaggeration};"))
mt = M + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, M + "/🧩️projection", N, "EN 1991 scene projection: traffic, hazards and the composed fragment", ["semio-s-artifact-norm-en1991-checks", "semio-s-artifact-norm-en1991-model", "semio-s-artifact-norm-en1991-scene-fields", P, "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-mechanics", "semio-s-plugin-norm-structural-scene"], P, mt)
sct = ANY + "/🎬️scene/📦️packages/🦀️rust/Cargo.toml"
add_dep(sct, N)
