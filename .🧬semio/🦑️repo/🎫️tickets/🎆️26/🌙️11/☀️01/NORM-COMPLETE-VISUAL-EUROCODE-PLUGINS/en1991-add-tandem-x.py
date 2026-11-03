#!/usr/bin/env python3
"""Adds the snapshot field `bridgeTandemX` (LM1 tandem position from the left support in m, default 10.0) and its edit leaf
`change-bridge-tandem-x` through every EN 1991 layer: model, artifact, outline, json / graphql / proto / ts twins, diff,
sqlite (sql, rust, ts, tests), examples, committed fixtures and the leaf (clone of `change-bridge-span`). Idempotent. Input script."""
import copy, glob, json, os, re

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = f"{ROOT}/🧬️schema"
MUT = f"{SCH}/🧬️mutations"
FIX = f"{ROOT}/🧪️tests/🧫️fixtures/🧬️mutations"
SQ = f"{ROOT}/🚪️io/🪶️sqlite/📸️snapshot"
KIND, DIR, P, S = "change-bridge-tandem-x", "📍change-bridge-tandem-x", "ChangeBridgeTandemX", "change_bridge_tandem_x"
SRC_KIND, SRC_DIR = "change-bridge-span", "🏗️change-bridge-span"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def sub(path, old, new, count=1, marker=None):
    s = rd(path)
    if (marker or new) in s:
        return
    assert s.count(old) == count, (path, old[:80], s.count(old))
    wr(path, s.replace(old, new))


def after(path, anchor, add, count=1):
    sub(path, anchor, anchor + add, count, marker=add)


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def snapshot_field():
    after(f"{SCH}/📸️snapshot/🦀️.rs", "    pub assumed_silo_wall_friction: f64,\n", '    #[dsl(unit = "m")]\n    #[state(artifact)]\n    pub bridge_tandem_x: f64,\n')
    after(f"{SCH}/📸️snapshot/🦀️.rs", "            assumed_silo_wall_friction: 15_000.0,\n", "            bridge_tandem_x: 10.0,\n")
    a = f"{SCH}/🦀️.rs"
    after(a, "    pub assumed_silo_wall_friction: f64,\n", "    #[state(artifact)]\n    pub bridge_tandem_x: f64,\n")
    after(a, "            assumed_silo_wall_friction: self.assumed_silo_wall_friction,\n", "            bridge_tandem_x: self.bridge_tandem_x,\n")
    after(a, "            assumed_silo_wall_friction: snapshot.assumed_silo_wall_friction,\n", "            bridge_tandem_x: snapshot.bridge_tandem_x,\n")
    d = f"{SCH}/🔺️diff/🦀️.rs"
    after(d, "    pub assumed_silo_wall_friction: Option<f64>,\n", "    #[state(artifact)]\n    pub bridge_tandem_x: Option<f64>,\n")
    after(d, "            self.assumed_silo_wall_friction = other.assumed_silo_wall_friction;\n        }\n", "        if other.bridge_tandem_x.is_some() {\n            self.bridge_tandem_x = other.bridge_tandem_x;\n        }\n")
    after(d, "            assumed_silo_wall_friction: self.assumed_silo_wall_friction.unwrap_or(base.assumed_silo_wall_friction),\n", "            bridge_tandem_x: self.bridge_tandem_x.unwrap_or(base.bridge_tandem_x),\n")
    after(d, "            && self.assumed_silo_wall_friction.is_none()\n", "            && self.bridge_tandem_x.is_none()\n")
    after(d, "            assumed_silo_wall_friction: self.assumed_silo_wall_friction.as_ref().map(|_| base.assumed_silo_wall_friction),\n", "            bridge_tandem_x: self.bridge_tandem_x.as_ref().map(|_| base.bridge_tandem_x),\n")
    after(f"{SCH}/💡️inferences/🧾outline/🦀️.rs", '    "assumed_silo_wall_friction",\n', '    "bridge_tandem_x",\n')
    after(f"{ROOT}/📚️examples/🧬️subjects/🦀️.rs", "    s.hoisting_speed = 0.5;\n", "    s.bridge_tandem_x = 10.0;\n")


def twins():
    for rel in ["🔣️.json", "📸️snapshot/🔣️.json"]:
        p = f"{SCH}/{rel}"
        after(p, '    "assumedSiloWallFriction",\n', '    "bridgeTandemX",\n')
        after(p, '    "assumedSiloWallFriction": {\n      "type": "number",\n      "format": "double",\n      "x-semio-state": "artifact"\n    },\n', '    "bridgeTandemX": {\n      "type": "number",\n      "format": "double",\n      "x-semio-state": "artifact"\n    },\n')
    p = f"{SCH}/🔺️diff/🔣️.json"
    anchor = '    "assumedSiloWallFriction": {\n      "anyOf": [\n        {\n          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1991/snapshot.json#/properties/assumedSiloWallFriction"\n        },\n        {\n          "type": "null"\n        }\n      ],\n      "x-semio-state": "artifact"\n    },\n'
    after(p, anchor, anchor.replace("assumedSiloWallFriction", "bridgeTandemX"))
    for rel, line in [("🔗️.graphql", "  assumedSiloWallFriction: Float! @state(class: ARTIFACT)\n"), ("📸️snapshot/🔗️.graphql", "  assumedSiloWallFriction: Float! @state(class: ARTIFACT)\n"), ("🔺️diff/🔗️.graphql", "  assumedSiloWallFriction: Float @state(class: ARTIFACT)\n")]:
        after(f"{SCH}/{rel}", line, line.replace("assumedSiloWallFriction", "bridgeTandemX"))
    for rel in ["🛰️.proto", "📸️snapshot/🛰️.proto", "🔺️diff/🛰️.proto"]:
        p = f"{SCH}/{rel}"
        s = rd(p)
        if "bridge_tandem_x" in s:
            continue
        for old, new in [("accidental_cases = 49", "accidental_cases = 50"), ("wind_faces = 48", "wind_faces = 49"), ("roofs = 47", "roofs = 48"), ("self_weight_elements = 46", "self_weight_elements = 47"), ("floors = 45", "floors = 46")]:
            assert s.count(old) == 1, (p, old)
            s = s.replace(old, new)
        anchor = "  double assumed_silo_wall_friction = 44;\n"
        assert s.count(anchor) == 1
        wr(p, s.replace(anchor, anchor + "  // @state artifact\n  double bridge_tandem_x = 45;\n"))
    for rel, req in [("🟦️.ts", "normWireRequired(normWireNumber)"), ("📸️snapshot/🟦️.ts", "normWireRequired(normWireNumber)"), ("🔺️diff/🟦️.ts", "normWireDefault(normWireNullable(normWireNumber), () => null)")]:
        p = f"{SCH}/{rel}"
        sub(p, f"assumedSiloWallFriction: {req}, floors:", f"assumedSiloWallFriction: {req}, bridgeTandemX: {req}, floors:")
        nullable = " | null" if "diff" in rel else ""
        sub(p, f"  assumedSiloWallFriction: number{nullable};\n", f"  assumedSiloWallFriction: number{nullable};\n  bridgeTandemX: number{nullable};\n")


def sqlite():
    p = f"{SQ}/🗄️.sql"
    sub(p, "assumed_silo_pressure REAL, assumed_silo_patch REAL, assumed_silo_wall_friction REAL,\n", "assumed_silo_pressure REAL, assumed_silo_patch REAL, assumed_silo_wall_friction REAL, bridge_tandem_x REAL,\n")
    sub(p, "assumed_silo_wall_friction_ieee754_bits INTEGER NOT NULL, assumed_silo_wall_friction_ieee754_class TEXT NOT NULL);", "assumed_silo_wall_friction_ieee754_bits INTEGER NOT NULL, assumed_silo_wall_friction_ieee754_class TEXT NOT NULL,\nbridge_tandem_x_ieee754_bits INTEGER NOT NULL, bridge_tandem_x_ieee754_class TEXT NOT NULL);")
    r = f"{SQ}/🦀️.rs"
    sub(r, "FloatColumn::Binary64(65)];", "FloatColumn::Binary64(65), FloatColumn::Binary64(66)];")
    sub(r, 'entities(database, "en1991_document", 66,', 'entities(database, "en1991_document", 67,')
    sub(r, "Cell::Real(self.assumed_silo_wall_friction)\n", "Cell::Real(self.assumed_silo_wall_friction), Cell::Real(self.bridge_tandem_x)\n")
    sub(r, "assumed_silo_wall_friction: document.real(65)?, floors,", "assumed_silo_wall_friction: document.real(65)?, bridge_tandem_x: document.real(66)?, floors,")
    t = f"{SQ}/🟦️.ts"
    sub(t, '|"assumedSiloWallFriction"|"floors"', '|"assumedSiloWallFriction"|"bridgeTandemX"|"floors"')
    sub(t, "assumedSiloWallFriction:Binary64;\n", "assumedSiloWallFriction:Binary64;bridgeTandemX:Binary64;\n")
    sub(t, "s.assumedSiloWallFriction],DOCUMENT_FLOATS", "s.assumedSiloWallFriction,s.bridgeTandemX],DOCUMENT_FLOATS")
    sub(t, "assumedSiloWallFriction:readBinary64(d,65,DOCUMENT_FLOATS),floors", "assumedSiloWallFriction:readBinary64(d,65,DOCUMENT_FLOATS),bridgeTandemX:readBinary64(d,66,DOCUMENT_FLOATS),floors")
    sub(t, "entities(tables[0]!,66,DOCUMENT_FLOATS", "entities(tables[0]!,67,DOCUMENT_FLOATS")
    s = rd(t)
    m = re.search(r"const DOCUMENT_FLOATS:readonly Ieee754Column\[\]=\[[0-9,]*?,65\]", s)
    if m:
        wr(t, s.replace(m.group(0), m.group(0)[:-1] + ",66]"))
    else:
        assert re.search(r"const DOCUMENT_FLOATS:readonly Ieee754Column\[\]=\[[0-9,]*?,66\]", s), "DOCUMENT_FLOATS"
    tt = f"{SQ}/🧪️tests/🟦️.ts"
    sub(tt, "assumedSiloWallFriction:binary64(f.assumedSiloWallFriction),", "assumedSiloWallFriction:binary64(f.assumedSiloWallFriction),bridgeTandemX:binary64(f.bridgeTandemX),")
    sub(tt, "s.assumedSiloWallFriction=v;", "s.assumedSiloWallFriction=v;s.bridgeTandemX=v;")
    sub(f"{SQ}/🧪️tests/🦀️.rs", "snapshot.assumed_silo_wall_friction = value;", "snapshot.assumed_silo_wall_friction = value; snapshot.bridge_tandem_x = value;")


def fixtures():
    for f in glob.glob(f"{FIX}/*/✅apply/**/🔣️.json", recursive=True):
        s = rd(f)
        if '"bridgeTandemX"' in s:
            continue
        m = re.search(r'^(\s*)"assumedSiloWallFriction": [^\n]*?,\n', s, re.M)
        if not m:
            continue
        value = "null" if "🔺️diff" in f else "10.0"
        wr(f, s[: m.end()] + f'{m.group(1)}"bridgeTandemX": {value},\n' + s[m.end():])


def leaf():
    if os.path.isdir(f"{MUT}/{DIR}"):
        return
    def conv(s):
        return (s.replace("change-bridge-span", KIND).replace("change_bridge_span", S).replace("ChangeBridgeSpan", P).replace("ChangedBridgeSpan", "ChangedBridgeTandemX")
                .replace("new_bridge_span", "new_bridge_tandem_x").replace("newBridgeSpan", "newBridgeTandemX").replace("bridge_span", "bridge_tandem_x").replace("bridge-span", "bridge-tandem-x")
                .replace("Change bridge span", "Change tandem position").replace("Stützweite der Brücke ändern", "Tandemlage ändern").replace("🏗️", "📍").replace("🏗", "📍"))
    for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs", "🧬️schema/🟦️.ts"]:
        wr(f"{MUT}/{DIR}/{rel}", conv(rd(f"{MUT}/{SRC_DIR}/{rel}")))
    j = json.loads(rd(f"{MUT}/{SRC_DIR}/🔣️.json"))
    j.update(owner=f"{OWNER}/{DIR}", semanticKind=KIND, displayName=P, emoji="📍", aggregateVariant=P, binaryTag=89)
    jdump(f"{MUT}/{DIR}/🔣️.json", j)
    sj = json.loads(rd(f"{MUT}/{SRC_DIR}/🧬️schema/🔣️.json"))
    sj["$id"] = sj["$id"].replace("change-bridge-span", KIND)
    sj["title"] = P
    sj["required"] = ["newBridgeTandemX"]
    prop = sj["properties"].pop("newBridgeSpan")
    prop["x-semio-ui"].update(label={"en": "Tandem position x from the left support", "de": "Tandemlage x ab linkem Auflager"}, step=0.25)
    sj["properties"] = {"newBridgeTandemX": prop}
    jdump(f"{MUT}/{DIR}/🧬️schema/🔣️.json", sj)
    mount = f'#[path = "../{DIR}/🦀️.rs"]\npub mod {S};\n'
    sub(f"{MUT}/🪜️traffic-load-models/🦀️.rs", "//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts", marker=mount)
    sub(f"{MUT}/🌉️traffic/🦀️.rs", "change_bridge_lane_width, change_bridge_span};", f"change_bridge_lane_width, change_bridge_span, {S}}};")
    agg = f"{MUT}/🦀️.rs"
    after(agg, "use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_span;\n", f"use semio_s_artifact_norm_en1991_mut_traffic::{S};\n")
    after(agg, "    ChangeBridgeSpan(change_bridge_span::ChangeBridgeSpan),\n", f"    {P}({S}::{P}),\n")
    after(agg, '    "change-bridge-span",\n', f'    "{KIND}",\n')
    after(f"{ROOT}/🚪️io/📝️text/🧬️mutations/🦀️.rs", "use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_span;\n", f"use semio_s_artifact_norm_en1991_mut_traffic::{S};\n")
    ts = f"{MUT}/🟦️.ts"
    after(ts, 'import { type ChangeBridgeSpan, parseChangeBridgeSpan } from "./🏗️change-bridge-span/🧬️schema/🟦️.ts";\n', f'import {{ type {P}, parse{P} }} from "./{DIR}/🧬️schema/🟦️.ts";\n')
    after(ts, "  | { ChangeBridgeSpan: ChangeBridgeSpan }\n", f"  | {{ {P}: {P} }}\n")
    after(ts, "  ChangeBridgeSpan: parseChangeBridgeSpan,\n", f"  {P}: parse{P},\n")
    aj = json.loads(rd(f"{MUT}/🔣️.json"))
    for key, v in aj.items():
        if isinstance(v, list) and any("ChangeBridgeSpan" in json.dumps(x) for x in v) and not any(P in json.dumps(x) for x in v):
            i = next(i for i, x in enumerate(v) if "ChangeBridgeSpan" in json.dumps(x))
            v.insert(i + 1, json.loads(json.dumps(v[i]).replace("ChangeBridgeSpan", P).replace("change-bridge-span", KIND)))
    jdump(f"{MUT}/🔣️.json", aj)
    after(f"{MUT}/🧭️edit-rules/🦀️.rs", '        SetFieldRule { path: "bridgeSpan", kind: "change-bridge-span", selectors: &[], value: "newBridgeSpan" },\n', f'        SetFieldRule {{ path: "bridgeTandemX", kind: "{KIND}", selectors: &[], value: "newBridgeTandemX" }},\n')
    proto = f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(proto)
    if f"record {KIND} " not in s:
        wr(proto, s.rstrip("\n") + f"\nrecord {KIND} tag=87\nfield payload bytes\n")
    om = f"{ROOT}/🔮️oracles/🔣️.json"
    o = json.loads(rd(om))
    def walk(node):
        if isinstance(node, dict):
            for v in node.values():
                yield from walk(v)
        elif isinstance(node, list):
            yield node
            for v in node:
                yield from walk(v)
    for lst in list(walk(o)):
        if KIND in json.dumps(lst):
            continue
        for i, x in enumerate(lst):
            if x == SRC_KIND:
                lst.insert(i + 1, KIND)
                break
            if isinstance(x, dict) and x.get("mutationId") == SRC_KIND:
                n = copy.deepcopy(x)
                n.update(mutationId=KIND, sourceMutationDirectoryName=DIR, mutationDirectoryName=DIR)
                lst.insert(i + 1, n)
                break
            if isinstance(x, dict) and x.get("id") == SRC_KIND:
                n = copy.deepcopy(x)
                n["id"] = KIND
                n["productionDispatch"].update(operation=KIND, variant=P)
                lst.insert(i + 1, n)
                break
    jdump(om, o)
    ft = f"{ROOT}/🧪️tests/🏋️mutate-en1991-1/🥒️.feature"
    s = rd(ft)
    if f"| {KIND} " not in s:
        out = []
        for l in s.split("\n"):
            out.append(l)
            if l.strip().startswith(f"| {SRC_KIND} "):
                out.append(l.replace(SRC_KIND, KIND).replace("🏗️", "📍"))
        wr(ft, "\n".join(out))
    src = f"{FIX}/{SRC_DIR}/✅apply"
    dst = f"{FIX}/{DIR}/✅apply"
    before = json.loads(rd(f"{src}/🔣️.before.json"))
    aft = copy.deepcopy(before)
    aft["bridgeTandemX"] = 14.0
    jdump(f"{dst}/🔣️.before.json", before)
    jdump(f"{dst}/🔣️.after.json", aft)
    jdump(f"{dst}/🔣️.mutation.json", {P: {"newBridgeTandemX": 14.0}})
    jdump(f"{dst}/🔣️.outcome.json", {"status": "applied"})
    d = json.loads(rd(f"{src}/🔣️.diff.json"))
    d["bridgeSpan"] = None
    d["bridgeTandemX"] = 14.0
    jdump(f"{dst}/🔣️.diff.json", d)


snapshot_field()
twins()
sqlite()
fixtures()
leaf()
print("done")
