#!/usr/bin/env python3
"""Writes the language-agnostic corpus of the local config event log: log text -> projection or refusal line."""
import json
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D = "staged-config/files/" + L + "/🏗️build/🎚️config/🧪️tests/"
A, B, C = "0a1b2c3d-0000-4000-8000-000000000001", "0a1b2c3d-0000-4000-8000-000000000002", "0a1b2c3d-0000-4000-8000-000000000003"
T = "2026-10-21T17:00:00.000Z"
def ev(id, type, **kw): return json.dumps({"version": 1, "id": id, "at": T, "type": type, **kw}, separators=(",", ":"))
de, en = lambda i: ev(i, "language-chosen", language="de"), lambda i: ev(i, "language-chosen", language="en")
clr = lambda i: ev(i, "language-cleared")
def ok(name, text, events, language=None): return {"name": name, "text": text, "expected": {"events": events, **({"language": language} if language else {})}}
def bad(name, text, line): return {"name": name, "text": text, "expected": {"refusal": {"line": line}}}
logs = [
 ok("an empty log", "", 0),
 ok("one chosen language", de(A) + "\n", 1, "de"),
 ok("the last choice wins", de(A) + "\n" + en(B) + "\n", 2, "en"),
 ok("a clearing event removes the choice", de(A) + "\n" + clr(B) + "\n", 2),
 ok("a choice after a clearing event", clr(A) + "\n" + en(B) + "\n", 2, "en"),
 ok("blank lines are ignored", "\n" + de(A) + "\n\n" + en(B) + "\n\n", 2, "en"),
 ok("a torn last line is ignored", de(A) + "\n" + en(B)[:30], 1, "de"),
 ok("a last line without a newline that is complete counts", de(A) + "\n" + en(B), 2, "en"),
 bad("an invalid line in the middle is refused", de(A) + "\nnot json\n" + en(B) + "\n", 2),
 bad("an unoffered language is refused", ev(A, "language-chosen", language="fr") + "\n", 1),
 bad("an unknown event type is refused", de(A) + "\n" + ev(B, "theme-chosen") + "\n", 2),
 bad("an extra property is refused", ev(A, "language-cleared", language="de") + "\n", 1),
 bad("a missing language is refused", ev(A, "language-chosen") + "\n", 1),
 bad("a bad id is refused", de("not-an-id") + "\n", 1),
 bad("an instant without milliseconds is refused", json.dumps({"version": 1, "id": A, "at": "2026-10-21T17:00:00Z", "type": "language-cleared"}) + "\n", 1),
 bad("version two is refused", json.dumps({"version": 2, "id": A, "at": T, "type": "language-cleared"}) + "\n", 1),
 bad("an array is refused", "[1]\n", 1),
]
open(D + "🔣️.json", "w", encoding="utf8").write(json.dumps({"version": 1, "logs": logs}, ensure_ascii=False, indent=1) + "\n")
schema = {"$schema": "http://json-schema.org/draft-07/schema#", "title": "Local Config Log Corpus", "description": "Language-agnostic cases of the local config event log: the text of a log and the configuration it folds to, or the line that is refused.", "type": "object", "additionalProperties": False, "required": ["version", "logs"], "properties": {"version": {"const": 1}, "logs": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["name", "text", "expected"], "properties": {"name": {"type": "string", "minLength": 1}, "text": {"type": "string"}, "expected": {"type": "object", "additionalProperties": False, "properties": {"events": {"type": "integer", "minimum": 0}, "language": {"enum": ["en", "de"]}, "refusal": {"type": "object", "additionalProperties": False, "required": ["line"], "properties": {"line": {"type": "integer", "minimum": 1}}}}}}}}}}
open(D + "🧬️schema/🔣️.json", "w", encoding="utf8").write(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
print(len(logs), "logs")
