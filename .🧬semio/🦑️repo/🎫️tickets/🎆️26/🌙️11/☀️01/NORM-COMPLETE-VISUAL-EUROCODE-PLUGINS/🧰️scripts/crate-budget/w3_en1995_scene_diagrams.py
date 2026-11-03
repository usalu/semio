"""🪚 EN 1995 scene: the `diagrams` module (135 KB of the 262144-byte budget) becomes `-scene-diagrams`; idempotent."""
import os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
ART = NORM + "/🗿️artifacts/🪵️en1995"
SC = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
P = "semio-s-artifact-norm-en1995-scene"
N = P + "-diagrams"
DEPS = [P + "-entities", P + "-facts", "semio-s-artifact-norm-en1995-model", "semio-s-artifact-norm-en1995-schema", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene"]
import lib
lib.lock_files = lambda old: [ART + "/Cargo.lock"]
root = SC + "/🦀️.rs"
s = rd(root)
old = '#[path = "📈️diagrams/🦀️.rs"]\npub mod diagrams;\npub use diagrams::diagrams;'
if old in s:
    f = SC + "/📈️diagrams/🦀️.rs"
    t = rd(f)
    if "extern crate" not in t:
        docs, rest = t.split("\n\nuse super::facts", 1)
        rest = "use super::facts" + rest
        for a, b in [
            ("use super::facts::{self, Dynamics};\nuse super::kit::{loc, worst};\n", "use entities::worst;\nuse facts::Dynamics;\nuse scene::projection_kit::loc;\n"),
            ("use super::model::", "use model::"), ("use super::norm::", "use norm::"), ("use super::schema::", "use schema::"),
            ("use super::scene::", "use scene::"), ("super::scene::utilisation_tone", "scene::utilisation_tone"), ("super::norm::UTILIZATION_TOLERANCE", "norm::UTILIZATION_TOLERANCE"),
            ("pub(crate) fn case_diagrams", "pub fn case_diagrams"),
        ]:
            assert a in rest, a
            rest = rest.replace(a, b)
        externs = "extern crate semio_s_artifact_norm_en1995_model as model;\nextern crate semio_s_artifact_norm_en1995_scene_entities as entities;\nextern crate semio_s_artifact_norm_en1995_scene_facts as facts;\nextern crate semio_s_artifact_norm_en1995_schema as schema;\nextern crate semio_s_plugin_norm_compliance as norm;\nextern crate semio_s_plugin_norm_structural_scene as scene;\n\n"
        wr(f, docs + "\n\n" + externs + rest)
    pkg = SC + "/📈️diagrams/📦️packages/🦀️rust"
    toml = '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1995 scene diagrams: 2D diagrams with table twins and draggable markers"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % (relp(ART, pkg), N)
    toml += "".join("%s = { workspace = true }\n" % d for d in sorted(DEPS))
    wr(pkg + "/Cargo.toml", toml)
    register_workspace(ART, N, pkg)
    script_register(ART, P, N)
    lock_register(P, N, new_deps=DEPS)
    add_dep(SC + "/📦️packages/🦀️rust/Cargo.toml", N)
    wr(root, s.replace(old, "pub use %s as diagrams;\npub use diagrams::diagrams;" % N.replace("-", "_")))
    f = "/home/deck/semio/📋️build-owners.json"
    b = rd(f)
    anchor = '    {\n      "name": "%s-entities",' % P
    rel = os.path.relpath(pkg, "/home/deck/semio")
    entry = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/Cargo.toml",\n      "packages": []\n    },\n' % (N, rel, rel)
    assert anchor in b and N not in b
    wr(f, b.replace(anchor, entry + anchor, 1))
print("done")
