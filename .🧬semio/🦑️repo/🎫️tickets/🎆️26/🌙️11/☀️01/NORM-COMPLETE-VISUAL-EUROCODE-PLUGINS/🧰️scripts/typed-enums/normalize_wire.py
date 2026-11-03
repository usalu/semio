#!/usr/bin/env python3
"""🧹 Typed enums v5: rewrites the legacy lower-case enum tokens that the converter left in committed JSON fixtures of one artifact to the wire tokens of the spec (variant name), keyed by the field name and, for the ambiguous `kind`/`category`/`shape`, by the owning array.
usage: normalize_wire.py ART [--apply]   (without --apply it only counts)"""
import glob, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import convert as C
art = sys.argv[1]
apply = "--apply" in sys.argv
spec = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "specs", art + ".json"), encoding="utf-8"))
root = C.artifact_root(art)
OWNERS = {"MaterialKind": "materials", "SectionKind": "sections", "ActionKind": "loadCases", "JointKind": "joints", "JointCategory": "joints", "BuiltUpKind": "builtUpMembers", "ParametricShape": "parametricSections"}
rules = {}
AMBIGUOUS = {"kind", "category", "shape"}
for c in spec["conversions"]:
    legacy = {}
    for name, toks, *_ in c["variants"]:
        for t in toks + [name, name.lower()]:
            legacy[t] = name
    for host in c["hosts"]:
        key = C.camel(host.split(".")[1])
        if c["enum"] in OWNERS:
            rules[(OWNERS[c["enum"]], key)] = (legacy, c["default"])
        elif key not in AMBIGUOUS:
            rules[(None, key)] = (legacy, c["default"])
INSERT = {"material": "materials", "section": "sections", "load-case": "loadCases", "joint": "joints", "built-up-member": "builtUpMembers", "parametric-section": "parametricSections"}
changed = files = 0


def walk(node, owner):
    global changed
    if isinstance(node, dict):
        for k, v in list(node.items()):
            rk = (owner, k) if (owner, k) in rules else (None, k)
            if isinstance(v, str) and rk in rules:
                legacy, default = rules[rk]
                new = legacy.get(v, None)
                if new is None and v not in legacy.values():
                    new = default if (owner, k) in rules and "-alt" not in v and not v.startswith("changed-") else None
                if new is not None and new != v:
                    node[k] = new
                    changed += 1
            else:
                walk(v, k if isinstance(v, list) else owner)
    elif isinstance(node, list):
        for v in node:
            walk(v, owner)


for p in glob.glob(root + "/**/*.json", recursive=True):
    if "/target/" in p or "/node_modules/" in p or p.endswith(("🔣️.json",)) and "🧬️schema" in p:
        continue
    t = open(p, encoding="utf-8", newline="").read()
    try:
        d = json.loads(t)
    except Exception:
        continue
    out = json.dumps(d, indent=2, ensure_ascii=False) + ("\n" if t.endswith("\n") else "")
    if out != t:
        print("skip (layout not canonical):", p.replace(root, ""))
        continue
    before = changed
    m = re.search(r"insert-([a-z-]+)/", p)
    walk(d, INSERT.get(m.group(1)) if m else None)
    if changed != before and apply:
        out = json.dumps(d, indent=2, ensure_ascii=False) + ("\n" if t.endswith("\n") else "")
        open(p, "w", encoding="utf-8", newline="").write(out)
        files += 1
print(f"{art}: {changed} values, {files} files written" if apply else f"{art}: {changed} values would change")
