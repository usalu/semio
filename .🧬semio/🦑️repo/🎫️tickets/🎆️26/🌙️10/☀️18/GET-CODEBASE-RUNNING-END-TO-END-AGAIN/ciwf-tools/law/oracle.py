#!/usr/bin/env python3
"""🔮 Independent oracle for the workflow law: PyYAML (strict loader that rejects duplicate keys) + jsonschema (Draft 7) against the schemastore schemas next to this file; also lists every external `uses:` ref. usage: python3 oracle.py <root> [--uses]   (needs PyYAML and jsonschema; exit 3 when they are missing, never a silent pass)."""
import glob, json, os, re, sys
try:
    import yaml, jsonschema
except ImportError as e:
    print("oracle unavailable:", e); sys.exit(3)
HERE = os.path.dirname(os.path.abspath(__file__))
FIX = os.path.join(HERE, "..", "fixtures")
class Strict(yaml.SafeLoader): pass
def mapping(loader, node, deep=False):
    seen = set()
    for k, _ in node.value:
        key = loader.construct_object(k, deep=deep)
        if key in seen: raise yaml.constructor.ConstructorError(None, None, "duplicate key %r" % (key,), k.start_mark)
        seen.add(key)
    return yaml.SafeLoader.construct_mapping(loader, node, deep)
Strict.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, mapping)
def schema(name):
    with open(os.path.join(FIX, name), encoding="utf8") as f: return json.load(f)
def check(path, sch):
    out = []
    try:
        with open(path, encoding="utf8") as f: doc = yaml.load(f, Strict)
        if isinstance(doc, dict) and True in doc: doc["on"] = doc.pop(True)
    except Exception as e:
        return [(path, "yaml", str(e).replace("\n", " "))], None
    for err in sorted(jsonschema.Draft7Validator(sch).iter_errors(doc), key=lambda e: list(map(str, e.path))):
        out.append((path, "/".join(map(str, err.absolute_path)), err.message[:200]))
    return out, doc
def main():
    root = sys.argv[1]; uses = set(); findings = []
    wf, ac = schema("github-workflow.schema.json"), schema("github-action.schema.json")
    for p in sorted(glob.glob(os.path.join(root, ".github", "workflows", "*.y*ml"))):
        f, doc = check(p, wf); findings += f
        for j in ((doc or {}).get("jobs") or {}).values():
            if "uses" in j: uses.add(j["uses"])
            for s in j.get("steps", []):
                if "uses" in s: uses.add(s["uses"])
    for p in sorted(glob.glob(os.path.join(root, ".github", "actions", "*", "action.y*ml"))):
        f, doc = check(p, ac); findings += f
        for s in ((doc or {}).get("runs") or {}).get("steps", []):
            if "uses" in s: uses.add(s["uses"])
    for f in findings: print("%s: %s: %s" % (os.path.relpath(f[0], root), f[1], f[2]))
    print("%d finding(s)" % len(findings))
    if "--uses" in sys.argv:
        for u in sorted(uses): print("uses", u)
    sys.exit(1 if findings else 0)
main()
