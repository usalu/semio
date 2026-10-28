"""python3 router-read.py <precision-dir> : adds the `router` read to the input precision engine and its schema.
A router script registers its commands (`.register("test-x", ClassName)`); the read takes the class registered under the first word of the INNER command (the words after the last script token, so process-module wrappers work), narrows it to the `if (segments[0] === "word")` branch of the next word when the class has one, reads the literals of that text plus the top-level string lists it mentions (Cargo package names, repository paths, test files) and bounds the target by them: the crate closure of the named packages, the import closure of the TypeScript files, the files and directories named. Source snippets live in patches/router/*.ts.txt.
Anchored, idempotent, all-or-nothing; the schema is validated before it is written."""
import json, os, sys
base = sys.argv[1]
here = os.path.join(os.path.dirname(os.path.abspath(__file__)), "router")
engine, schema = base + "/🟦️.ts", base + "/🧬️schema/🔣️.json"
E = open(engine, encoding="utf-8").read()
S = open(schema, encoding="utf-8").read()
functions = open(os.path.join(here, "functions.ts.txt"), encoding="utf-8").read()
apply = open(os.path.join(here, "apply.ts.txt"), encoding="utf-8").read()


def edit(text, anchor, new, marker):
    if marker in text:
        return text
    if text.count(anchor) != 1:
        sys.exit(f"anchor not unique ({text.count(anchor)}): {anchor[:80]}")
    return text.replace(anchor, new)


E = edit(E, "  | {readonly owner: \"route\"}\n", "  | {readonly router: {readonly source: string; readonly classes?: string; readonly bases?: readonly string[]}}\n  | {readonly owner: \"route\"}\n", "readonly router: {readonly source")
E = edit(E, "/** 🔖️ What one dispatcher branch names:", functions + "/** 🔖️ What one dispatcher branch names:", "export function innerCommandWordsV1")
E = edit(E, "    } else if (\"program\" in read) {\n      const program = await programInputsV1(", apply + "    } else if (\"program\" in read) {\n      const program = await programInputsV1(", "} else if (\"router\" in read)")
d = json.loads(S)
defs = d["definitions"]
if "RouterRead" not in defs:
    defs["RouterRead"] = {"type": "object", "additionalProperties": False, "required": ["router"], "properties": {"router": {"type": "object", "additionalProperties": False, "required": ["source"], "properties": {"source": {"$ref": "#/definitions/Path"}, "classes": {"$ref": "#/definitions/Path"}, "bases": {"type": "array", "minItems": 1, "maxItems": 8, "items": {"$ref": "#/definitions/Path"}}}}}}
    for name in ("Read", "FlatRead"):
        defs[name]["oneOf"].append({"$ref": "#/definitions/RouterRead"})
    S2 = json.dumps(d, ensure_ascii=False, indent=2) + "\n"
else:
    S2 = S
json.loads(S2)
open(engine, "w", encoding="utf-8").write(E)
open(schema, "w", encoding="utf-8").write(S2)
print("router read applied to", base)
