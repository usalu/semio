#!/usr/bin/env python3
"""🧰️ sf3 target 2, step 1: the four generation3d geometry test supports become crates that depend on nothing above their consumers.

The catalogue (io-text) and the registry (geometry) are no longer imported by the supports; the consumers pass them in. The peer-sweep damage (`semio_s_artifact_procedural_3d_<consumer>::tests::...::oracle::`) is restored to the module alias.

usage: g5a_support_files.py [--write]
"""
import os, re, sys

REPO = "/Users/ueli/Documents/semio"
WRITE = "--write" in sys.argv
G = f"{REPO}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry"
O = f"{G}/🧪️tests/🧰️oracle-support/🦀️.rs"
C = f"{G}/🧪️tests/🧰️construction-support/🦀️.rs"
P = f"{G}/⏱️phased-job/🧪️tests/🧰️support/🦀️.rs"
H = f"{G}/🧱️mesh-support/🧪️tests/🧰️mesh-harness/🦀️.rs"
OID, CID, PID, HID = ("semio_s_artifact_procedural_3d_geometry_" + x for x in ("oracle_support", "construction_support", "phased_job_support", "mesh_harness"))


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, t):
    print(("write " if WRITE else "(dry) ") + p.replace(REPO + "/", "")[-110:], len(t.split("\n")), "lines")
    if WRITE:
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write(t)


def sub1(t, a, b, n=1):
    assert t.count(a) == n, (t.count(a), n, a[:100])
    return t.replace(a, b)


def glue(domain_rel_doc):
    return f"//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️.rs.\n\n#[path = \"../../🦀️.rs\"]\nmod component;\npub use component::*;\n"


o = rd(O)
o = sub1(o, "use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;\nuse semio_s_artifact_procedural_3d_model::catalogue::Kind;", "use semio_s_artifact_procedural_3d_model::catalogue::Catalogue;\nuse semio_s_artifact_procedural_3d_model::catalogue::Kind;")
o = sub1(o, "pub fn kind_of(id: &str) -> &'static Kind {\n    catalogue().kind(id)", "pub fn kind_of(catalogue: &'static Catalogue, id: &str) -> &'static Kind {\n    catalogue.kind(id)")
o = sub1(o, "pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {", "pub fn run_fixture(catalogue: &'static Catalogue, text: &str, entries: &[ComputeEntry]) -> usize {")
o = sub1(o, "        let kind = kind_of(case[\"kind\"].as_str().expect(\"case kind\"));\n        let case_tolerance", "        let kind = kind_of(catalogue, case[\"kind\"].as_str().expect(\"case kind\"));\n        let case_tolerance")
o = sub1(o, "pub fn assert_category(category: &str, entries: &[ComputeEntry], text: &str) {", "pub fn assert_category(catalogue: &Catalogue, category: &str, entries: &[ComputeEntry], text: &str) {")
o = sub1(o, "catalogue().category(category)", "catalogue.category(category)")
assert "catalogue()" not in o.replace("catalogue.category", "")
wr(O, o)

DAMAGE_C = "semio_s_artifact_procedural_3d_brep_surface::tests::support::oracle::"
c = rd(C)
c = sub1(c, "#[path = \"../🧰️oracle-support/🦀️.rs\"]\npub mod oracle;", f"pub use {OID} as oracle;")
n = c.count(DAMAGE_C)
assert n == 13, n
c = c.replace(DAMAGE_C, "oracle::")
c = sub1(c, "use semio_s_artifact_procedural_3d_geometry::registry;\n", "use semio_s_artifact_procedural_3d_geometry_compute::StartFn;\nuse semio_s_artifact_procedural_3d_model::catalogue::Catalogue;\n")
c = sub1(c, "//#region 🔖️Inputs\n", "//#region 🔖️Inputs\n/// 🗃️ The registered compute start of a kind id, supplied by the caller because the registry sits above the constructions.\npub type Lookup = fn(&str) -> Option<StartFn>;\n\n")
c = sub1(c, "pub fn shape_of(recipe: &Value) -> Arc<ShapeValue> {", "pub fn shape_of(catalogue: &'static Catalogue, lookup: Lookup, recipe: &Value) -> Arc<ShapeValue> {")
c = sub1(c, "    let kind = kind_of(recipe[\"kind\"].as_str().expect(\"widget kind\"));\n    let start = registry::lookup(&kind.id).unwrap_or_else(|| panic!(\"{} has a compute\", kind.id));\n    let evaluation = drive(start(kind, inputs_of(kind, recipe)), usize::MAX);", "    let kind = kind_of(catalogue, recipe[\"kind\"].as_str().expect(\"widget kind\"));\n    let start = lookup(&kind.id).unwrap_or_else(|| panic!(\"{} has a compute\", kind.id));\n    let evaluation = drive(start(kind, inputs_of(catalogue, lookup, kind, recipe)), usize::MAX);")
c = sub1(c, "pub fn inputs_of(kind: &Kind, case: &Value) -> WidgetInputs {", "pub fn inputs_of(catalogue: &'static Catalogue, lookup: Lookup, kind: &Kind, case: &Value) -> WidgetInputs {")
c = sub1(c, "GeometryValue::Shape(shape_of(item))", "GeometryValue::Shape(shape_of(catalogue, lookup, item))")
c = sub1(c, "PortType::Shape => GeometryValue::Shape(shape_of(value)),", "PortType::Shape => GeometryValue::Shape(shape_of(catalogue, lookup, value)),")
for name in ("run_fixture", "assert_deterministic", "assert_cancellable"):
    c = sub1(c, f"pub fn {name}(text: &str, entries: &[ComputeEntry]) -> usize {{", f"pub fn {name}(catalogue: &'static Catalogue, lookup: Lookup, text: &str, entries: &[ComputeEntry]) -> usize {{")
c = sub1(c, "pub fn first_shape(text: &str, entries: &[ComputeEntry], name: &str) -> Arc<ShapeValue> {", "pub fn first_shape(catalogue: &'static Catalogue, lookup: Lookup, text: &str, entries: &[ComputeEntry], name: &str) -> Arc<ShapeValue> {")
k = c.count("kind_of(case[\"kind\"].as_str().expect(\"case kind\"))")
assert k == 4, k
c = c.replace("kind_of(case[\"kind\"].as_str().expect(\"case kind\"))", "kind_of(catalogue, case[\"kind\"].as_str().expect(\"case kind\"))")
ki = c.count("inputs_of(kind, case)")
assert ki == 6, ki
c = c.replace("inputs_of(kind, case)", "inputs_of(catalogue, lookup, kind, case)")
assert "registry::" not in c.replace("//", "")
wr(C, c)

DAMAGE_P = "semio_s_artifact_procedural_3d_brep_feature::tests::support::base::"
p = rd(P)
p = sub1(p, "#[path = \"../../../🧪️tests/🧰️oracle-support/🦀️.rs\"]\npub mod base;", f"pub use {OID} as base;")
n = p.count(DAMAGE_P)
assert n == 8, n
p = p.replace(DAMAGE_P, "base::")
p = sub1(p, "use semio_s_artifact_procedural_3d_model::catalogue::Kind;\n", "use semio_s_artifact_procedural_3d_model::catalogue::Catalogue;\nuse semio_s_artifact_procedural_3d_model::catalogue::Kind;\n")
p = sub1(p, "pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {", "pub fn run_fixture(catalogue: &'static Catalogue, text: &str, entries: &[ComputeEntry]) -> usize {")
k = p.count("kind_of(case[\"kind\"].as_str().expect(\"case kind\"))")
assert k == 2, k
p = p.replace("kind_of(case[\"kind\"].as_str().expect(\"case kind\"))", "kind_of(catalogue, case[\"kind\"].as_str().expect(\"case kind\"))")
p = sub1(p, "pub fn assert_category(category: &str, entries: &[ComputeEntry], text: &str) {\n    base::assert_category(category, entries, text);", "pub fn assert_category(catalogue: &Catalogue, category: &str, entries: &[ComputeEntry], text: &str) {\n    base::assert_category(catalogue, category, entries, text);")
p = sub1(p, "pub fn refresh(path: &str, text: &str, entries: &[ComputeEntry]) {", "pub fn refresh(catalogue: &'static Catalogue, path: &str, text: &str, entries: &[ComputeEntry]) {")
wr(P, p)

DAMAGE_H = "semio_s_artifact_procedural_3d_mesh_primitive::tests::h::oracle::"
h = rd(H)
h = sub1(h, "#[path = \"../../../🧪️tests/🧰️oracle-support/🦀️.rs\"]\npub mod oracle;", f"pub use {OID} as oracle;")
n = h.count(DAMAGE_H)
assert n >= 10, n
h = h.replace(DAMAGE_H, "oracle::")
h = sub1(h, "pub fn run_fixture(text: &str, entries: &[ComputeEntry]) -> usize {", "pub fn run_fixture(catalogue: &'static Catalogue, text: &str, entries: &[ComputeEntry]) -> usize {")
h = sub1(h, "oracle::kind_of(case[\"kind\"].as_str().expect(\"case kind\"))", "oracle::kind_of(catalogue, case[\"kind\"].as_str().expect(\"case kind\"))")
h = sub1(h, "use semio_s_artifact_procedural_3d_model::catalogue::Kind;\n", "use semio_s_artifact_procedural_3d_model::catalogue::Catalogue;\nuse semio_s_artifact_procedural_3d_model::catalogue::Kind;\n")
wr(H, h)
print("assert_category in harness:", "assert_category" in h, "| kind_of defs:", len(re.findall(r"pub fn kind_of", h)))
for d, t in (("🧪️tests/🧰️oracle-support", None), ("🧪️tests/🧰️construction-support", None), ("⏱️phased-job/🧪️tests/🧰️support", None), ("🧱️mesh-support/🧪️tests/🧰️mesh-harness", None)):
    gp = f"{G}/{d}/📦️packages/🦀️rust/🦀️.rs"
    wr(gp, glue(None))
