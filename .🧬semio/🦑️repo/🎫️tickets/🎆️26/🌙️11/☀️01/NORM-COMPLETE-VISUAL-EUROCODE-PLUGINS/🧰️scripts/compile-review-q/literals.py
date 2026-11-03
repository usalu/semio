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
    t = re.sub(r'//[^\n]*', '', t)
    t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t)
    return t
def block(t, i):
    d = 0
    for j in range(i, len(t)):
        if t[j] == '{': d += 1
        elif t[j] == '}':
            d -= 1
            if d == 0: return j
    return None
def split_top(s):
    out = []; d = 0; cur = ""
    for ch in s:
        if ch in "([{<" and not (ch == '<' ): d += 1
        if ch in ")]}": d -= 1
        if ch == ',' and d == 0: out.append(cur); cur = ""
        else: cur += ch
    out.append(cur); return [x.strip() for x in out if x.strip()]
defs = {}; srcs = {}
for f in files:
    t = strip(open(f, errors="ignore").read()); srcs[f] = t
    for m in re.finditer(r'\bstruct\s+([A-Z]\w*)\s*(?:<[^>{]*>)?\s*(?:where[^{]*)?\{', t):
        e = block(t, m.end()-1)
        if e is None: continue
        body = t[m.end():e]
        fields = []
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part).strip()
            fm = re.match(r'(?:pub(?:\([a-z]+\))?\s+)?(\w+)\s*:', part)
            if fm: fields.append(fm.group(1))
        defs.setdefault(m.group(1), []).append((f, frozenset(fields)))
    # enum struct variants
    for m in re.finditer(r'\benum\s+([A-Z]\w*)[^{]*\{', t):
        e = block(t, m.end()-1)
        if e is None: continue
        body = t[m.end():e]
        for vm in re.finditer(r'(?:^|,|\})\s*(?:#\[[^\]]*\]\s*)*([A-Z]\w*)\s*\{', body):
            e2 = block(body, vm.end()-1)
            if e2 is None: continue
            fields = [re.match(r'(\w+)\s*:', p).group(1) for p in split_top(body[vm.end():e2]) if re.match(r'(\w+)\s*:', p)]
            defs.setdefault(m.group(1)+"::"+vm.group(1), []).append((f, frozenset(fields)))
issues = []
for f in files:
    if now - os.path.getmtime(f) > MIN*60: continue
    if FILTERS and not any(x in f for x in FILTERS): continue
    t = srcs[f]
    for m in re.finditer(r'(?<![\w:])((?:[A-Z]\w*::)?[A-Z]\w*)\s*\{', t):
        name = m.group(1)
        pre = t[max(0, m.start()-25):m.start()]
        if re.search(r'(struct|enum|impl|for|trait|mod|union|where|dyn|->|use)\s*$', pre): continue
        if re.search(r'(impl<[^>]*>|impl)\s+[\w:<>, ]*$', pre): continue
        e = block(t, m.end()-1)
        if e is None: continue
        after = t[e+1:e+6].lstrip()
        if after.startswith('=>') or (after.startswith('=') and not after.startswith('==')) or after.startswith('|'): continue
        body = t[m.end():e]
        if re.search(r'(^|,)\s*\.\.', body) : continue
        key = name
        if key not in defs and "::" in name:
            key = name.split("::")[-2] + "::" + name.split("::")[-1]
            if key not in defs: key = name.split("::")[-1]
        if key not in defs:
            continue
        cands = defs[key]
        if len({c[1] for c in cands}) != 1: continue
        want = next(iter(cands))[1]
        got = []
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part).strip()
            fm = re.match(r'(\w+)\s*(?::|$)', part)
            if fm: got.append(fm.group(1))
            else: got = None; break
        if got is None: continue
        gs = set(got)
        if gs != want and len(want) > 0:
            line = t.count("\n", 0, m.start()) + 1
            issues.append((f, line, name, sorted(want - gs), sorted(gs - want)))
print(len(issues), "literal mismatches")
for f, l, n, miss, extra in sorted(issues):
    print(f"{f.replace(ROOT,'')}:{l}\t{n}\tmissing={miss}\textra={extra}")
