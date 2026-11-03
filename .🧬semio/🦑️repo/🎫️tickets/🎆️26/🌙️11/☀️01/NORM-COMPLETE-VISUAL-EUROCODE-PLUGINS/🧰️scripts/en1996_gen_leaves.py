"""Generates the EN 1996 wave 2 mutation leaves (rust, diff, inverse, descriptor, payload schema, ts twin), their group crates and every aggregate
that lists them. Idempotent. Run: /home/deck/semio/.venv/bin/python -I en1996_gen_leaves.py"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))

MODEL = "semio_s_artifact_norm_en1996_model"
DIFFC = "semio_s_artifact_norm_en1996_diff"
DERIVE = "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[mutation_leaf(contract = ::protocol)]\n#[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n#[value(rename_all = \"camelCase\")]\n"


def rust_ty(f):
    return "f64" if f["kind"] == "f64" else f"{MODEL}::{f['kind']}"


def ts_ty(f):
    return "number" if f["kind"] == "f64" else f"En1996{f['kind']}"


def label(leaf):
    op = leaf["op"]
    if op == "insert":
        r = leaf["row"]
        return (f"Insert {r['noun_en'].lower()}", f"{r['noun_de']} einfügen")
    if op == "remove":
        r = leaf["row"]
        return (f"Remove {r['noun_en'].lower()}", f"{r['noun_de']} entfernen")
    f = leaf["field"]
    if op == "field":
        r = leaf["row"]
        return (f"Change {r['noun_en'].lower()} {f['en'].lower()}", f"{f['de']} ({r['noun_de']}) ändern")
    return (f"Change {f['noun_en']}", f"{f['noun_de']} ändern")


def payload_fields(leaf):
    op = leaf["op"]
    if op in ("insert", "remove"):
        out = [("wall_index", "usize", "Wall", "Wand", "Zero-based position of the wall in its list.", "Nullbasierte Position der Wand in der Liste."), ("index", "usize", "Position" if op == "insert" else leaf["row"]["noun_en"], "Position" if op == "insert" else leaf["row"]["noun_de"], "Zero-based list position at which the record is inserted." if op == "insert" else "Zero-based position of the record in its list.", "Nullbasierte Listenposition, an der der Datensatz eingefügt wird." if op == "insert" else "Nullbasierte Position des Datensatzes in der Liste.")]
        if op == "insert":
            out.append(("row", f"{MODEL}::{leaf['row']['struct']}", leaf["row"]["noun_en"], leaf["row"]["noun_de"], "The complete record to insert into the wall.", "Der vollständige in die Wand einzufügende Datensatz."))
        return out
    wall = ("wall_index", "usize", "Wall", "Wand", "Zero-based position of the wall in its list.", "Nullbasierte Position der Wand in der Liste.")
    new = (f"new_{leaf['field']['name']}", rust_ty(leaf["field"]), None, None, None, None)
    if op == "scalar":
        return [("index", "usize", "Wall", "Wand", "Zero-based position of the wall in its list.", "Nullbasierte Position der Wand in der Liste."), new]
    if op == "field":
        r = leaf["row"]
        return [wall, ("index", "usize", r["noun_en"], r["noun_de"], f"Zero-based position of the {r['noun_en'].lower()} within the wall in its list.", f"Nullbasierte Position in der Wand in der Liste."), new]
    if op == "opening":
        return [wall, ("index", "usize", "Opening", "Öffnung", "Zero-based position of the opening within the wall in its list.", "Nullbasierte Position der Öffnung in der Wand in der Liste."), new]
    return [wall, ("load_case_index", "usize", "Load case", "Lastfall", "Zero-based position of the load case within the wall in its list.", "Nullbasierte Position des Lastfalls in der Wand in der Liste."), ("index", "usize", "Concentrated load", "Einzellast", "Zero-based position of the concentrated load in its list.", "Nullbasierte Position der Einzellast in der Liste."), new]


def leaf_rs(leaf):
    P, S, kind = pascal(leaf["kind"]), snake(leaf["kind"]), leaf["kind"]
    op = leaf["op"]
    en, de = label(leaf)
    fields = "".join(f"    pub {n}: {t},\n" for n, t, *_ in payload_fields(leaf))
    ent = kind.split("-", 1)[1]
    verb = kind.split("-", 1)[0]
    rec = {"change": "Changed", "insert": "Inserted", "remove": "Removed"}[verb] + pascal(ent)
    if op == "insert":
        other = f"remove_{snake(leaf['row']['key'])}"
        other_t = f"Remove{leaf['row']['struct'] and pascal(leaf['row']['key'])}"
    elif op == "remove":
        other = f"insert_{snake(leaf['row']['key'])}"
        other_t = f"Insert{pascal(leaf['row']['key'])}"
    else:
        other, other_t = S, P
    return f"""//! {leaf['emoji']}{kind}
use {MODEL}::En1996Snapshot;

{DERIVE}pub struct {P} {{
{fields}}}

impl<Op> protocol::MutationKind<En1996Snapshot, Op> for {P} where Op: protocol::Mutation<En1996Snapshot, Diff = {DIFFC}::En1996Diff> + From<crate::{other}::{other_t}> {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{
        verb: "{verb}",
        entity: "{ent}",
        kind: "{kind}",
        record: "{rec}",
    }};

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {{
        semio_framework_ui_locale::LocalizedLabel::native("{en}", "{de}")
    }}
}}

impl<Op> protocol::ColdMutationKind<En1996Snapshot, Op> for {P} where Op: protocol::ColdMutation<En1996Snapshot, ReferenceDiff = {DIFFC}::En1996Diff> + From<crate::{other}::{other_t}> {{
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1996Snapshot>>::ReferenceDiff> {{
        crate::{S}::diff::diff(self, base)
    }}
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
        Ok({{
            crate::{S}::inverse::inverse(self, base)?
        }})
    }}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
"""


def guard(leaf):
    op = leaf["op"]
    if op == "scalar":
        return "payload.index >= base.walls.len()", "Invalid wall index.", "base.walls.len()"
    if op in ("field", "opening"):
        coll = leaf["row"]["coll"] if op == "field" else "openings"
        return f"payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].{coll}.len()", f"Invalid {(leaf['row']['key'] if op == 'field' else 'opening')} index.", None
    if op == "conc":
        return "payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len()", "Invalid concentrated index.", None
    return None, None, None


def leaf_diff(leaf):
    P, S = pascal(leaf["kind"]), snake(leaf["kind"])
    op = leaf["op"]
    head = f"use crate::{S}::{P};\nuse {DIFFC}::En1996Diff;\nuse {MODEL}::En1996Snapshot;\n"
    if op in ("insert", "remove"):
        r = leaf["row"]
        pre, coll = r["pre"], r["coll"]
        head += f"use {DIFFC}::{{En1996{pre}Delta, En1996WallDelta, En1996WallPatch}};\n"
        if op == "insert":
            return head + f"""pub fn diff(payload: &{P}, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {{
    if payload.wall_index >= base.walls.len() || payload.index > base.walls[payload.wall_index].{coll}.len() {{
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid {r['key']} insert."), Vec::<String>::new());
    }}
    let wall = &base.walls[payload.wall_index];
    if wall.{coll}.iter().any(|existing| existing.id == payload.row.id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {{}} already exists.", payload.row.id), [payload.row.id.clone()]);
    }}
    protocol::MutationOutcome::new(En1996Diff {{ walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {{ {coll}: En1996{pre}Delta::insertion(payload.index, payload.row.clone()), ..Default::default() }}), ..Default::default() }})
}}
"""
        return head + f"""pub fn diff(payload: &{P}, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {{
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].{coll}.len() {{
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid {r['key']} remove."), Vec::<String>::new());
    }}
    let wall = &base.walls[payload.wall_index];
    protocol::MutationOutcome::new(En1996Diff {{ walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {{ {coll}: En1996{pre}Delta::removal(&wall.{coll}, payload.index), ..Default::default() }}), ..Default::default() }})
}}
"""
    f = leaf["field"]
    n = f["name"]
    cond, msg, _ = guard(leaf)
    if op == "scalar":
        head += f"use {DIFFC}::{{En1996WallDelta, En1996WallPatch}};\n"
        return head + f"""pub fn diff(payload: &{P}, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {{
    if {cond} {{
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("{msg}"), Vec::<String>::new());
    }}
    let wall = &base.walls[payload.index];
    protocol::MutationOutcome::new(En1996Diff {{ walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {{ {n}: Some(payload.new_{n}), ..Default::default() }}), ..Default::default() }})
}}
"""
    if op in ("field", "opening"):
        pre, coll = (leaf["row"]["pre"], leaf["row"]["coll"]) if op == "field" else ("Opening", "openings")
        head += f"use {DIFFC}::{{En1996{pre}Delta, En1996{pre}Patch, En1996WallDelta, En1996WallPatch}};\n"
        return head + f"""pub fn diff(payload: &{P}, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {{
    if {cond} {{
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("{msg}"), Vec::<String>::new());
    }}
    let wall = &base.walls[payload.wall_index];
    let row = &wall.{coll}[payload.index];
    protocol::MutationOutcome::new(En1996Diff {{ walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {{ {coll}: En1996{pre}Delta::modification(&row.id, En1996{pre}Patch {{ {n}: Some(payload.new_{n}), ..Default::default() }}), ..Default::default() }}), ..Default::default() }})
}}
"""
    head += f"use {DIFFC}::{{En1996ConcentratedDelta, En1996ConcentratedPatch, En1996LoadCaseDelta, En1996LoadCasePatch, En1996WallDelta, En1996WallPatch}};\n"
    return head + f"""pub fn diff(payload: &{P}, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {{
    if {cond} {{
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("{msg}"), Vec::<String>::new());
    }}
    let wall = &base.walls[payload.wall_index];
    let load_case = &wall.load_cases[payload.load_case_index];
    let concentrated = &load_case.concentrated[payload.index];
    protocol::MutationOutcome::new(En1996Diff {{
        walls: En1996WallDelta::modification(&wall.id, En1996WallPatch {{
            load_cases: En1996LoadCaseDelta::modification(&load_case.id, En1996LoadCasePatch {{ concentrated: En1996ConcentratedDelta::modification(&concentrated.id, En1996ConcentratedPatch {{ {n}: Some(payload.new_{n}), ..Default::default() }}), ..Default::default() }}),
            ..Default::default()
        }}),
        ..Default::default()
    }})
}}
"""


def leaf_inverse(leaf):
    P, S = pascal(leaf["kind"]), snake(leaf["kind"])
    op = leaf["op"]
    if op in ("insert", "remove"):
        r = leaf["row"]
        coll = r["coll"]
        if op == "insert":
            o, OT = f"remove_{snake(r['key'])}", f"Remove{pascal(r['key'])}"
            return f"""use crate::{S}::{P};
use {MODEL}::En1996Snapshot;
use crate::{o};
pub fn inverse<Op>(payload: &{P}, _base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{o}::{OT}> {{
    Ok(vec![Op::from({o}::{OT} {{ wall_index: payload.wall_index, index: payload.index }})])
}}
"""
        o, OT = f"insert_{snake(r['key'])}", f"Insert{pascal(r['key'])}"
        return f"""use crate::{S}::{P};
use {MODEL}::En1996Snapshot;
use crate::{o};
pub fn inverse<Op>(payload: &{P}, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{o}::{OT}> {{
    Ok(if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].{coll}.len() {{ Vec::new() }} else {{ vec![Op::from({o}::{OT} {{ wall_index: payload.wall_index, index: payload.index, row: base.walls[payload.wall_index].{coll}[payload.index].clone() }})] }})
}}
"""
    f = leaf["field"]
    n = f["name"]
    cond, _, _ = guard(leaf)
    if op == "scalar":
        path, ctor = "base.walls[payload.index]", "index: payload.index"
    elif op in ("field", "opening"):
        coll = leaf["row"]["coll"] if op == "field" else "openings"
        path, ctor = f"base.walls[payload.wall_index].{coll}[payload.index]", "wall_index: payload.wall_index, index: payload.index"
    else:
        path, ctor = "base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index]", "wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index"
    return f"""use crate::{S}::{P};
use {MODEL}::En1996Snapshot;
pub fn inverse<Op>(payload: &{P}, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{S}::{P}> {{
    Ok(if {cond} {{ Vec::new() }} else {{ vec![Op::from({P} {{ {ctor}, new_{n}: {path}.{n} }})] }})
}}
"""


def ui_index(name, en, de, den, dde, order):
    return {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "step": 1, "precision": 0, "group": "target", "order": order}}


def value_schema(f, group, order):
    if f["kind"] != "f64":
        en = ENUMS[f["kind"]]
        return {"$ref": f"{SNAPSHOT_REF}#/$defs/{f['kind']}", "x-semio-ui": {"role": "value", "label": {"en": f["en"], "de": f["de"]}, "description": {"en": " | ".join(en["variants"]), "de": " | ".join(en["variants"])}, "group": group, "order": order}}
    ui = {"widget": "stepper", "role": "value", "label": {"en": f["en"], "de": f["de"]}, "unit": f["unit"], "step": f["step"], "precision": f["prec"], "group": group, "order": order}
    if f.get("disp"):
        ui["displayUnit"] = f["disp"]
        ui["displayFactor"] = f["factor"]
    out = {"type": "number"}
    if f["min"] is not None:
        out["minimum"] = f["min"]
    if f["max"] is not None:
        out["maximum"] = f["max"]
    out["x-semio-ui"] = ui
    return out


def leaf_schema(leaf):
    P = pascal(leaf["kind"])
    pf = payload_fields(leaf)
    props, req = {}, []
    order = 10
    for name, t, en, de, den, dde in pf:
        c = camel(name)
        req.append(c)
        if name == "row":
            props[c] = {"$ref": f"{SNAPSHOT_REF}#/$defs/{leaf['row']['struct']}", "x-semio-ui": {"role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "group": "record", "order": order}}
        elif t == "usize":
            props[c] = ui_index(name, en, de, den, dde, order)
        else:
            fld = leaf["field"]
            grp = leaf["row"]["coll"] if leaf["op"] == "field" else ("openings" if leaf["op"] == "opening" else ("concentrated" if leaf["op"] == "conc" else "site"))
            props[c] = value_schema(fld, grp, order)
        order += 10
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{SCHEMA_ID}/mutation/{leaf['kind']}/schema.json", "title": P, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def leaf_ts(leaf):
    P = pascal(leaf["kind"])
    pf = payload_fields(leaf)
    imports = {"normWireInteger", "normWireObject", "normWireRange", "NormWireReader", "normWireRequired"}
    fields, parsers = [], []
    snap_imports = []
    for name, t, *_ in pf:
        c = camel(name)
        if name == "row":
            st = leaf["row"]["struct"]
            fields.append(f"  {c}: {st};")
            parsers.append(f"{c}: normWireRequired(parse{st})")
            snap_imports.append(st)
        elif t == "usize":
            fields.append(f"  {c}: number;")
            parsers.append(f'{c}: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}}))')
        else:
            fld = leaf["field"]
            if fld["kind"] == "f64":
                imports.add("normWireNumber")
                fields.append(f"  {c}: number;")
                rng = '{"minimum":%s' % fld["min"] if fld["min"] is not None else None
                if rng is not None:
                    if fld["max"] is not None:
                        rng += ',"maximum":%s' % fld["max"]
                    rng += "}"
                    parsers.append(f"{c}: normWireRequired(normWireRange(normWireNumber, {rng}))")
                else:
                    parsers.append(f"{c}: normWireRequired(normWireNumber)")
            else:
                fields.append(f"  {c}: En1996{fld['kind']};")
                parsers.append(f"{c}: normWireRequired(parseEn1996{fld['kind']})")
                snap_imports.append(f"En1996{fld['kind']}")
    imp = ", ".join(sorted(imports, key=lambda s: s.lower()))
    extra = ""
    if snap_imports:
        names = sorted(set(snap_imports))
        extra = "import { " + ", ".join([f"type {n}" for n in names] + [f"parse{n}" for n in names]) + ' } from "../../../📸️snapshot/🟦️.ts";\n'
    return f"""/** {leaf['emoji']} `{leaf['kind']}` wire twin: the leaf payload `{P}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ {imp.replace('NormWireReader', 'type NormWireReader')} }} from "@semio-tech/norm-js/registry/contract";
{extra}
export interface {P} {{
{chr(10).join(fields)}
}}

export const parse{P}: NormWireReader<{P}> = normWireObject<{P}>({{ {", ".join(parsers)} }});
"""


def descriptor(leaf, tag):
    P = pascal(leaf["kind"])
    return {"schemaVersion": 1, "owner": f"{OWNER_REL}/{dirname(leaf)}", "semanticKind": leaf["kind"], "displayName": " ".join(p.capitalize() for p in leaf["kind"].split("-")), "emoji": leaf["emoji"], "aggregateVariant": P, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}


def crate_rs(c, ls):
    mounts = "".join(f'#[path = "../{dirname(l)}/🦀️.rs"]\npub mod {snake(l["kind"])};\n' for l in ls)
    return f"""//! {CRATE_EMOJI[c]} EN 1996 {CRATES[c]} mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
{mounts}//#endregion 🪢️Mounts
"""


def crate_toml(c):
    return rd(f"{MUT}/🪟️openings/📦️packages/🦀️rust/Cargo.toml").replace("mut-openings", f"mut-{c}").replace("EN 1996 openings mutation leaves", f"EN 1996 {CRATES[c]} mutation leaves")


def insert_after_last(text, pattern, add):
    ms = list(re.finditer(pattern, text, re.M))
    if not ms:
        raise SystemExit(f"anchor not found: {pattern}")
    j = ms[-1].end()
    return text[:j] + add + text[j:]


def main():
    ls = leaves()
    proto_path = f"{IO}/💾️binary/🧬️mutations/📡️.protocol.semio"
    ptxt = rd(proto_path)
    next_tag = max(int(x) for x in re.findall(r"tag=(\d+)", ptxt)) + 1
    by_crate = {}
    new_leaves = []
    for l in ls:
        by_crate.setdefault(l["crate"], []).append(l)
        if os.path.isdir(f"{MUT}/{dirname(l)}"):
            continue
        new_leaves.append(l)
    tags = {}
    for l in new_leaves:
        if f"record {l['kind']} " in ptxt:
            continue
        tags[l["kind"]] = next_tag
        next_tag += 1
    for l in new_leaves:
        d = f"{MUT}/{dirname(l)}"
        wr(f"{d}/🦀️.rs", leaf_rs(l))
        wr(f"{d}/🔺️diff/🦀️.rs", leaf_diff(l))
        wr(f"{d}/↩️inverse/🦀️.rs", leaf_inverse(l))
        jdump(f"{d}/🔣️.json", descriptor(l, tags.get(l["kind"], 0)))
        jdump(f"{d}/🧬️schema/🔣️.json", leaf_schema(l))
        wr(f"{d}/🧬️schema/🟦️.ts", leaf_ts(l))
    for c, group in by_crate.items():
        cd = f"{MUT}/{crate_dir(c)}"
        wr(f"{cd}/🦀️.rs", crate_rs(c, group))
        if not os.path.exists(f"{cd}/📦️packages/🦀️rust/Cargo.toml"):
            wr(f"{cd}/📦️packages/🦀️rust/Cargo.toml", crate_toml(c))
    if not new_leaves:
        print("leaves already present")
        return
    names = [l for l in new_leaves]
    mut_rs = f"{MUT}/🦀️.rs"
    s = rd(mut_rs)
    uses = "".join(f"use {crate_lib(l['crate'])}::{snake(l['kind'])};\n" for l in names if f"use {crate_lib(l['crate'])}::{snake(l['kind'])};" not in s)
    s = insert_after_last(s, r"^use semio_s_artifact_norm_en1996_mut_\w+::\w+;\n", uses)
    variants = "".join(f"    {pascal(l['kind'])}({snake(l['kind'])}::{pascal(l['kind'])}),\n" for l in names if f"    {pascal(l['kind'])}(" not in s)
    s = insert_after_last(s, r"^    \w+\(\w+::\w+\),\n", variants)
    kinds = "".join(f'    "{l["kind"]}",\n' for l in names if f'"{l["kind"]}",' not in s)
    s = insert_after_last(s, r'^    "change-hk-earth",\n', kinds)
    wr(mut_rs, s)
    tx = f"{IO}/📝️text/🧬️mutations/🦀️.rs"
    s = rd(tx)
    uses = "".join(f"use {crate_lib(l['crate'])}::{snake(l['kind'])};\n" for l in names if f"use {crate_lib(l['crate'])}::{snake(l['kind'])};" not in s)
    wr(tx, insert_after_last(s, r"^use semio_s_artifact_norm_en1996_mut_\w+::\w+;\n", uses))
    ptxt = rd(proto_path)
    add = "".join(f"record {l['kind']} tag={tags[l['kind']]}\nfield payload bytes\n" for l in names if l["kind"] in tags)
    wr(proto_path, ptxt.rstrip("\n") + "\n" + add)
    aj = jload(f"{MUT}/🔣️.json")
    have = json.dumps(aj)
    for l in names:
        P = pascal(l["kind"])
        if f'"{P}"' not in have:
            aj["oneOf"].append({"type": "object", "additionalProperties": False, "required": [P], "properties": {P: {"$ref": f"{SCHEMA_ID}/mutation/{l['kind']}/schema.json"}}})
    jdump(f"{MUT}/🔣️.json", aj)
    ts_path = f"{MUT}/🟦️.ts"
    s = rd(ts_path)
    for l in names:
        P = pascal(l["kind"])
        if f"parse{P}" in s:
            continue
        s = insert_after_last(s, r'^import \{[^\n]*\} from "\./[^\n]*/🧬️schema/🟦️\.ts";\n', f'import {{ type {P}, parse{P} }} from "./{dirname(l)}/🧬️schema/🟦️.ts";\n')
    for l in names:
        P = pascal(l["kind"])
        if f"{{ {P}: {P} }}" not in s:
            m = list(re.finditer(r"^  \| \{ \w+: \w+ \};$", s, re.M))[-1]
            s = s[: m.end() - 1] + f"\n  | {{ {P}: {P} }};" + s[m.end():]
        if f"  {P}: parse{P}," not in s:
            m = list(re.finditer(r"^  \w+: parse\w+,$", s, re.M))[-1]
            s = s[: m.end()] + f"\n  {P}: parse{P}," + s[m.end():]
    wr(ts_path, s)
    pp = f"{MUT}/🛰️.proto"
    s = rd(pp)
    m = re.search(r"oneof kind \{(.*?)\n  \}", s, re.S)
    last = max(int(x) for x in re.findall(r"= (\d+);", m.group(1)))
    oneof = ""
    msgs = ""
    for l in names:
        P = pascal(l["kind"])
        if f"message {P} " in s:
            continue
        last += 1
        oneof += f"    {P} {snake(l['kind'])} = {last};\n"
        body = ""
        for i, (n, t, *_ ) in enumerate(payload_fields(l), 1):
            pt = "uint32" if t == "usize" else ("double" if t == "f64" else "string")
            body += f"  {pt} {n} = {i};\n"
        msgs += f"message {P} {{\n{body}}}\n"
    s = s[: m.end(1)] + "\n" + oneof.rstrip("\n") + s[m.end(1):]
    wr(pp, s.rstrip("\n") + "\n" + msgs)
    gp = f"{MUT}/🔗️.graphql"
    s = rd(gp)
    lines = s.split("\n")
    union_add = "".join(f" | {pascal(l['kind'])}" for l in names if f" {pascal(l['kind'])}" not in lines[0] + " ")
    lines[0] += union_add
    s = "\n".join(lines)
    types = ""
    for l in names:
        P = pascal(l["kind"])
        if f"type {P} " in s:
            continue
        body = ""
        for n, t, *_ in payload_fields(l):
            gt = "Int" if t == "usize" else ("Float" if t == "f64" else "String")
            body += f"  {camel(n)}: {gt}\n"
        types += f"\ntype {P} {{\n{body}}}\n"
    wr(gp, s.rstrip("\n") + "\n" + types)
    rules = f"{MUT}/🧭️edit-rules/🦀️.rs"
    s = rd(rules)
    set_add, ins_add, rem_add = "", "", ""
    for l in names:
        op = l["op"]
        if op == "insert":
            r = l["row"]
            ins_add += f'        InsertItemRule {{ path: "walls[].{r["coll"]}", kind: "{l["kind"]}", selectors: &[SelectorKey::Index("wallIndex")], index: "index", item: "row" }},\n'
        elif op == "remove":
            r = l["row"]
            rem_add += f'        RemoveItemRule {{ path: "walls[].{r["coll"]}", kind: "{l["kind"]}", selectors: &[SelectorKey::Index("wallIndex")], index: "index" }},\n'
        else:
            f = l["field"]
            nv = "new" + pascal(f["name"])
            if op == "scalar":
                path, sel = f'walls[].{f["camel"]}', 'SelectorKey::Index("index")'
            elif op == "field":
                path, sel = f'walls[].{l["row"]["coll"]}[].{f["camel"]}', 'SelectorKey::Index("wallIndex"), SelectorKey::Index("index")'
            elif op == "opening":
                path, sel = f'walls[].openings[].{f["camel"]}', 'SelectorKey::Index("wallIndex"), SelectorKey::Index("index")'
            else:
                path, sel = f'walls[].loadCases[].concentrated[].{f["camel"]}', 'SelectorKey::Index("wallIndex"), SelectorKey::Index("loadCaseIndex"), SelectorKey::Index("index")'
            set_add += f'        SetFieldRule {{ path: "{path}", kind: "{l["kind"]}", selectors: &[{sel}], value: "{nv}" }},\n'
    s = insert_after_last(s, r'^        SetFieldRule \{[^\n]*\},\n', set_add)
    s = insert_after_last(s, r'^        InsertItemRule \{[^\n]*\},\n', ins_add)
    s = insert_after_last(s, r'^        RemoveItemRule \{[^\n]*\},\n', rem_add)
    wr(rules, s)
    print("generated", len(new_leaves), "leaves; tags", min(tags.values()) if tags else None, "-", max(tags.values()) if tags else None)


main()
