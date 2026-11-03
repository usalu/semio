"""🔍 Static audit: clip scripts for primitive-built objects, cast ids, energy views, orphans and stale allowlist entries."""

import ast
import json
import re
import sys
from pathlib import Path

REPO = Path("C:/git/semio")
A = REPO / "🎓️teaching" / "🏛️architecture"
V = A / "⚡️energy" / "🏗️authoring" / "🎨️visuals"
MANIFEST = json.loads((V / "🧍️staffage" / "🔣️.json").read_text(encoding="utf-8"))["casts"]
AL = json.loads((V / "🧪️tests" / "🔣️staffage_only.json").read_text(encoding="utf-8"))
RULE_A = AL["rule_a"]
RULE_B = AL["rule_b"]
CAST_CALLS = set(AL["cast_calls"]) | {"watt_anchor", "pendant_lamp"}
SKIP = set(AL["skip_folders"])
NOUN = re.compile(RULE_A["object_nouns"])
PRIMS = set(RULE_A["primitives"])
STAFF = set(RULE_A["staffage_calls"])
OUT = []


def emit(text=""):
    OUT.append(text)


def rel(path):
    return path.relative_to(A).as_posix()


CACHE = {}


def source(path):
    return path.read_text(encoding="utf-8", errors="replace")


def parse(path):
    if path not in CACHE:
        CACHE[path] = ast.parse(source(path), filename=str(path))
    return CACHE[path]


def all_py():
    return sorted(p for p in A.rglob("*.py") if not set(p.parts) & SKIP)


def is_clip(path):
    return AL["clip_folder"] in path.parts and "🧪️tests" not in path.parts


def is_helper(path):
    return path in (V / "🐍️.py", V / "🧍️staffage" / "🐍️.py")


def is_clip_file(path):
    return path.name == AL["file_name"] and (is_clip(path) or is_helper(path))


def string_parts(node, consts):
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value, True
    if isinstance(node, ast.Name) and node.id in consts:
        return consts[node.id], True
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Add):
        left, ok = string_parts(node.left, consts)
        if not ok:
            return left, False
        right, ok2 = string_parts(node.right, consts)
        return left + right, ok2
    if isinstance(node, ast.JoinedStr):
        text = ""
        for part in node.values:
            piece, ok = string_parts(part.value if isinstance(part, ast.FormattedValue) else part, consts)
            text += piece
            if not ok:
                return text, False
        return text, True
    return "", False


def module_consts(tree):
    out = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
            for t in node.targets:
                if isinstance(t, ast.Name):
                    out[t.id] = node.value.value
    return out


def callee(call):
    f = call.func
    return f.id if isinstance(f, ast.Name) else f.attr if isinstance(f, ast.Attribute) else None


def cast_refs(path):
    """Cast-call arguments: (line, text, complete) for staffage calls and on_cast/off_cast kwargs."""
    tree = parse(path)
    consts = module_consts(tree)
    refs = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        args = []
        if callee(node) in CAST_CALLS:
            args += node.args[:1]
        args += [k.value for k in node.keywords if k.arg in ("on_cast", "off_cast")]
        for a in args:
            text, ok = string_parts(a, consts)
            if ok and not text and not isinstance(a, ast.Constant):
                continue
            refs.append((node.lineno, text, ok))
    return refs


def expand(text, ok):
    if ok:
        return [text] if text in MANIFEST else [], text
    if text.endswith("-"):
        return [n for n in MANIFEST if n.startswith(text)], text + "*"
    return [], None


def section(title):
    emit("")
    emit("## " + title)


clips = [p for p in all_py() if is_clip_file(p)]
helpers = [p for p in clips if is_helper(p)]
emit(f"clip+helper files: {len(clips)} (helpers {len(helpers)})")

# 1. Missing ids
section("1 missing cast ids (literal not in manifest)")
missing, dynamic_unmatched = [], []
used = {}
for p in clips:
    for line, text, ok in cast_refs(p):
        matched, label = expand(text, ok)
        if ok and text not in MANIFEST:
            missing.append((rel(p), line, text))
        elif not ok and text.endswith("-") and not matched:
            dynamic_unmatched.append((rel(p), line, text))
        for m in matched:
            used.setdefault(m, set()).add(rel(p))
for m in missing:
    emit(f"MISSING {m[0]}:{m[1]} '{m[2]}'")
emit(f"missing count {len(missing)}")
for d in dynamic_unmatched:
    emit(f"DYNAMIC-NO-MATCH {d[0]}:{d[1]} '{d[2]}'")

# 2. Energy view rule
section("2 energy clip casts that are not thermal and not allowlisted (rule_b)")
allowed_b = set(RULE_B["allowed"])
viol = []
for p in clips:
    if "⚡️energy" not in p.parts:
        continue
    for line, text, ok in cast_refs(p):
        matched, label = expand(text, ok)
        for m in matched:
            if MANIFEST[m]["view"] != "thermal" and m not in allowed_b:
                viol.append((rel(p), line, m, MANIFEST[m]["view"]))
for v in sorted(set(viol)):
    emit(f"VIEW {v[0]}:{v[1]} {v[2]} view={v[3]}")
emit(f"view violations {len(set(viol))}")

# 3. Orphans: manifest ids with no literal or prefix use anywhere in clips, helpers, or other non-test code
section("3 manifest orphans (no clip/helper reference)")
test_refs = {}
other_refs = {}
for p in all_py():
    consts = module_consts(parse(p))
    vals = set()
    for node in ast.walk(parse(p)):
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            vals.add(node.value)
    for name in MANIFEST:
        hit = name in vals or any(v.endswith("-") and name.startswith(v) and len(v) > 4 for v in vals)
        if hit:
            bucket = test_refs if "🧪️tests" in p.parts else other_refs
            bucket.setdefault(name, set()).add(rel(p))
orphans = [n for n in MANIFEST if n not in used and n not in other_refs]
test_only = [n for n in MANIFEST if n not in used and n not in other_refs and n in test_refs]
emit("orphans (no clip, helper or non-test reference):")
for n in orphans:
    emit(f"  ORPHAN {n} testonly={sorted(test_refs.get(n, []))}")
emit(f"orphan count {len(orphans)}")
emit("referenced only by tests (non-orphan):")
for n in test_only:
    emit(f"  TESTONLY {n}")
for n in sorted(other_refs):
    if n not in used:
        emit(f"  NONCLIP {n} in {sorted(other_refs[n])}")
emit(f"manifest casts {len(MANIFEST)}, used by clips/helpers {len(used)}")

# 4. Stale allowlist
section("4 stale allowlist entries")
defined = {}
for p in all_py():
    names = set()
    for node in ast.walk(parse(p)):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names.add(node.name)
        elif isinstance(node, ast.Assign):
            for t in node.targets:
                if isinstance(t, ast.Name):
                    names.add(t.id)
    defined[rel(p)] = names
for key in RULE_A["allowed"]:
    path_part, _, name = key.partition("::")
    target = A / path_part
    if not target.exists():
        emit(f"STALE-A file gone: {key}")
    elif name not in defined.get(path_part, set()):
        emit(f"STALE-A name gone: {key}")
    else:
        emit(f"ok-A {key}")
for cast in RULE_B["allowed"]:
    if cast not in MANIFEST:
        emit(f"STALE-B not in manifest: {cast}")
    elif cast not in used:
        emit(f"STALE-B unused by any clip: {cast}")
    else:
        emit(f"ok-B {cast} used by {sorted(used[cast])[:2]}")
for cast in AL["forbidden_casts"]:
    if cast in MANIFEST:
        emit(f"FORBIDDEN-PRESENT in manifest: {cast}")
for name in AL["forbidden_names"]:
    hits = [rel(p) for p in all_py() if name in source(p)]
    if hits and name in json.dumps(AL):
        emit(f"forbidden name '{name}' still text in {len(hits)} file(s)")

# 5. Primitive-built functions without staffage calls (wide scan)
section("5 functions calling primitives without any staffage call (* = object noun)")
wide = []
svg = []
for p in clips:
    tree = parse(p)
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            calls = {callee(c) for c in ast.walk(node) if isinstance(c, ast.Call)}
            prims = sorted(calls & PRIMS)
            if prims and not calls & STAFF:
                wide.append((rel(p), node.lineno, node.name, prims, bool(NOUN.search(node.name))))
        if isinstance(node, ast.Call) and callee(node) in ("SVGMobject", "ImageMobject", "ImageMobjectFromArray"):
            svg.append((rel(p), node.lineno, callee(node), ast.unparse(node)[:90]))
for w in wide:
    emit(f"{'*' if w[4] else ' '} {w[0]}:{w[1]} {w[2]} {','.join(w[3])}")
emit(f"wide count {len(wide)}, noun hits {sum(1 for w in wide if w[4])}")
section("6 raw image/svg mobjects")
for s in svg:
    emit(f"SVG {s[0]}:{s[1]} {s[2]} {s[3]}")
emit(f"svg count {len(svg)}")

out = REPO / Path(*[] or [])
text = "\n".join(OUT) + "\n"
dest = Path(sys.argv[1]) if len(sys.argv) > 1 else None
if dest:
    dest.write_text(text, encoding="utf-8")
else:
    sys.stdout.write(text)
