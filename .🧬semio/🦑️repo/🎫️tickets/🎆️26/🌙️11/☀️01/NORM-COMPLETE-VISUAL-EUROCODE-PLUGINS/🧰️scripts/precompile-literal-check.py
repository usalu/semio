#!/usr/bin/env python3
"""🩺 Static check (E0063/E0560): struct and enum-variant literals `Name { f: v, .. }` carry exactly the fields of their definition. Definitions are collected tree-wide by name; ambiguous names (several different field sets) are skipped."""
import os, re, sys
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
norm = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")
sel = sys.argv[1:]
SKIP = ("target", "node_modules", "🗑️generated", "🔮️oracles", "🏭️bridge", "👽️guest")
files = []
for d, ds, fs in os.walk(norm):
    ds[:] = [x for x in ds if x not in SKIP]
    for f in fs:
        if f.endswith(".rs"): files.append(os.path.join(d, f))
def match_brace(t, i):
    depth = 0; j = i; ins = False
    while j < len(t):
        c = t[j]
        if c == '"' : 
            j += 1
            while j < len(t) and t[j] != '"':
                j += 2 if t[j] == '\\' else 1
        elif c == '{': depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0: return j
        j += 1
    return -1
def split_top(body):
    out, depth, cur, j = [], 0, [], 0
    while j < len(body):
        c = body[j]
        if c == '"':
            k = j + 1
            while k < len(body) and body[k] != '"': k += 2 if body[k] == '\\' else 1
            cur.append(body[j:k + 1]); j = k + 1; continue
        if c in "([{<" and not (c == '<' and False): depth += 1 if c != '<' else 0
        if c in ")]}": depth -= 1
        if c == ',' and depth == 0: out.append("".join(cur)); cur = []
        else: cur.append(c)
        j += 1
    out.append("".join(cur)); return [x.strip() for x in out if x.strip()]
defs = {}
for f in files:
    t = open(f, encoding="utf-8").read()
    t2 = re.sub(r'//[^\n]*', '', t)
    for m in re.finditer(r'\bstruct\s+(\w+)\s*(?:<[^>{]*>)?\s*\{', t2):
        e = match_brace(t2, m.end() - 1)
        if e < 0: continue
        fields = set(re.findall(r'(?:^|,|\{)\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([a-z]+\))?\s+)?(?:r#)?(\w+)\s*:(?!:)', t2[m.end():e] if True else ""))
        # only top-level
        body = t2[m.end():e]; fl = set()
        for part in split_top(body):
            part = re.sub(r'^(?:#\[[^\]]*\]\s*)+', '', part)
            mm = re.match(r'(?:pub(?:\([a-z]+\))?\s+)?(?:r#)?(\w+)\s*:(?!:)', part)
            if mm: fl.add(mm.group(1))
        defs.setdefault(m.group(1), []).append((frozenset(fl), f))
    # macro-defined structs (list_delta, scalar_patch) skipped
uniq = {n: v[0][0] for n, v in defs.items() if len({x[0] for x in v}) == 1 and v[0][0]}
out = []
for f in files:
    if sel and not any(s in f for s in sel): continue
    t = open(f, encoding="utf-8").read(); t2 = re.sub(r'//[^\n]*', '', t)
    for m in re.finditer(r'(?<![\w:.])([A-Z]\w*)\s*\{', t2):
        n = m.group(1)
        if n not in uniq: continue
        pre = t2[max(0, m.start() - 12):m.start()]
        if re.search(r'(struct|enum|impl|trait|for|mod|union)\s*$', pre) or re.search(r'(struct|enum|impl|trait|for)\s+$', t2[max(0, m.start()-12):m.start()]): continue
        s = t2[:m.start()].rstrip()
        if s.endswith(("->", "impl", "for", "struct", "enum", "trait", "dyn", "where", "else", "if", "while", "match", "in")) : continue
        e = match_brace(t2, m.end() - 1)
        if e < 0: continue
        body = t2[m.end():e]
        parts = split_top(body); given = set(); rest = False; ok = True
        for p in parts:
            if p.startswith(".."): rest = True; continue
            mm = re.match(r'(?:r#)?(\w+)\s*(?::(?!:)|$)', p)
            if not mm: ok = False; break
            given.add(mm.group(1))
        if not ok or (not given and not rest): continue
        want = uniq[n]
        if not given <= want or (not rest and given != want):
            ln = t2.count("\n", 0, m.start()) + 1
            extra = sorted(given - want); miss = sorted(want - given) if not rest else []
            if extra or miss: out.append(f"{os.path.relpath(f, norm).replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}:{ln}: {n} literal extra={extra} missing={miss}")
print("\n".join(out)); print(len(out), "issues")
