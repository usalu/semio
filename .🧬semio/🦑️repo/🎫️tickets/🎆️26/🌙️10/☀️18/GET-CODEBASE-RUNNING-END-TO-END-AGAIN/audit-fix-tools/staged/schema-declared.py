#!/usr/bin/env python3
"""Closes the project schema (and the execution contract schema, which gains the optional native `preparation` enum none|production|test: the one place where preparation and input classification currently disagree) over the execution declaration: `metadata.semio.execution` is a closed object (kind enum source|native|orchestration, native requires a scope enum) and every task with a command or commands requires it. Dry run by default; --out <path> writes the patched copy; --write patches live (apply only on GO)."""
import json, os, sys
REPO = os.environ.get("AUDIT_FIX_REPO", "/Users/ueli/Documents/semio")
P = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json")
raw = open(P, encoding="utf8").read(); d = json.loads(raw)
if json.dumps(d, indent=2, ensure_ascii=False) + "\n" != raw: sys.exit("refused: schema is not in canonical two-space form")
defs = d["$defs"]
if "ExecutionDeclaration" in defs: sys.exit("refused: already patched")
task = defs["Task"]; semio = task["properties"]["metadata"]["properties"]["semio"]["properties"]
if "execution" in semio or "allOf" not in task: sys.exit("refused: anchor drift")
defs["ExecutionDeclaration"] = {
  "type": "object", "additionalProperties": False, "required": ["version", "kind"],
  "properties": {"version": {"const": 1}, "kind": {"enum": ["source", "native", "orchestration"]}, "scope": {"enum": ["production", "test"]}, "preparation": {"enum": ["none", "production", "test"]}, "children": {"type": "array"}},
  "allOf": [{"if": {"properties": {"kind": {"const": "native"}}, "required": ["kind"]}, "then": {"required": ["scope"]}, "else": {"not": {"anyOf": [{"required": ["scope"]}, {"required": ["preparation"]}]}}}]}
semio["execution"] = {"$ref": "#/$defs/ExecutionDeclaration"}
task["allOf"].append({"if": {"anyOf": [{"required": ["command"]}, {"required": ["options"], "properties": {"options": {"anyOf": [{"required": ["command"]}, {"required": ["commands"]}]}}}]}, "then": {"required": ["metadata"], "properties": {"metadata": {"required": ["semio"], "properties": {"semio": {"required": ["execution"]}}}}}})
out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
EX = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json")
exraw = open(EX, encoding="utf8").read(); ex = json.loads(exraw)
if json.dumps(ex, indent=2, ensure_ascii=False) + "\n" != exraw: sys.exit("refused: execution schema is not in canonical two-space form")
native = ex["definitions"]["Native"]["properties"]
if "preparation" in native: sys.exit("refused: execution schema already patched")
native["preparation"] = {"enum": ["none", "production", "test"]}
exout = json.dumps(ex, indent=2, ensure_ascii=False) + "\n"
if "--write" in sys.argv: open(P, "w", encoding="utf8").write(out); open(EX, "w", encoding="utf8").write(exout); print("write ok")
elif "--out" in sys.argv: open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf8").write(out); open(sys.argv[sys.argv.index("--out") + 1] + ".execution.json", "w", encoding="utf8").write(exout); print("out ok")
else: print("dry-run ok", len(out) - len(raw), "bytes added")
