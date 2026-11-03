"""🪚 Crate-budget I: EN 1994 scene statics, facts, layout, kit and handles become the crate `-scene-base` (root `🧱️base`); the scene keeps the projections, tracks and diagrams and re-imports the modules."""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🧩️en1994"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
P = "semio-s-artifact-norm-en1994-scene"
N = P + "-base"
MODS = [("🌀️statics", "statics"), ("🧮️facts", "facts"), ("📍️layout", "layout"), ("🧰️kit", "kit"), ("🕹️handles", "handles")]
root = rd(D + "/🦀️.rs")
if N.replace("-", "_") in root:
    sys.exit("done")
kit = rd(D + "/🧰️kit/🦀️.rs")
kit = kit.replace("super::tracks::LAYER_MODE_BUCKLING", "LAYER_MODE_BUCKLING").replace("super::tracks::LAYER_MODE_LTB", "LAYER_MODE_LTB").replace("    super::tracks::finish(scene);\n", "")
i = kit.index("pub(crate) const LAYER_STEEL")
kit = kit[:i] + 'pub const LAYER_MODE_BUCKLING: &str = "mode-buckling";\npub const LAYER_MODE_LTB: &str = "mode-ltb";\n' + kit[i:]
wr(D + "/🧰️kit/🦀️.rs", kit)
tr = rd(D + "/🎞️tracks/🦀️.rs")
tr = tr.replace('pub(crate) const LAYER_MODE_BUCKLING: &str = "mode-buckling";\npub(crate) const LAYER_MODE_LTB: &str = "mode-ltb";\n', "")
wr(D + "/🎞️tracks/🦀️.rs", tr)
root = root.replace("        kit::finish(scene, summary(&input_of(doc, summary_only)));\n", "        kit::finish(scene, summary(&input_of(doc, summary_only)));\n        tracks::finish(scene);\n")
files = [f for d, _ in MODS for f in glob.glob(D + "/" + d + "/**/*.rs", recursive=True)]
for f in files:
    t = rd(f)
    n = re.sub(r"pub\((?:crate|super)\)", "pub", t)
    if n != t:
        wr(f, n)
text = "\n".join(rd(f) for f in files)
ext = [("semio_s_artifact_norm_en1994_checks", "checks"), ("semio_s_artifact_norm_en1994_model", "model"), ("semio_s_plugin_norm_compliance", "norm"), ("semio_s_plugin_norm_structural_scene", "scene")]
base = "//! 🧱️ EN 1994 scene base: statics, facts, layout, kit and handles shared by every projection. See [`super`].\n\n" + "".join("extern crate %s as %s;\n" % x for x in ext) + "\n"
base += "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in MODS)
wr(D + "/🧱️base/🦀️.rs", base)
for d, m in MODS:
    pat = '#[path = "%s/🦀️.rs"]\n' % d
    assert pat in root, d
    root = re.sub(re.escape(pat) + r"(?:pub )?mod %s;\n" % m, "", root)
root = root.replace("pub use layout::{Layout, Placement};\n", "pub use layout::{Layout, Placement};\n")
root = root.replace("extern crate semio_s_plugin_norm_structural_scene as scene;\n", "extern crate semio_s_plugin_norm_structural_scene as scene;\nextern crate %s as base;\n" % N.replace("-", "_"), 1)
root = root.replace("use kit::Out;\n", "pub use base::{facts, handles, layout};\nuse base::{kit, statics};\nuse kit::Out;\n", 1)
root = re.sub(r"\n{3,}", "\n\n", root)
wr(D + "/🦀️.rs", root)
toml = D + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, D + "/🧱️base", N, "EN 1994 scene base: statics, facts, layout, kit and handles", ["semio-s-artifact-norm-en1994-checks", "semio-s-artifact-norm-en1994-model", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene"], P, toml)
add_dep(toml, N)
