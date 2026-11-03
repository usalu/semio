#!/usr/bin/env python3
"""Replaces the `panels[].layup` string by typed child rows `panels[].layers[]` and adds the optional child rows `fasteners[].positions[]` (spec.py CHILDREN) through every schema layer, idempotently, no migration: model structs, snapshot twins (ts, json), diff lanes (nested `list_delta!` with local key markers: rs, ts, json), field-meta, SQLite (DDL, Rust and TS codecs, admission, tests, contract) and the committed fixtures (layup string to layer rows, empty `positions`/`layers` deltas).
Usage: python3 -I nested.py [--dry] [--root PATH_TO_SUBSET_DIR]"""
import difflib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.dirname(HERE) + "/placements-adder")
import spec as S
from spec import ENTITIES, CHILDREN, RS_TY, TS_TY, WIRE, SNAP_URL, DSL_UNIT, FIELD_META_UNIT
import padd
import schema as SC

DRY = "--dry" in sys.argv
ROOT = SC.ROOT
SCH, Q = SC.SCH, SC.Q
PANEL, FASTENER = ENTITIES[3], ENTITIES[2]
assert PANEL["ent"] == "Panel" and FASTENER["ent"] == "Fastener"
LAYER, POSITION = CHILDREN
OLD_LAYUP = ("layup", "layup")
EXAMPLE_ORIENTATION = ["longitudinal", "transverse"]


def rd(p):
    return open(p, encoding="utf-8").read()


def edit(p, fn):
    SC.DRY = DRY
    SC.edit(p, fn)


def sub1(t, pat, repl, flags=0, count=1):
    return SC.sub1(t, pat, repl, flags, count)


def rs_child(c, attrs):
    out = ["", "/// %s %s" % (c["emoji"], c["doc"])] + attrs + ["pub struct %s {" % c["ent"], "    pub id: String,"]
    for fl in c["rest"]:
        if fl["unit"] in DSL_UNIT and fl["ty"] == "f64":
            out.append('    #[dsl(unit = "%s")]' % DSL_UNIT[fl["unit"]])
        out.append("    pub %s: %s," % (fl["name"], RS_TY[fl["ty"]]))
    out.append("}")
    return "\n".join(out) + "\n"


def layer_model():
    def go(t):
        if "pub struct PanelLayer {" in t:
            return t
        m = re.search(r"((?:^#\[[^\n]*\n)+)pub struct Panel \{\n", t, re.M)
        attrs = [l for l in m.group(1).split("\n") if l.startswith("#[")]
        derive = [a for a in attrs if a.startswith("#[derive")]
        start = t.rfind("/// ", 0, m.start())
        doc_start = t.rfind("\n", 0, m.start() - 1)
        panel_doc = re.search(r"^/// 🧱️ [^\n]*\n(?=#\[derive)", t, re.M)
        assert panel_doc
        t = t[:panel_doc.start()] + rs_child(LAYER, attrs).lstrip("\n") + "\n" + t[panel_doc.start():]
        fm = re.search(r"^/// 🔩️ Fastener pattern of one connection[^\n]*\n(?=#\[derive)", t, re.M)
        assert fm
        t = t[:fm.start()] + rs_child(POSITION, attrs).lstrip("\n") + "\n" + t[fm.start():]
        t = sub1(t, r"    pub strength_class: String,\n    pub layup: String,\n", "    pub strength_class: String,\n    #[dsl(table)]\n    pub layers: Vec<PanelLayer>,\n")
        t = sub1(t, r"(pub struct Fastener \{.*?    pub load_angle_rad: f64,\n)(\})", lambda m: m.group(1) + "    #[dsl(table)]\n    pub positions: Vec<FastenerPosition>,\n" + m.group(2), re.S)
        t = t.replace("/// 🧱️ Cross-laminated timber panel with its layup and fire exposure", "/// 🧱️ Cross-laminated timber panel with its typed layers (child rows) and fire exposure")
        return t
    edit(SCH + "📸️snapshot/🦀️.rs", go)


def ts_iface_child(c):
    return "export interface %s {\n  id: string;\n%s}\n" % (c["ent"], "".join("  %s: %s;\n" % (fl["camel"], TS_TY[fl["ty"]]) for fl in c["rest"]))


def layer_snapshot_ts():
    def go(t):
        if "export interface PanelLayer " in t:
            return t
        t = sub1(t, r"  strengthClass: string;\n  layup: string;\n", "  strengthClass: string;\n  layers: PanelLayer[];\n")
        t = sub1(t, r"(export interface Fastener \{\n(?:  [^\n]*\n)*?  loadAngleRad: number;\n)(\}\n)", lambda m: m.group(1) + "  positions: FastenerPosition[];\n" + m.group(2) + "\n" + ts_iface_child(POSITION))
        t = sub1(t, r"(export interface Panel \{\n(?:  [^\n]*\n)*?\}\n)", lambda m: m.group(1) + "\n" + ts_iface_child(LAYER))
        t = sub1(t, r"layup: normWireRequired\(normWireString\)", "layers: normWireRequired(normWireArray(normWireRef(() => parsePanelLayer)))")
        t = sub1(t, r"(export const parseFastener: [^\n]*?loadAngleRad: normWireRequired\(normWireNumber\))( \}\);\n)", lambda m: m.group(1) + ", positions: normWireRequired(normWireArray(normWireRef(() => parseFastenerPosition)))" + m.group(2) + "\n" + SC.ts_parse(POSITION) + "\n")
        t = sub1(t, r"(export const parsePanel: [^\n]*\n)", lambda m: m.group(1) + "\n" + SC.ts_parse(LAYER) + "\n")
        return t
    edit(SCH + "📸️snapshot/🟦️.ts", go)


def array_prop(c):
    return {"type": "array", "items": {"$ref": "#/$defs/" + c["ent"]}, "x-semio-ui": {"label": {"en": c["label_en"], "de": c["label_de"]}}}


def layer_snapshot_json():
    def go(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        if "PanelLayer" in data["$defs"]:
            return t
        D = {}
        for k, v in data["$defs"].items():
            if k == "Panel":
                d = json.loads(json.dumps(v))
                props = {}
                for pk, pv in d["properties"].items():
                    if pk == "layup":
                        props["layers"] = array_prop(LAYER)
                    else:
                        props[pk] = pv
                d["properties"] = props
                d["required"] = ["layers" if r == "layup" else r for r in d["required"]]
                cd = SC.json_def(LAYER)
                cd["x-semio-formats"] = v["x-semio-formats"]
                D["PanelLayer"] = cd
                D[k] = d
            elif k == "Fastener":
                d = json.loads(json.dumps(v))
                d["properties"]["positions"] = array_prop(POSITION)
                d["required"] = d["required"] + ["positions"]
                pd = SC.json_def(POSITION)
                pd["x-semio-formats"] = v["x-semio-formats"]
                D["FastenerPosition"] = pd
                D[k] = d
            else:
                D[k] = v
        data["$defs"] = D
        return padd.dumps_with(form, data) + tail
    edit(SCH + "📸️snapshot/🔣️.json", go)


def child_blocks(c):
    key, keyof, patch, delta = SC.diff_rs_blocks(dict(c, plural=c["plural"]))
    return key, keyof, patch, delta


def child_names(c):
    return "En1995" + c["ent"]


def layer_diff():
    P = {"panels": ("Panel", LAYER), "fasteners": ("Fastener", POSITION)}

    def rs(t):
        if "pub struct PanelLayerKey;" in t:
            return t
        keys = "".join(child_blocks(c)[0] for c in CHILDREN)
        t = sub1(t, r"(pub struct PlacementKey;\n\n)", lambda m: m.group(1) + keys)
        keyofs = "".join(child_blocks(c)[1] for c in CHILDREN)
        t = sub1(t, r"(impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Placement> for PlacementKey \{.*?\n\}\n\n)", lambda m: m.group(1) + keyofs, re.S)
        t = sub1(t, r"layup: String, ", "")
        t = sub1(t, r"(En1995PanelPatch of semio_s_artifact_norm_en1995_model::Panel \{ set \{[^}]*\})( \})", lambda m: m.group(1) + " nest { layers: En1995PanelLayerDelta }" + m.group(2))
        t = sub1(t, r"(En1995FastenerPatch of semio_s_artifact_norm_en1995_model::Fastener \{ set \{[^}]*\})( \})", lambda m: m.group(1) + " nest { positions: En1995FastenerPositionDelta }" + m.group(2))
        for parent, c in (("Panel", LAYER), ("Fastener", POSITION)):
            patch, delta = child_blocks(c)[2], child_blocks(c)[3]
            t = sub1(t, r"(protocol::list_delta! \{\n(?:[^\n]*\n){3}    pub En1995%sDelta [^\n]*\n\}\n\n)" % parent, lambda m: m.group(1) + patch + delta)
        return t
    edit(SCH + "🔺️diff/🦀️.rs", rs)

    def dts(t):
        if "interface En1995PanelLayerPatch" in t:
            return t
        t = sub1(t, r" type Panel, type Floor,", " type Panel, type PanelLayer, type FastenerPosition, type Floor,")
        t = sub1(t, r" parsePanel, parseFloor,", " parsePanel, parsePanelLayer, parseFastenerPosition, parseFloor,")
        t = sub1(t, r"  strengthClass: string \| null;\n  layup: string \| null;\n(  spanM: number \| null;\n  widthM: number \| null;\n  loadPa: number \| null;\n  fireDurationS: number \| null;\n)", r"  strengthClass: string | null;\n\1  layers?: En1995PanelLayerDelta;\n")
        t = sub1(t, r"(export interface En1995FastenerPatch \{\n(?:  [^\n]*\n)*?  loadAngleRad: number \| null;\n)", lambda m: m.group(1) + "  positions?: En1995FastenerPositionDelta;\n")
        nn = lambda: 'normWireRequired(normWireRange(normWireInteger, {"minimum":0}))'
        for parent, c in (("Panel", LAYER), ("Fastener", POSITION)):
            P = "En1995" + c["ent"]
            ifaces = "\nexport interface %sPatch {\n%s}\n" % (P, "".join("  %s: %s | null;\n" % (fl["camel"], TS_TY[fl["ty"]]) for fl in c["rest"]))
            ifaces += "export interface %sRemoval {\n  id: string;\n  index: number;\n}\n" % P
            ifaces += "export interface %sInsertion {\n  index: number;\n  row: %s;\n}\n" % (P, c["ent"])
            ifaces += "export interface %sRelocation {\n  id: string;\n  from: number;\n  to: number;\n}\n" % P
            ifaces += "export interface %sModification {\n  id: string;\n  patch: %sPatch;\n}\n" % (P, P)
            ifaces += "export interface %sDelta {\n  removed: %sRemoval[];\n  inserted: %sInsertion[];\n  moved: %sRelocation[];\n  modified: %sModification[];\n}\n" % (P, P, P, P, P)
            t = sub1(t, r"(export interface En1995%sDelta \{\n(?:[^\n]*\n){4}\}\n)" % parent, lambda m: m.group(1) + ifaces)
            pf = ", ".join("%s: normWireDefault(normWireNullable(%s), () => null)" % (fl["camel"], WIRE[fl["ty"]]) for fl in c["rest"])
            parses = "\nexport const parse%sPatch: NormWireReader<%sPatch> = normWireObject<%sPatch>({ %s });\n" % (P, P, P, pf)
            parses += "export const parse%sRemoval: NormWireReader<%sRemoval> = normWireObject<%sRemoval>({ id: normWireRequired(normWireString), index: %s });\n" % (P, P, P, nn())
            parses += "export const parse%sInsertion: NormWireReader<%sInsertion> = normWireObject<%sInsertion>({ index: %s, row: normWireRequired(parse%s) });\n" % (P, P, P, nn(), c["ent"])
            parses += "export const parse%sRelocation: NormWireReader<%sRelocation> = normWireObject<%sRelocation>({ id: normWireRequired(normWireString), from: %s, to: %s });\n" % (P, P, P, nn(), nn())
            parses += "export const parse%sModification: NormWireReader<%sModification> = normWireObject<%sModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse%sPatch)) });\n" % (P, P, P, P)
            parses += "export const parse%sDelta: NormWireReader<%sDelta> = normWireObject<%sDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parse%sRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parse%sInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parse%sRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parse%sModification))) });\n" % (P, P, P, P, P, P, P)
            t = sub1(t, r"(export const parseEn1995%sDelta: [^\n]*\n)" % parent, lambda m: m.group(1) + parses)
        t = sub1(t, r"strengthClass: normWireDefault\(normWireNullable\(normWireString\), \(\) => null\), layup: normWireDefault\(normWireNullable\(normWireString\), \(\) => null\), (spanM: [^\n]*?fireDurationS: normWireDefault\(normWireNullable\(normWireNumber\), \(\) => null\)) \}\);", r"strengthClass: normWireDefault(normWireNullable(normWireString), () => null), \1, layers: normWireOptional(normWireRef(() => parseEn1995PanelLayerDelta)) });")
        t = sub1(t, r"(export const parseEn1995FastenerPatch: [^\n]*?loadAngleRad: normWireDefault\(normWireNullable\(normWireNumber\), \(\) => null\))( \}\);)", lambda m: m.group(1) + ", positions: normWireOptional(normWireRef(() => parseEn1995FastenerPositionDelta))" + m.group(2))
        return t
    edit(SCH + "🔺️diff/🟦️.ts", dts)

    def djson(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        D = data["$defs"]
        if "En1995PanelLayerPatch" in D:
            return t
        fmts = D["En1995PlacementDelta"].get("x-semio-formats")
        nd = {}
        for k, v in D.items():
            nd[k] = v
            for parent, c in (("Panel", LAYER), ("Fastener", POSITION)):
                if k == "En1995%sDelta" % parent:
                    P = "En1995" + c["ent"]
                    patch_props = {fl["camel"]: {"anyOf": [{"$ref": "%s#/$defs/%s/properties/%s" % (SNAP_URL, c["ent"], fl["camel"])}, {"type": "null"}]} for fl in c["rest"]}
                    for suffix in ("Patch", "Removal", "Insertion", "Relocation", "Modification", "Delta"):
                        d = json.loads(json.dumps(D["En1995Placement" + suffix]).replace("En1995Placement", P).replace("snapshot.json#/$defs/Placement", "snapshot.json#/$defs/" + c["ent"]))
                        if suffix == "Patch":
                            d["properties"] = patch_props
                        nd[P + suffix] = d
        for parent, c in (("Panel", LAYER), ("Fastener", POSITION)):
            pp = nd["En1995%sPatch" % parent]["properties"]
            if parent == "Panel":
                pp.pop("layup")
            pp[c["plural"]] = {"$ref": "#/$defs/En1995%sDelta" % c["ent"]}
        data["$defs"] = nd
        return padd.Hybrid.dumps(data) + tail
    edit(SCH + "🔺️diff/🔣️.json", djson)


def meta_child_rows(c):
    rows = ['    ("%s[].%s", FieldSpec::new("%s", "%s")),' % (c["parent"], c["plural"], c["label_en"], c["label_de"]), '    ("%s[].%s[].id", FieldSpec::new("%s", "%s")),' % (c["parent"], c["plural"], c["id_en"], c["id_de"])]
    for fl in c["rest"]:
        path = "%s[].%s[].%s" % (c["parent"], c["plural"], fl["camel"])
        if fl["options"]:
            rows.append('    ("%s", FieldSpec::choice("%s", "%s", %s)),' % (path, fl["en"], fl["de"], SC.choices_const(c, fl)))
        elif fl["unit"] in FIELD_META_UNIT and fl["ty"] == "f64":
            rows.append('    ("%s", FieldSpec::quantity("%s", "%s", %s)),' % (path, fl["en"], fl["de"], FIELD_META_UNIT[fl["unit"]]))
        else:
            rows.append('    ("%s", FieldSpec::new("%s", "%s")),' % (path, fl["en"], fl["de"]))
    return rows


def layer_meta():
    def go(t):
        if '"panels[].layers"' in t:
            return t
        consts = ""
        for c in CHILDREN:
            for fl in c["rest"]:
                if fl["options"]:
                    consts += "const %s: &[NormFieldChoice] = &[\n%s];\n" % (SC.choices_const(dict(c, table=c["table"]), fl), "".join('    choice("%s", "%s", "%s"),\n' % o for o in fl["options"]))
        t = sub1(t, r"(const ANNEX: &\[NormFieldChoice\])", lambda m: consts + m.group(1))
        lines = [l for l in t.split("\n") if not l.startswith('    ("panels[].layup"')]
        out = []
        for l in lines:
            out.append(l)
            if l.startswith('    ("panels[].strengthClass"'):
                out.extend(meta_child_rows(LAYER))
            if l.startswith('    ("fasteners[].loadAngleRad"'):
                out.extend(meta_child_rows(POSITION))
        assert any(l.startswith('    ("panels[].layers"') for l in out) and any(l.startswith('    ("fasteners[].positions"') for l in out)
        return "\n".join(out)
    edit(ROOT + "✏️editor/🏷️field-meta/🦀️.rs", go)


def ddl_child(c, parent_tbl):
    s = "CREATE TABLE %s (\n  id INTEGER PRIMARY KEY,\n  %s_id INTEGER NOT NULL REFERENCES %s(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  logical_id TEXT NOT NULL,\n" % (c["tbl"], c["pslug"], parent_tbl)
    cols = []
    for fl in c["rest"]:
        cols.append("%s TEXT NOT NULL" % fl["name"] if fl["ty"] == "String" else "%s REAL" % fl["name"])
    s += "  " + ",\n  ".join(cols) + ",\n"
    s += ",\n".join("  %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL" % (fl["name"], fl["name"]) for fl in c["rest"] if fl["ty"] == "f64") + "\n);\n"
    return s


def layer_sqlite():
    N = 14
    total = N + len(CHILDREN)
    pf = PANEL["floats"]
    assert pf == [5, 6, 7, 8] and PANEL["cols"] == 9

    def sql(t):
        if "en1995_panel_layer" in t:
            return t
        t = sub1(t, r"  strength_class TEXT NOT NULL,\n  layup TEXT NOT NULL,\n  span_m REAL,\n  width_m REAL,\n  load_pa REAL,", "  strength_class TEXT NOT NULL,\n  span_m REAL,\n  width_m REAL,\n  load_pa REAL,")
        return t.rstrip("\n") + "\n" + ddl_child(LAYER, PANEL["tbl"]) + ddl_child(POSITION, FASTENER["tbl"])
    edit(Q + "🗄️.sql", sql)

    def rs(t):
        if "PANEL_LAYER_FLOATS" in t:
            return t
        t = sub1(t, r"Fastener, Panel, Floor, MemberRole", "Fastener, FastenerPosition, Panel, PanelLayer, Floor, MemberRole")
        t = sub1(t, r"const PANEL_FLOATS:[^\n]*\n", "const PANEL_FLOATS:&[FloatColumn]=&[%s];\n" % ",".join("FloatColumn::Binary64(%d)" % i for i in pf) + "".join("const %s:&[FloatColumn]=&[%s];\n" % (c["const"], ",".join("FloatColumn::Binary64(%d)" % i for i in c["floats"])) for c in CHILDREN))
        cell = lambda fl, v: "Cell::Text(&%s.%s)" % (v, fl["name"]) if fl["ty"] == "String" else "Cell::Real(%s.%s)" % (v, fl["name"])

        def child_write(c, parent_var, var):
            return 'for(position,%s)in %s.%s.iter().enumerate(){out.insert_float("%s",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&%s.id),%s],%s)?;}' % (var, parent_var, c["plural"], c["tbl"], var, ",".join(cell(fl, var) for fl in c["rest"]), c["const"])
        t = sub1(t, r'        for\(position,fastener\)in self\.fasteners\.iter\(\)\.enumerate\(\)\{out\.insert_float\("en1995_fastener",(&\[[^\n]*?\]),FASTENER_FLOATS\)\?;\}\n',
                 lambda m: '        for(position,fastener)in self.fasteners.iter().enumerate(){\n            let id=out.insert_float("en1995_fastener",%s,FASTENER_FLOATS)?;\n            %s\n        }\n' % (m.group(1), child_write(POSITION, "fastener", "point")))
        t = sub1(t, r'        for\(position,panel\)in self\.panels\.iter\(\)\.enumerate\(\)\{out\.insert_float\("en1995_panel",&\[Cell::Integer\(1\),ordinal\(position\)\?,Cell::Text\(&panel\.id\),Cell::Text\(&panel\.strength_class\),Cell::Text\(&panel\.layup\),([^\n]*?)\],PANEL_FLOATS\)\?;\}\n',
                 lambda m: '        for(position,panel)in self.panels.iter().enumerate(){\n            let id=out.insert_float("en1995_panel",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&panel.id),Cell::Text(&panel.strength_class),%s],PANEL_FLOATS)?;\n            %s\n        }\n' % (m.group(1), child_write(LAYER, "panel", "layer")))
        t = sub1(t, r"SQLITE_SCHEMA,14,70", "SQLITE_SCHEMA,%d,70" % total)
        t = sub1(t, r"for\(index,member\)in self\.members\.iter\(\)\.enumerate\(\)\{rows=add\(rows,member\.actions\.len\(\)\)\?;", "for panel in &self.panels{rows=add(rows,panel.layers.len())?;control.check_rows(rows)?;}for fastener in &self.fasteners{rows=add(rows,fastener.positions.len())?;control.check_rows(rows)?;}for(index,member)in self.members.iter().enumerate(){rows=add(rows,member.actions.len())?;")
        t = sub1(t, r"for text in \[&panel\.id,&panel\.strength_class,&panel\.layup\]\{bound\.repeated\(text\.len\(\),24\)\?;\}\}", "for text in [&panel.id,&panel.strength_class]{bound.repeated(text.len(),24)?;}for layer in &panel.layers{bound.add(2048)?;for text in [&layer.id,&layer.orientation,&layer.grade]{bound.repeated(text.len(),24)?;}}}")
        t = sub1(t, r"for text in \[&fastener\.id,&fastener\.connection_id,&fastener\.member_id\]\{bound\.repeated\(text\.len\(\),24\)\?;\}\}", "for text in [&fastener.id,&fastener.connection_id,&fastener.member_id]{bound.repeated(text.len(),24)?;}for point in &fastener.positions{bound.add(2048)?;bound.repeated(point.id.len(),24)?;}}")

        def fastener_read(m):
            body = m.group(1)
            return ('        let fastener_rows=entities(database,"en1995_fastener",15,FASTENER_FLOATS,control)?;let mut fastener_positions=groups(entities(database,"%s",%d,%s,control)?,&fastener_rows,control)?;\n' % (POSITION["tbl"], POSITION["cols"], POSITION["const"])
                    + '        let mut fasteners=Vec::new();for row in ordered(fastener_rows.into_values(),1,control)?{checkpoint(control,fasteners.len(),0)?;let mut positions=Vec::new();for point in ordered(fastener_positions.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,positions.len(),0)?;positions.push(FastenerPosition{id:reconstruct_text(control,point.text(3)?)?,x_m:point.real(4)?,y_m:point.real(5)?});}fasteners.push(Fastener{%s,positions});}\n' % body)
        t = sub1(t, r'        let mut fasteners=Vec::new\(\);for row in ordered\(entities\(database,"en1995_fastener",15,FASTENER_FLOATS,control\)\?\.into_values\(\),1,control\)\?\{checkpoint\(control,fasteners\.len\(\),0\)\?;fasteners\.push\(Fastener\{(.*?)\}\);\}\n', fastener_read)

        def panel_read(m):
            return ('        let panel_rows=entities(database,"en1995_panel",9,PANEL_FLOATS,control)?;let mut panel_layers=groups(entities(database,"%s",%d,%s,control)?,&panel_rows,control)?;\n' % (LAYER["tbl"], LAYER["cols"], LAYER["const"])
                    + '        let mut panels=Vec::new();for row in ordered(panel_rows.into_values(),1,control)?{checkpoint(control,panels.len(),0)?;let mut layers=Vec::new();for layer in ordered(panel_layers.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,layers.len(),0)?;layers.push(PanelLayer{id:reconstruct_text(control,layer.text(3)?)?,thickness_m:layer.real(4)?,orientation:reconstruct_text(control,layer.text(5)?)?,grade:reconstruct_text(control,layer.text(6)?)?});}panels.push(Panel{id:reconstruct_text(control,row.text(3)?)?,strength_class:reconstruct_text(control,row.text(4)?)?,layers,span_m:row.real(5)?,width_m:row.real(6)?,load_pa:row.real(7)?,fire_duration_s:row.real(8)?});}\n')
        t = sub1(t, r'        let mut panels=Vec::new\(\);for row in ordered\(entities\(database,"en1995_panel",10,PANEL_FLOATS,control\)\?\.into_values\(\),1,control\)\?\{[^\n]*\n', panel_read)
        return t
    edit(Q + "🦀️.rs", rs)

    def ts(t):
        if "PANEL_LAYER_FLOATS" in t:
            return t
        t = sub1(t, r"Fastener,Panel,Floor\}from", "Fastener,FastenerPosition,Panel,PanelLayer,Floor}from")
        t = sub1(t, r"export type En1995SqliteFastener=Omit<Fastener,([^>]*?)>&\{([^\n]*?)\};\n", lambda m: 'export type En1995SqliteFastenerPosition=Omit<FastenerPosition,"xM"|"yM">&{xM:Binary64;yM:Binary64};\nexport type En1995SqliteFastener=Omit<Fastener,%s|"positions">&{%s;positions:En1995SqliteFastenerPosition[]};\n' % (m.group(1), m.group(2)))
        t = sub1(t, r'export type En1995SqlitePanel=Omit<Panel,"spanM"\|"widthM"\|"loadPa"\|"fireDurationS">&\{([^\n]*?)\};\n', lambda m: 'export type En1995SqlitePanelLayer=Omit<PanelLayer,"thicknessM">&{thicknessM:Binary64};\nexport type En1995SqlitePanel=Omit<Panel,"spanM"|"widthM"|"loadPa"|"fireDurationS"|"layers">&{%s;layers:En1995SqlitePanelLayer[]};\n' % m.group(1))
        t = sub1(t, r"const PANEL_FLOATS:[^\n]*\n", "const PANEL_FLOATS:readonly Ieee754Column[]=[%s].map(index=>({index,width:64}));\n" % ",".join(str(i) for i in pf) + "".join("const %s:readonly Ieee754Column[]=[%s].map(index=>({index,width:64}));\n" % (c["const"], ",".join(str(i) for i in c["floats"])) for c in CHILDREN))
        t = sub1(t, r' for\(let position=0;position<snapshot\.fasteners\.length;position\+\+\)\{const p=snapshot\.fasteners\[position\]!;await write\(out,"en1995_fastener",(\[[^\n]*?\]),FASTENER_FLOATS,options\)\}\n',
                 lambda m: ' for(let position=0;position<snapshot.fasteners.length;position++){const p=snapshot.fasteners[position]!,id=await write(out,"en1995_fastener",%s,FASTENER_FLOATS,options);for(let at=0;at<p.positions.length;at++){const q=p.positions[at]!;await write(out,"%s",[id,BigInt(at),q.id,q.xM,q.yM],%s,options)}}\n' % (m.group(1), POSITION["tbl"], POSITION["const"]))
        t = sub1(t, r' for\(let position=0;position<snapshot\.panels\.length;position\+\+\)\{const p=snapshot\.panels\[position\]!;await write\(out,"en1995_panel",\[1n,BigInt\(position\),p\.id,p\.strengthClass,p\.layup,([^\n]*?)\],PANEL_FLOATS,options\)\}\n',
                 lambda m: ' for(let position=0;position<snapshot.panels.length;position++){const p=snapshot.panels[position]!,id=await write(out,"en1995_panel",[1n,BigInt(position),p.id,p.strengthClass,%s],PANEL_FLOATS,options);for(let at=0;at<p.layers.length;at++){const q=p.layers[at]!;await write(out,"%s",[id,BigInt(at),q.id,q.thicknessM,q.orientation,q.grade],%s,options)}}\n' % (m.group(1), LAYER["tbl"], LAYER["const"]))

        def fastener_read(m):
            return (' const fastenersRows=await entities(tables[11]!,15,FASTENER_FLOATS,options),fastenerPositions=await groups(await entities(tables[15]!,%d,%s,options),fastenersRows,options),fasteners:En1995SqliteFastener[]=[];for(const r of await ordered(fastenersRows.values(),1n,options)){await checkpoint(options,fasteners.length,0);const positions:En1995SqliteFastenerPosition[]=[];for(const q of await ordered(fastenerPositions.get(r.rowid)??[],r.rowid,options)){await checkpoint(options,positions.length,0);positions.push({id:artifactSqliteText(q,3),xM:readBinary64(q,4,%s),yM:readBinary64(q,5,%s)})}fasteners.push({%s,positions})}\n' % (POSITION["cols"], POSITION["const"], POSITION["const"], POSITION["const"], m.group(1)))
        t = sub1(t, r" const fastenersRows=await entities\(tables\[11\]!,15,FASTENER_FLOATS,options\),fasteners:En1995SqliteFastener\[\]=\[\];for\(const r of await ordered\(fastenersRows\.values\(\),1n,options\)\)\{await checkpoint\(options,fasteners\.length,0\);fasteners\.push\(\{(.*?)\}\)\}\n", fastener_read)
        t = sub1(t, r" const panelsRows=await entities\(tables\[12\]!,10,PANEL_FLOATS,options\),panels:En1995SqlitePanel\[\]=\[\];for\(const r of await ordered\(panelsRows\.values\(\),1n,options\)\)\{[^\n]*\n",
                 ' const panelsRows=await entities(tables[12]!,9,PANEL_FLOATS,options),panelLayers=await groups(await entities(tables[14]!,%d,%s,options),panelsRows,options),panels:En1995SqlitePanel[]=[];for(const r of await ordered(panelsRows.values(),1n,options)){await checkpoint(options,panels.length,0);const layers:En1995SqlitePanelLayer[]=[];for(const q of await ordered(panelLayers.get(r.rowid)??[],r.rowid,options)){await checkpoint(options,layers.length,0);layers.push({id:artifactSqliteText(q,3),thicknessM:readBinary64(q,4,%s),orientation:artifactSqliteText(q,5),grade:artifactSqliteText(q,6)})}panels.push({id:artifactSqliteText(r,3),strengthClass:artifactSqliteText(r,4),layers,spanM:readBinary64(r,5,PANEL_FLOATS),widthM:readBinary64(r,6,PANEL_FLOATS),loadPa:readBinary64(r,7,PANEL_FLOATS),fireDurationS:readBinary64(r,8,PANEL_FLOATS)})}\n' % (LAYER["cols"], LAYER["const"], LAYER["const"]))
        return t
    edit(Q + "🟦️.ts", ts)

    def adm(t):
        if "en1995_panel_layer" in t:
            return t
        t = sub1(t, r"n::entity\(out,\"en1995_fastener\",1,index,&n::cells\(n::record\(p\)\?,(\[[^\n]*?\])\)\?,FASTENER_FLOATS\)\?;\}\n",
                 lambda m: 'let parent=n::entity(out,"en1995_fastener",1,index,&n::cells(n::record(p)?,%s)?,FASTENER_FLOATS)?;\n for(index,q)in n::list(n::field(n::record(p)?,12)?)?.iter().enumerate(){n::entity(out,"%s",parent,index,&n::cells(n::record(q)?,[Text(0),Real(1),Real(2)])?,%s)?;}}\n' % (m.group(1), POSITION["tbl"], POSITION["const"]))
        t = sub1(t, r"n::entity\(out,\"en1995_panel\",1,index,&n::cells\(n::record\(p\)\?,\[Text\(0\),Text\(1\),Text\(2\),Real\(3\),Real\(4\),Real\(5\),Real\(6\)\]\)\?,PANEL_FLOATS\)\?;\}\n",
                 'let parent=n::entity(out,"en1995_panel",1,index,&n::cells(n::record(p)?,[Text(0),Text(1),Real(3),Real(4),Real(5),Real(6)])?,PANEL_FLOATS)?;\n for(index,q)in n::list(n::field(n::record(p)?,2)?)?.iter().enumerate(){n::entity(out,"%s",parent,index,&n::cells(n::record(q)?,[Text(0),Real(1),Text(2),Text(3)])?,%s)?;}}\n' % (LAYER["tbl"], LAYER["const"]))
        t = sub1(t, r"rows=n::add\(rows,n::list\(n::field\(root,8\)\?\)\?\.len\(\)\)\?;rows=n::add\(rows,n::list\(n::field\(root,9\)\?\)\?\.len\(\)\)\?;", "for(id,children)in[(8,12),(9,2)]{let owners=n::list(n::field(root,id)?)?;rows=n::add(rows,owners.len())?;for owner in owners{rows=n::add(rows,n::list(n::field(n::record(owner)?,children)?)?.len())?}}")
        t = sub1(t, r"SQLITE_SCHEMA,14,70", "SQLITE_SCHEMA,%d,70" % total)
        t = t.replace("Exact fourteen-table", "Exact sixteen-table")
        return t
    edit(Q + "🛂️admission/🦀️.rs", adm)

    def rtests(t):
        t = re.sub(r'\("tables",14\)', '("tables",%d)' % total, t)
        t = re.sub(r"database\(&empty\)\.tables\.len\(\),14", "database(&empty).tables.len(),%d" % total, t)
        return t
    edit(Q + "🧪️tests/🦀️.rs", rtests)

    def ttests(t):
        return re.sub(r"database\.tables\.length\)\.toBe\(14\)", "database.tables.length).toBe(%d)" % total, t)
    edit(Q + "🧪️tests/🟦️.ts", ttests)

    def sqltext():
        t = rd(Q + "🗄️.sql")
        if "en1995_panel_layer" not in t:
            t = t.replace("  strength_class TEXT NOT NULL,\n  layup TEXT NOT NULL,\n  span_m REAL,", "  strength_class TEXT NOT NULL,\n  span_m REAL,", 1) + ddl_child(LAYER, PANEL["tbl"]) + ddl_child(POSITION, FASTENER["tbl"])
        return t
    new_bytes = SC.schema_bytes(sqltext())
    widths = {PANEL["tbl"]: PANEL["width"], LAYER["tbl"]: LAYER["width"], POSITION["tbl"]: POSITION["width"]}

    def sj(t):
        d = json.loads(t)
        d["schemaBytes"] = new_bytes
        d["tableWidths"].update(widths)
        d["tableWidths"] = dict(sorted(d["tableWidths"].items()))
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    edit(Q + "🧪️tests/🧫️fixtures/🎛️semantic.json", sj)

    def tt(t):
        t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_bytes), t)
        return re.sub(r'("tableWidths"\]\)\.toEqual\(\{)([^}]*)\}', lambda m: m.group(1) + ",".join(sorted(set(x for x in m.group(2).split(",") if not x.startswith('"en1995_panel":')) | {'"%s":%d' % (k, v) for k, v in widths.items()})) + "}", t)
    edit(Q + "🧪️tests/🟦️.ts", tt)


def layers_of(layup):
    parts = [float(x) for x in layup.split("/")]
    return [{"id": "layer-%d" % (i + 1), "thicknessM": round(p / 1000.0, 9), "orientation": EXAMPLE_ORIENTATION[i % 2], "grade": "C24"} for i, p in enumerate(parts)]


def walk(node, fn):
    if isinstance(node, dict):
        fn(node)
        for v in list(node.values()):
            walk(v, fn)
    elif isinstance(node, list):
        for v in node:
            walk(v, fn)


EMPTY = {"removed": [], "inserted": [], "moved": [], "modified": []}


def fix_node(node):
    if "layup" in node and isinstance(node["layup"], str) and "strengthClass" in node:
        keys = list(node)
        new = {}
        for k in keys:
            if k == "layup":
                new["layers"] = layers_of(node["layup"])
            else:
                new[k] = node[k]
        node.clear()
        node.update(new)
    elif "layup" in node and node["layup"] is None and "strengthClass" in node:
        new = {}
        for k in list(node):
            if k != "layup":
                new[k] = node[k]
        new["layers"] = json.loads(json.dumps(EMPTY))
        node.clear()
        node.update(new)
    if "loadAngleRad" in node and "offsetYM" in node and "positions" not in node:
        node["positions"] = [] if "id" in node else json.loads(json.dumps(EMPTY))


def fixture_sweep():
    n = 0
    for dp, ds, fs in os.walk(ROOT):
        ds[:] = [d for d in ds if d not in ("target", "node_modules")]
        for fn in fs:
            if not fn.endswith(".json"):
                continue
            p = os.path.join(dp, fn)
            t = rd(p)
            if '"loadAngleRad"' not in t and '"layup"' not in t:
                continue
            if "change-panel-layup" in p:
                continue
            if "/🧫️fixtures/" not in p and "/📚️examples/" not in p:
                print("  skipped (not a fixture):", p.replace(ROOT, "")[:120])
                continue
            try:
                data = json.loads(t)
            except ValueError:
                continue
            if not isinstance(data, (dict, list)):
                continue
            before = json.dumps(data, sort_keys=True)
            walk(data, fix_node)
            if json.dumps(data, sort_keys=True) == before:
                continue
            indent = 2 if t.startswith("{\n  ") or t.startswith("[\n  ") else None
            nt = json.dumps(data, indent=indent, ensure_ascii=False, separators=None if indent else (",", ":")) + ("\n" if t.endswith("\n") else "")
            n += 1
            if not DRY:
                open(p, "w", encoding="utf-8").write(nt)
    print("  fixtures updated:", n)


if __name__ == "__main__":
    layer_model()
    layer_snapshot_ts()
    layer_snapshot_json()
    layer_diff()
    layer_meta()
    layer_sqlite()
    fixture_sweep()
