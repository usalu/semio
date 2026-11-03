#!/usr/bin/env python3
"""Adds crane runway and silo geometry fields after `assumedRailSw2` (derived from en1991-add-rail.py). Original: adds the rail snapshot fields (railAlpha, assumedRailLm71, assumedRailLm71Udl, assumedRailSw0, assumedRailSw2) after
`bridgeTandemX` and their edit leaves through every EN 1991 layer (recipe of `en1991-add-tandem-x.py`). Idempotent. Input script."""
import copy, glob, json, os, re

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = f"{ROOT}/🧬️schema"
MUT = f"{SCH}/🧬️mutations"
FIX = f"{ROOT}/🧪️tests/🧫️fixtures/🧬️mutations"
SQ = f"{ROOT}/🚪️io/🪶️sqlite/📸️snapshot"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SRC_DIR, SRC_KIND = "📍change-bridge-tandem-x", "change-bridge-tandem-x"

FIELDS = [
    dict(snake="crane_x", camel="craneX", en="Crane position on the runway", de="Kranposition auf der Kranbahn", unit="m", disp="m", factor=1, step=0.25, prec=2, default=15.0, lo=0, hi=None, fix=12.5, crate="cranes", emoji="🏗️", group="crane"),
    dict(snake="crane_runway_length", camel="craneRunwayLength", en="Runway length", de="Kranbahnlänge", unit="m", disp="m", factor=1, step=0.5, prec=2, default=30.0, lo=1, hi=None, fix=36.0, crate="cranes", emoji="📏", group="crane"),
    dict(snake="crane_wheel_base", camel="craneWheelBase", en="Crane wheel base", de="Kranradstand", unit="m", disp="m", factor=1, step=0.05, prec=2, default=3.5, lo=0.5, hi=None, fix=4.0, crate="cranes", emoji="🛞", group="crane"),
    dict(snake="silo_diameter", camel="siloDiameter", en="Silo diameter", de="Silodurchmesser", unit="m", disp="m", factor=1, step=0.1, prec=2, default=6.0, lo=0.5, hi=None, fix=7.0, crate="silos", emoji="⭕", group="silo"),
    dict(snake="silo_eccentricity", camel="siloEccentricity", en="Filling eccentricity e_f", de="Befüllungsausmitte e_f", unit="m", disp="m", factor=1, step=0.05, prec=2, default=0.0, lo=0, hi=None, fix=0.4, crate="silos", emoji="🎯", group="silo"),
    dict(snake="silo_hopper_angle_deg", camel="siloHopperAngleDeg", en="Hopper wall angle to horizontal", de="Trichterneigung zur Horizontalen", unit=None, disp="°", factor=1, step=1, prec=1, default=60.0, lo=0, hi=90, fix=45.0, crate="silos", emoji="🔻", group="silo"),
]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def sub(path, old, new, count=1, marker=None):
    s = rd(path)
    if (marker or new) in s:
        return
    assert s.count(old) == count, (path, old[:90], s.count(old))
    wr(path, s.replace(old, new))


def after(path, anchor, add, count=1):
    sub(path, anchor, anchor + add, count, marker=add)


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def num(v):
    return f"{v:_.1f}"


def pascal(snake):
    return "".join(p.capitalize() for p in snake.split("_"))


def kind_of(f):
    return "change-" + f["snake"].replace("_", "-")


def one(prev, f, pidx, prev_default, prev_tag, tag_idx):
    new, nc, pc = f["snake"], f["camel"], prev["camel"]
    ps = prev["snake"]
    unit = f'    #[dsl(unit = "{f["unit"]}")]\n' if f["unit"] else ""
    after(f"{SCH}/📸️snapshot/🦀️.rs", f"    pub {ps}: f64,\n", f"{unit}    #[state(artifact)]\n    pub {new}: f64,\n")
    after(f"{SCH}/📸️snapshot/🦀️.rs", f"            {ps}: {prev_default},\n", f"            {new}: {num(f['default'])},\n".replace("_.0,", ".0,") if False else f"            {new}: {num(f['default'])},\n")
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
    after(f"{ROOT}/📚️examples/🧬️subjects/🦀️.rs", f"    s.{ps} = {prev_default.replace('_', '') if False else prev_default};\n", f"    s.{new} = {num(f['default'])};\n")
    for rel in ["🔣️.json", "📸️snapshot/🔣️.json"]:
        p = f"{SCH}/{rel}"
        after(p, f'    "{pc}",\n', f'    "{nc}",\n')
        blk = f'    "{pc}": {{\n      "type": "number",\n      "format": "double",\n      "x-semio-state": "artifact"\n    }},\n'
        after(p, blk, blk.replace(pc, nc))
    p = f"{SCH}/🔺️diff/🔣️.json"
    blk = f'    "{pc}": {{\n      "anyOf": [\n        {{\n          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1991/snapshot.json#/properties/{pc}"\n        }},\n        {{\n          "type": "null"\n        }}\n      ],\n      "x-semio-state": "artifact"\n    }},\n'
    after(p, blk, blk.replace(pc, nc))
    for rel, line in [("🔗️.graphql", f"  {pc}: Float! @state(class: ARTIFACT)\n"), ("📸️snapshot/🔗️.graphql", f"  {pc}: Float! @state(class: ARTIFACT)\n"), ("🔺️diff/🔗️.graphql", f"  {pc}: Float @state(class: ARTIFACT)\n")]:
        after(f"{SCH}/{rel}", line, line.replace(pc, nc))
    for rel in ["🛰️.proto", "📸️snapshot/🛰️.proto", "🔺️diff/🛰️.proto"]:
        p = f"{SCH}/{rel}"
        s = rd(p)
        if f" {ps if False else new} = " in s:
            continue
        m = re.search(rf"  double {ps} = (\d+);\n", s)
        assert m, (p, ps)
        t = int(m.group(1)) + 1
        s = re.sub(r"(= )(\d+)(;)", lambda mm: f"{mm.group(1)}{int(mm.group(2)) + 1}{mm.group(3)}" if int(mm.group(2)) >= t and "double" not in "" else mm.group(0), s)
        m = re.search(rf"  double {ps} = (\d+);\n", s)
        s = s[: m.end()] + f"  // @state artifact\n  double {new} = {t};\n" + s[m.end():]
        wr(p, s)
    for rel, req in [("🟦️.ts", "normWireRequired(normWireNumber)"), ("📸️snapshot/🟦️.ts", "normWireRequired(normWireNumber)"), ("🔺️diff/🟦️.ts", "normWireDefault(normWireNullable(normWireNumber), () => null)")]:
        p = f"{SCH}/{rel}"
        sub(p, f"{pc}: {req}, ", f"{pc}: {req}, {nc}: {req}, ")
        nullable = " | null" if "diff" in rel else ""
        sub(p, f"  {pc}: number{nullable};\n", f"  {pc}: number{nullable};\n  {nc}: number{nullable};\n")
    p = f"{SQ}/🗄️.sql"
    sub(p, f"{ps} REAL,", f"{ps} REAL, {new} REAL,", marker=f"{new} REAL,")
    sub(p, f"{ps}_ieee754_bits INTEGER NOT NULL, {ps}_ieee754_class TEXT NOT NULL", f"{ps}_ieee754_bits INTEGER NOT NULL, {ps}_ieee754_class TEXT NOT NULL,\n{new}_ieee754_bits INTEGER NOT NULL, {new}_ieee754_class TEXT NOT NULL", marker=f"{new}_ieee754_bits")
    r = f"{SQ}/🦀️.rs"
    sub(r, f"FloatColumn::Binary64({pidx})];", f"FloatColumn::Binary64({pidx}), FloatColumn::Binary64({pidx + 1})];")
    sub(r, f'entities(database, "en1991_document", {pidx + 1},', f'entities(database, "en1991_document", {pidx + 2},')
    sub(r, f"Cell::Real(self.{ps})\n", f"Cell::Real(self.{ps}), Cell::Real(self.{new})\n")
    sub(r, f"{ps}: document.real({pidx})?, ", f"{ps}: document.real({pidx})?, {new}: document.real({pidx + 1})?, ")
    t = f"{SQ}/🟦️.ts"
    sub(t, f'|"{pc}"|', f'|"{pc}"|"{nc}"|')
    sub(t, f"{pc}:Binary64;", f"{pc}:Binary64;{nc}:Binary64;")
    sub(t, f"s.{pc}", f"s.{pc},s.{nc}", marker=f"s.{nc}")
    sub(t, f"{pc}:readBinary64(d,{pidx},DOCUMENT_FLOATS),", f"{pc}:readBinary64(d,{pidx},DOCUMENT_FLOATS),{nc}:readBinary64(d,{pidx + 1},DOCUMENT_FLOATS),")
    sub(t, f"entities(tables[0]!,{pidx + 1},DOCUMENT_FLOATS", f"entities(tables[0]!,{pidx + 2},DOCUMENT_FLOATS")
    s = rd(t)
    m = re.search(rf"const DOCUMENT_FLOATS:readonly Ieee754Column\[\]=\[[0-9,]*?,{pidx}\]", s)
    if m:
        wr(t, s.replace(m.group(0), m.group(0)[:-1] + f",{pidx + 1}]"))
    tt = f"{SQ}/🧪️tests/🟦️.ts"
    sub(tt, f"{pc}:binary64(f.{pc}),", f"{pc}:binary64(f.{pc}),{nc}:binary64(f.{nc}),")
    sub(tt, f"s.{pc}=v;", f"s.{pc}=v;s.{nc}=v;")
    sub(f"{SQ}/🧪️tests/🦀️.rs", f"snapshot.{ps} = value;", f"snapshot.{ps} = value; snapshot.{new} = value;")
    sub(f"{SQ}/🧪️tests/🧫️fixtures/🔣️.json", f'"{pc}": ', f'"{nc}": {f["default"]}, "{pc}": ', marker=f'"{nc}"') if False else None


def fixtures(prev, f):
    pc, nc = prev["camel"], f["camel"]
    for fp in glob.glob(f"{FIX}/*/✅apply/**/🔣️.json", recursive=True):
        s = rd(fp)
        if f'"{nc}"' in s:
            continue
        m = re.search(rf'^(\s*)"{pc}": [^\n]*?,\n', s, re.M)
        if not m:
            continue
        value = "null" if "🔺️diff" in fp else repr(float(f["default"]))
        wr(fp, s[: m.end()] + f'{m.group(1)}"{nc}": {value},\n' + s[m.end():])


def leaf(prev, f, n):
    kind = kind_of(f)
    P = "".join(p.capitalize() for p in kind.split("-"))
    S = kind.replace("-", "_")
    DIR = f["emoji"] + kind
    if os.path.isdir(f"{MUT}/{DIR}"):
        return
    npascal = pascal(f["snake"])
    def conv(s):
        return (s.replace("change-bridge-tandem-x", kind).replace("change_bridge_tandem_x", S).replace("ChangeBridgeTandemX", P).replace("ChangedBridgeTandemX", "Changed" + npascal)
                .replace("new_bridge_tandem_x", "new_" + f["snake"]).replace("newBridgeTandemX", "new" + npascal).replace("bridge_tandem_x", f["snake"]).replace("bridge-tandem-x", kind[len("change-"):])
                .replace("Change tandem position", "Change " + f["en"].lower()).replace("Tandemlage ändern", f["de"] + " ändern").replace("📍", f["emoji"]))
    for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs", "🧬️schema/🟦️.ts"]:
        wr(f"{MUT}/{DIR}/{rel}", conv(rd(f"{MUT}/{SRC_DIR}/{rel}")))
    j = json.loads(rd(f"{MUT}/{SRC_DIR}/🔣️.json"))
    j.update(owner=f"{OWNER}/{DIR}", semanticKind=kind, displayName=P, emoji=f["emoji"], aggregateVariant=P, binaryTag=95 + n)
    jdump(f"{MUT}/{DIR}/🔣️.json", j)
    sj = json.loads(rd(f"{MUT}/{SRC_DIR}/🧬️schema/🔣️.json"))
    sj["$id"] = f"https://json.schemas.assets.semio-tech.com/s/norm/en1991/mutation/{kind}/schema.json"
    sj["title"] = P
    newkey = "new" + npascal
    sj["required"] = [newkey]
    prop = sj["properties"].pop("newBridgeTandemX")
    ui = prop["x-semio-ui"]
    ui.update(label={"en": f["en"], "de": f["de"]}, step=f["step"], precision=f["prec"], group=f["group"])
    ui["unit"] = f["unit"] or "1"
    if f["unit"]:
        ui["displayUnit"], ui["displayFactor"] = f["disp"], f["factor"]
    else:
        ui.pop("displayUnit", None)
        ui.pop("displayFactor", None)
    prop["minimum"] = f["lo"]
    if f["hi"] is not None:
        prop["maximum"] = f["hi"]
    sj["properties"] = {newkey: prop}
    jdump(f"{MUT}/{DIR}/🧬️schema/🔣️.json", sj)
    rng = '{"minimum":%s' % f["lo"] + (',"maximum":%s' % f["hi"] if f["hi"] is not None else "") + "}"
    ts = rd(f"{MUT}/{DIR}/🧬️schema/🟦️.ts")
    ts = re.sub(r'\{"minimum":0\}', rng, ts)
    wr(f"{MUT}/{DIR}/🧬️schema/🟦️.ts", ts)
    prev_kind = "change-assumed-rail-sw2" if n == 0 else kind_of(FIELDS[n - 1])
    prev_crate = "traffic" if n == 0 else FIELDS[n - 1]["crate"]
    prev_S = prev_kind.replace("-", "_")
    prev_P = "".join(p.capitalize() for p in prev_kind.split("-"))
    prev_dir = "🚆change-assumed-rail-sw2" if n == 0 else FIELDS[n - 1]["emoji"] + prev_kind
    mount = f'#[path = "../{DIR}/🦀️.rs"]\npub mod {S};\n'
    sub(f"{MUT}/{'🏗️cranes' if f['crate'] == 'cranes' else '🏭️silos'}/🦀️.rs", "//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts", marker=mount)
    agg = f"{MUT}/🦀️.rs"
    after(agg, f"use semio_s_artifact_norm_en1991_mut_{prev_crate}::{prev_S};\n", f"use semio_s_artifact_norm_en1991_mut_{f['crate']}::{S};\n")
    after(agg, f"    {prev_P}({prev_S}::{prev_P}),\n", f"    {P}({S}::{P}),\n")
    after(agg, f'    "{prev_kind}",\n', f'    "{kind}",\n')
    after(f"{ROOT}/🚪️io/📝️text/🧬️mutations/🦀️.rs", f"use semio_s_artifact_norm_en1991_mut_{prev_crate}::{prev_S};\n", f"use semio_s_artifact_norm_en1991_mut_{f['crate']}::{S};\n")
    tsp = f"{MUT}/🟦️.ts"
    after(tsp, f'import {{ type {prev_P}, parse{prev_P} }} from "./{prev_dir}/🧬️schema/🟦️.ts";\n', f'import {{ type {P}, parse{P} }} from "./{DIR}/🧬️schema/🟦️.ts";\n')
    after(tsp, f"  | {{ {prev_P}: {prev_P} }}\n", f"  | {{ {P}: {P} }}\n")
    after(tsp, f"  {prev_P}: parse{prev_P},\n", f"  {P}: parse{P},\n")
    aj = json.loads(rd(f"{MUT}/🔣️.json"))
    for key, v in aj.items():
        if isinstance(v, list) and any(prev_P in json.dumps(x) for x in v) and not any(P in json.dumps(x) for x in v):
            i = next(i for i, x in enumerate(v) if prev_P in json.dumps(x))
            v.insert(i + 1, json.loads(json.dumps(v[i]).replace(prev_P, P).replace(prev_kind, kind)))
    jdump(f"{MUT}/🔣️.json", aj)
    after(f"{MUT}/🧭️edit-rules/🦀️.rs", f'        SetFieldRule {{ path: "{prev["camel"]}", kind: "{prev_kind}", selectors: &[], value: "new{pascal(prev["snake"])}" }},\n', f'        SetFieldRule {{ path: "{f["camel"]}", kind: "{kind}", selectors: &[], value: "{newkey}" }},\n')
    proto = f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(proto)
    if f"record {kind} " not in s:
        wr(proto, s.rstrip("\n") + f"\nrecord {kind} tag={93 + n}\nfield payload bytes\n")
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
        if kind in json.dumps(lst):
            continue
        for i, x in enumerate(lst):
            if x == prev_kind:
                lst.insert(i + 1, kind)
                break
            if isinstance(x, dict) and x.get("mutationId") == prev_kind:
                nn = copy.deepcopy(x)
                nn.update(mutationId=kind, sourceMutationDirectoryName=DIR, mutationDirectoryName=DIR)
                lst.insert(i + 1, nn)
                break
            if isinstance(x, dict) and x.get("id") == prev_kind:
                nn = copy.deepcopy(x)
                nn["id"] = kind
                nn["productionDispatch"].update(operation=kind, variant=P)
                lst.insert(i + 1, nn)
                break
    jdump(om, o)
    ft = f"{ROOT}/🧪️tests/🏋️mutate-en1991-1/🥒️.feature"
    s = rd(ft)
    if f"| {kind} " not in s:
        out = []
        for l in s.split("\n"):
            out.append(l)
            if l.strip().startswith(f"| {prev_kind} "):
                out.append(l.replace(prev_dir, DIR).replace(prev_kind, kind))
        wr(ft, "\n".join(out))
    src = f"{FIX}/{SRC_DIR}/✅apply"
    dst = f"{FIX}/{DIR}/✅apply"
    before = json.loads(rd(f"{src}/🔣️.before.json"))
    for g in FIELDS:
        before[g["camel"]] = float(g["default"])
    aft = copy.deepcopy(before)
    aft[f["camel"]] = f["fix"]
    jdump(f"{dst}/🔣️.before.json", before)
    jdump(f"{dst}/🔣️.after.json", aft)
    jdump(f"{dst}/🔣️.mutation.json", {P: {newkey: f["fix"]}})
    jdump(f"{dst}/🔣️.outcome.json", {"status": "applied"})
    dj = json.loads(rd(f"{src}/🔣️.diff.json"))
    dj["bridgeTandemX"] = None
    for g in FIELDS:
        dj[g["camel"]] = None
    dj[f["camel"]] = f["fix"]
    jdump(f"{dst}/🔣️.diff.json", dj)


def reorder(path):
    return None


prev = dict(snake="assumed_rail_sw2", camel="assumedRailSw2")
prev_default = "150_000.0"
pidx, tag = 71, 50
for n, f in enumerate(FIELDS):
    if f"pub {f['snake']}: f64" not in rd(f"{SCH}/📸️snapshot/🦀️.rs"):
        one(prev, f, pidx, prev_default, None, None)
        fixtures(prev, f)
    prev_default = num(f["default"])
    prev = f
    pidx += 1
for n, f in enumerate(FIELDS):
    leaf(FIELDS[n - 1] if n else dict(snake="assumed_rail_sw2", camel="assumedRailSw2"), f, n)
print("done")
