#!/usr/bin/env python3
"""⬆️ Hoists the depth-1 `use` statements of a Rust function to module level (after the last module-level `use`), skipping texts already imported there and aborting on a name clash.
usage: hoist-uses.py <file> <fn> [nth] [APPLY]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn
file, fname = sys.argv[1], sys.argv[2]
nth = int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3].isdigit() else 0
APPLY = "APPLY" in sys.argv
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, fname, nth)
sts = rsfn.statements(toks, op, cl)
fn_uses = [(s, e) for (s, e) in sts if toks[s][1] == "use"]
def names_of(t):
    t = re.sub(r"\s+", " ", t.strip().rstrip(";"))
    t = re.sub(r"^(pub(\([^)]*\))? )?use ", "", t)
    out = {}
    def walk(prefix, body):
        body = body.strip()
        if body.startswith("{") and body.endswith("}"):
            parts, d, cur = [], 0, ""
            for ch in body[1:-1]:
                if ch == "{": d += 1
                if ch == "}": d -= 1
                if ch == "," and d == 0: parts.append(cur); cur = ""
                else: cur += ch
            if cur.strip(): parts.append(cur)
            for p in parts: walk(prefix, p)
            return
        m = re.match(r"^(.*?)::(\{.*\})$", body)
        if m: walk(prefix + m.group(1) + "::", m.group(2)); return
        if body.endswith("*"): out.setdefault("*", []).append(prefix + body); return
        m = re.match(r"^(.*?)\s+as\s+(\w+)$", body)
        if m: out[m.group(2)] = prefix + m.group(1); return
        leaf = body.split("::")[-1]
        if leaf == "self": leaf = prefix.rstrip(":").split("::")[-1]; out[leaf] = prefix.rstrip(":"); return
        out[leaf] = prefix + body
    walk("", t)
    return out
mod_uses = []
depth = 0
q = 0
while q < len(toks):
    t = toks[q][1]
    if t == "{": depth += 1
    elif t == "}": depth -= 1
    elif depth == 0 and t == "use" and toks[q][0] == "id":
        e = q
        while toks[e][1] != ";": e += 1
        mod_uses.append((q, e)); q = e
    q += 1
mod_text = [text[toks[s][3]:toks[e][4]] for s, e in mod_uses]
mod_names = {}
for t in mod_text:
    for n, p in names_of(t).items():
        if n != "*": mod_names[n] = p
items = set(re.findall(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:fn|struct|enum|const|static|type|trait|mod)\s+(\w+)", text))
fn_texts = [text[toks[s][3]:toks[e][4]] for s, e in fn_uses]
fn_names = {}
conflicts = []
add = []
for t in fn_texts:
    if any(re.sub(r"\s+", " ", t) == re.sub(r"\s+", " ", m) for m in mod_text): continue
    ns = names_of(t)
    for n, p in ns.items():
        if n == "*": continue
        if n in mod_names and mod_names[n] != p: conflicts.append((n, p, mod_names[n]))
        if n in items: conflicts.append((n, p, "module item"))
        if n in fn_names and fn_names[n] != p: conflicts.append((n, p, fn_names[n]))
        fn_names[n] = p
    add.append(t)
print("fn uses", len(fn_uses), "to hoist", len(add), "conflicts", conflicts[:5])
if conflicts: sys.exit(1)
if APPLY:
    lines = text.split("\n")
    # remove fn use lines (whole lines); indentation stripped
    kill = set()
    for s, e in fn_uses:
        for ln in range(toks[s][2], toks[e][2] + 1): kill.add(ln)
    last_use_line = toks[mod_uses[-1][1]][2]
    out = []
    for i, l in enumerate(lines, 1):
        if i in kill: continue
        out.append(l)
        if i == last_use_line:
            out.extend(re.sub(r"(?m)^\s+", "", t) if "\n" not in t else t for t in add)
    open(file, "w", encoding="utf-8").write("\n".join(out))
    print("APPLIED")
