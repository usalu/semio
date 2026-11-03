#!/usr/bin/env python3
"""EN 1995 wave 2: hand-applies the snapshot/diff schema change (mCritNm removed; ulsEquation + cases added) to json, ts, proto and graphql twins. Idempotent per run guard."""
import json, re, sys
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/"
SCH = ANY + "🧬️schema/"
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s): open(p, "w", encoding="utf-8").write(s)
def lab(en, de, **kw): return {"label": {"en": en, "de": de}, **kw}
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json"

p = SCH + "📸️snapshot/🔣️.json"; d = json.loads(rd(p))
if "cases" in d["properties"]: sys.exit("already applied")
d["required"] = [x for x in d["required"]] + ["ulsEquation", "cases"]
d["properties"]["ulsEquation"] = {"type": "string", "x-semio-state": "artifact", "x-semio-ui": {"widget": "select", "options": ["6.10", "6.10ab"], **lab("ULS combination equation", "Grundkombination (GZT)"), "description": {"en": "EN 1990 equation 6.10, or the more favourable of 6.10a and 6.10b (xi = 0.85).", "de": "EN 1990 Gleichung 6.10 oder die ungünstigere von 6.10a und 6.10b (xi = 0,85)."}}}
d["properties"]["cases"] = {"type": "array", "items": {"$ref": "#/$defs/CatalogueCase"}, "x-semio-state": "artifact", "x-semio-ui": lab("Catalogue cases", "Katalogfälle")}
tm = d["$defs"]["TimberMember"]; tm["required"].remove("mCritNm"); del tm["properties"]["mCritNm"]
fmt = ["🔣️jsonschema", "🟦️typescript"]
def num(en, de, unit=None, **kw):
    ui = {"label": {"en": en, "de": de}, "widget": "stepper", "precision": 4, "step": 1}
    if unit: ui["unit"] = unit
    ui.update(kw); return {"type": "number", "x-semio-ui": ui}
def txt(en, de, desc=None):
    ui = {"widget": "text", "label": {"en": en, "de": de}}
    if desc: ui["description"] = desc
    return {"type": "string", "x-semio-ui": ui}
d["$defs"]["CaseNumber"] = {"title": "CaseNumber", "type": "object", "additionalProperties": False, "required": ["id", "value"], "properties": {"id": {"type": "string", "x-semio-ui": {"widget": "text", **lab("Input key", "Eingabeschlüssel")}}, "value": num("Value", "Wert", None, precision=6, step=0.1)}, "x-semio-formats": fmt}
d["$defs"]["CaseText"] = {"title": "CaseText", "type": "object", "additionalProperties": False, "required": ["id", "value"], "properties": {"id": {"type": "string", "x-semio-ui": {"widget": "text", **lab("Input key", "Eingabeschlüssel")}}, "value": txt("Value", "Wert")}, "x-semio-formats": fmt}
d["$defs"]["CatalogueCase"] = {"title": "CatalogueCase", "type": "object", "additionalProperties": False, "required": ["id", "labelEn", "labelDe", "entryId", "numbers", "texts"], "properties": {
    "id": {"type": "string"},
    "labelEn": txt("English label", "Englische Bezeichnung"),
    "labelDe": txt("German label", "Deutsche Bezeichnung"),
    "entryId": txt("Catalogue entry", "Katalogeintrag", {"en": "Verification id T-nn of the EN 1995 catalogue.", "de": "Nachweis-Id T-nn des EN-1995-Katalogs."}),
    "numbers": {"type": "array", "items": {"$ref": "#/$defs/CaseNumber"}, "x-semio-ui": lab("Numeric inputs", "Zahlenwerte")},
    "texts": {"type": "array", "items": {"$ref": "#/$defs/CaseText"}, "x-semio-ui": lab("Text inputs", "Textwerte")}}, "x-semio-formats": fmt}
wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")

p = SCH + "🔺️diff/🔣️.json"; x = json.loads(rd(p)); D = x["$defs"]
x["properties"]["ulsEquation"] = {"anyOf": [{"$ref": SNAP + "#/properties/ulsEquation"}, {"type": "null"}], "x-semio-state": "artifact"}
x["properties"]["cases"] = {"$ref": "#/$defs/En1995CaseDelta", "x-semio-state": "artifact"}
del D["En1995MemberPatch"]["properties"]["mCritNm"]
def nullable(ref): return {"anyOf": [{"$ref": ref}, {"type": "null"}]}
def rows(name, ty, fields, nest):
    props = {f: nullable(f"{SNAP}#/$defs/{ty}/properties/{f}") for f in fields}
    for k, v in nest.items(): props[k] = {"$ref": f"#/$defs/{v}"}
    pre = "En1995" + name
    D[pre + "Patch"] = {"title": pre + "Patch", "type": "object", "additionalProperties": False, "properties": props, "x-semio-formats": fmt}
    D[pre + "Removal"] = {"title": pre + "Removal", "type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}, "x-semio-formats": fmt}
    D[pre + "Insertion"] = {"title": pre + "Insertion", "type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0}, "row": {"$ref": f"{SNAP}#/$defs/{ty}"}}, "x-semio-formats": fmt}
    D[pre + "Relocation"] = {"title": pre + "Relocation", "type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0}, "to": {"type": "integer", "minimum": 0}}, "x-semio-formats": fmt}
    D[pre + "Modification"] = {"title": pre + "Modification", "type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": f"#/$defs/{pre}Patch"}}, "x-semio-formats": fmt}
    D[pre + "Delta"] = {"title": pre + "Delta", "type": "object", "additionalProperties": False, "required": ["removed", "inserted", "moved", "modified"], "properties": {"removed": {"type": "array", "items": {"$ref": f"#/$defs/{pre}Removal"}}, "inserted": {"type": "array", "items": {"$ref": f"#/$defs/{pre}Insertion"}}, "moved": {"type": "array", "items": {"$ref": f"#/$defs/{pre}Relocation"}}, "modified": {"type": "array", "items": {"$ref": f"#/$defs/{pre}Modification"}}}, "x-semio-formats": fmt}
rows("CaseNumber", "CaseNumber", ["value"], {})
rows("CaseText", "CaseText", ["value"], {})
rows("Case", "CatalogueCase", ["labelEn", "labelDe", "entryId"], {"numbers": "En1995CaseNumberDelta", "texts": "En1995CaseTextDelta"})
wr(p, json.dumps(x, indent=2, ensure_ascii=False) + "\n")

p = SCH + "📸️snapshot/🟦️.ts"; s = rd(p)
s = s.replace("  mCritNm: number;\n", "").replace("mCritNm: normWireRequired(normWireNumber), ", "")
s = s.replace("  connections: TimberConnection[];\n}", "  connections: TimberConnection[];\n  /** 🧮️ @state artifact */\n  ulsEquation: string;\n  /** 📇️ @state artifact */\n  cases: CatalogueCase[];\n}", 1)
s = s.replace("export interface TimberConnection {", """export interface CaseNumber {
  id: string;
  value: number;
}

export interface CaseText {
  id: string;
  value: string;
}

export interface CatalogueCase {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: CaseNumber[];
  texts: CaseText[];
}

export interface TimberConnection {""", 1)
s = s.replace("connections: normWireRequired(normWireArray(normWireRef(() => parseTimberConnection))) });", "connections: normWireRequired(normWireArray(normWireRef(() => parseTimberConnection))), ulsEquation: normWireRequired(normWireString), cases: normWireRequired(normWireArray(normWireRef(() => parseCatalogueCase))) });", 1)
s = s.replace("export const parseTimberConnection:", """export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCatalogueCase: NormWireReader<CatalogueCase> = normWireObject<CatalogueCase>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
export const parseTimberConnection:""", 1)
wr(p, s)

p = SCH + "🔺️diff/🟦️.ts"; s = rd(p)
s = s.replace("  mCritNm: number | null;\n", "").replace("mCritNm: normWireDefault(normWireNullable(normWireNumber), () => null), ", "")
s = s.replace("  connections?: En1995ConnectionDelta;\n}", "  connections?: En1995ConnectionDelta;\n  /** 🧮️ @state artifact */\n  ulsEquation: string | null;\n  /** 📇️ @state artifact */\n  cases?: En1995CaseDelta;\n}", 1)
s = s.replace("normWireOptional(normWireRef(() => parseEn1995ConnectionDelta)) });", "normWireOptional(normWireRef(() => parseEn1995ConnectionDelta)), ulsEquation: normWireDefault(normWireNullable(normWireString), () => null), cases: normWireOptional(normWireRef(() => parseEn1995CaseDelta)) });", 1)
def tsrows(name, ty, fields, nest):
    pre = "En1995" + name
    fl = "".join(f"  {f}: {t} | null;\n" for f, t in fields) + "".join(f"  {k}?: {v};\n" for k, v in nest)
    iface = f"""export interface {pre}Patch {{
{fl}}}

export interface {pre}Removal {{
  id: string;
  index: number;
}}

export interface {pre}Insertion {{
  index: number;
  row: {ty};
}}

export interface {pre}Relocation {{
  id: string;
  from: number;
  to: number;
}}

export interface {pre}Modification {{
  id: string;
  patch: {pre}Patch;
}}

export interface {pre}Delta {{
  removed: {pre}Removal[];
  inserted: {pre}Insertion[];
  moved: {pre}Relocation[];
  modified: {pre}Modification[];
}}

"""
    pf = ", ".join(f"{f}: normWireDefault(normWireNullable(normWireString), () => null)" if t == "string" else f"{f}: normWireDefault(normWireNullable(normWireNumber), () => null)" for f, t in fields)
    if nest: pf += ", " + ", ".join(f"{k}: normWireOptional(normWireRef(() => parse{v}))" for k, v in nest)
    I = '{"minimum":0}'
    par = f"""export const parse{pre}Patch: NormWireReader<{pre}Patch> = normWireObject<{pre}Patch>({{ {pf} }});
export const parse{pre}Removal: NormWireReader<{pre}Removal> = normWireObject<{pre}Removal>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {I})) }});
export const parse{pre}Insertion: NormWireReader<{pre}Insertion> = normWireObject<{pre}Insertion>({{ index: normWireRequired(normWireRange(normWireInteger, {I})), row: normWireRequired(parse{ty}) }});
export const parse{pre}Relocation: NormWireReader<{pre}Relocation> = normWireObject<{pre}Relocation>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {I})), to: normWireRequired(normWireRange(normWireInteger, {I})) }});
export const parse{pre}Modification: NormWireReader<{pre}Modification> = normWireObject<{pre}Modification>({{ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse{pre}Patch)) }});
export const parse{pre}Delta: NormWireReader<{pre}Delta> = normWireObject<{pre}Delta>({{ removed: normWireRequired(normWireArray(normWireRef(() => parse{pre}Removal))), inserted: normWireRequired(normWireArray(normWireRef(() => parse{pre}Insertion))), moved: normWireRequired(normWireArray(normWireRef(() => parse{pre}Relocation))), modified: normWireRequired(normWireArray(normWireRef(() => parse{pre}Modification))) }});
"""
    return iface, par
ifs, pars = "", ""
for a in [("CaseNumber", "CaseNumber", [("value", "number")], []), ("CaseText", "CaseText", [("value", "string")], []), ("Case", "CatalogueCase", [("labelEn", "string"), ("labelDe", "string"), ("entryId", "string")], [("numbers", "En1995CaseNumberDelta"), ("texts", "En1995CaseTextDelta")])]:
    i, q = tsrows(*a); ifs += i; pars += q
s = s.replace("export const parseEn1995Diff:", ifs + "export const parseEn1995Diff:", 1)
s = s.rstrip("\n") + "\n" + pars
s = re.sub(r"import \{ type CharacteristicAction, type ConnectionAction,", "import { type CatalogueCase, type CaseNumber, type CaseText, type CharacteristicAction, type ConnectionAction,", s, 1)
s = s.replace("parseCharacteristicAction, parseConnectionAction,", "parseCaseNumber, parseCaseText, parseCatalogueCase, parseCharacteristicAction, parseConnectionAction,", 1)
wr(p, s)

p = SCH + "📸️snapshot/🛰️.proto"; s = rd(p); s = s.replace("  repeated bytes connections = 3;\n", "  repeated bytes connections = 3;\n  string uls_equation = 4;\n  repeated bytes cases = 5;\n"); wr(p, s)
p = SCH + "📸️snapshot/🔗️.graphql"; s = rd(p); s = s.replace("  connections: [JSON!]!\n", "  connections: [JSON!]!\n  ulsEquation: String!\n  cases: [JSON!]!\n"); wr(p, s)
p = SCH + "🔺️diff/🛰️.proto"; s = rd(p); s = s.replace("  bytes connections = 4;\n", "  bytes connections = 4;\n  // @state artifact\n  string uls_equation = 5;\n  // @state artifact\n  bytes cases = 6;\n"); wr(p, s)
p = SCH + "🔺️diff/🔗️.graphql"; s = rd(p); s = s.replace("  connections: JSON @state(class: ARTIFACT)\n", "  connections: JSON @state(class: ARTIFACT)\n  ulsEquation: String @state(class: ARTIFACT)\n  cases: JSON @state(class: ARTIFACT)\n"); wr(p, s)
print("ok")
