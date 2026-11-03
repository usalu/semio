"""🪚 Crate-budget G: the EN 1992 checks oracle wiring (feature-gated, 517 lines) becomes the crate `-checks-oracle-wiring`; file stays in place, `crate::` becomes `checks::`."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
ART = NORM + "/🗿️artifacts/🏛️en1992"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
CHK = ANY + "/🧬️schema/💡️inferences/⚖️checks"
OLD, NEW = "semio-s-artifact-norm-en1992-checks", "semio-s-artifact-norm-en1992-checks-oracle-wiring"
W = CHK + "/🧪️tests/🎯️oracle-wiring"
root = CHK + "/🦀️.rs"
s = rd(root)
m = '#[cfg(any(test, feature = "oracle-wiring"))]\n#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]\npub mod oracle_wiring;\n\n'
if m in s:
    wr(root, s.replace(m, "", 1))
    t = rd(W + "/🦀️.rs")
    head, rest = t.split("\n\n", 1)
    wr(W + "/🦀️.rs", head + "\n\nextern crate semio_s_artifact_norm_en1992_checks as checks;\n\n" + rest.replace("crate::", "checks::"))
    new_crate(ART, W, NEW, "EN 1992 oracle wiring: the adapter registries of the oracle comparison", [OLD, "semio-s-plugin-norm-mechanics"], OLD, CHK + "/📦️packages/🦀️rust/Cargo.toml")
    toml = CHK + "/📦️packages/🦀️rust/Cargo.toml"
    c = rd(toml)
    c = re.sub(r'\[features\]\noracle-wiring = [^\n]*\n\n', "", c).replace('semio-s-plugin-norm-mechanics = { workspace = true, optional = true }\n', "")
    wr(toml, c)
    p = W + "/📦️packages/🦀️rust/Cargo.toml"
    c = rd(p)
    c = c.replace("semio-s-plugin-norm-compliance = { workspace = true }\n", "")
    c = c.replace("[dependencies]\n", '[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true, features = ["compliance-testing"] }\n', 1)
    wr(p, c)
cmp = ANY + "/🧪️tests/🎯️oracle-comparison-en1992-1/🦀️.rs"
c = rd(cmp)
wr(cmp, c.replace("semio_s_artifact_norm_en1992_checks::oracle_wiring::registry()", "semio_s_artifact_norm_en1992_checks_oracle_wiring::registry()"))
j = ANY + "/🔮️oracles/🔣️.json"
c = rd(j)
old = '''      "package": "semio-s-artifact-norm-en1992-checks",
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust",
      "rationale": "📇️ The catalogue examples drive the entry evaluators the checks crate owns.",
      "features": [
        "oracle-wiring"
      ]'''
new = '''      "package": "semio-s-artifact-norm-en1992-checks-oracle-wiring",
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/📦️packages/🦀️rust",
      "rationale": "📇️ The catalogue examples drive the entry evaluators through the oracle wiring crate of the checks."'''
if old in c:
    wr(j, c.replace(old, new, 1))
else:
    print("json block unmatched")
