import os, re, sys, glob, json
R = '/Users/ueli/Documents/semio'
F = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
def name_of(path):
    t = open(path, encoding='utf8').read()
    if not re.search(r'^\[package\]', t, re.M): return None
    m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
    return m.group(1) if m else None
def coverage(art):
    base = os.path.join(R, art)
    facade = name_of(os.path.join(base, '📦️packages/🦀️rust/Cargo.toml'))
    total = 0; gated = 0
    for root, dirs, files in os.walk(base):
        dirs[:] = [d for d in dirs if d not in ('node_modules', 'target', 'dist')]
        if 'Cargo.toml' in files and root.endswith('📦️packages/🦀️rust'):
            rel = os.path.relpath(root, base)
            if rel.startswith('🔮️oracles') or '🏭️bridge/' in rel + '/': continue
            n = name_of(os.path.join(root, 'Cargo.toml'))
            if n is None: continue
            total += 1
            if n == facade or n.startswith(facade + '-'): gated += 1
    return facade, total, gated
if __name__ == '__main__':
    out = {}
    for q in ('a', 'b', 'c', 'd'):
        for l in open(os.path.join(F, 'pgr-tools', 'queue-%s.txt' % q), encoding='utf8'):
            l = l.strip()
            if not l: continue
            tag, pkg, base = l.split('|')
            art = pkg[:-len('/📦️packages/🦀️rust')]
            out[tag] = (art,) + coverage(art)
    json.dump(out, open(os.path.join(F, '🗑️generated', 'pgr-coverage.json'), 'w', encoding='utf8'), ensure_ascii=False)
    for k, v in out.items(): print(k, v[1], v[2], v[3])
