"""🪚 Crate-budget H: the EN 1999 member, connection, fire/fatigue, thin-wall, weld-run, outline and free-outline checks become the crate `-checks-members` (root `🧱️members`); checks re-exports it."""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🪶️en1999"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
P = "semio-s-artifact-norm-en1999-checks"
N = P + "-members"
MODS = [("🧱️member-checks", "member_checks"), ("🔗️connection-checks", "connection_checks"), ("🔥️fire-fatigue-checks", "fire_fatigue_checks"), ("🥫️thin-wall-checks", "thin_wall_checks"), ("🧵️weld-run-checks", "weld_run_checks"), ("🔷️outline-checks", "outline_checks"), ("🌀️free-outline-stability", "free_outline_stability"), ("🪢️free-outline-torsion", "free_outline_torsion")]
root = rd(D + "/🦀️.rs")
if N.replace("-", "_") in root:
    sys.exit("done")
text = "\n".join(rd(f) for d, _ in MODS for f in glob.glob(D + "/" + d + "/**/*.rs", recursive=True))
head = root[: root.index("pub use semio_s_artifact_norm_en1999_check_parts::*;")]
helpers_start = head.index("//#region 🔖️ComplianceHelpers")
helpers = re.sub(r"(?m)^fn ", "pub fn ", head[helpers_start:])
use_model = re.search(r"use semio_s_artifact_norm_en1999_model::\{([^}]*)\};", head).group(1).split(", ")
model = [m for m in use_model if re.search(r"\b%s\b" % m, text)]
scope = re.search(r"\bScope\b", text)
comp = [m for m in ["AnnexChoice", "CheckReport", "CheckResult", "ClauseId", "LocalizedCopy", "Quantity", "QuantityKind", "Remedy", "SubjectRef"] if re.search(r"\b%s\b" % m, text + helpers)]
new_root = "//! 🧱️ EN 1999 member checks: members, connections, fire and fatigue, thin-wall shells and sheets, weld runs, outlines and free-outline stability and torsion. See [`super`].\n\n#![allow(async_fn_in_trait)]\n\n"
new_root += "".join("extern crate %s as %s;\n" % x for x in [("semio_framework_os_kernel", "dsl"), ("semio_framework_os_kernel", "protocol"), ("semio_framework_os_kernel", "store"), ("semio_framework_schema", "framework_schema"), ("semio_framework_value_derive", "value_derive")]) + "\n"
new_root += "use semio_s_artifact_norm_en1999_model::{%s};\n" % ", ".join(model)
if scope:
    new_root += "use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;\n"
new_root += "use semio_s_plugin_norm_compliance::{%s};\n\n" % ", ".join(comp)
new_root += helpers.rstrip("\n") + "\n\npub use semio_s_artifact_norm_en1999_check_parts::*;\n\n"
for d, m in MODS:
    new_root += '#[path = "../%s/🦀️.rs"]\n%smod %s;\npub use %s::*;\n\n' % (d, "pub " if False else "", m, m)
wr(D + "/🧱️members/🦀️.rs", new_root.rstrip("\n").replace("//#endregion 🔖️ComplianceHelpers", "").rstrip("\n") + "\n")
retarget_visibility([f for d, _ in MODS for f in glob.glob(D + "/" + d + "/**/*.rs", recursive=True)])
for d, m in MODS:
    pat = '#[path = "%s/🦀️.rs"]\nmod %s;\n' % (d, m)
    assert pat in root, d
    root = re.sub(re.escape(pat) + r"(?:pub )?use %s::\*;\n\n?" % m, "", root)
root = root.replace("pub use semio_s_artifact_norm_en1999_check_parts::*;\n", "pub use %s::*;\n" % N.replace("-", "_"), 1)
root = re.sub(r"(?m)^fn (q_\w+|loc)\(.*\n", "", root)
root = re.sub(r"\n{3,}", "\n\n", root)
wr(D + "/🦀️.rs", root)
toml = D + "/📦️packages/🦀️rust/Cargo.toml"
dev = rd(toml).split("[dev-dependencies]\n")[1]
deps = re.findall(r"(?m)^(semio-[\w-]+) = ", rd(toml).split("[dependencies]\n")[1].split("[dev-dependencies]")[0])
new_crate(ART, D + "/🧱️members", N, "EN 1999 member, connection, fire, fatigue, shell, weld and outline checks", deps, P, toml, dev=dev)
add_dep(toml, N)
