import os, re, sys, time
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
MIN = int(sys.argv[1]); FILTERS = sys.argv[2:]
now = time.time()
files = []
for d, ds, fs in os.walk(ROOT):
    ds[:] = [x for x in ds if x not in ("target", "node_modules")]
    for f in fs:
        if f.endswith(".rs"): files.append(os.path.join(d, f))
def strip(t):
    t = re.sub(r'(?<!:)//[^\n]*', '', t)
    t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t)
    return t
def close(t, i, o="(", c=")"):
    d = 0
    for j in range(i, len(t)):
        if t[j] == o: d += 1
        elif t[j] == c:
            d -= 1
            if d == 0: return j
    return None
def count_args(s):
    d = 0; n = 0; cur = ""; items = []
    for ch in s:
        if ch in "([{": d += 1
        if ch in ")]}": d -= 1
        if ch == "," and d == 0: items.append(cur); cur = ""
        else: cur += ch
    items.append(cur)
    return len([i for i in items if i.strip()])
def count_params(s):
    d = 0; cur = ""; items = []
    for ch in s:
        if ch in "([{<": d += 1
        if ch in ")]}>" and not (ch == '>' and cur.endswith('-')): d -= 1
        if ch == "," and d == 0: items.append(cur); cur = ""
        else: cur += ch
    items.append(cur)
    return len([i for i in items if i.strip()])
defs = {}
srcs = {}
for f in files:
    t = strip(open(f, errors="ignore").read()); srcs[f] = t
    for m in re.finditer(r'\bfn\s+(\w+)\s*(?:<[^()]*?>)?\s*\(', t):
        e = close(t, m.end()-1)
        if e is None: continue
        defs.setdefault(m.group(1), set()).add(count_params(t[m.end():e]))
    for m in re.finditer(r'macro_rules!\s+(\w+)', t): defs.setdefault(m.group(1), set()).add(-1)
SKIP = {"std", "core", "self", "super", "crate", "Self", "serde_json", "mem", "fmt", "iter", "cmp", "f64", "char", "str", "ptr", "env", "fs", "io", "thread", "time", "any", "array", "slice", "collections", "ops", "convert", "hash", "num", "path", "process", "sync", "rc", "cell", "marker", "borrow", "string", "vec", "option", "result", "default", "clone", "from", "into", "u8","u32","u64","i32","i64","usize","isize","f32","bool"}
issues = set()
for f in files:
    if now - os.path.getmtime(f) > MIN*60: continue
    if FILTERS and not any(x in f for x in FILTERS): continue
    t = srcs[f]
    for m in re.finditer(r'(?<![\w:.])((?:[a-z_]\w*::)+)([a-z_]\w*)\s*(?:::<[^()]*>)?\s*\(', t):
        mods = m.group(1).split("::")[:-1]
        if mods[0] in SKIP: continue
        name = m.group(2)
        if name not in defs or -1 in defs[name]: continue
        e = close(t, m.end()-1)
        if e is None: continue
        a = count_args(t[m.end():e])
        if a not in defs[name]:
            line = t.count("\n", 0, m.start()) + 1
            issues.add((f.replace(ROOT, ""), line, "::".join(mods) + "::" + name, a, tuple(sorted(defs[name]))))
    for m in re.finditer(r'(?<![\w:.])([a-z_]\w*)\s*\(', t):
        name = m.group(1)
        if name in ("if","while","match","for","return","loop","fn","in","let","as","some","ok","err","vec","format","assert","println","move","mut","ref","dyn","impl","where"): continue
        pre = t[max(0,m.start()-12):m.start()]
        if re.search(r'fn\s+$', pre): continue
        if name not in defs: continue
        # only flag unqualified calls with unique single-arity definitions defined in the same file
        if name in re.findall(r'\bfn\s+(\w+)', t):
            ar = set()
            for dm in re.finditer(r'\bfn\s+' + name + r'\s*(?:<[^()]*?>)?\s*\(', t):
                e = close(t, dm.end()-1)
                if e is not None: ar.add(count_params(t[dm.end():e]))
            e = close(t, m.end()-1)
            if e is None or not ar: continue
            a = count_args(t[m.end():e])
            if a not in ar and not any(x-1 == a for x in ar):
                line = t.count("\n", 0, m.start()) + 1
                issues.add((f.replace(ROOT, ""), line, name, a, tuple(sorted(ar))))
print(len(issues), "arity mismatches")
for i in sorted(issues): print(f"{i[0]}:{i[1]}\t{i[2]}\tcalled with {i[3]}, defined {list(i[4])}")
