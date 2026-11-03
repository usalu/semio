"""🪚 Crate-budget H: all EN 1997 check modules except the project checks and the units become the crate `-checks-geotechnics` (root `🧱️geotechnics`); checks re-exports them."""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🌍️en1997"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
P = "semio-s-artifact-norm-en1997-checks"
N = P + "-geotechnics"
MODS = [("📐️geotechnical-parts", "geotechnical_parts", "pub use"), ("🪨️layer-and-duplicate-id-helpers", "layer_and_duplicate_id_helpers", "pub use"), ("📏️sls-pairs", "sls_pairs", "pub use"), ("📍️station-fields", "station_fields", "pub use"), ("🌐️geo-entities", "geo_entities", "pub use"), ("🧾️entity-catalogue-cases", "entity_catalogue_cases", "pub use"), ("🌋️lateral-and-seismic-checks", "lateral_and_seismic", "pub use"), ("🧱️pile-wall-slope-and-uplift-checks", "pile_wall_slope_and_uplift_checks", "pub use")]
root = rd(D + "/🦀️.rs")
if N.replace("-", "_") in root:
    sys.exit("done")
files = [f for d, _, _ in MODS for f in glob.glob(D + "/" + d + "/**/*.rs", recursive=True)]
text = "\n".join(rd(f) for f in files)
retarget_visibility(files)
for f in files:
    t = rd(f)
    if "pub(crate)" in t:
        wr(f, t.replace("pub(crate)", "pub"))
body = "//! 🧱️ EN 1997 geotechnical checks: parts, helpers, serviceability pairs, station fields, geo entities, entity catalogue cases, lateral and seismic checks and the pile, wall, slope and uplift checks. See [`super`].\n\n#![allow(async_fn_in_trait)]\n\n"
body += "".join("extern crate %s as %s;\n" % x for x in [("semio_framework_os_kernel", "dsl"), ("semio_framework_os_kernel", "protocol"), ("semio_framework_os_kernel", "store"), ("semio_framework_schema", "framework_schema"), ("semio_framework_value_derive", "value_derive")]) + "\n"
model = [m for m in "En1997Snapshot, FoundationLoadCase, SoilLayer, SpreadFoundation".split(", ") if re.search(r"\b%s\b" % m, text)]
comp = [m for m in "CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef".split(", ") if re.search(r"\b%s\b" % m, text)]
if re.search(r"\bScope\b", text):
    body += "use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;\n"
if re.search(r"\bAnnexChoice\b", text):
    body += "use semio_s_plugin_norm_compliance::AnnexChoice;\n"
body += "use semio_s_artifact_norm_en1997_model::{%s};\nuse semio_s_plugin_norm_compliance::{%s};\n\npub use semio_s_artifact_norm_en1997_checks_catalogue::*;\n\n" % (", ".join(model), ", ".join(comp))
for d, m, _ in MODS:
    body += '#[path = "../%s/🦀️.rs"]\npub mod %s;\npub use %s::*;\n\n' % (d, m, m)
wr(D + "/🧱️geotechnics/🦀️.rs", body.rstrip("\n") + "\n")
for d, m, _ in MODS:
    root, n = re.subn(r'#\[path = "%s/🦀️\.rs"\]\n(?:pub )?mod %s;\n(?:(?:pub )?use %s::\*;\n)?\n?' % (re.escape(d), m, m), "", root)
    assert n == 1, d
root = root.replace("pub use semio_s_artifact_norm_en1997_checks_catalogue::*;\n", "pub use semio_s_artifact_norm_en1997_checks_catalogue::*;\npub use %s::*;\n" % N.replace("-", "_"), 1)
wr(D + "/🦀️.rs", root)
toml = D + "/📦️packages/🦀️rust/Cargo.toml"
t = rd(toml)
dev = t.split("[dev-dependencies]\n")[1] if "[dev-dependencies]" in t else ""
deps = re.findall(r"(?m)^(semio-[\w-]+) = ", t.split("[dependencies]\n")[1].split("\n[")[0])
new_crate(ART, D + "/🧱️geotechnics", N, "EN 1997 geotechnical checks", deps, P, toml, dev=dev)
add_dep(toml, N)
