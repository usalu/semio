"""🪚 Crate-budget H: EN 1998 inference helpers, part details and geotechnical details become the crate `-inferences-details` (root `🧱️details`); the inference imports them."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences"
P = "semio-s-artifact-norm-en1998-inferences"
N = P + "-details"
MODS = [("🧰️seismic-helpers", "seismic_helpers"), ("🪨️part-details", "part_details"), ("🌊️geotechnical-details", "geotechnical_details")]
root = D + "/🦀️.rs"
r = rd(root)
if N.replace("-", "_") in r:
    sys.exit("done")
new_root = D + "/🧱️details/🦀️.rs"
body = "//! 🧱️ EN 1998 inference details: the quantity constructors and annex resolution, the part details (towers, chord rotation, piers, members, N2) and the geotechnical details (liquefaction, bearing, piles, soil profile). See [`super`].\n\n#![allow(async_fn_in_trait)]\n\n"
body += "use semio_s_artifact_norm_en1998_model::En1998Snapshot;\nuse semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};\nuse semio_s_artifact_norm_en1998_schema::{cracked_stiffness, na_de, part_1, part_2, part_3, part_5, part_6, AnnexParams};\n\n"
body += "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in MODS) + "//#endregion 🪢️Mounts\n"
wr(new_root, body)
retarget_visibility([D + "/%s/🦀️.rs" % d for d, _ in MODS])
for d, m in MODS:
    pat = '#[path = "%s/🦀️.rs"]\nmod %s;\nuse %s::*;\n\n' % (d, m, m)
    assert pat in r, d
    r = r.replace(pat, "")
use = "use %s::{geotechnical_details::*, part_details::*, seismic_helpers::*};\n\n" % N.replace("-", "_")
anchor = "use semio_s_artifact_norm_en1998_structure_checks::"
assert anchor in r
r = r.replace(anchor, use + anchor, 1)
wr(root, r)
old_toml = D + "/📦️packages/🦀️rust/Cargo.toml"
dev = ""
new_crate(ART, D + "/🧱️details", N, "EN 1998 inference details: helpers, part details and geotechnical details", ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance"], P, old_toml, dev=dev)
add_dep(old_toml, N)
