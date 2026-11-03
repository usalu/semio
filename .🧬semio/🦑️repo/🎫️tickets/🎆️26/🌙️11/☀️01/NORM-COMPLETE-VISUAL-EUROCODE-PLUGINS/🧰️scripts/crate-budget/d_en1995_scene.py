"""🪚 EN 1995 scene: the pure `facts` module becomes `-scene-facts`."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from d_lib import *
ART = NORM + "/🗿️artifacts/🪵️en1995"
SC = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
P = "semio-s-artifact-norm-en1995-scene"
N = P + "-facts"
root = SC + "/🦀️.rs"
s = rd(root)
old = '#[path = "🧮️facts/🦀️.rs"]\npub mod facts;'
if old in s:
    f = SC + "/🧮️facts/🦀️.rs"
    t = rd(f)
    t = t.replace("use super::model::", "use model::", 1).replace("use super::norm::", "use norm::", 1).replace("use super::schema::", "use schema::", 1)
    head, rest = t.split("\n\n", 1)
    t = head + "\n\nextern crate semio_s_artifact_norm_en1995_model as model;\nextern crate semio_s_artifact_norm_en1995_schema as schema;\nextern crate semio_s_plugin_norm_compliance as norm;\n\n" + rest
    wr(f, t)
    new_crate(ART, SC + "/🧮️facts", N, "EN 1995 scene facts", ["semio-s-artifact-norm-en1995-model", "semio-s-artifact-norm-en1995-schema", "semio-s-plugin-norm-compliance"], P, SC + "/📦️packages/🦀️rust/Cargo.toml")
    add_dep(SC + "/📦️packages/🦀️rust/Cargo.toml", N)
    wr(root, s.replace(old, "pub use %s as facts;" % N.replace("-", "_")))
