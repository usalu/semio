"""🪚 Crate-budget K: EN 1998 scene kit, layout, tracks, handles, horizontal elements, ground and bridges and walls become the crate `-scene-geometry` (root `📐️geometry`); `-scene-core` keeps the building, vessel and joint-row layers and the live-scene contract and re-exports the modules. Idempotent."""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
C = D + "/🧱️core"
P = "semio-s-artifact-norm-en1998-scene-core"
N = "semio-s-artifact-norm-en1998-scene-geometry"
NU = N.replace("-", "_")
MODS = ["🧰️kit", "📍️layout", "🎞️tracks", "🧱️horizontal-elements", "🌉️ground-and-bridges", "🕹️handles", "🧱️walls"]
root = rd(C + "/🦀️.rs")
if NU in root:
    sys.exit("done")
for d in MODS:
    for f in glob.glob(D + "/" + d + "/**/*.rs", recursive=True):
        t = rd(f)
        n = re.sub(r"pub\((?:crate|super)\)", "pub", t)
        if n != t:
            wr(f, n)
lines = root.split("\n")
head = "//! 📐️ EN 1998 scene geometry: kit, layout, tracks, handles, horizontal elements, ground and bridges and walls shared by every layer. See [`semio_s_artifact_norm_en1998_scene_core`].\n\n"
head += "".join(l + "\n" for l in lines if l.startswith("extern crate")) + "\n"
mod_names = {"🧰️kit": "kit", "📍️layout": "layout", "🎞️tracks": "tracks", "🧱️horizontal-elements": "horizontal_elements", "🌉️ground-and-bridges": "ground_and_bridges", "🕹️handles": "handles", "🧱️walls": "walls"}
for d, m in mod_names.items():
    pat = re.compile(r'#\[path = "\.\./%s/🦀️\.rs"\]\n(?:pub(?:\(crate\))? )?mod %s;\n' % (re.escape(d), m))
    assert pat.search(root), d
    head += '#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (d, m)
    root = pat.sub("", root, 1)
head += "pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS, Out};\npub use layout::{Layout, Placement};\npub use handles::handles;\n"
head += "pub use semio_s_artifact_norm_en1998_scene_analysis::{assessment, capacity, geotechnical, response, storey_response};\npub use model::En1998Snapshot;\npub use norm::CheckReport;\npub use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};\npub use scene::projection_kit;\npub use scene::tessellation::Lod;\npub use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};\npub use std::collections::BTreeMap;\n\n"
m = re.search(r"/// 🎯️ Everything one projection reads.*?\n}\n\n", root, re.S)
head += m.group(0)
root = root.replace(m.group(0), "", 1)
wr(D + "/📐️geometry/🦀️.rs", head.rstrip("\n") + "\n")
root = root.replace("pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS};\n", "", 1)
root = root.replace("pub use layout::{Layout, Placement};\n", "", 1)
root = root.replace("pub use handles::handles;\n", "", 1)
root = root.replace("pub use kit::Out;\n", "", 1)
root = root.replace("pub use semio_s_artifact_norm_en1998_scene_analysis::{assessment, capacity, geotechnical, response, storey_response};\n", "pub use %s::{entities_of, entity_dependencies, entity_kind_of, ground_and_bridges, handles, horizontal_elements, kit, layout, tracks, walls, EntityKindSpec, Layout, Out, Placement, SceneInput, ENTITY_KINDS};\npub use semio_s_artifact_norm_en1998_scene_analysis::{assessment, capacity, geotechnical, response, storey_response};\n" % NU, 1)
root = re.sub(r"\n{3,}", "\n\n", root)
wr(C + "/🦀️.rs", root)
toml = C + "/📦️packages/🦀️rust/Cargo.toml"
deps = re.findall(r"(?m)^(semio-[\w-]+) = ", rd(toml).split("[dependencies]\n")[1].split("\n[")[0])
new_crate(ART, D + "/📐️geometry", N, "EN 1998 scene geometry: kit, layout, tracks, handles, horizontal elements, ground and bridges and walls", deps, P, toml)
add_dep(toml, N)
