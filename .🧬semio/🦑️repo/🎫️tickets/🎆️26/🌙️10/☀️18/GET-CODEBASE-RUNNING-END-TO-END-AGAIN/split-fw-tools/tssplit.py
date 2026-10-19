#!/usr/bin/env python3
"""Lossless TS/TSX splitter. usage: tssplit.py spec.json [--apply]
spec: {source, importsEnd?, parts:[{path, header, ranges:[[a,b]...], kit:bool, kitImport:str, extraImports:[...]}]}
imports are regenerated per part (only used specifiers). kit parts get `export ` on top-level decls.
"""
import json, re, sys, os

spec = json.load(open(sys.argv[1]))
apply = "--apply" in sys.argv
src = open(spec["source"], encoding="utf-8").read().split("\n")
if src and src[-1] == "":
    src.pop()
N = len(src)

def import_block():
    start = None
    i = 0
    end = None
    stmts = []
    while i < N:
        l = src[i]
        if l.startswith("import "):
            j = i
            while not re.search(r"""from\s+["'][^"']+["'];?\s*$""", src[j]) and not re.match(r"""import\s+["']""", src[j]):
                j += 1
            stmts.append((i, j))
            end = j
            i = j + 1
            continue
        if stmts and l.strip() and not l.startswith("//") and not l.startswith("/*") and not l.startswith(" *"):
            if not l.startswith("import "):
                break
        i += 1
    return stmts

stmts = import_block()
imp_lines = set()
parsed = []
for a, b in stmts:
    text = " ".join(x.strip() for x in src[a:b + 1])
    for k in range(a, b + 1):
        imp_lines.add(k)
    m = re.match(r"""import\s+(type\s+)?(.*?)\s+from\s+(["'][^"']+["']);?$""", text)
    if not m:
        parsed.append({"raw": src[a:b + 1]})
        continue
    typ, clause, mod = m.group(1), m.group(2), m.group(3)
    default = None
    named = []
    mm = re.match(r"^([\w$]+)?\s*,?\s*(\{(.*)\})?$", clause)
    if clause.startswith("{"):
        inner = clause[1:clause.rindex("}")]
    elif mm and mm.group(3) is not None:
        default = mm.group(1)
        inner = mm.group(3)
    else:
        default = clause
        inner = ""
    for s in [x.strip() for x in inner.split(",") if x.strip()]:
        named.append(s)
    parsed.append({"typ": bool(typ), "default": default, "named": named, "mod": mod})

first_import = stmts[0][0] if stmts else 0
header_end = first_import

def strip_comments(t):
    return "\n".join(l for l in t.split("\n") if not re.match(r"^\s*(//|/\*|\*)", l))

def used(name, body):
    return re.search(r"(?<![\w$])(?<![^.]\.)" + re.escape(name) + r"(?![\w$])", body) is not None

def spec_name(s):
    s2 = re.sub(r"^type\s+", "", s)
    m = re.match(r"^([\w$]+)(\s+as\s+([\w$]+))?$", s2)
    return m.group(3) or m.group(1)

DECL = re.compile(r"^(?:export\s+)?(?:declare\s+)?(?:(?:async\s+)?function\*?|class|abstract class|const|let|type|interface|enum)\s+([\w$]+)")

def toplevel_names(lines):
    out = []
    for l in lines:
        m = DECL.match(l)
        if m:
            out.append(m.group(1))
    return out

parts = spec["parts"]
kit_names = {}
for p in parts:
    p["lines"] = []
    for a, b in p["ranges"]:
        p["lines"].extend(src[a - 1:b])
    if p.get("kit"):
        for n in toplevel_names(p["lines"]):
            kit_names[n] = p["path"]

outputs = {}
bodies = []
for p in parts:
    lines = list(p["lines"])
    if p.get("kit"):
        lines = [("export " + l if DECL.match(l) and not l.startswith("export ") else l) for l in lines]
    body = "\n".join(lines)
    code = strip_comments(body)
    imps = []
    for ps in parsed:
        if "raw" in ps:
            continue
        keep = []
        d = ps["default"] if ps["default"] and used(ps["default"], code) else None
        for s in ps["named"]:
            nm = spec_name(s)
            if used(nm, code) and kit_names.get(nm) != p["path"] and nm not in kit_names:
                keep.append(s)
        if d or keep:
            t = "type " if ps["typ"] else ""
            clause = (d or "") + (", " if d and keep else "") + ("{ " + ", ".join(keep) + " }" if keep else "")
            imps.append(f'import {t}{clause} from {ps["mod"]};')
    kit_import = {}
    for n, path in kit_names.items():
        if path != p["path"] and used(n, code):
            kit_import.setdefault(path, []).append(n)
    for path, ns in kit_import.items():
        rel = p["kitImport"] if p.get("kitImport") else path
        imps.append(f'import {{ {", ".join(sorted(ns))} }} from "{rel}";')
    imps.extend(p.get("extraImports", []))
    head = p["header"].rstrip("\n").split("\n")
    out = head + [""] + imps + [""] + lines
    outputs[p["path"]] = out
    bodies.append(out[len(head) + 1 + len(imps) + 1:])

# oracle
from collections import Counter
orig_hdr = set(range(0, header_end))
orig = Counter(l for k, l in enumerate(src) if k not in imp_lines and k not in orig_hdr and l.strip())
new = Counter()
for p in parts:
    lines = p["lines"]
    new.update(l for l in lines if l.strip())
# new lines come from same ranges -> compare with orig minus unassigned
assigned = Counter()
for p in parts:
    assigned.update(l for l in p["lines"] if l.strip())
missing = orig - assigned
extra = assigned - orig
outn = Counter()
for b in bodies:
    outn.update(re.sub(r"^export (?=(?:async function|function|class|const|let|type|interface|enum)\b)", "", l) for l in b if l.strip())
print("OUTPUT vs ORIG diff:", sum((orig - outn).values()), sum((outn - orig).values()))
print("assigned", sum(assigned.values()), "orig body", sum(orig.values()))
print("MISSING (orig not in parts):", sum(missing.values()))
for l, c in list(missing.items())[:20]:
    print("   ", c, l[:120])
print("EXTRA (parts not in orig):", sum(extra.values()))
for l, c in list(extra.items())[:20]:
    print("   ", c, l[:120])
for p, out in outputs.items():
    print(f"{len(out):5d} {p}")
if apply:
    for p, out in outputs.items():
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print("written")
