#!/usr/bin/env python3
"""Splits the EN 1994 action row type: beams keep `CharacteristicAction` (no `yM`), slabs get `SlabAction` (with `yM`, the position across the strip), through every schema layer, and removes the beam `change-beam-action-y-m` kind (dir, mount, enum variant, KINDS, json/ts twins, protocol tag, edit rule, testkit, harness, oracle manifest, fixtures).
Also repairs the SQLite admission (it listed only four numeric action cells for six float columns) and strips `yM` from every beam action object of the committed JSON fixtures.
All edits are computed first and written only when every assertion holds. Usage: python3 -I en1994-split-slab-action.py [--dry]
"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, os, re, shutil, sys

SEMIO = "/home/deck/semio"
ANY = SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
DRY = "--dry" in sys.argv
SNAP, DIFF, MUT = ANY + "/🧬️schema/📸️snapshot", ANY + "/🧬️schema/🔺️diff", ANY + "/🧬️schema/🧬️mutations"
SQL = ANY + "/🚪️io/🪶️sqlite/📸️snapshot"
KIND = "change-beam-action-y-m"
PENDING = {}
BEAM_OWNERS = ("beams", "beam", "InsertBeam", "InsertBeamAction")
SLAB_OWNERS = ("slabs", "slab", "InsertSlab", "InsertSlabAction", "ChangeSlabActionYM")


def rd(path):
    return PENDING[path] if path in PENDING else open(path, encoding="utf-8").read()


def put(path, text):
    PENDING[path] = text


def edit(path, fn):
    text = rd(path)
    new = fn(text)
    if new != text:
        put(path, new)
        print("  edit", path.replace(ANY, "…"))


def once(text, pattern, repl, flags=0, count=1):
    found = len(re.findall(pattern, text, flags))
    assert found == count, (pattern[:120], "expected", count, "found", found)
    return re.sub(pattern, repl, text, flags=flags)


def lit(text, old, new, count=1):
    assert text.count(old) == count, (old[:120], "expected", count, "found", text.count(old))
    return text.replace(old, new)


def jedit(path, fn):
    text = rd(path)
    data = json.loads(text)
    assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == text, ("not canonical", path)
    fn(data)
    new = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
    if new != text:
        put(path, new)
        print("  edit", path.replace(ANY, "…"))


def snapshot_rs(text):
    if "pub struct SlabAction" in text:
        return text
    field = "    /// 📍️ Position of a slab point force across the strip width [m] (0 = strip axis).\n    #[dsl(unit = \"m\")]\n    pub y_m: f64,\n}\n"
    head = text.index("/// ")
    start = text.rindex("/// 🏋", 0, text.index("pub struct CharacteristicAction")) if "/// 🏋" in text[:text.index("pub struct CharacteristicAction")] else None
    block_start = text.rindex("\n\n", 0, text.index("pub struct CharacteristicAction")) + 2
    block_end = text.index(field, block_start) + len(field)
    block = text[block_start:block_end]
    beam = lit(block, field, "}\n")
    slab = block.replace("pub struct CharacteristicAction", "pub struct SlabAction")
    slab = lit(slab, "/// ", "/// 🧱️ Characteristic action of a composite slab strip: the beam action row plus the position `y_m` of a point force across the strip. ", 1) if False else slab
    slab_doc = "/// 🧱️ Characteristic action of one composite slab strip (EN 1990 combination of effects): the fields of [`CharacteristicAction`] plus the position `y_m` of a point force across the strip width.\n"
    slab_derive_at = slab.index("#[derive(")
    slab = slab_doc + slab[slab_derive_at:]
    x_doc_old = "/// 📍️ Position of a concentrated force along the member span [m] from its start (0 = mid-span)."
    assert x_doc_old in slab
    text = text[:block_start] + beam + "\n" + slab + text[block_end:]
    text = once(text, r"delta_tau_k_pa: 0\.0, x_m: 0\.0, y_m: 0\.0,", "delta_tau_k_pa: 0.0, x_m: 0.0,", count=5)
    text = lit(text, "    pub actions: Vec<CharacteristicAction>,\n}\n\n/// 📍️ Persisted pose", "    pub actions: Vec<SlabAction>,\n}\n\n/// 📍️ Persisted pose")
    text = lit(text, "                CharacteristicAction::permanent_area(\"G-slab\", \"composite\", \"self\", 3.5e3),\n                CharacteristicAction::imposed_area(\"Q-slab\", \"composite\", \"B\", 2.0e3),", "                SlabAction::permanent_area(\"G-slab\", \"composite\", \"self\", 3.5e3),\n                SlabAction::imposed_area(\"Q-slab\", \"composite\", \"B\", 2.0e3),")
    impl = '''impl SlabAction {
    /// 📐️ Permanent area load of the strip.
    pub fn permanent_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self::from(&CharacteristicAction::permanent_area(id, stage, category, q_area_pa))
    }
    pub fn imposed_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self::from(&CharacteristicAction::imposed_area(id, stage, category, q_area_pa))
    }
}

impl From<&CharacteristicAction> for SlabAction {
    /// 🔁️ The strip action with the same loads and the point force on the strip axis.
    fn from(a: &CharacteristicAction) -> Self {
        Self { id: a.id.clone(), kind: a.kind.clone(), category: a.category.clone(), stage: a.stage.clone(), q_area_pa: a.q_area_pa, f_k_n: a.f_k_n, delta_sigma_k_pa: a.delta_sigma_k_pa, delta_tau_k_pa: a.delta_tau_k_pa, x_m: a.x_m, y_m: 0.0 }
    }
}

impl From<&SlabAction> for CharacteristicAction {
    /// 🔁️ The statics view of a strip action: the beam action row without the transverse position.
    fn from(a: &SlabAction) -> Self {
        Self { id: a.id.clone(), kind: a.kind.clone(), category: a.category.clone(), stage: a.stage.clone(), q_area_pa: a.q_area_pa, f_k_n: a.f_k_n, delta_sigma_k_pa: a.delta_sigma_k_pa, delta_tau_k_pa: a.delta_tau_k_pa, x_m: a.x_m }
    }
}

impl CompositeSlab {
    /// 🧮️ The actions of the strip as beam action rows for the shared EN 1990 statics.
    pub fn statics_actions(&self) -> Vec<CharacteristicAction> {
        self.actions.iter().map(CharacteristicAction::from).collect()
    }
}

'''
    return lit(text, "impl ColumnAction {\n    /// 🏛️ Column characteristic N_k", impl + "impl ColumnAction {\n    /// 🏛️ Column characteristic N_k")


def strip_ui(props):
    return props


def schema_json(data):
    defs = data["$defs"]
    if "SlabAction" in defs:
        return
    slab = json.loads(json.dumps(defs["CharacteristicAction"]))
    slab["title"] = "SlabAction"
    beam = defs["CharacteristicAction"]
    beam["properties"].pop("yM")
    beam["required"].remove("yM")
    rebuilt = {}
    for key, value in defs.items():
        rebuilt[key] = value
        if key == "CharacteristicAction":
            rebuilt["SlabAction"] = slab
    data["$defs"] = rebuilt
    ref = rebuilt["CompositeSlab"]["properties"]["actions"]["items"]
    assert ref == {"$ref": "#/$defs/CharacteristicAction"}
    ref["$ref"] = "#/$defs/SlabAction"


def ts_snapshot(text):
    if "export interface SlabAction" in text:
        return text
    iface = re.search(r"export interface CharacteristicAction \{\n(.*?)\n\}\n", text, re.S)
    body = iface.group(1)
    assert body.endswith("  yM: number;")
    text = text.replace(iface.group(0), "export interface CharacteristicAction {\n" + body.replace("\n  yM: number;", "") + "\n}\n\nexport interface SlabAction {\n" + body + "\n}\n", 1)
    parse = re.search(r"export const parseCharacteristicAction: [^\n]*\n", text)
    line = parse.group(0)
    assert ", yM: normWireRequired(normWireNumber) });" in line
    slab_line = line.replace("parseCharacteristicAction", "parseSlabAction").replace("NormWireReader<CharacteristicAction>", "NormWireReader<SlabAction>").replace("normWireObject<CharacteristicAction>", "normWireObject<SlabAction>")
    text = text.replace(line, line.replace(", yM: normWireRequired(normWireNumber) });", " });") + slab_line, 1)
    comp = re.search(r"export interface CompositeSlab \{.*?\n\}\n", text, re.S).group(0)
    text = text.replace(comp, lit(comp, "  actions: CharacteristicAction[];", "  actions: SlabAction[];"), 1)
    row = re.search(r"export const parseCompositeSlab: [^\n]*\n", text).group(0)
    return text.replace(row, lit(row, "parseCharacteristicAction", "parseSlabAction"), 1)


def graphql_snapshot(text):
    if "type SlabAction" in text:
        return text
    block = re.search(r"type CharacteristicAction \{.*?\n\}\n", text, re.S).group(0)
    slab = block.replace("type CharacteristicAction", "type SlabAction")
    text = text.replace(block, lit(block, "  yM: Float!\n", "") + slab, 1)
    comp = re.search(r"type CompositeSlab \{.*?\n\}\n", text, re.S).group(0)
    return text.replace(comp, lit(comp, "[CharacteristicAction!]!", "[SlabAction!]!"), 1)


def proto_snapshot(text):
    if "message SlabAction" in text:
        return text
    block = re.search(r"message CharacteristicAction \{.*?\n\}\n", text, re.S).group(0)
    slab = block.replace("message CharacteristicAction", "message SlabAction")
    text = text.replace(block, lit(block, "  double y_m = 12;\n", "") + slab, 1)
    comp = re.search(r"message CompositeSlab \{.*?\n\}\n", text, re.S).group(0)
    return text.replace(comp, lit(comp, "repeated CharacteristicAction", "repeated SlabAction"), 1)


def diff_json(text):
    lines = text.split("\n")
    out = []
    for line in lines:
        if line.lstrip().startswith('"En1994BeamsActionsPatch"'):
            line = lit(line, ',"xM","yM"]', ',"xM"]')
            line = once(line, r',"yM":\{"anyOf":\[\{"\$ref":"[^"]*CharacteristicAction/properties/yM"\},\{"type":"null"\}\]\}', "")
        if line.lstrip().startswith('"En1994SlabsActionsPatch"') or line.lstrip().startswith('"En1994SlabsActionsInserted"'):
            line = line.replace("snapshot.json#/$defs/CharacteristicAction", "snapshot.json#/$defs/SlabAction")
            assert "CharacteristicAction" not in line
        out.append(line)
    return "\n".join(out)


def diff_ts(text):
    if "type SlabAction" in text:
        return text
    text = lit(text, "type CharacteristicAction, type ColumnAction,", "type CharacteristicAction, type ColumnAction,", 1)
    text = once(text, r"type Placement, type SteelSection, parseAnnexChoice, parseCharacteristicAction,", "type Placement, type SlabAction, type SteelSection, parseAnnexChoice, parseCharacteristicAction,")
    text = once(text, r"parsePlacement, parseSteelSection \}", "parsePlacement, parseSlabAction, parseSteelSection }")
    beams = re.search(r"export interface En1994BeamsActionsPatch \{.*?\n\}\n", text, re.S).group(0)
    text = text.replace(beams, lit(beams, "  yM: number | null;\n", ""), 1)
    ins = re.search(r"export interface En1994SlabsActionsInserted \{.*?\n\}\n", text, re.S).group(0)
    text = text.replace(ins, lit(ins, "row: CharacteristicAction;", "row: SlabAction;"), 1)
    line = re.search(r"export const parseEn1994BeamsActionsPatch: [^\n]*\n", text).group(0)
    text = text.replace(line, lit(line, ", yM: normWireRequired(normWireNullable(normWireNumber)) });", " });"), 1)
    line = re.search(r"export const parseEn1994SlabsActionsInserted: [^\n]*\n", text).group(0)
    return text.replace(line, lit(line, "parseCharacteristicAction", "parseSlabAction"), 1)


def diff_graphql(text):
    beams = re.search(r"type En1994BeamsActionsPatch \{.*?\n\}\n", text, re.S).group(0)
    new = lit(beams, "  yM: Float\n", "")
    text = text.replace(beams, new, 1)
    ins = re.search(r"type En1994SlabsActionsInserted \{.*?\n\}\n", text, re.S).group(0)
    return text.replace(ins, lit(ins, "CharacteristicAction!", "SlabAction!"), 1)


def diff_proto(text):
    beams = re.search(r"message En1994BeamsActionsPatch \{.*?\n\}\n", text, re.S).group(0)
    return text.replace(beams, lit(beams, "  optional double y_m = 10;\n", ""), 1)


def diff_rs_beam(text):
    return lit(text, "x_m: f64, y_m: f64 }", "x_m: f64 }")


def diff_rs_slab(text):
    text = text.replace("semio_s_artifact_norm_en1994_model::CharacteristicAction", "semio_s_artifact_norm_en1994_model::SlabAction")
    return text


def diff_root_rs(text):
    text = lit(text, "En1994SlabsActionKey for CharacteristicAction;", "En1994SlabsActionKey for SlabAction;")
    return lit(text, "use semio_s_artifact_norm_en1994_model::En1994Snapshot;", "use semio_s_artifact_norm_en1994_model::En1994Snapshot;")


def sql_ddl(text):
    lines = text.split("\n")
    n = 0
    for i, line in enumerate(lines):
        if line.startswith("CREATE TABLE en1994_beam_action "):
            line = lit(line, "x_m REAL,y_m REAL,", "x_m REAL,")
            line = lit(line, ",y_m_ieee754_bits INTEGER NOT NULL,y_m_ieee754_class TEXT NOT NULL", "")
            lines[i] = line
            n += 1
    assert n == 1
    return "\n".join(lines)


def sqlite_rs(text):
    text = lit(text, "const ACTION:&[FloatColumn]=&[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12)];", "const BEAM_ACTION:&[FloatColumn]=&[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11)];\nconst SLAB_ACTION:&[FloatColumn]=&[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12)];")
    w = re.search(r"fn actions\(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&\[CharacteristicAction\]\)->Result<\(\), ValueError>\{[^\n]*\n", text).group(0)
    beam_w = w.replace("fn actions(", "fn beam_actions(").replace(",Cell::Real(value.x_m),Cell::Real(value.y_m)],ACTION)", ",Cell::Real(value.x_m)],BEAM_ACTION)")
    slab_w = w.replace("fn actions(", "fn slab_actions(").replace("&[CharacteristicAction]", "&[SlabAction]").replace("],ACTION)", "],SLAB_ACTION)")
    assert "BEAM_ACTION" in beam_w and "SLAB_ACTION" in slab_w
    text = text.replace(w, beam_w + slab_w, 1)
    r = re.search(r"fn read_actions\(rows:[^\n]*\n", text).group(0)
    beam_r = r.replace("fn read_actions(", "fn read_beam_actions(").replace(",x_m:row.real(11)?,y_m:row.real(12)?}", ",x_m:row.real(11)?}")
    slab_r = r.replace("fn read_actions(", "fn read_slab_actions(").replace("Vec<CharacteristicAction>", "Vec<SlabAction>").replace("result.push(CharacteristicAction{", "result.push(SlabAction{")
    assert "y_m:row.real(12)" not in beam_r and "y_m:row.real(12)" in slab_r
    text = text.replace(r, beam_r + slab_r, 1)
    b = re.search(r"fn bound_actions\(bound:[^\n]*\n", text).group(0)
    nb = lit(b, "fn bound_actions(bound:&mut NativeEncodingBound<'_,'_>,values:&[CharacteristicAction])->Result<(), ValueError>{for value in values{bound.add(16384)?;for text in [&value.id,&value.kind,&value.category,&value.stage]{", "fn bound_actions<'a>(bound:&mut NativeEncodingBound<'_,'_>,values:impl Iterator<Item=[&'a String;4]>)->Result<(), ValueError>{for value in values{bound.add(16384)?;for text in value{")
    text = text.replace(b, nb, 1)
    text = lit(text, 'actions(out,"en1994_beam_action",parent,&b.actions)?', 'beam_actions(out,"en1994_beam_action",parent,&b.actions)?')
    text = lit(text, 'actions(out,"en1994_slab_action",parent,&s.actions)?', 'slab_actions(out,"en1994_slab_action",parent,&s.actions)?')
    text = lit(text, "read_actions(siblings(&beam_actions,b.rowid)", "read_beam_actions(siblings(&beam_actions,b.rowid)")
    text = lit(text, "read_actions(siblings(&slab_actions,s.rowid)", "read_slab_actions(siblings(&slab_actions,s.rowid)")
    text = lit(text, 'entities(database,"en1994_beam_action",13,ACTION,control)', 'entities(database,"en1994_beam_action",12,BEAM_ACTION,control)')
    text = lit(text, 'entities(database,"en1994_slab_action",13,ACTION,control)', 'entities(database,"en1994_slab_action",13,SLAB_ACTION,control)')
    text = lit(text, "bound_actions(&mut bound,&beam.actions)?", "bound_actions(&mut bound,beam.actions.iter().map(|a|[&a.id,&a.kind,&a.category,&a.stage]))?")
    text = lit(text, "bound_actions(&mut bound,&slab.actions)?", "bound_actions(&mut bound,slab.actions.iter().map(|a|[&a.id,&a.kind,&a.category,&a.stage]))?")
    text = lit(text, "HeadedStuds, CharacteristicAction, ColumnAction}", "HeadedStuds, CharacteristicAction, SlabAction, ColumnAction}")
    return text


def sqlite_ts(text):
    text = lit(text, "HeadedStuds,CharacteristicAction,ColumnAction}", "HeadedStuds,CharacteristicAction,SlabAction,ColumnAction}")
    text = lit(text, 'export type En1994SqliteAction=Exact<CharacteristicAction,"qAreaPa"|"fKN"|"deltaSigmaKPa"|"deltaTauKPa"|"xM"|"yM">;', 'export type En1994SqliteBeamAction=Exact<CharacteristicAction,"qAreaPa"|"fKN"|"deltaSigmaKPa"|"deltaTauKPa"|"xM">;\nexport type En1994SqliteSlabAction=Exact<SlabAction,"qAreaPa"|"fKN"|"deltaSigmaKPa"|"deltaTauKPa"|"xM"|"yM">;')
    w = re.search(r"async function actions\([^\n]*\n", text).group(0)
    beam_w = w.replace("function actions(", "function beamActions(").replace("En1994SqliteAction[]", "En1994SqliteBeamAction[]").replace("value.xM,value.yM],ACTION", "value.xM],BEAM_ACTION")
    slab_w = w.replace("function actions(", "function slabActions(").replace("En1994SqliteAction[]", "En1994SqliteSlabAction[]").replace("],ACTION", "],SLAB_ACTION")
    text = text.replace(w, beam_w + slab_w, 1)
    r = re.search(r"async function readActions\([^\n]*\n", text).group(0)
    beam_r = r.replace("function readActions(", "function readBeamActions(").replace("En1994SqliteAction", "En1994SqliteBeamAction").replace(",yM:readBinary64(row,12,ACTION)", "").replace("ACTION)", "BEAM_ACTION)")
    slab_r = r.replace("function readActions(", "function readSlabActions(").replace("En1994SqliteAction", "En1994SqliteSlabAction").replace("ACTION)", "SLAB_ACTION)")
    assert "BEAM_ACTION" in beam_r and "SLAB_ACTION" in slab_r and "yM" not in beam_r
    text = text.replace(r, beam_r + slab_r, 1)
    text = lit(text, "const ACTION:readonly Ieee754Column[]=[7,8,9,10,11,12].map(index=>({index,width:64}));", "const BEAM_ACTION:readonly Ieee754Column[]=[7,8,9,10,11].map(index=>({index,width:64}));\nconst SLAB_ACTION:readonly Ieee754Column[]=[7,8,9,10,11,12].map(index=>({index,width:64}));")
    text = lit(text, "await actions(out,", "await beamActions(out,", count=text.count("await actions(out,"))
    return text


def admission(text):
    old = "fn actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7)])?,ACTION)?;}Ok(())}"
    new = "fn beam_actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,BEAM_ACTION)?;}Ok(())}\nfn slab_actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,SLAB_ACTION)?;}Ok(())}"
    text = lit(text, old, new)
    text = lit(text, ' actions(out,"en1994_beam_action",parent,b,17)?', ' beam_actions(out,"en1994_beam_action",parent,b,17)?')
    return lit(text, 'actions(out,"en1994_slab_action",parent,s,9)?', 'slab_actions(out,"en1994_slab_action",parent,s,9)?')


def sqlite_tests_rs(text):
    marker = "for a in &mut b.actions{a.q_area_pa=v;a.f_k_n=v;a.delta_sigma_k_pa=v;a.delta_tau_k_pa=v;a.x_m=v;a.y_m=v;}"
    return lit(text, marker, marker.replace("a.x_m=v;a.y_m=v;", "a.x_m=v;"))


def sqlite_tests_ts(text):
    marker = 'actions:b.actions.map(a=>floats(a,["qAreaPa","fKN","deltaSigmaKPa","deltaTauKPa","xM","yM"]))'
    return lit(text, marker, marker.replace(',"xM","yM"]', ',"xM"]'))


def sqlite_fixture(text):
    cut = text.index('"columns"')
    head, tail = text[:cut], text[cut:]
    assert head.count(',"yM":0}') == 2
    return head.replace(',"yM":0}', "}") + tail


def semantic(text):
    data = json.loads(text)
    data["tableWidths"]["en1994_beam_action"] = 24
    return json.dumps(data, indent=2, ensure_ascii=False) + "\n"


def field_meta(text):
    line = [l for l in text.split("\n") if l.startswith('    ("beams[].actions[].yM"')]
    assert len(line) == 1
    return text.replace(line[0] + "\n", "", 1)


def drop_lines(text, needles, count_each=1):
    lines = text.split("\n")
    out = []
    removed = 0
    for line in lines:
        if any(n in line for n in needles):
            removed += 1
            continue
        out.append(line)
    assert removed == len(needles) * count_each, (needles, removed)
    return "\n".join(out)


def mutations_rs(text):
    return drop_lines(text, ["use semio_s_artifact_norm_en1994_mut_beam_action_position::change_beam_action_y_m;", "ChangeBeamActionYM(change_beam_action_y_m::ChangeBeamActionYM),", '    "change-beam-action-y-m",'])


def mutations_ts(text):
    return drop_lines(text, ['import { type ChangeBeamActionYM, parseChangeBeamActionYM }', "| { ChangeBeamActionYM: ChangeBeamActionYM }", "ChangeBeamActionYM: parseChangeBeamActionYM,"])


def mutations_json(data):
    def walk(node):
        if isinstance(node, list):
            node[:] = [x for x in node if not (isinstance(x, dict) and x.get("required") == ["ChangeBeamActionYM"])]
            for x in node:
                walk(x)
        elif isinstance(node, dict):
            for v in node.values():
                walk(v)
    walk(data)
    assert "ChangeBeamActionYM" not in json.dumps(data)


def position_mount(text):
    text = lit(text, "beam action x m, beam action y m.", "beam action x m.")
    return lit(text, '#[path = "../🏋️change-beam-action-y-m/🦀️.rs"]\npub mod change_beam_action_y_m;\n', "")


def testkit(text):
    return drop_lines(text, ["En1994Mutation::ChangeBeamActionYM("])


def edit_rules(text):
    return drop_lines(text, ['"beams[].actions[].yM"'])


def protocol(text):
    text = lit(text, "record change-beam-action-y-m tag=117\nfield payload bytes\n", "")
    text = lit(text, "record change-slab-action-x-m tag=118", "record change-slab-action-x-m tag=117")
    return lit(text, "record change-slab-action-y-m tag=119", "record change-slab-action-y-m tag=118")


def oracles(data):
    def walk(node):
        if isinstance(node, list):
            node[:] = [x for x in node if x != KIND and not (isinstance(x, dict) and (x.get("mutationId") == KIND or x.get("id") == KIND))]
            for x in node:
                walk(x)
        elif isinstance(node, dict):
            for v in node.values():
                walk(v)
    walk(data)
    assert KIND not in json.dumps(data)


def harness_py(text):
    text = lit(text, '    "change-beam-action-y-m",\n', "")
    return lit(text, '    "change-beam-action-y-m": ("🏋️change-beam-action-y-m", "✅apply"),\n', "")


def harness_feature(text):
    return drop_lines(text, ["| change-beam-action-y-m |"], 2)


def harness_rs(text):
    text = lit(text, '    "change-beam-action-y-m",\n', "")
    start = text.index('        "change-beam-action-y-m" => (')
    end = text.index("        ),\n", start) + len("        ),\n")
    return text[:start] + text[end:]


def insert_slab_ts(text):
    text = lit(text, "import { type CharacteristicAction, parseCharacteristicAction }", "import { type SlabAction, parseSlabAction }")
    text = lit(text, "action: CharacteristicAction;", "action: SlabAction;")
    return lit(text, "parseCharacteristicAction", "parseSlabAction")


def insert_slab_json(data):
    action = data["properties"]["action"]
    action["$ref"] = action["$ref"].replace("CharacteristicAction", "SlabAction")


def insert_slab_rs(text):
    return lit(text, "pub action: semio_s_artifact_norm_en1994_model::CharacteristicAction,", "pub action: semio_s_artifact_norm_en1994_model::SlabAction,")


def walk_fixture(node, trail, stats):
    if isinstance(node, dict):
        if "yM" in node:
            owner = next((k for k in reversed(trail) if k in BEAM_OWNERS + SLAB_OWNERS), None)
            if owner in BEAM_OWNERS:
                del node["yM"]
                stats[0] += 1
            else:
                assert owner in SLAB_OWNERS, (owner, trail)
        for key, value in list(node.items()):
            walk_fixture(value, trail + [key], stats)
    elif isinstance(node, list):
        for value in node:
            walk_fixture(value, trail, stats)


def fixtures():
    files = objects = skipped = 0
    for dirpath, dirs, names in os.walk(ANY):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "__pycache__") and d != "🏋️change-beam-action-y-m"]
        if "/🧬️schema" in dirpath and "/🧪️tests/" not in dirpath and "🧫️fixtures" not in dirpath:
            continue
        for name in names:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            if "sqlite" in path or "mutation-leaf" in path or "🔮️oracles" in path:
                continue
            text = rd(path)
            if '"yM"' not in text:
                continue
            data = json.loads(text)
            if json.dumps(data, indent=2, ensure_ascii=False) + "\n" != text:
                skipped += 1
                print("  skipped (not canonical)", path.replace(ANY, "…"))
                continue
            stats = [0]
            walk_fixture(data, [], stats)
            if stats[0]:
                put(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
                files += 1
                objects += stats[0]
    print("  fixtures:", files, "files,", objects, "beam action objects,", skipped, "skipped")


def main():
    edit(SNAP + "/🦀️.rs", snapshot_rs)
    jedit(SNAP + "/🔣️.json", schema_json)
    jedit(ANY + "/🧬️schema/🔣️.json", schema_json)
    edit(SNAP + "/🟦️.ts", ts_snapshot)
    edit(ANY + "/🧬️schema/🟦️.ts", ts_snapshot)
    edit(SNAP + "/🔗️.graphql", graphql_snapshot)
    edit(ANY + "/🧬️schema/🔗️.graphql", graphql_snapshot)
    edit(SNAP + "/🛰️.proto", proto_snapshot)
    edit(DIFF + "/🔣️.json", diff_json)
    edit(DIFF + "/🟦️.ts", diff_ts)
    edit(DIFF + "/🔗️.graphql", diff_graphql)
    edit(DIFF + "/🛰️.proto", diff_proto)
    edit(DIFF + "/🏗️beam-rows/🦀️.rs", diff_rs_beam)
    edit(DIFF + "/🧱️slab-rows/🦀️.rs", diff_rs_slab)
    edit(DIFF + "/🦀️.rs", diff_root_rs)
    edit(SQL + "/🗄️.sql", sql_ddl)
    edit(SQL + "/🦀️.rs", sqlite_rs)
    edit(SQL + "/🟦️.ts", sqlite_ts)
    edit(SQL + "/🛂️admission/🦀️.rs", admission)
    edit(SQL + "/🧪️tests/🦀️.rs", sqlite_tests_rs)
    edit(SQL + "/🧪️tests/🟦️.ts", sqlite_tests_ts)
    edit(SQL + "/🧪️tests/🧫️fixtures/🔣️.json", sqlite_fixture)
    edit(SQL + "/🧪️tests/🧫️fixtures/🎛️semantic.json", semantic)
    edit(ANY + "/✏️editor/🏷️field-meta/🦀️.rs", field_meta)
    edit(MUT + "/🦀️.rs", mutations_rs)
    edit(MUT + "/🟦️.ts", mutations_ts)
    jedit(MUT + "/🔣️.json", mutations_json)
    edit(MUT + "/🏋️beam-action-position/🦀️.rs", position_mount)
    edit(MUT + "/🧰️testkit/🦀️.rs", testkit)
    edit(MUT + "/🧭️edit-rules/🦀️.rs", edit_rules)
    edit(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", protocol)
    jedit(ANY + "/🔮️oracles/🔣️.json", oracles)
    edit(ANY + "/🧪️tests/🧩️mutate-en1994-1/🐍️.py", harness_py)
    edit(ANY + "/🧪️tests/🧩️mutate-en1994-1/🥒️.feature", harness_feature)
    edit(ANY + "/🧪️tests/🧩️mutate-en1994-1/🦀️.rs", harness_rs)
    edit(MUT + "/🧱️insert-slab-action/🧬️schema/🟦️.ts", insert_slab_ts)
    jedit(MUT + "/🧱️insert-slab-action/🧬️schema/🔣️.json", insert_slab_json)
    edit(MUT + "/🧱️insert-slab-action/🦀️.rs", insert_slab_rs)
    fixtures()
    if DRY:
        print("dry run:", len(PENDING), "files would change")
        return
    for path, text in PENDING.items():
        open(path, "w", encoding="utf-8").write(text)
    for doomed in (MUT + "/🏋️change-beam-action-y-m", ANY + "/🧪️tests/🧫️fixtures/🧬️mutations/🏋️change-beam-action-y-m"):
        shutil.rmtree(doomed)
    print("wrote", len(PENDING), "files")


if __name__ == "__main__":
    main()
