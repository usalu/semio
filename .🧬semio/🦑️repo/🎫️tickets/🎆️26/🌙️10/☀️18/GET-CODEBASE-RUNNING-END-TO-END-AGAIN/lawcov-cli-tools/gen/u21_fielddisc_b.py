import json, lib
from lib import S
V = "️"
def fld(name, optional, card, scalar, state=""):
    return {"name": name, "optional": optional, "cardinality": card, "scalar": scalar, "state": state}
FLD = S.obj({"name": S.str, "optional": S.bool, "cardinality": S.enum("scalar", "list", "fixedList", "map"), "scalar": S.str, "state": S.str})
EXTRACT = S.obj({"typeName": S.str, "fields": S.arr(FLD)})
# ======================= json-schema
UJ = "🧬schema/🔍field-discovery/🔣json-schema"
scalars = [({"contentEncoding": "base64", "type": "string"}, "bytes"), ({"contentMediaType": "application/json", "type": "string"}, "string"), ({"type": "string"}, "string"), ({"type": "boolean"}, "bool"), ({"type": "integer", "format": "int32"}, "int32"), ({"type": "integer", "format": "uint32"}, "uint32"),
           ({"type": "integer", "format": "int64"}, "int64"), ({"type": "integer", "format": "uint64"}, "uint64"), ({"type": "integer"}, "int32"), ({"type": "integer", "format": "weird"}, "int32"), ({"type": "number", "format": "float"}, "float32"), ({"type": "number", "format": "double"}, "float64"), ({"type": "number"}, "float64"),
           ({"$ref": "#/$defs/Foo"}, "Foo"), ({"$ref": "plain"}, "plain"), ({"title": "Thing"}, "Thing"), ({"type": "null"}, "null"), ({}, "unknown"), ({"type": ["string", "null"]}, "unknown"), ({"type": "integer", "$ref": "x"}, "int32"), ({"$ref": "x", "title": "T"}, "x"), ({"type": "object"}, "object"), ({"type": "array"}, "array")]
props = [({"type": "array", "items": {"type": "string"}}, "list", "string"), ({"type": "array", "items": {"type": "number"}, "minItems": 2, "maxItems": 2}, "fixedList", "float64"), ({"type": "array"}, "list", "unknown"), ({"type": "array", "items": {"type": "string"}, "minItems": 2, "maxItems": 3}, "list", "string"), ({"type": "array", "items": {"type": "string"}, "minItems": 2}, "list", "string"),
         ({"type": "array", "items": {"$ref": "#/$defs/Item"}, "minItems": 1, "maxItems": 1}, "fixedList", "Item"), ({"type": "object", "additionalProperties": {"type": "string"}}, "map", "string"), ({"type": "object", "additionalProperties": True}, "map", "unknown"), ({"type": "object", "additionalProperties": False}, "scalar", "object"), ({"type": "object", "additionalProperties": {}}, "map", "unknown"),
         ({"type": "object", "additionalProperties": None}, "scalar", "object"), ({"type": "string"}, "scalar", "string"), ({"$ref": "#/$defs/Foo"}, "scalar", "Foo")]
doc1 = {"title": "Widget", "type": "object", "required": ["id", "count", "tags"], "properties": {"id": {"type": "string"}, "displayName": {"type": "string"}, "count": {"type": "integer", "format": "int32"}, "big": {"type": "integer", "format": "uint64"}, "tags": {"type": "array", "items": {"type": "string"}}, "pair": {"type": "array", "items": {"type": "number"}, "minItems": 2, "maxItems": 2},
        "labels": {"type": "object", "additionalProperties": {"type": "string"}}, "blob": {"type": "string", "contentEncoding": "base64"}, "status": {"type": "string", "x-semio-state": "PERSISTED_SHARED"}, "mode": {"type": "string", "x-semio-state": 7}, "emptyState": {"type": "string", "x-semio-state": ""}}}
exp1 = {"typeName": "Widget", "fields": [fld("id", False, "scalar", "string"), fld("displayName", True, "scalar", "string"), fld("count", False, "scalar", "int32"), fld("big", True, "scalar", "uint64"), fld("tags", False, "list", "string"), fld("pair", True, "fixedList", "float64"), fld("labels", True, "map", "string"), fld("blob", True, "scalar", "bytes"), fld("status", True, "scalar", "string", "persisted-shared"), fld("mode", True, "scalar", "string"), fld("emptyState", True, "scalar", "string")]}
extracts = [
 {"name": "a full document", "text": json.dumps(doc1), "expected": exp1, "ajv": True},
 {"name": "invalid JSON yields nothing", "text": "{not json", "expected": {"typeName": "", "fields": []}, "ajv": False},
 {"name": "a document without properties", "text": json.dumps({"title": "Empty"}), "expected": {"typeName": "Empty", "fields": []}, "ajv": True},
 {"name": "a title that is not a string", "text": json.dumps({"title": 3, "properties": {"a": {"type": "string"}}}), "expected": {"typeName": "", "fields": [fld("a", True, "scalar", "string")]}, "ajv": False},
 {"name": "required that is not an array makes everything optional", "text": json.dumps({"title": "T", "required": "a", "properties": {"a": {"type": "string"}}}), "expected": {"typeName": "T", "fields": [fld("a", True, "scalar", "string")]}, "ajv": False},
 {"name": "a null property is treated as an empty schema", "text": json.dumps({"title": "T", "properties": {"a": None}}), "expected": {"typeName": "T", "fields": [fld("a", True, "scalar", "unknown")]}, "ajv": False},
 {"name": "the empty array is not an object document", "text": "[]", "expected": {"typeName": "", "fields": []}, "ajv": False},
]
corpus_j = {"version": 1, "scalar": [{"schema": s, "expected": e} for s, e in scalars], "property": [{"schema": s, "cardinality": c, "scalar": sc} for s, c, sc in props], "extract": extracts}
sch_j = S.obj({"version": S.const(1), "scalar": S.arr(S.obj({"schema": S.anyv, "expected": S.str}), 1), "property": S.arr(S.obj({"schema": S.anyv, "cardinality": S.str, "scalar": S.str}), 1), "extract": S.arr(S.obj({"name": S.str, "text": S.str, "expected": EXTRACT, "ajv": S.bool}), 1)})
body_j = '''for(const[index,row]of corpus.scalar.entries())test("scalar of schema "+index+": "+JSON.stringify(row.schema),()=>{expect(policyJsonSchemaScalar(row.schema as Record<string,unknown>)).toBe(row.expected);});
for(const[index,row]of corpus.property.entries())test("property shape "+index+": "+JSON.stringify(row.schema),()=>{expect(policyParseJsonSchemaProperty(row.schema as Record<string,unknown>)).toEqual({cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("extract: "+row.name,()=>{expect(policyExtractJsonSchemaFields(row.text)).toEqual(row.expected);});
for(const row of corpus.extract.filter(item=>item.ajv))test("Ajv reads the same title and required set as the extractor: "+row.name,()=>{
 const ajv=new Ajv({strict:false,allErrors:true}),document=JSON.parse(row.text);expect(ajv.validateSchema(document)).toBe(true);
 const validate=ajv.compile(document);expect(validate({})).toBe(Object.values(row.expected.fields).some(field=>!field.optional)?false:true);
 const missing=(validate.errors??[]).filter(error=>error.keyword==="required").map(error=>(error.params as {missingProperty:string}).missingProperty).sort();
 expect(missing).toEqual(row.expected.fields.filter(field=>!field.optional).map(field=>field.name).sort());expect(document.title??"").toBe(row.expected.typeName);
 expect(Object.keys(document.properties??{})).toEqual(row.expected.fields.map(field=>field.name));
});'''
ts_j = lib.ts_wrap(UJ, "🔣️ Contract: the JSON Schema field extractor maps each property to a canonical scalar and cardinality, reads optionality from `required` and the state class from `x-semio-state`, and yields nothing for invalid JSON; the corpus is language-agnostic, Ajv is the third-party oracle for title, property order and the required set, and `@@O@@` re-derives the shapes.",
 'import {policyExtractJsonSchemaFields,policyJsonSchemaScalar,policyParseJsonSchemaProperty} from "@@' + UJ + '/🟦.ts@@";', body_j).replace('import Ajv from "ajv";', 'import Ajv from "ajv";', 1)
orc_j = lib.PY_HEAD.format(doc="🐍️ Independent model of the JSON Schema field extraction (stdlib json only).") + r'''
def scalar(s):
    if s.get("contentEncoding") == "base64":
        return "bytes"
    if s.get("contentMediaType") == "application/json":
        return "string"
    t, f = s.get("type"), s.get("format")
    if t == "string":
        return "string"
    if t == "boolean":
        return "bool"
    if t == "integer":
        return f if f in ("int32", "uint32", "int64", "uint64") else "int32"
    if t == "number":
        return {"float": "float32", "double": "float64"}.get(f, "float64")
    if isinstance(s.get("$ref"), str):
        return s["$ref"].split("/")[-1]
    if isinstance(s.get("title"), str):
        return s["title"]
    return t if isinstance(t, str) else "unknown"


def prop(p):
    if p.get("type") == "array":
        sc = scalar(p["items"]) if p.get("items") else "unknown"
        fixed = isinstance(p.get("minItems"), int) and not isinstance(p.get("minItems"), bool) and p.get("minItems") == p.get("maxItems")
        return ("fixedList" if fixed else "list", sc)
    ap = p.get("additionalProperties")
    if p.get("type") == "object" and "additionalProperties" in p and ap is not None and ap is not False:
        return ("map", scalar(ap) if isinstance(ap, dict) else "unknown")
    return ("scalar", scalar(p))


def extract(text):
    try:
        doc = json.loads(text)
    except ValueError:
        return {"typeName": "", "fields": []}
    if not isinstance(doc, dict):
        return {"typeName": "", "fields": []}
    props = doc.get("properties") or {}
    req = set(doc["required"]) if isinstance(doc.get("required"), list) else set()
    out = []
    for name, p in props.items():
        p = p or {}
        card, sc = prop(p)
        st = p.get("x-semio-state")
        out.append({"name": name, "optional": name not in req, "cardinality": card, "scalar": sc, "state": st.strip().lower().replace("_", "-") if isinstance(st, str) and st else ""})
    return {"typeName": doc["title"] if isinstance(doc.get("title"), str) else "", "fields": out}


for c in corpus["scalar"]:
    if scalar(c["schema"]) != c["expected"]:
        failures.append("scalar %r" % c["schema"])
for c in corpus["property"]:
    if prop(c["schema"]) != (c["cardinality"], c["scalar"]):
        failures.append("property %r" % c["schema"])
for c in corpus["extract"]:
    if extract(c["text"]) != c["expected"]:
        failures.append("extract: " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UJ, corpus_j, sch_j, ts_j, orc_j)
