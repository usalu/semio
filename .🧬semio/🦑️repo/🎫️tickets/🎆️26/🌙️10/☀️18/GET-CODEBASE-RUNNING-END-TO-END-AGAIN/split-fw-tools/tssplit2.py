#!/usr/bin/env python3
"""Lossless TS splitter with cross-part exports. usage: tssplit2.py spec.json [--apply]
spec: {source, origDir, parts:[{path, header, ranges, [after: raw lines after imports]}]}
 - ranges are 1-based inclusive line ranges of the original source.
 - names declared at column 0 in one part and used in another become `export`ed + imported (relative path).
 - original imports are re-emitted per part, only used specifiers, relative specifiers re-based.
"""
import json, re, sys, os
from collections import Counter

spec = json.load(open(sys.argv[1]))
apply = "--apply" in sys.argv
src = open(spec["source"], encoding="utf-8").read().split("\n")
if src and src[-1] == "":
    src.pop()
N = len(src)
origDir = spec["origDir"]

FROM = re.compile(r"""from\s*["'][^"']+["'](\s+with\s*\{[^}]*\})?;?\s*$""")
stmts = []
i = 0
while i < N:
    l = src[i]
    if re.match(r"""^import[\s{*"']""", l):
        j = i
        while not FROM.search(src[j]) and not re.match(r"""import\s+["']""", src[j]):
            j += 1
        stmts.append((i, j))
        i = j + 1
        continue
    if stmts and l.strip() and not l.startswith(("//", "/*", " *", "*/")):
        break
    i += 1
imp_lines = set()
parsed = []
for a, b in stmts:
    text = " ".join(x.strip() for x in src[a:b + 1])
    imp_lines.update(range(a, b + 1))
    m = re.match(r"""import\s*(type\s+)?(.*?)\s*from\s*(["'][^"']+["'])(\s+with\s*\{[^}]*\})?;?$""", text)
    if not m:
        raise SystemExit("unparsed import: " + text[:120])
    typ, clause, mod, withc = m.group(1), m.group(2), m.group(3), m.group(4) or ""
    default = None
    inner = ""
    if clause.startswith("{"):
        inner = clause[1:clause.rindex("}")]
    elif "{" in clause:
        default = clause.split(",")[0].strip()
        inner = clause[clause.index("{") + 1:clause.rindex("}")]
    else:
        default = clause
    named = [x.strip() for x in inner.split(",") if x.strip()]
    parsed.append({"typ": bool(typ), "default": default, "named": named, "mod": mod[1:-1], "quote": mod[0], "with": withc})

first_import = stmts[0][0] if stmts else 0
header_lines = set(range(0, first_import))
for _k in range(0, first_import):
    if src[_k].strip() and not re.match(r"^(#!|//|/\*|\s\*|\*/)", src[_k]):
        raise SystemExit("non-comment line before first import: L%d %s" % (_k + 1, src[_k][:80]))
for _a, _b in spec.get("ignore", []):
    header_lines.update(range(_a - 1, _b))
for _p in spec["parts"]:
    if _p["header"].startswith("@range:"):
        _a, _b = map(int, _p["header"][7:].split("-"))
        header_lines.update(range(_a - 1, _b))
# doc comment between imports (non-import lines before body start) belongs to header
body_start = (stmts[-1][1] + 1) if stmts else 0

def strip_comments(t):
    return "\n".join(l for l in t.split("\n") if not re.match(r"^\s*(//|/\*|\*)", l))

def used(name, body):
    return re.search(r"(?<![\w$])(?<![^.]\.)" + re.escape(name) + r"(?![\w$])", body) is not None

def spec_name(s):
    s2 = re.sub(r"^type\s+", "", s)
    m = re.match(r"^([\w$]+)(\s+as\s+([\w$]+))?$", s2)
    return m.group(3) or m.group(1)

DECL = re.compile(r"^(export\s+)?(?:declare\s+)?(?:(?:async\s+)?function\*?|class|abstract class|const|let|var|type|interface|enum)\s+([\w$]+)")
TYPEY = re.compile(r"^(export\s+)?(?:type|interface)\s")

parts = spec["parts"]
for p in parts:
    p["lines"] = []
    for a, b in p["ranges"]:
        p["lines"].extend(src[a - 1:b])
    p["decls"] = {}
    for l in p["lines"]:
        m = DECL.match(l)
        if m:
            p["decls"][m.group(2)] = bool(TYPEY.match(l))
    p["code"] = strip_comments("\n".join(p["lines"]))

owner = {}
for p in parts:
    for n in p["decls"]:
        if n in owner:
            print("DUPLICATE decl", n, owner[n]["path"], p["path"])
        owner[n] = p

def rel(frm, to):
    r = os.path.relpath(to, os.path.dirname(frm))
    return r if r.startswith(".") else "./" + r

outputs = {}
bodies = []
deps = {}
for p in parts:
    needed_exports = set()
    lines = list(p["lines"])
    imps = []
    code = p["code"]
    for ps in parsed:
        keep = []
        d = ps["default"] if ps["default"] and used(ps["default"], code) and ps["default"] not in p["decls"] else None
        for s in ps["named"]:
            nm = spec_name(s)
            if used(nm, code) and nm not in p["decls"]:
                keep.append(s)
        if not (d or keep):
            continue
        mod = ps["mod"]
        if mod.startswith("."):
            absmod = os.path.normpath(os.path.join(origDir, mod))
            mod = os.path.relpath(absmod, os.path.dirname(p["path"]))
            if not mod.startswith("."):
                mod = "./" + mod
        t = "type " if ps["typ"] else ""
        clause = (d or "") + (", " if d and keep else "") + ("{ " + ", ".join(keep) + " }" if keep else "")
        imps.append(f'import {t}{clause} from {ps["quote"]}{mod}{ps["quote"]}{ps["with"]};')
    byprov = {}
    for n, q in owner.items():
        if q is not p and used(n, code):
            byprov.setdefault(q["path"], []).append(n)
    for path, ns in sorted(byprov.items()):
        deps.setdefault(p["path"], set()).add(path)
        q = next(x for x in parts if x["path"] == path)
        names = ["type " + n if q["decls"][n] else n for n in sorted(ns)]
        alltype = all(q["decls"][n] for n in ns)
        if alltype:
            imps.append(f'import type {{ {", ".join(sorted(ns))} }} from "{rel(p["path"], path)}";')
        else:
            imps.append(f'import {{ {", ".join(names)} }} from "{rel(p["path"], path)}";')
    p["imps"] = imps
    p["byprov"] = byprov

for p in parts:
    exported = set()
    for q in parts:
        if q is p:
            continue
        for path, ns in q["byprov"].items():
            if path == p["path"]:
                exported.update(ns)
    lines = []
    for l in p["lines"]:
        m = DECL.match(l)
        if m and m.group(2) in exported and not l.startswith("export "):
            l = "export " + l
        lines.append(l)
    if p["header"].startswith("@range:"):
        a, b = map(int, p["header"][7:].split("-"))
        head = src[a - 1:b]
    else:
        head = p["header"].rstrip("\n").split("\n")
    pre, suf = p.get("prefix", []), p.get("suffix", [])
    notes = []
    for a, b in p.get("importComments", []):
        notes += src[a - 1:b]
    imp_block = notes + p["imps"]
    if p.get("importRegion") and imp_block:
        imp_block = ["// #region 🔌️Imports"] + imp_block + ["// #endregion 🔌️Imports"]
    p["imps_n"] = len(imp_block)
    out = head + [""] + imp_block + [""] + pre + lines + suf
    for old, new in p.get("replace", []):
        n = sum(l.count(old) for l in out)
        if n == 0:
            print("REPLACE MISS", old)
        out = [l.replace(old, new) for l in out]
    outputs[p["path"]] = out
    bodies.append(lines)

def cycles():
    seen, stack, found = set(), [], []
    def dfs(n):
        stack.append(n); seen.add(n)
        for m in deps.get(n, ()):
            if m in stack:
                found.append(stack[stack.index(m):] + [m])
            elif m not in seen:
                dfs(m)
        stack.pop()
    for n in list(deps):
        if n not in seen:
            dfs(n)
    return found

NORM = re.compile(r"^export (?=(?:declare |async function|function|class|abstract class|const|let|var|type|interface|enum)\b)")
orig = Counter(NORM.sub("", l) for k, l in enumerate(src) if k not in imp_lines and k not in header_lines and l.strip() and not any(k + 1 >= a and k + 1 <= b for a, b in [])) 
# unassigned original lines (not header/import/blank)
assigned_idx = set()
for p in parts:
    for a, b in p.get("importComments", []):
        header_lines.update(range(a - 1, b))
for p in parts:
    for a, b in p["ranges"]:
        assigned_idx.update(range(a - 1, b))
unassigned = [k for k in range(N) if k not in assigned_idx and k not in imp_lines and k not in header_lines and src[k].strip()]
outn = Counter()
for b in bodies:
    outn.update(NORM.sub("", l) for l in b if l.strip())
# original body = all non-import non-header lines that were assigned
orig_assigned = Counter(NORM.sub("", src[k]) for k in assigned_idx if src[k].strip())
print("unassigned original non-blank body lines:", len(unassigned))
for k in unassigned[:15]:
    print("   L%d %s" % (k + 1, src[k][:120]))
print("OUTPUT vs ORIG(assigned) diff:", sum((orig_assigned - outn).values()), sum((outn - orig_assigned).values()))
dups = [k for k, c in Counter(k for p in parts for a, b in p["ranges"] for k in range(a - 1, b)).items() if c > 1]
print("lines assigned twice:", len(dups))
for p in parts:  # byprov-print
    for path, ns in p["byprov"].items():
        print("  dep", os.path.basename(os.path.dirname(p["path"])), "<-", os.path.basename(os.path.dirname(path)), ns)
cy = cycles()
print("cycles:", [[os.path.basename(os.path.dirname(x)) for x in c] for c in cy])
for p, out in outputs.items():
    print(f"{len(out):5d} {p}")
if apply:
    for p, out in outputs.items():
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print("written")
