#!/usr/bin/env python3
"""🩺 Static check (E0061): free-function calls pass as many arguments as the unique definition of that name takes. Names with several definitions of different arity are skipped."""
import os, re, sys
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
norm = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")
sel = sys.argv[1:]
SKIP = ("target", "node_modules", "🗑️generated", "🔮️oracles", "🏭️bridge", "👽️guest")
files = []
for d, ds, fs in os.walk(norm):
    ds[:] = [x for x in ds if x not in SKIP]
    files += [os.path.join(d, f) for f in fs if f.endswith(".rs")]
def balanced(t, i, o, c):
    depth = 0; j = i
    while j < len(t):
        ch = t[j]
        if ch == '"':
            j += 1
            while j < len(t) and t[j] != '"': j += 2 if t[j] == '\\' else 1
        elif ch == "'" and j + 2 < len(t) and t[j + 2] == "'": j += 2
        elif ch == o: depth += 1
        elif ch == c:
            depth -= 1
            if depth == 0: return j
        j += 1
    return -1
def split_args(body):
    out, depth, cur, j = [], 0, [], 0
    while j < len(body):
        c = body[j]
        if c == '"':
            k = j + 1
            while k < len(body) and body[k] != '"': k += 2 if body[k] == '\\' else 1
            cur.append(body[j:k + 1]); j = k + 1; continue
        if c in "([{": depth += 1
        elif c in ")]}": depth -= 1
        elif c == '|' and depth == 0: pass
        if c == ',' and depth == 0: out.append("".join(cur)); cur = []
        else: cur.append(c)
        j += 1
    out.append("".join(cur)); return [x for x in out if x.strip()]
defs = {}
for f in files:
    t = re.sub(r'//[^\n]*', '', open(f, encoding="utf-8").read())
    for m in re.finditer(r'(?<![\w])fn\s+(\w+)\s*(?:<[^({]*>)?\s*\(', t):
        e = balanced(t, m.end() - 1, '(', ')')
        if e < 0: continue
        params = split_args(t[m.end():e])
        if any(re.match(r'\s*(&\s*(mut\s+)?)?self\b', p) or re.match(r'\s*mut\s+self\b', p) for p in params): continue
        defs.setdefault(m.group(1), set()).add(len(params))
uniq = {n: next(iter(v)) for n, v in defs.items() if len(v) == 1}
out = []
for f in files:
    if sel and not any(s in f for s in sel): continue
    t = re.sub(r'//[^\n]*', '', open(f, encoding="utf-8").read())
    for m in re.finditer(r'(?<![\w.])((?:[a-z_]\w*::)*)([a-z_]\w*)\s*\(', t):
        n = m.group(2)
        if n not in uniq: continue
        pre = t[max(0, m.start() - 4):m.start()]
        if re.search(r'fn\s*$', t[max(0, m.start() - 6):m.start()]): continue
        if n in ("new", "from", "into", "default", "map", "len", "min", "max"): continue
        e = balanced(t, m.end() - 1, '(', ')')
        if e < 0: continue
        n_args = len(split_args(t[m.end():e]))
        if n_args != uniq[n]:
            out.append(f"{os.path.relpath(f, norm).replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}:{t.count(chr(10),0,m.start())+1}: {m.group(1)}{n}() takes {uniq[n]} args, called with {n_args}")
print("\n".join(out)); print(len(out), "issues")
