"""🪚 Crate-budget H: the EN 1991 inference oracle wiring becomes the crate `-inferences-oracle-wiring` (file stays in place); the comparison test and the oracle manifest point to it."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🏋️en1991"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
INF = ANY + "/🧬️schema/💡️inferences"
OLD, NEW = "semio-s-artifact-norm-en1991-inferences", "semio-s-artifact-norm-en1991-inferences-oracle-wiring"
W = INF + "/🧪️tests/🎯️oracle-wiring"
root = INF + "/🦀️.rs"
s = rd(root)
m = '#[cfg(any(test, feature = "oracle-wiring"))]\n#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]\npub mod oracle_wiring;\n'
if m not in s:
    sys.exit("done")
wr(root, s.replace(m, "", 1))
toml = INF + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, W, NEW, "EN 1991 oracle wiring: the adapter registry of the oracle comparison", ["semio-s-artifact-norm-en1991-checks"], OLD, toml)
c = rd(toml)
c = re.sub(r'oracle-wiring = [^\n]*\n', "", c)
wr(toml, c)
p = W + "/📦️packages/🦀️rust/Cargo.toml"
c = rd(p)
c = c.replace("[dependencies]\n", '[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true, features = ["compliance-testing"] }\n', 1)
wr(p, c)
lock_register(OLD, NEW, new_deps=["semio-s-artifact-norm-en1991-checks", "semio-s-plugin-norm-compliance"])
cmp = ANY + "/🧪️tests/🎯️oracle-comparison-en1991-1/🦀️.rs"
wr(cmp, rd(cmp).replace("semio_s_artifact_norm_en1991_inferences::oracle_wiring::registry()", "semio_s_artifact_norm_en1991_inferences_oracle_wiring::registry()"))
j = ANY + "/🔮️oracles/🔣️.json"
c = rd(j)
old = '''      "package": "semio-s-artifact-norm-en1991-inferences",
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust",
      "rationale": "🚦️ The check-outcome case runs the production evaluate of the inferences crate on a committed snapshot.",
      "features": [
        "oracle-wiring"
      ]'''
new = '''      "package": "semio-s-artifact-norm-en1991-inferences-oracle-wiring",
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🎯️oracle-wiring/📦️packages/🦀️rust",
      "rationale": "🚦️ The check-outcome case runs the production evaluate through the oracle wiring crate of the inferences."'''
assert old in c
wr(j, c.replace(old, new, 1))
