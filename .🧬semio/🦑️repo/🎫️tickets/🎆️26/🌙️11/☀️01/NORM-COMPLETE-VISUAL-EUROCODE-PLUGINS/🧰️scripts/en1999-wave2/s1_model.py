"""Stage 1: adds WeldRun and SectionVertex to the EN 1999 model, root artifact, snapshot and root twins (rust, json, ts, graphql, proto). Idempotent. Usage: python -I s1_model.py"""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from spec import *

DERIVE = "#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n#[value(rename_all = \"camelCase\")]\n"


def rd(p):
    return pathlib.Path(p).read_text(encoding="utf8")


def wr(p, t):
    pathlib.Path(p).write_text(t, encoding="utf8")


def rust_struct(e):
    out = f"/// {e.emoji} {e.doc}\n{DERIVE}pub struct {e.struct} {{\n    pub id: String,\n"
    for f in e.fields:
        out += f"    /// {f.emoji} {f.doc}\n    pub {f.snake}: {f.rust},\n"
    return out + "}\n\n"


def model_rs():
    p = SNAP / "🦀️.rs"
    t = rd(p)
    if "pub struct WeldRun" in t:
        return
    anchor = "//#endregion 🔖️SubjectEntities"
    t = t.replace(anchor, "".join(rust_struct(e) for e in ENTS) + anchor)
    t = t.replace("    pub placements: Vec<Placement>,\n}", "    pub placements: Vec<Placement>,\n" + "".join(f"    #[state(artifact)]\n    #[dsl(table)]\n    pub {e.field}: Vec<{e.struct}>,\n" for e in ENTS) + "}")
    t = t.replace("            placements: Vec::new(),\n", "            placements: Vec::new(),\n" + "".join(f"            {e.field}: Vec::new(),\n" for e in ENTS))
    wr(p, t)


def root_rs():
    p = SCHEMA / "🦀️.rs"
    t = rd(p)
    if "WeldRun" in t:
        return
    t = t.replace("AluminiumShell, Placement,", "AluminiumShell, Placement, WeldRun, SectionVertex,", 1)
    t = t.replace("    pub placements: Vec<Placement>,\n}", "    pub placements: Vec<Placement>,\n" + "".join(f"    #[state(artifact)]\n    pub {e.field}: Vec<{e.struct}>,\n" for e in ENTS) + "}")
    t = t.replace("            placements: self.placements.clone(),\n", "            placements: self.placements.clone(),\n" + "".join(f"            {e.field}: self.{e.field}.clone(),\n" for e in ENTS))
    t = t.replace("            placements: snapshot.placements,\n", "            placements: snapshot.placements,\n" + "".join(f"            {e.field}: snapshot.{e.field},\n" for e in ENTS))
    wr(p, t)


def json_def(e):
    props = {"id": {"type": "string", "x-semio-ui": {"widget": "reference", "label": {"en": e.id_en, "de": e.id_de}, "ref": {"kind": "entity"}}}}
    for f in e.fields:
        props[f.camel] = {"type": f.json, "x-semio-ui": f.schema_ui()}
    return {"title": e.struct, "type": "object", "additionalProperties": False, "required": ["id"] + [f.camel for f in e.fields], "properties": props, "x-semio-formats": ["🔣️jsonschema", "🦀️rust", "🟦️typescript", "🔗️graphql", "🛰️protobuf"]}


def snapshot_json():
    p = SNAP / "🔣️.json"
    d = json.loads(rd(p))
    if "WeldRun" in d["$defs"]:
        return
    for e in ENTS:
        d["required"].append(e.camel)
        d["properties"][e.camel] = {"type": "array", "items": {"$ref": f"#/$defs/{e.struct}"}, "x-semio-state": "artifact"}
        d["$defs"][e.struct] = json_def(e)
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def root_json():
    p = SCHEMA / "🔣️.json"
    d = json.loads(rd(p))
    if "weldRuns" in d["properties"]:
        return
    for e in ENTS:
        d["required"].append(e.camel)
        props = {"id": {"type": "string"}, **{f.camel: {"type": f.json} for f in e.fields}}
        d["properties"][e.camel] = {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": list(props), "properties": props}, "x-semio-state": "artifact"}
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def ts_iface(e):
    return f"export interface {e.struct} {{\n  id: string;\n" + "".join(f"  {f.camel}: {f.ts};\n" for f in e.fields) + "}\n\n"


def ts_parser(e):
    body = ", ".join(["id: normWireRequired(normWireString)"] + [f"{f.camel}: normWireRequired({f.wire})" for f in e.fields])
    return f"export const parse{e.struct}: NormWireReader<{e.struct}> = normWireObject<{e.struct}>({{ {body} }});\n"


def snapshot_ts():
    p = SNAP / "🟦️.ts"
    t = rd(p)
    if "export interface WeldRun" in t:
        return
    emojis = {"weld_runs": "🧵️", "outline_vertices": "🔷️"}
    t = t.replace("  placements: Placement[];\n}", "  placements: Placement[];\n" + "".join(f"  /** {emojis[e.field]} @state artifact */\n  {e.camel}: {e.struct}[];\n" for e in ENTS) + "}", 1)
    i = t.index("export interface Placement {")
    j = t.index("}\n\n", i) + 3
    t = t[:j] + "".join(ts_iface(e) for e in ENTS) + t[j:]
    t = t.replace("placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))) });", "placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), " + ", ".join(f"{e.camel}: normWireRequired(normWireArray(normWireRef(() => parse{e.struct})))" for e in ENTS) + " });", 1)
    k = t.index("export const parsePlacement:")
    k = t.index("\n", k) + 1
    t = t[:k] + "".join(ts_parser(e) for e in ENTS) + t[k:]
    wr(p, t)


def root_ts():
    p = SCHEMA / "🟦️.ts"
    t = rd(p)
    if "weldRuns:" in t:
        return
    emojis = {"weld_runs": "🧵️", "outline_vertices": "🔷️"}

    def inline(e):
        return "{ id: string; " + " ".join(f"{f.camel}: {f.ts};" for f in e.fields) + " }"

    t = t.replace("  placements: { id: string; x: number; y: number; z: number; yaw: number; }[];\n}", "  placements: { id: string; x: number; y: number; z: number; yaw: number; }[];\n" + "".join(f"  /** {emojis[e.field]} @state artifact */\n  {e.camel}: {inline(e)}[];\n" for e in ENTS) + "}", 1)
    reader = lambda e: f"{e.camel}: normWireRequired(normWireArray(normWireObject<{inline(e)}>({{ " + ", ".join(["id: normWireRequired(normWireString)"] + [f"{f.camel}: normWireRequired({f.wire})" for f in e.fields]) + " })))"
    tail = "yaw: normWireRequired(normWireNumber) }))) });"
    assert tail in t
    t = t.replace(tail, "yaw: normWireRequired(normWireNumber) }))), " + ", ".join(reader(e) for e in ENTS) + " });", 1)
    wr(p, t)


def gql_type(e):
    return f"type {e.struct} {{ id: String!, " + ", ".join(f"{f.camel}: {f.gql}!" for f in e.fields) + " }\n"


def graphql():
    for p in (SCHEMA / "🔗️.graphql", SNAP / "🔗️.graphql"):
        t = rd(p)
        if "WeldRun" in t:
            continue
        t = t.replace("  placements: [Placement!]! @state(artifact)\n", "  placements: [Placement!]! @state(artifact)\n" + "".join(f"  {e.camel}: [{e.struct}!]! @state(artifact)\n" for e in ENTS))
        i = t.index("type Placement {")
        i = t.index("\n", i) + 1
        t = t[:i] + "".join(gql_type(e) for e in ENTS) + t[i:]
        wr(p, t)


def proto_msg(e):
    return f"message {e.struct} {{ string id = 1; " + " ".join(f"{f.proto} {f.snake} = {n};" for n, f in enumerate(e.fields, 2)) + " }\n"


def proto():
    for p in (SCHEMA / "🛰️.proto", SNAP / "🛰️.proto"):
        t = rd(p)
        if "WeldRun" in t:
            continue
        t = t.replace("  repeated Placement placements = 10;\n", "  repeated Placement placements = 10;\n" + "".join(f"  repeated {e.struct} {e.field} = {11 + n};\n" for n, e in enumerate(ENTS)))
        i = t.index("message Placement {")
        i = t.index("\n", i) + 1
        t = t[:i] + "".join(proto_msg(e) for e in ENTS) + t[i:]
        wr(p, t)


if __name__ == "__main__":
    model_rs(); root_rs(); snapshot_json(); root_json(); snapshot_ts(); root_ts(); graphql(); proto()
    print("stage 1 done")
