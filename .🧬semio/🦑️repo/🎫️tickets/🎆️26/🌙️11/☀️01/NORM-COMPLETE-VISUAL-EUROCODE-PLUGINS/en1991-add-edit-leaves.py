#!/usr/bin/env python3
"""Adds the seven EN 1991 field-edit mutation leaves requested by the visual agent (roof pitch, parapet height,
drift obstruction height, floor area, layer thickness, wind face z, loaded area) by cloning the committed
`change-<x>-assumed-<y>` leaves. Idempotent: leaves that exist are skipped. Input script, kept in the ticket."""
import copy, json, os, re, shutil, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = f"{ROOT}/🧬️schema/🧬️mutations"
FIX = f"{ROOT}/🧪️tests/🧫️fixtures/🧬️mutations"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

TEMPLATES = {
    "roofs": dict(dir="🌨️change-roof-assumed-sk", kind="change-roof-assumed-sk", field="assumed_sk", camel="assumedSk", noun_en="Roof", noun_de="Dach", group="❄️snow", crate="semio_s_artifact_norm_en1991_mut_snow", unit_ctx="roofs", coll_json="roofs", pre="Roof"),
    "floors": dict(dir="🏢change-floor-assumed-qk", kind="change-floor-assumed-qk", field="assumed_qk", camel="assumedQk", noun_en="Floor", noun_de="Decke", group="🏢️gravity", crate="semio_s_artifact_norm_en1991_mut_gravity", unit_ctx="floors", coll_json="floors", pre="Floor"),
    "self": dict(dir="🚧change-self-weight-assumed-gk", kind="change-self-weight-assumed-gk", field="assumed_gk", camel="assumedGk", noun_en="Layer", noun_de="Schicht", group="🏢️gravity", crate="semio_s_artifact_norm_en1991_mut_gravity", unit_ctx="selfWeightElements", coll_json="selfWeightElements", pre="SelfWeight"),
    "faces": dict(dir="🛡️change-wind-face-assumed-wp", kind="change-wind-face-assumed-wp", field="assumed_wp", camel="assumedWp", noun_en="Wind face", noun_de="Windfläche", group="🪁️wind-faces", crate="semio_s_artifact_norm_en1991_mut_wind_faces", unit_ctx="windFaces", coll_json="windFaces", pre="WindFace"),
}

# emoji, template, kind suffix, field snake, field camel, en label, de label, unit, display, factor, step, precision, min, max, fixture value
SPECS = [
    ("📐", "roofs", "change-roof-pitch-deg", "pitch_deg", "pitchDeg", "Roof pitch", "Dachneigung", "deg", "°", 1, 1, 1, 0, 90, 20.0),
    ("📏", "roofs", "change-roof-parapet-height", "parapet_height", "parapetHeight", "Parapet height", "Attikahöhe", "m", "m", 1, 0.1, 2, 0, None, 0.5),
    ("🧱", "roofs", "change-roof-drift-obstruction-height", "drift_obstruction_height", "driftObstructionHeight", "Drift obstruction height", "Verwehungs-Hindernishöhe", "m", "m", 1, 0.1, 2, 0, None, 1.0),
    ("🗺️", "floors", "change-floor-area", "area", "area", "Floor area", "Deckenfläche", "m2", "m²", 1, 1, 1, 0, None, 300.0),
    ("🥞", "self", "change-self-weight-thickness", "thickness", "thickness", "Layer thickness", "Schichtdicke", "m", "m", 1, 0.01, 3, 0, None, 0.25),
    ("↕️", "faces", "change-wind-face-z", "z", "z", "Reference height z", "Bezugshöhe z", "m", "m", 1, 0.5, 2, 0, None, 10.0),
    ("🔲", "faces", "change-wind-face-loaded-area", "loaded_area", "loadedArea", "Loaded area A", "Belastete Fläche A", "m2", "m²", 1, 1, 1, 0, None, 5.0),
]


def pascal(kind):
    return "".join(p.capitalize() for p in kind.split("-"))


def snake(kind):
    return kind.replace("-", "_")


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def jload(p):
    return json.loads(rd(p))


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


ALL_NAMES = []


def insert_after(path, anchor, lines):
    s = rd(path)
    if lines.strip().splitlines()[0] in s:
        return
    i = s.index(anchor)
    j = s.index("\n", i) + 1
    while True:
        k = s.find("\n", j)
        nxt = s[j:k] if k >= 0 else s[j:]
        if nxt and any(n in nxt for n in ALL_NAMES):
            j = k + 1
        else:
            break
    wr(path, s[:j] + lines + s[j:])


def main():
    for _e, _t, k, *_r in SPECS:
        ALL_NAMES.extend([k, snake(k), pascal(k)])
    mut_rs = f"{MUT}/🦀️.rs"
    proto = f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    tag = 80
    leaf_tag = 82
    for emoji, tkey, kind, field, camel, en, de, unit, disp, factor, step, prec, mn, mx, fixture_value in SPECS:
        t = TEMPLATES[tkey]
        P, S = pascal(kind), snake(kind)
        dname = f"{emoji}{kind}"
        if os.path.isdir(f"{MUT}/{dname}"):
            tag += 1
            leaf_tag += 1
            continue
        tp, ts = pascal(t["kind"]), snake(t["kind"])
        new_field = f"new_{field}"
        new_camel = "new" + camel[0].upper() + camel[1:]
        tcamel_new = "new" + t["camel"][0].upper() + t["camel"][1:]
        # leaf rust, diff, inverse
        for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs"]:
            s = rd(f"{MUT}/{t['dir']}/{rel}")
            s = s.replace("Changed" + tp[len("Change"):], "Changed" + P[len("Change"):]).replace(tp, P).replace(ts, S).replace(t["kind"], kind)
            s = s.replace("new_" + t["field"], new_field).replace(f"assumed_{t['field'].split('_',1)[1]}" if False else t["field"], field)
            s = re.sub(r'entity: "[^"]*"', f'entity: "{kind[len("change-"):]}"', s)
            s = re.sub(r'LocalizedLabel::native\("[^"]*", "[^"]*"\)', f'LocalizedLabel::native("Change {en.lower()}", "{de} ändern")', s)
            wr(f"{MUT}/{dname}/{rel}", s)
        # leaf json
        j = jload(f"{MUT}/{t['dir']}/🔣️.json")
        j.update(owner=f"{OWNER}/{dname}", semanticKind=kind, displayName=P, emoji=emoji, aggregateVariant=P, binaryTag=leaf_tag)
        jdump(f"{MUT}/{dname}/🔣️.json", j)
        # payload schema
        sj = jload(f"{MUT}/{t['dir']}/🧬️schema/🔣️.json")
        props = sj["properties"]
        old_val_key = [k for k in props if k != "index"][0]
        val = props.pop(old_val_key)
        val["x-semio-ui"].update(label={"en": en, "de": de}, unit=unit, displayUnit=disp, displayFactor=factor, step=step, precision=prec, group=t["unit_ctx"])
        val["x-semio-ui"]["description"] = {"en": "Value of the design; the checks and the 3D scene derive their actions from it.", "de": "Wert des Entwurfs; Nachweise und 3D-Szene leiten ihre Einwirkungen daraus ab."}
        val["minimum"] = mn
        if mx is not None:
            val["maximum"] = mx
        props[new_camel] = val
        props["index"]["x-semio-ui"]["label"] = {"en": t["noun_en"], "de": t["noun_de"]}
        sj["properties"] = {"index": props["index"], new_camel: props[new_camel]}
        sj["required"] = ["index", new_camel]
        sj["title"] = P
        sj["$id"] = f"https://json.schemas.assets.semio-tech.com/s/norm/en1991/mutation/{kind}/schema.json"
        jdump(f"{MUT}/{dname}/🧬️schema/🔣️.json", sj)
        rng = f'{{"minimum":{mn}' + (f',"maximum":{mx}' if mx is not None else "") + "}"
        wr(f"{MUT}/{dname}/🧬️schema/🟦️.ts", f'''/** {emoji} `{kind}` wire twin: the leaf payload `{P}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired }} from "@semio-tech/norm-js/registry/contract";

export interface {P} {{
  index: number;
  {new_camel}: number;
}}

export const parse{P}: NormWireReader<{P}> = normWireObject<{P}>({{ index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})), {new_camel}: normWireRequired(normWireRange(normWireNumber, {rng})) }});
''')
        # group mount
        g = f"{MUT}/{t['group']}/🦀️.rs"
        s = rd(g)
        mount = f'#[path = "../{dname}/🦀️.rs"]\npub mod {S};\n'
        if mount not in s:
            s = s.replace("//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts")
            wr(g, s)
        # aggregate rust
        insert_after(mut_rs, "use semio_s_artifact_norm_en1991_mut_general::remove_accidental_cases;", f"use {t['crate']}::{S};\n")
        insert_after(mut_rs, "    RemoveAccidentalCases(remove_accidental_cases::RemoveAccidentalCases),", f"    {P}({S}::{P}),\n")
        insert_after(mut_rs, '    "remove-accidental-cases",', f'    "{kind}",\n')
        # aggregate json schema
        aj = jload(f"{MUT}/🔣️.json")
        refs = json.dumps(aj)
        if P not in refs:
            for key in aj:
                if isinstance(aj[key], list) and any("RemoveAccidentalCases" in json.dumps(x) for x in aj[key]):
                    aj[key].append({"type": "object", "additionalProperties": False, "required": [P], "properties": {P: {"$ref": f"https://json.schemas.assets.semio-tech.com/s/norm/en1991/mutation/{kind}/schema.json"}}})
                    break
            else:
                raise SystemExit("aggregate oneOf not found")
            jdump(f"{MUT}/🔣️.json", aj)
        # aggregate ts
        ts_path = f"{MUT}/🟦️.ts"
        s = rd(ts_path)
        if P not in s:
            s = s.replace('import { parseRemoveAccidentalCases, type RemoveAccidentalCases } from "./➖️remove-accidental-cases/🧬️schema/🟦️.ts";', f'import {{ parseRemoveAccidentalCases, type RemoveAccidentalCases }} from "./➖️remove-accidental-cases/🧬️schema/🟦️.ts";\nimport {{ type {P}, parse{P} }} from "./{dname}/🧬️schema/🟦️.ts";')
            m = list(re.finditer(r"^  \| \{ [A-Za-z0-9]+: [A-Za-z0-9]+ \};$", s, re.M))[-1]
            s = s[: m.end() - 1] + f"\n  | {{ {P}: {P} }};" + s[m.end():]
            m = list(re.finditer(r"^  [A-Za-z0-9]+: parse[A-Za-z0-9]+,$", s, re.M))[-1]
            s = s[: m.end()] + f"\n  {P}: parse{P}," + s[m.end():]
            wr(ts_path, s)
        # io text
        insert_after(f"{ROOT}/🚪️io/📝️text/🧬️mutations/🦀️.rs", "use semio_s_artifact_norm_en1991_mut_general::remove_accidental_cases;", f"use {t['crate']}::{S};\n")
        # binary protocol
        s = rd(proto)
        if f"record {kind} " not in s:
            wr(proto, s.rstrip("\n") + f"\nrecord {kind} tag={tag}\nfield payload bytes\n")
        tag += 1
        # edit rule
        er = f"{MUT}/🧭️edit-rules/🦀️.rs"
        path = f'{t["coll_json"]}[].{camel}'
        anchors = {"roofs": 'path: "roofs[].assumedSk"', "floors": 'path: "floors[].assumedQk"', "self": 'path: "selfWeightElements[].assumedGk"', "faces": 'path: "windFaces[].assumedWp"'}
        s = rd(er)
        if f'path: "{path}"' not in s:
            i = s.index(anchors[tkey])
            i = s.rfind("\n", 0, i) + 1
            j = s.index("\n", i) + 1
            s = s[:j] + f'        SetFieldRule {{ path: "{path}", kind: "{kind}", selectors: &[SelectorKey::Index("index")], value: "{new_camel}" }},\n' + s[j:]
            wr(er, s)
        # oracle manifest
        om = f"{ROOT}/🔮️oracles/🔣️.json"
        o = jload(om)
        txt = json.dumps(o)
        if f'"{kind}"' not in txt:
            def walk(node):
                if isinstance(node, dict):
                    for k, v in node.items():
                        yield node, k, v
                        yield from walk(v)
                elif isinstance(node, list):
                    for v in node:
                        yield from walk(v)
            for parent, k, v in list(walk(o)):
                if isinstance(v, list) and "remove-accidental-cases" in v and all(isinstance(x, str) for x in v):
                    v.append(kind)
                elif isinstance(v, list) and v and isinstance(v[0], dict):
                    for x in list(v):
                        if x.get("mutationId") == "remove-accidental-cases":
                            n = copy.deepcopy(x)
                            n.update(mutationId=kind, sourceMutationDirectoryName=dname, mutationDirectoryName=dname)
                            v.append(n)
                        elif x.get("id") == "remove-accidental-cases":
                            n = copy.deepcopy(x)
                            n["id"] = kind
                            n["productionDispatch"].update(operation=kind, variant=P)
                            v.append(n)
            jdump(om, o)
        # feature
        ft = f"{ROOT}/🧪️tests/🏋️mutate-en1991-1/🥒️.feature"
        s = rd(ft)
        row_old = [l for l in s.splitlines() if l.strip().startswith("| remove-accidental-cases ")]
        if f"| {kind} " not in s:
            lines = s.split("\n")
            out = []
            for idx, l in enumerate(lines):
                out.append(l)
                nxt = lines[idx + 1] if idx + 1 < len(lines) else ""
                if l in row_old or (any(n in l for n in ALL_NAMES) and l.strip().startswith("|")):
                    if not (any(n in nxt for n in ALL_NAMES) and nxt.strip().startswith("|")):
                        out.append(row_old[0].replace("remove-accidental-cases", kind, 1).replace("➖️remove-accidental-cases", dname))
            wr(ft, "\n".join(out))
        # fixtures
        src = f"{FIX}/{t['dir']}/✅apply"
        dst = f"{FIX}/{dname}/✅apply"
        before = jload(f"{src}/🔣️.before.json")
        after = copy.deepcopy(before)
        after[t["coll_json"]][0][camel] = fixture_value
        jdump(f"{dst}/🔣️.before.json", before)
        jdump(f"{dst}/🔣️.after.json", after)
        jdump(f"{dst}/🔣️.mutation.json", {P: {"index": 0, new_camel: fixture_value}})
        jdump(f"{dst}/🔣️.outcome.json", {"status": "applied"})
        diff = jload(f"{src}/🔣️.diff.json")
        for patch in diff[t["coll_json"]]["modified"]:
            for k in patch["patch"]:
                patch["patch"][k] = None
            patch["patch"][camel] = fixture_value
        jdump(f"{dst}/🔣️.diff.json", diff)
        leaf_tag += 1
        print("added", kind)


main()
