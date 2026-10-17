import os, re, sys
R = '/Users/ueli/Documents/semio'
os.chdir(R)
pat = re.compile(r'env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"((?:/\.\./)+[^"]*|/(?:\.\./)+[^"]*)"')
bad = []
n = 0
for line in open(sys.argv[1], encoding='utf8'):
    p = line.rstrip('\n')
    try: t = open(p, encoding='utf8').read()
    except Exception: continue
    if 'CARGO_MANIFEST_DIR' not in t: continue
    n += 1
    d = os.path.dirname(p)
    cur = d; man = None
    while cur and cur != '.':
        cand = os.path.join(cur, '📦️packages/🦀️rust')
        if os.path.exists(os.path.join(cand, 'Cargo.toml')): man = cand; break
        if os.path.exists(os.path.join(cur, 'Cargo.toml')): man = cur; break
        cur = os.path.dirname(cur)
    if man is None: continue
    for i, l in enumerate(t.split('\n'), 1):
        for m in pat.finditer(l):
            rel = m.group(1).lstrip('/')
            if not rel.startswith('..'): continue
            target = os.path.normpath(os.path.join(man, rel))
            if not os.path.exists(target) and 'include' in l:
                bad.append((p, i, man, rel[:60]))
print('files with manifest-relative includes', n, 'unresolvable', len(bad))
for b in bad: print(b[1], b[0].replace('✏️s/🔌️plugins/', ''), '| manifest', b[2].replace('✏️s/🔌️plugins/', '')[-60:])
