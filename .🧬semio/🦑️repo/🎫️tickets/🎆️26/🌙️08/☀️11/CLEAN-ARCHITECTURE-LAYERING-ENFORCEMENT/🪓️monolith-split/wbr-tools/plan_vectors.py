"""vectors: lift the committed vector reader and the law set of the remodeling mutation vector library into children (hand-cut, coverage-checked)."""
import os, sys, re, shutil
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

name = "vectors"
T = TARGETS[name]
src = read(snap_dir(name) + "/" + T["file"])
ls = lines_of(src)
cov = Coverage(len(ls))
base = os.path.dirname(T["file"])


def seg(a, b):
    return ls[a - 1:b]


assert ls[15].startswith("//#region 🧫️Vector") and ls[97].startswith("//#endregion 🧫️Vector")
assert ls[99].startswith("//#region ⚖️Laws") and ls[206].startswith("//#endregion ⚖️Laws")
assert ls[8].startswith("use semio_framework_os_kernel as protocol;") and ls[13].startswith("use std::path")
cov.claim("root-doc", 1, 8 - 1)
cov.claim("imports", 9, 14)
cov.claim("vector", 16, 98)
cov.claim("laws", 100, 207)
for i in (8, 15, 99):
    cov.claim("blank", i, i) if ls[i - 1] == "" else None
cov.owner[8] = cov.owner[8] or "blank"

vec = seg(16, 98)
WIDEN_FIELDS = ["label", "before", "after", "mutation", "diff", "absent_marker"]
WIDEN_FNS = ["expected_after", "declared", "status", "produced", "committed_diff"]
out = []
for l in vec:
    m = re.match(r"^(\s*)(label|before|after|mutation|diff|absent_marker): ", l)
    if m and l.startswith("    ") and not l.startswith("     ") and l.rstrip().endswith(","):
        l = m.group(1) + "pub(super) " + l[len(m.group(1)):]
    m = re.match(r"^(\s*)fn (expected_after|declared|status|produced|committed_diff)\(", l)
    if m:
        l = m.group(1) + "pub(super) " + l[len(m.group(1)):]
    m = re.match(r"^fn (json_of|refusal_level)", l)
    if m:
        l = "pub(super) " + l
    out.append(l)
widened = sum(1 for a, b in zip(vec, out) if a != b)
assert widened == 6 + 5 + 2, widened

root = ls[0:7] + [
    "",
    '#[path = "🧫️committed-vector/🦀️.rs"]',
    "mod committed_vector;",
    '#[path = "⚖️vector-laws/🦀️.rs"]',
    "mod vector_laws;",
    "",
    "pub use committed_vector::Vector;",
    "pub use vector_laws::{assert_case, assert_inverse_sum};",
]

vector_child = [
    "//! 🧫️ One committed quintet of a mutation vector: its raw texts read from the fixture mirror, and the typed views the laws start from.",
    "",
    "use semio_framework_os_kernel as protocol;",
    "use semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};",
    ls[10].replace("RemodelingSnapshot", "RemodelingSnapshot"),
    ls[11],
    "use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;",
    "use std::path::{Path, PathBuf};",
    "",
] + out
laws_child = [
    "//! ⚖️ The law set a committed vector is held to: applied, no-op and rejected outcomes, the inverse and the canonical form.",
    "",
    "use super::committed_vector::{json_of, refusal_level, Vector};",
    "use semio_framework_os_kernel as protocol;",
    "use semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};",
    ls[10],
    ls[11],
    "use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation};",
    "",
] + seg(100, 207)
assert ls[12].startswith("use semio_s_artifact_remodel_remodeling_mutations")
files = {
    T["file"]: "\n".join(root) + "\n",
    base + "/🧫️committed-vector/🦀️.rs": "\n".join(vector_child) + "\n",
    base + "/⚖️vector-laws/🦀️.rs": "\n".join(laws_child) + "\n",
}
sd = stage_dir(name)
if os.path.exists(sd):
    shutil.rmtree(sd)
for rel, txt in files.items():
    write(f"{sd}/{rel}", txt)
new = []
for txt in files.values():
    new.extend(lines_of(txt))
gone, added = diff_multiset(ls, new)
print("files:", {k.split("/")[-2]: len(lines_of(v)) for k, v in files.items()})
print("REMOVED", sum(gone.values()), dict(gone))
print("ADDED", sum(added.values()), dict(added))
