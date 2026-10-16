import sys, os, re, collections
sys.path.insert(0, ".")
from rs_lex import strip_code
from emit import parse_members, HEADVIS
M = os.environ.get("DEFSCAN_ROOT", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/")
files = {}
for dp, dn, fn in os.walk(M):
    if "🤖️generated" in dp: continue
    for f in fn:
        if f.endswith(".rs"):
            p = os.path.join(dp, f); files[p] = strip_code(open(p, encoding="utf-8").read())
def crate_of(p):
    rel = os.path.relpath(p, M).split("/")
    return rel[0]
# crate of a file = node dir containing the lib root mounting it
mount = {}
for p, t in files.items():
    if p.endswith("/🦀️.rs") and os.path.exists(os.path.join(os.path.dirname(p), "📦️packages")) or p.endswith("/📦️packages/🦀️rust/🦀️.rs"):
        base = os.path.dirname(p)
        for m in re.finditer(r'#\[path = "([^"]+)"\]', open(p, encoding="utf-8").read()):
            mount[os.path.normpath(os.path.join(base, m.group(1)))] = p
STRUCT = re.compile(r'(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*(pub(?:\([^)]*\))?\s+)?(struct|enum)\s+([A-Za-z_]\w*)')
IMPL = re.compile(r'(?m)^[ \t]*impl\b[^{;]*?\b([A-Za-z_]\w*)\s*(?:<[^{]*?>)?\s*(?:where[^{]*)?\{')
def items_by_type():
    out = collections.defaultdict(list)   # type -> [(file, member name, vis, kind)]
    for p, t in files.items():
        for m in STRUCT.finditer(t):
            name = m.group(3)
            # struct text
            i = m.start(); depth = 0; j = t.find("{", i); k = t.find(";", i)
            if m.group(2) == "struct" and j != -1 and (k == -1 or j < k):
                d = 0; e = j
                while e < len(t):
                    if t[e] == "{": d += 1
                    elif t[e] == "}":
                        d -= 1
                        if d == 0: break
                    e += 1
                body = t[i:e+1]
                for fname, info in parse_members(body, False, False).items():
                    out[name].append((p, fname, info["vis"], "field"))
            elif m.group(2) == "struct":
                e = t.find(";", i)
                for fname, info in parse_members(t[i:e+1], False, False).items():
                    out[name].append((p, fname, info["vis"], "field"))
        for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*impl(?:<[^>]*>)?\s+(?:[A-Za-z_][\w:<>, ]*?\s+for\s+)?([A-Za-z_]\w*)[^{;]*\{', t):
            if " for " in t[m.start():m.end()]: continue
            name = m.group(1)
            d = 0; e = m.end() - 1
            s = e
            while e < len(t):
                if t[e] == "{": d += 1
                elif t[e] == "}":
                    d -= 1
                    if d == 0: break
                e += 1
            for fname, info in parse_members(t[m.start():e+1], True, False).items():
                if info["kind"] in ("fn", "const"): out[name].append((p, fname, info["vis"], info["kind"]))
    return out
mem = items_by_type()
def owner(p): return mount.get(p, p)
bad = collections.Counter(); ex = []
for tname, lst in mem.items():
    defining = {p for (p, _, _, _) in lst}
    for (p, mname, vis, kind) in lst:
        if vis: continue   # already at least pub(...)
        for q, t in files.items():
            if q == p: continue
            if owner(q) == owner(p) and q in defining: pass
            if not re.search(r'(?<![\w])' + re.escape(tname) + r'(?![\w])', t): continue
            pat = (r'\.\s*' + re.escape(mname) + r'\s*(?:::<[^>]*>)?\s*\(') if kind != "field" else (r'\.\s*' + re.escape(mname) + r'\b(?!\s*\()' )
            if re.search(pat, t) or (kind == "field" and re.search(r'\b' + re.escape(mname) + r'\s*:', t) and False):
                # a non-pub member used from a different file that mentions the type
                bad[(tname, mname, kind)] += 1
                if len(ex) < 40: ex.append((tname, mname, kind, os.path.relpath(p, M)[-45:], os.path.relpath(q, M)[-45:]))
print("private members referenced from other files that mention the type:", len(bad))
for e in ex[:40]: print("  ", e)
