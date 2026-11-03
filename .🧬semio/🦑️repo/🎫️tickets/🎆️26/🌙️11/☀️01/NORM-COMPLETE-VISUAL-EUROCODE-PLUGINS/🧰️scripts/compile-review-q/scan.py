import os, re, sys, time
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
MIN = int(sys.argv[1]) if len(sys.argv) > 1 else 600
FILTER = sys.argv[2] if len(sys.argv) > 2 else ""
now = time.time()
crates = {}
for d, ds, fs in os.walk("/home/deck/semio/✏️s"):
    ds[:] = [x for x in ds if x not in ("target", "node_modules", ".git")]
    if "Cargo.toml" in fs:
        t = open(os.path.join(d, "Cargo.toml"), errors="ignore").read()
        m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
        if not m: continue
        lp = re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"', t, re.M | re.S)
        path = os.path.normpath(os.path.join(d, lp.group(1))) if lp else os.path.join(d, "src/lib.rs")
        deps = set(re.findall(r'^([a-z0-9_-]+)\s*=', t[t.find("[dependencies"):] if "[dependencies" in t else "", re.M))
        deps |= set(re.findall(r'^\[(?:dev-|build-)?dependencies\.([a-z0-9_-]+)\]', t, re.M))
        crates[m.group(1)] = dict(dir=d, lib=path, toml=t, files=[], deps=deps)
file_crate = {}
PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;')
missing = []
def walk(crate, f, seen):
    if f in seen: return
    if not os.path.exists(f):
        missing.append((crate, f)); return
    seen.add(f); file_crate.setdefault(f, set()).add(crate)
    src = open(f, errors="ignore").read()
    for p, _ in PATH_RE.findall(src):
        walk(crate, os.path.normpath(os.path.join(os.path.dirname(f), p)), seen)
    # plain `mod x;`
    for m in re.finditer(r'^\s*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;', src, re.M):
        pre = src[max(0, m.start()-300):m.start()]
        if re.search(r'#\[path\s*=[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*$', pre): continue
        base = os.path.dirname(f); n = m.group(1)
        for c in (os.path.join(base, n + ".rs"), os.path.join(base, n, "mod.rs")):
            if os.path.exists(c): walk(crate, c, seen); break
        else:
            missing.append((crate, f + " mod " + n))
for n, c in crates.items():
    seen = set(); walk(n, c["lib"], seen); c["files"] = sorted(seen)
ident = lambda n: n.replace("-", "_")
by_ident = {ident(n): n for n in crates}
DEF = re.compile(r'(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:async\s+)?(fn|struct|enum|const|static|type|trait|mod|macro_rules!)\s+(\w+)')
names = {}; fns = {}
def args_arity(s, i):
    d = 0; n = 0; has = False; j = i
    while j < len(s):
        ch = s[j]
        if ch in "([{": d += 1
        elif ch in ")]}":
            d -= 1
            if d == 0: return (n + 1 if has else 0), j
        elif ch == "<" and d >= 1 and False: pass
        elif ch == "," and d == 1: n += 1; has = has and True
        elif not ch.isspace() and d >= 1: has = True
        j += 1
    return None, j
srcs = {}
for n, c in crates.items():
    s = set(); fs = {}
    for f in c["files"]:
        t = open(f, errors="ignore").read(); srcs[f] = t
        s |= set(re.findall(r'\w+', t))
        for m in re.finditer(r'\bfn\s+(\w+)\s*(?:<[^>{]*>)?\s*\(', t):
            a, _ = args_arity(t, m.end()-1)
            body = t[m.end()-1:m.end()+400]
            sig = t[m.end()-1: t.find("{", m.end())] if "{" in t[m.end():] else ""
            has_self = bool(re.match(r'\(\s*(?:mut\s+)?&?\s*(?:mut\s+)?(?:\'\w+\s+)?self', t[m.end()-1:m.end()+30]))
            if a is not None: fs.setdefault(m.group(1), set()).add(a + (0 if not has_self else 0) if not has_self else -(a))
        # enum variants and use aliases count as names
        for m in re.finditer(r'\buse\s+[^;]*?\b(\w+)\s*;|\bas\s+(\w+)', t): pass
    names[n] = s; fns[n] = fs
for _ in range(4):
    for n, c in crates.items():
        for f in c["files"]:
            for m in re.finditer(r'pub\s+use\s+(semio_\w+)::(?:\*|\{[^}]*\*[^}]*\})', srcs[f]):
                o = by_ident.get(m.group(1))
                if o and o in names: names[n] |= names[o]
issues = []
USE = re.compile(r'\buse\s+([^;]+);', re.S)
def expand(p):
    p = re.sub(r'\s+as\s+\w+', '', p)
    p = re.sub(r'\s+', '', p)
    out = []
    def rec(prefix, s):
        if "{" not in s:
            out.append(prefix + s); return
        i = s.index("{"); pre = s[:i]; depth = 0; j = i; parts = []; cur = ""
        for ch in s[i+1:]:
            if ch == "{": depth += 1
            if ch == "}":
                if depth == 0: break
                depth -= 1
            if ch == "," and depth == 0: parts.append(cur); cur = ""
            else: cur += ch
        parts.append(cur)
        for q in parts:
            if q: rec(prefix + pre, q)
    rec("", p); return out
glob_re = {}
for fpath, crs in file_crate.items():
    if FILTER and FILTER not in fpath: continue
    if now - os.path.getmtime(fpath) > MIN * 60: continue
    t = srcs.get(fpath) or open(fpath, errors="ignore").read()
    for cr in crs:
        c = crates[cr]
        for m in USE.finditer(t):
            for u in expand(m.group(1)):
                seg = u.split("::")
                if len(seg) < 2: continue
                top = seg[0]
                if top in by_ident:
                    tn = by_ident[top]
                    if tn != cr and tn not in c["deps"] and tn.replace("_", "-") not in c["deps"]:
                        issues.append((fpath, "dep-missing", f"{cr} uses {tn} without dependency"))
                    last = seg[-1]
                    if last in ("*", "self"): continue
                    if last and last not in names[tn]:
                        issues.append((fpath, "use-unresolved", f"{u}"))
        # qualified calls a::b( where b has fixed arity in target
        for m in re.finditer(r'\b([a-z_][a-z0-9_]*)::([a-z_][a-z0-9_]*)\s*\(', t):
            mod, fn = m.groups()
            if mod in ("std", "self", "super", "crate", "Self", "f64", "usize", "u8", "u32", "i32", "u64", "String", "Vec", "Option", "Result") : continue
            a, _ = args_arity(t, m.end()-1)
            if a is None: continue
            tg = None
            if mod in by_ident: tg = [by_ident[mod]]
            elif mod == "kernel" or mod in ("fastening", "fibre", "geotechnics", "section"): tg = ["semio-s-plugin-norm-mechanics-applied"] if mod in ("kernel", "fastening", "geotechnics") else ["semio-s-plugin-norm-mechanics"]
            if not tg: continue
            ar = set()
            for tn in tg:
                if tn in fns:
                    ar |= {x for x in fns[tn].get(fn, set()) if x >= 0}
            if ar and a not in ar:
                issues.append((fpath, "arity", f"{mod}::{fn} called with {a}, defined {sorted(ar)}"))
print("MISSING #[path]/mod targets:")
for c, f in missing:
    if not FILTER or FILTER in f: print("  ", c, f)
print("ISSUES:")
for f, k, d in sorted(set(issues)):
    print(f"{k}\t{f.replace(ROOT,'')}\t{d}")

print("INTRA-CRATE use:")
DEFS = re.compile(r'(?:fn|struct|enum|const|static|type|trait|mod|union)\s+(\w+)|macro_rules!\s+(\w+)')
cdefs = {}
for n, c in crates.items():
    st = set()
    for f in c["files"]:
        t = srcs[f]
        for m in DEFS.finditer(t): st.add(m.group(1) or m.group(2))
        for m in re.finditer(r'\buse\s+([^;]+);', t):
            for u in expand(re.sub(r'\s+as\s+(\w+)', r' as \1', m.group(1)).replace(" as ", "@")):
                if "@" in u: st.add(u.split("@")[1])
                else: st.add(u.split("::")[-1])
        for m in re.finditer(r'extern\s+crate\s+\w+\s+as\s+(\w+)', t): st.add(m.group(1))
        for m in re.finditer(r'\b([A-Z]\w*)\s*[,({=]', t): st.add(m.group(1))
    cdefs[n] = st
for fpath, crs in file_crate.items():
    if FILTER and FILTER not in fpath: continue
    if now - os.path.getmtime(fpath) > MIN * 60: continue
    t = srcs.get(fpath) or open(fpath, errors="ignore").read()
    for cr in crs:
        for m in USE.finditer(t):
            raw = re.sub(r'\s+as\s+\w+', '', m.group(1))
            for u in expand(raw):
                seg = u.split("::")
                if seg[0] not in ("crate", "super", "self") or len(seg) < 2: continue
                last = seg[-1]
                if last in ("*", "self", ""): continue
                if last not in cdefs[cr]:
                    print(f"intra-unresolved\t{fpath.replace(ROOT,'')}\t{u}")
