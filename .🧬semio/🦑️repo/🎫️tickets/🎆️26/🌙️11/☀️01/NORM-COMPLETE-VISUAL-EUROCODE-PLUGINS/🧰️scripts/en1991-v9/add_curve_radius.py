#!/usr/bin/env python3
"""Adds the snapshot field bridgeCurveRadius (horizontal deck radius in m, 0 = straight deck) after siloHopperAngleDeg through every EN 1991 layer plus its edit leaf
`change-bridge-curve-radius` (recipe of en1991-add-crane-silo.py). All edits are staged in memory and written only when every anchor matched. Idempotent. Input script."""
import copy, glob, json, os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH, MUT = f"{ROOT}/🧬️schema", f"{ROOT}/🧬️schema/🧬️mutations"
FIX, SQ = f"{ROOT}/🧪️tests/🧫️fixtures/🧬️mutations", f"{ROOT}/🚪️io/🪶️sqlite/📸️snapshot"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SRC_DIR = "📍change-bridge-tandem-x"
F = dict(snake="bridge_curve_radius", camel="bridgeCurveRadius", pascal="BridgeCurveRadius", en="Horizontal deck radius (0 = straight deck)", de="Horizontaler Deckradius (0 = gerades Deck)", unit="m", step=10, prec=1, default=0.0, lo=0, fix=400.0, emoji="🌀", group="traffic")
PREV = dict(snake="silo_hopper_angle_deg", camel="siloHopperAngleDeg", default="60.0")
KIND = "change-bridge-curve-radius"
DIR = F["emoji"] + KIND
P = "ChangeBridgeCurveRadius"
S = KIND.replace("-", "_")
DOC_IDX = 79
TAG = 245
OVERLAY, EXISTS = {}, {}


def rd(p):
    if p in OVERLAY:
        return OVERLAY[p]
    return open(p, encoding="utf-8").read()


def wr(p, s):
    OVERLAY[p] = s


def sub(path, old, new, count=1, marker=None):
    s = rd(path)
    if (marker or new) in s:
        return
    assert s.count(old) == count, (path.split("/✳️any/")[-1], old[:100], s.count(old))
    wr(path, s.replace(old, new))


def after(path, anchor, add, count=1):
    sub(path, anchor, anchor + add, count, marker=add)


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def snapshot_layers():
    ps, new, pc, nc = PREV["snake"], F["snake"], PREV["camel"], F["camel"]
    unit = f'    #[dsl(unit = "{F["unit"]}")]\n'
    after(f"{SCH}/📸️snapshot/🦀️.rs", f"    pub {ps}: f64,\n", f"{unit}    #[state(artifact)]\n    pub {new}: f64,\n")
    after(f"{SCH}/📸️snapshot/🦀️.rs", f"            {ps}: {PREV['default']},\n", f"            {new}: 0.0,\n")
    a = f"{SCH}/🦀️.rs"
    after(a, f"    pub {ps}: f64,\n", f"    #[state(artifact)]\n    pub {new}: f64,\n")
    after(a, f"            {ps}: self.{ps},\n", f"            {new}: self.{new},\n")
    after(a, f"            {ps}: snapshot.{ps},\n", f"            {new}: snapshot.{new},\n")
    d = f"{SCH}/🔺️diff/🦀️.rs"
    after(d, f"    pub {ps}: Option<f64>,\n", f"    #[state(artifact)]\n    pub {new}: Option<f64>,\n")
    after(d, f"            self.{ps} = other.{ps};\n        }}\n", f"        if other.{new}.is_some() {{\n            self.{new} = other.{new};\n        }}\n")
    after(d, f"            {ps}: self.{ps}.unwrap_or(base.{ps}),\n", f"            {new}: self.{new}.unwrap_or(base.{new}),\n")
    after(d, f"            && self.{ps}.is_none()\n", f"            && self.{new}.is_none()\n")
    after(d, f"            {ps}: self.{ps}.as_ref().map(|_| base.{ps}),\n", f"            {new}: self.{new}.as_ref().map(|_| base.{new}),\n")
    after(f"{SCH}/💡️inferences/🧾outline/🦀️.rs", f'    "{ps}",\n', f'    "{new}",\n')
    after(f"{ROOT}/📚️examples/🧬️subjects/🦀️.rs", f"    s.{ps} = {PREV['default']};\n", f"    s.{new} = 0.0;\n")
    for rel in ["🔣️.json", "📸️snapshot/🔣️.json"]:
        p = f"{SCH}/{rel}"
        after(p, f'    "{pc}",\n', f'    "{nc}",\n')
        blk = f'    "{pc}": {{\n      "type": "number",\n      "format": "double",\n      "x-semio-state": "artifact"\n    }},\n'
        after(p, blk, blk.replace(pc, nc))
    p = f"{SCH}/🔺️diff/🔣️.json"
    blk = f'"{pc}":{{"anyOf":[{{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1991/snapshot.json#/properties/{pc}"}},{{"type":"null"}}],"x-semio-state":"artifact"}},'
    after(p, blk, blk.replace(pc, nc))
    for rel, line in [("🔗️.graphql", f"  {pc}: Float! @state(class: ARTIFACT)\n"), ("📸️snapshot/🔗️.graphql", f"  {pc}: Float! @state(class: ARTIFACT)\n"), ("🔺️diff/🔗️.graphql", f"  {pc}: Float @state(class: ARTIFACT)\n")]:
        after(f"{SCH}/{rel}", line, line.replace(pc, nc))
    for rel, opt in [("🛰️.proto", ""), ("📸️snapshot/🛰️.proto", ""), ("🔺️diff/🛰️.proto", "optional ")]:
        p = f"{SCH}/{rel}"
        s = rd(p)
        if f" {new} = " in s:
            continue
        top = max(int(n) for n in re.findall(r"= (\d+);", s))
        m = re.search(rf"  {opt}double {ps} = \d+;\n", s)
        assert m, (p, ps)
        wr(p, s[: m.end()] + f"  // @state artifact\n  {opt}double {new} = {top + 1};\n" + s[m.end():])
    for rel, req in [("🟦️.ts", "normWireRequired(normWireNumber)"), ("📸️snapshot/🟦️.ts", "normWireRequired(normWireNumber)"), ("🔺️diff/🟦️.ts", "normWireDefault(normWireNullable(normWireNumber), () => null)")]:
        p = f"{SCH}/{rel}"
        sub(p, f"{pc}: {req}, ", f"{pc}: {req}, {nc}: {req}, ")
        nullable = " | null" if "diff" in rel else ""
        sub(p, f"  {pc}: number{nullable};\n", f"  {pc}: number{nullable};\n  {nc}: number{nullable};\n")


def sqlite_layers():
    new, nc, pc = F["snake"], F["camel"], PREV["camel"]
    p = f"{SQ}/🗄️.sql"
    sub(p, "fire_load_basis TEXT NOT NULL CHECK(fire_load_basis IN ('occupancyTable','userSpecified')),", f"fire_load_basis TEXT NOT NULL CHECK(fire_load_basis IN ('occupancyTable','userSpecified')), {new} REAL,", marker=f"{new} REAL,")
    sub(p, "silo_hopper_angle_deg_ieee754_bits INTEGER NOT NULL, silo_hopper_angle_deg_ieee754_class TEXT NOT NULL);", f"silo_hopper_angle_deg_ieee754_bits INTEGER NOT NULL, silo_hopper_angle_deg_ieee754_class TEXT NOT NULL,\n{new}_ieee754_bits INTEGER NOT NULL, {new}_ieee754_class TEXT NOT NULL);", marker=f"{new}_ieee754_bits")
    r = f"{SQ}/🦀️.rs"
    sub(r, "FloatColumn::Binary64(77)];", f"FloatColumn::Binary64(77), FloatColumn::Binary64({DOC_IDX})];")
    sub(r, 'entities(database, "en1991_document", 79,', f'entities(database, "en1991_document", {DOC_IDX + 1},')
    sub(r, 'FireLoadBasis::UserSpecified => "userSpecified" })\n        ], DOCUMENT_FLOATS)?;', f'FireLoadBasis::UserSpecified => "userSpecified" }}), Cell::Real(self.{new})\n        ], DOCUMENT_FLOATS)?;')
    sub(r, "_ => return Err(invalid(\"EN1991 fire load basis is not owned\")) }, floors,", f"_ => return Err(invalid(\"EN1991 fire load basis is not owned\")) }}, {new}: document.real({DOC_IDX})?, floors,")
    t = f"{SQ}/🟦️.ts"
    sub(t, f'|"{pc}"|', f'|"{pc}"|"{nc}"|')
    sub(t, f"{pc}:Binary64;", f"{pc}:Binary64;{nc}:Binary64;")
    sub(t, "fireLoadBasis(s.fireLoadBasis)],DOCUMENT_FLOATS", f"fireLoadBasis(s.fireLoadBasis),s.{nc}],DOCUMENT_FLOATS")
    sub(t, "fireLoadBasis:fireLoadBasis(artifactSqliteText(d,78)),floors,", f"fireLoadBasis:fireLoadBasis(artifactSqliteText(d,78)),{nc}:readBinary64(d,{DOC_IDX},DOCUMENT_FLOATS),floors,")
    sub(t, "entities(tables[0]!,79,DOCUMENT_FLOATS", f"entities(tables[0]!,{DOC_IDX + 1},DOCUMENT_FLOATS")
    s = rd(t)
    m = re.search(r"const DOCUMENT_FLOATS:readonly Ieee754Column\[\]=\[[0-9,]*?,77\]", s)
    if m:
        wr(t, s.replace(m.group(0), m.group(0)[:-1] + f",{DOC_IDX}]"))
    elif f",{DOC_IDX}]" not in s:
        raise AssertionError("DOCUMENT_FLOATS list not found")
    tt = f"{SQ}/🧪️tests/🟦️.ts"
    sub(tt, f"{pc}:binary64(f.{pc}),", f"{pc}:binary64(f.{pc}),{nc}:binary64(f.{nc}),")
    sub(tt, f"s.{pc}=v;", f"s.{pc}=v;s.{nc}=v;")
    sub(f"{SQ}/🧪️tests/🦀️.rs", f"snapshot.{PREV['snake']} = value;", f"snapshot.{PREV['snake']} = value; snapshot.{new} = value;")


def meta_and_tests():
    new, nc, pc = F["snake"], F["camel"], PREV["camel"]
    fm = f"{ROOT}/✏️editor/🏷️field-meta/🦀️.rs"
    after(fm, f'        "{pc}" => FieldSpec::quantity("Hopper wall angle to horizontal", "Trichterneigung zur Horizontalen", UnitId::Degree),\n', f'        "{nc}" => FieldSpec::quantity("{F["en"]}", "{F["de"]}", UnitId::Metre),\n')
    cr = f"{SCH}/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"
    after(cr, f'        "{pc}" => doc.{PREV["snake"]} = value,\n', f'        "{nc}" => doc.{new} = value,\n')
    sub(cr, f'"{pc}", ', f'"{pc}", "{nc}", ', count=3, marker=f'"{nc}", ')
    after(f"{MUT}/🧭️edit-rules/🦀️.rs", '        SetFieldRule { path: "bridgeTandemX", kind: "change-bridge-tandem-x", selectors: &[], value: "newBridgeTandemX" },\n', f'        SetFieldRule {{ path: "{nc}", kind: "{KIND}", selectors: &[], value: "new{F["pascal"]}" }},\n')


def fixtures():
    nc, pc = F["camel"], PREV["camel"]
    pattern = re.compile(rf'("{pc}":\s*[^,\n}}]+,)(\s*)')
    skip = ("/🧬️schema/", "/target/", "/node_modules/", "/🎬️scene/")
    for fp in glob.glob(f"{ROOT}/**/*.json", recursive=True):
        if any(x in fp for x in skip) or fp.endswith(("🎲️field-metadata.json", "🎛️semantic.json")):
            continue
        try:
            s = rd(fp)
        except OSError:
            continue
        if f'"{pc}"' not in s or f'"{nc}"' in s:
            continue
        value = "null" if "🔺️diff" in fp else repr(F["default"])
        out = pattern.sub(lambda m: f'{m.group(1)}{m.group(2)}"{nc}": {value},{m.group(2)}', s)
        if out != s:
            wr(fp, out)


def leaf():
    if os.path.isdir(f"{MUT}/{DIR}") or f"{MUT}/{DIR}/🦀️.rs" in OVERLAY:
        return
    npascal = F["pascal"]
    def conv(s):
        return (s.replace("change-bridge-tandem-x", KIND).replace("change_bridge_tandem_x", S).replace("ChangeBridgeTandemX", P).replace("ChangedBridgeTandemX", "Changed" + npascal)
                .replace("new_bridge_tandem_x", "new_" + F["snake"]).replace("newBridgeTandemX", "new" + npascal).replace("bridge_tandem_x", F["snake"]).replace("bridge-tandem-x", KIND[len("change-"):])
                .replace("Change tandem position", "Change " + F["en"].lower()).replace("Tandemlage ändern", "Deckradius ändern").replace("📍", F["emoji"]))
    for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs", "🧬️schema/🟦️.ts"]:
        wr(f"{MUT}/{DIR}/{rel}", conv(rd(f"{MUT}/{SRC_DIR}/{rel}")))
    j = json.loads(rd(f"{MUT}/{SRC_DIR}/🔣️.json"))
    j.update(owner=f"{OWNER}/{DIR}", semanticKind=KIND, displayName=P, emoji=F["emoji"], aggregateVariant=P, binaryTag=TAG)
    jdump(f"{MUT}/{DIR}/🔣️.json", j)
    sj = json.loads(rd(f"{MUT}/{SRC_DIR}/🧬️schema/🔣️.json"))
    sj["$id"] = f"https://json.schemas.assets.semio-tech.com/s/norm/en1991/mutation/{KIND}/schema.json"
    sj["title"] = P
    newkey = "new" + npascal
    sj["required"] = [newkey]
    prop = sj["properties"].pop("newBridgeTandemX")
    ui = prop["x-semio-ui"]
    ui.update(label={"en": F["en"], "de": F["de"]}, step=F["step"], precision=F["prec"], group=F["group"])
    ui["unit"] = F["unit"]
    prop["minimum"] = F["lo"]
    sj["properties"] = {newkey: prop}
    jdump(f"{MUT}/{DIR}/🧬️schema/🔣️.json", sj)
    mount = f'#[path = "../{DIR}/🦀️.rs"]\npub mod {S};\n'
    sub(f"{MUT}/🪜️traffic-load-models/🦀️.rs", "//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts", marker=mount)
    sub(f"{MUT}/🌉️traffic/🦀️.rs", "change_bridge_span, change_bridge_tandem_x};", f"change_bridge_span, change_bridge_tandem_x, {S}}};")
    agg = f"{MUT}/🦀️.rs"
    after(agg, "use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_tandem_x;\n", f"use semio_s_artifact_norm_en1991_mut_traffic::{S};\n")
    after(agg, "    ChangeBridgeTandemX(change_bridge_tandem_x::ChangeBridgeTandemX),\n", f"    {P}({S}::{P}),\n")
    after(agg, '    "change-bridge-tandem-x",\n', f'    "{KIND}",\n')
    after(f"{ROOT}/🚪️io/📝️text/🧬️mutations/🦀️.rs", "use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_tandem_x;\n", f"use semio_s_artifact_norm_en1991_mut_traffic::{S};\n")
    tsp = f"{MUT}/🟦️.ts"
    after(tsp, f'import {{ type ChangeBridgeTandemX, parseChangeBridgeTandemX }} from "./{SRC_DIR}/🧬️schema/🟦️.ts";\n', f'import {{ type {P}, parse{P} }} from "./{DIR}/🧬️schema/🟦️.ts";\n')
    after(tsp, "  | { ChangeBridgeTandemX: ChangeBridgeTandemX }\n", f"  | {{ {P}: {P} }}\n")
    after(tsp, "  ChangeBridgeTandemX: parseChangeBridgeTandemX,\n", f"  {P}: parse{P},\n")
    aj = json.loads(rd(f"{MUT}/🔣️.json"))
    for key, v in aj.items():
        if isinstance(v, list) and any("ChangeBridgeTandemX" in json.dumps(x) for x in v) and not any(P in json.dumps(x) for x in v):
            i = next(i for i, x in enumerate(v) if "ChangeBridgeTandemX" in json.dumps(x))
            v.insert(i + 1, json.loads(json.dumps(v[i]).replace("ChangeBridgeTandemX", P).replace("change-bridge-tandem-x", KIND)))
    jdump(f"{MUT}/🔣️.json", aj)
    proto = f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(proto)
    if f"record {KIND} " not in s:
        wr(proto, s.rstrip("\n") + f"\nrecord {KIND} tag={TAG}\nfield payload bytes\n")
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
    prev_kind, prev_dir = "change-bridge-tandem-x", SRC_DIR
    for lst in list(walk(o)):
        if KIND in json.dumps(lst):
            continue
        for i, x in enumerate(lst):
            if x == prev_kind:
                lst.insert(i + 1, KIND)
                break
            if isinstance(x, dict) and x.get("mutationId") == prev_kind:
                nn = copy.deepcopy(x)
                nn.update(mutationId=KIND, sourceMutationDirectoryName=DIR, mutationDirectoryName=DIR)
                lst.insert(i + 1, nn)
                break
            if isinstance(x, dict) and x.get("id") == prev_kind:
                nn = copy.deepcopy(x)
                nn["id"] = KIND
                nn["productionDispatch"].update(operation=KIND, variant=P)
                lst.insert(i + 1, nn)
                break
    jdump(om, o)
    ft = f"{ROOT}/🧪️tests/🏋️mutate-en1991-1/🥒️.feature"
    s = rd(ft)
    if f"| {KIND} " not in s:
        out = []
        for l in s.split("\n"):
            out.append(l)
            if l.strip().startswith(f"| {prev_kind} "):
                out.append(l.replace(prev_dir, DIR).replace(prev_kind, KIND))
        wr(ft, "\n".join(out))
    src, dst = f"{FIX}/{SRC_DIR}/✅apply", f"{FIX}/{DIR}/✅apply"
    before = json.loads(rd(f"{src}/🔣️.before.json"))
    before[F["camel"]] = F["default"]
    aft = copy.deepcopy(before)
    aft[F["camel"]] = F["fix"]
    jdump(f"{dst}/🔣️.before.json", before)
    jdump(f"{dst}/🔣️.after.json", aft)
    jdump(f"{dst}/🔣️.mutation.json", {P: {newkey: F["fix"]}})
    jdump(f"{dst}/🔣️.outcome.json", {"status": "applied"})
    dj = json.loads(rd(f"{src}/🔣️.diff.json"))
    dj["bridgeTandemX"] = None
    dj[F["camel"]] = F["fix"]
    jdump(f"{dst}/🔣️.diff.json", dj)


if F["snake"] not in rd(f"{SCH}/📸️snapshot/🦀️.rs"):
    snapshot_layers()
    sqlite_layers()
    meta_and_tests()
    fixtures()
leaf()
if "--dry" in sys.argv:
    print("would write", len(OVERLAY), "files")
else:
    for p, s in OVERLAY.items():
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write(s)
    print("wrote", len(OVERLAY), "files")
