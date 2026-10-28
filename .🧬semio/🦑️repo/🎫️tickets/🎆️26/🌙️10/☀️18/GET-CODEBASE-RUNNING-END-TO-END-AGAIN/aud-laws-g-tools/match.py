import glob, os, sys, re
C = sys.argv[1]
FE = '️'
def norm(s): return s.replace(FE, '')
res = {}
for f in sorted(glob.glob(os.path.join(C, 'result-*-g.md')) + glob.glob(os.path.join(C, 'result-*-f.md'))):
    res[os.path.basename(f)] = norm(open(f, encoding='utf-8', errors='replace').read())
big = '\n'.join(res.values())
rel = [l.strip() for l in open(os.path.join(C, '🗑️generated/aud-laws-g/recent-tests.txt'), encoding='utf-8') if re.search(r'\.(ts|rs|py|go|mjs)$', l.strip())]
out = []
for p in rel:
    q = norm(p)
    parts = q.split('/')
    if '🧪tests' in parts or '🧪️tests' in q or 'tests' in parts:
        idx = max(i for i,x in enumerate(parts) if 'tests' in x)
        parent = parts[idx-1] if idx>0 else ''
        nxt = parts[idx+1] if idx+1 < len(parts) else ''
        keys = []
        if nxt.endswith(('.ts','.rs','.py','.go','.mjs')) or '.' not in nxt:
            keys.append(f"{parent}/" + norm('🧪️tests') + (f"/{nxt}" if nxt and not nxt.endswith(('.ts','.rs','.py','.go','.mjs')) else ''))
        hit = any(k in big for k in keys if k)
        # full-trailing check for leaf files
        tail = '/'.join(parts[-3:])
        hit2 = tail in big
        if hit or hit2:
            who = [n for n,t in res.items() if (keys and any(k in t for k in keys)) or tail in t]
            out.append((p, ','.join(who)))
for p,w in out: print(p, '|', w)
print('TOTAL', len(out), file=sys.stderr)
