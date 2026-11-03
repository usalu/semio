"""Hand-port of the design-case collection into the en1994 JSON schemas (snapshot, artifact, diff), format preserving. Run: /home/deck/semio/.venv/bin/python -I cases_json.py"""
import json, os, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/catalogue-bridge-v2")
os.environ["TH_FORCE"] = "1"
from lib import *
import twins as W, jsonspan as J
A = Art("en1994")
p = A.snap + "/🔣️.json"
t = rd(p)
if '"cases"' not in json.loads(t)["properties"]:
    defs = W.tmpl_defs(A)
    t = J.add_member(t, ["properties"], "cases", {"type": "array", "items": {"$ref": "#/$defs/CaseRow"}, "x-semio-state": "artifact", "x-semio-dsl": {"table": True}})
    t = J.add_item(t, ["required"], "cases")
    for n in W.NAMES:
        d = json.loads(json.dumps(defs[n], ensure_ascii=False).replace("T-nn", "C94-nn"))
        d["x-semio-formats"] = ["🔣️jsonschema", "🟦️typescript"]
        t = J.add_member(t, ["$defs"], n, d)
    json.loads(t)
    wr(p, t)
p = A.schema + "/🔣️.json"
t = rd(p)
if '"cases"' not in json.loads(t)["properties"]:
    num = {"type": "object", "additionalProperties": False, "properties": {"id": {"type": "string"}, "value": {"type": "number", "format": "double"}}, "required": ["id", "value"]}
    txt = {"type": "object", "additionalProperties": False, "properties": {"id": {"type": "string"}, "value": {"type": "string"}}, "required": ["id", "value"]}
    row = {"type": "object", "additionalProperties": False, "properties": {"id": {"type": "string"}, "labelEn": {"type": "string"}, "labelDe": {"type": "string"}, "entryId": {"type": "string"}, "numbers": {"type": "array", "items": {"$ref": "#/$defs/CaseNumber"}}, "texts": {"type": "array", "items": {"$ref": "#/$defs/CaseText"}}}, "required": ["id", "labelEn", "labelDe", "entryId", "numbers", "texts"]}
    t = J.add_member(t, ["properties"], "cases", {"type": "array", "items": {"$ref": "#/$defs/CaseRow"}, "x-semio-state": "artifact"})
    t = J.add_item(t, ["required"], "cases")
    for n, d in (("CaseNumber", num), ("CaseText", txt), ("CaseRow", row)):
        t = J.add_member(t, ["$defs"], n, d)
    json.loads(t)
    wr(p, t)
p = A.diff + "/🔣️.json"
if '"cases"' not in json.loads(rd(p))["properties"]:
    W.diff_json(A)
print(len(LOG))
