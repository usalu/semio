"""📍 Run-time fixture and source path literals of the board test targets resolve against the package manifest directories."""
import os, re, sys
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
LIVE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
APPLY = sys.argv[1] if len(sys.argv) > 1 else SC + "/apply-b/INF"
n = 0; bad = []
def exists(rel):
    return os.path.exists(APPLY + '/' + rel) or os.path.exists(LIVE + '/' + rel)
for dp, dn, fn in os.walk(APPLY + '/🎲️board'):
    if 'node_modules' in dp: continue
    for f in fn:
        if not f.endswith('.rs'): continue
        p = os.path.join(dp, f); rel = p[len(APPLY) + 1:]
        t = open(p, encoding='utf-8').read()
        m = re.search(r'^(.*?)/(?:🧪️tests/.*|🦀️\.rs)$', rel)
        crate = None
        parts = rel.split('/')
        for k in range(len(parts), 0, -1):
            cand = '/'.join(parts[:k]) + '/📦️packages/🦀️rust'
            if os.path.exists(APPLY + '/' + cand + '/Cargo.toml'): crate = cand; break
        if crate is None: continue
        lits = re.findall(r'read_static\(\s*"([^"]+)"', t)
        for blk in re.findall(r'read_joined\(\s*&\[([^\]]*)\]', t): lits += re.findall(r'"([^"]+)"', blk)
        lits += re.findall(r'(?:include_(?:str|bytes)!|include!)\(\s*"([^"]+)"', t) + re.findall(r'#\[path\s*=\s*"([^"]+)"\]', t)
        for l in lits:
            n += 1
            tgt = os.path.normpath(crate + '/' + l) if not l.startswith('/') else l
            if not exists(tgt): bad.append((rel[-70:], l, tgt[-70:]))
print('literals', n, 'missing', len(bad))
for b in bad[:30]: print(b)
