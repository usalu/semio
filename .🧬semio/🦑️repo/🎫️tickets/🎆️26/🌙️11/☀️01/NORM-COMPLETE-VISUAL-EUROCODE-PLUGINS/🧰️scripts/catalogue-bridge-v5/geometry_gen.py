#!/usr/bin/env python3
"""Generates the Rust geometry tables of the design-case catalogues (BRIDGE v5) from the hand-maintained JSON sources.

The sole source of the declarations is `📐️geometry/🔣️.json` next to each catalogue `cases` module; there is no classification here. This script only validates a source against the registry and the schema of the forms and writes `📐️geometry/🦀️.rs`:
  - one row per registry entry, no other ids;
  - the form is a `CaseForm` variant, every `aux` role belongs to `forms::aux_roles(form)` or is an annotation role `dim_x|dim_xe|dim_y|dim_ye|dim_z|dim_ze|dim_angle` with an optional digit (length inputs; `dim_angle` an angle input);
  - every key is a declared input; dimension and length roles are length inputs (series for `layers`), count/angle/ratio roles are numbers, the load is a number input.
Usage: geometry_gen.py [--check]
"""
import json, os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
KIT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
CASES = {
    "⚖️en1990": ("En1990", f"{SUB}/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"),
    "🏋️en1991": ("En1991", f"{SUB}/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"),
    "🏛️en1992": ("En1992", f"{SUB}/🧬️schema/💡️inferences/⚖️checks/🗄️registry/🗂️cases"),
    "🪵️en1995": ("En1995", f"{SUB}/🧬️schema/⚖️kernel/📇️catalogue/🗂️cases"),
    "🪨️en1996": ("En1996", f"{SUB}/🧬️schema/📇️masonry-catalogue/🗂️cases"),
    "🌍️en1997": ("En1997", f"{SUB}/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"),
}
LEN = ("Metre", "Millimetre")
DIM = re.compile(r"^dim_(x|xe|y|ye|z|ze|angle)\d*$")
ANGLE = ("Degree", "Radian")
SCALAR = ("count", "rows", "columns", "angle", "ratio", "taper", "apex_angle", "ply_dir")
FORMS_RS = f"{KIT}/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs"
CASE_RS = f"{KIT}/⚖️compliance/🧰️design-case/🦀️.rs"


def extract_entries(code):
    out = {}
    for dp, _, files in os.walk(f"{ROOT}/{code}"):
        if "🧪️tests" in dp or "/tests" in dp:
            continue
        for f in files:
            if not f.endswith(".rs"):
                continue
            s = open(os.path.join(dp, f)).read()
            starts = [(m.start(), m.group(1)) for m in re.finditer(r'(?:\bEntry\s*\{\s*id:\s*|\b\w*entry\w*\(\s*)"([A-Z][A-Za-z0-9.\-]*)"', s)]
            for i, (st, eid) in enumerate(starts):
                seg = s[st:(starts[i + 1][0] if i + 1 < len(starts) else st + 2500)][:2500]
                strs = re.findall(r'"([^"]*)"', seg.split("&[")[0])
                im = re.search(r'inputs:\s*&\[([^\]]*)\]', seg) or re.search(r'&\[([^\]]*)\]', seg)
                if "id:" in seg[:20]:
                    en = re.search(r'\ben:\s*"([^"]*)"', seg)
                    title = en.group(1) if en else ""
                else:
                    title = (strs[2:5] + ["", "", ""])[1]
                out.setdefault(eid, (title, re.findall(r'"([^"]+)"', im.group(1)) if im else []))
    return out


def extract_vocabulary(code):
    rows = []
    for dp, _, files in os.walk(f"{ROOT}/{code}"):
        if "inputs" not in dp or "tests" in dp:
            continue
        for f in files:
            if f.endswith(".rs"):
                s = open(os.path.join(dp, f)).read()
                for m in re.finditer(r'InputSpec::(number|vector|series)\("([^"]+)",\s*"(?:[^"\\]|\\.)*",\s*"(?:[^"\\]|\\.)*",\s*UnitId::(\w+)\)([^\n]*)', s):
                    sc = re.search(r'\.scope\("([^"]*)"\)', m.group(4))
                    ex = re.search(r"\.example\(([-0-9.e+]+)\)", m.group(4))
                    rows.append(dict(kind=m.group(1), key=m.group(2), unit=m.group(3), scope=sc.group(1).split(",") if sc else [], ex=float(ex.group(1)) if ex else None))
    return rows


def schema():
    forms_src = open(FORMS_RS).read()
    roles = {}
    block = forms_src[forms_src.index("pub const fn aux_roles"):forms_src.index("pub const fn aux_is_scalar")]
    for m in re.finditer(r"CaseForm::(\w+) => &\[([^\]]*)\]", block):
        roles[m.group(1)] = re.findall(r'"(\w+)"', m.group(2))
    case_src = open(CASE_RS).read()
    enum = case_src[case_src.index("pub enum CaseForm {"):]
    variants = re.findall(r"^\s{4}(\w+),", enum[:enum.index("\n}")], re.M)
    return variants, roles


def validate(code, rows, entries, vocab, variants, roles):
    errors = []

    def spec(eid, key):
        found = [r for r in vocab if r["key"] == key and (not r["scope"] or eid in r["scope"])]
        scoped = [r for r in found if eid in r["scope"]]
        return scoped[0] if scoped else (found[0] if found else None)

    for eid in entries:
        if eid not in rows:
            errors.append(f"{eid}: no geometry row")
    for eid, row in rows.items():
        if eid not in entries:
            errors.append(f"{eid}: not a registry entry")
            continue
        inputs = entries[eid][1]
        form = row["form"]
        if form not in variants:
            errors.append(f"{eid}: unknown form {form}")
        for role in row["aux"]:
            if role not in roles.get(form, []) and not DIM.match(role):
                errors.append(f"{eid}: role {role} is not an aux role of {form}")
        keyed = [(role, row[role], "len") for role in ("width", "length", "height")] + [("load", row["load"], "num")] + [(role, key, "ang" if role.startswith("dim_angle") else "num" if role in SCALAR else "len") for role, key in row["aux"].items()]
        for role, key, kind in keyed:
            if not key:
                continue
            if key not in inputs:
                errors.append(f"{eid}: {role} key {key} is not a declared input")
                continue
            found = spec(eid, key)
            if found is None or found["kind"] == "text":
                errors.append(f"{eid}: {role} key {key} is no number input")
            elif kind == "ang" and found["unit"] not in ANGLE:
                errors.append(f"{eid}: {role} key {key} has unit {found['unit']}, not an angle")
            elif kind == "len" and found["unit"] not in LEN:
                errors.append(f"{eid}: {role} key {key} has unit {found['unit']}, not a length")
            elif role == "layers" and found["kind"] != "series":
                errors.append(f"{eid}: layers key {key} is no series input")
    return errors


def rust(name, rows):
    lines = [
        f"//! 📐️ Geometry declarations of the {name[:2].upper()}{name[2:]} design cases: per catalogue entry the form it draws in 3D, the inputs that size it and drive its load bar and the auxiliary roles of its form; generated from the hand-maintained `./🔣️.json` by `🧰️scripts/catalogue-bridge-v5/geometry_gen.py`. See [`super`].",
        "",
        "use semio_s_plugin_norm_compliance::design_case::{self, AuxRow, CaseForm, CaseGeometry, GeometryRow};",
        "",
        "const fn row(entry: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, load: &'static str, aux: &'static [AuxRow]) -> GeometryRow {",
        "    GeometryRow { entry, geometry: CaseGeometry { form, width, length, height, load, aux } }",
        "}",
        "",
        "/// 📐️ The declaration of every catalogue entry, in registry order.",
        "pub static GEOMETRY: &[GeometryRow] = &[",
    ]
    for eid, r in rows.items():
        aux = ", ".join(f'("{role}", "{key}")' for role, key in r["aux"].items())
        lines.append(f'    row("{eid}", CaseForm::{r["form"]}, "{r["width"]}", "{r["length"]}", "{r["height"]}", "{r["load"]}", &[{aux}]),')
    lines += ["];", "", "/// 📐️ The geometry declared for the catalogue entry `entry_id`.", "pub fn geometry_of(entry_id: &str) -> Option<CaseGeometry> {", "    design_case::geometry_in(GEOMETRY, entry_id)", "}", ""]
    return "\n".join(lines)


def main():
    check = "--check" in sys.argv
    variants, roles = schema()
    failed = False
    for code, (name, rel) in CASES.items():
        directory = f"{ROOT}/{code}/{rel}/📐️geometry"
        source = json.load(open(f"{directory}/🔣️.json"))
        rows = source["entries"]
        entries, vocab = extract_entries(code), extract_vocabulary(code)
        errors = validate(code, rows, entries, vocab, variants, roles)
        ordered = {eid: rows[eid] for eid in entries if eid in rows}
        for error in errors:
            print(code, error)
        failed = failed or bool(errors)
        text = rust(name, ordered)
        if check:
            failed = failed or open(f"{directory}/🦀️.rs").read() != text
        elif not errors:
            open(f"{directory}/🦀️.rs", "w").write(text)
        print(code, len(ordered), "rows", "errors" if errors else "ok")
    sys.exit(1 if failed else 0)


main()
