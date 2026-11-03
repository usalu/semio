"""🪚 Crate-budget H: the helpers and inline formula modules (parts 1-1 to 4) of the EN 1991 checks root become the crate `-checks-parts` (root `🧱️parts`); checks re-exports them and keeps the catalogue."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🏋️en1991"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
P = "semio-s-artifact-norm-en1991-checks"
N = P + "-parts"
root = rd(D + "/🦀️.rs")
if N.replace("-", "_") in root:
    sys.exit("done")
a = root.index("fn pressure_pa")
b = root.index("pub use semio_s_artifact_norm_en1991_spec_trace as spec_trace;")
region = root[a:b]
region = re.sub(r"(?m)^fn ", "pub fn ", region)
comp = [m for m in "CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef".split(", ") if re.search(r"\b%s\b" % m, region)]
head = "//! 🧱️ EN 1991 check parts: the unit and remedy helpers and the closed-form formulas of parts 1-1 to 4. See [`super`].\n\n#![allow(async_fn_in_trait)]\n\n"
if re.search(r"\bAnnexChoice\b", region):
    head += "use semio_s_plugin_norm_compliance::AnnexChoice;\n"
head += "use semio_s_plugin_norm_compliance::{%s};\n\n" % ", ".join(comp)
wr(D + "/🧱️parts/🦀️.rs", head + region.rstrip("\n") + "\n")
root = root[:a] + "pub use %s::*;\n\n" % N.replace("-", "_") + root[b:]
wr(D + "/🦀️.rs", root)
toml = D + "/📦️packages/🦀️rust/Cargo.toml"
t = rd(toml)
deps = re.findall(r"(?m)^(semio-[\w-]+) = ", t.split("[dependencies]\n")[1].split("\n[")[0])
new_crate(ART, D + "/🧱️parts", N, "EN 1991 check parts: helpers and closed-form formulas", [d for d in deps if "compliance" in d or "mechanics" in d or "framework" in d], P, toml, dev="")
add_dep(toml, N)
