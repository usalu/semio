# 📸 Pre-edit snapshots of every existing file a dump would edit (hash-named copies plus INDEX.txt), so a half-applied batch can be rolled back. usage: snap.py <dump.json> <dir>
import json, os, sys, hashlib
d = json.load(open(sys.argv[1])); out = sys.argv[2]
os.makedirs(out, exist_ok=True)
R = '/Users/ueli/Documents/semio'
idx = []
for p, t in d['out'].items():
    if not os.path.exists(p): continue
    cur = open(p, encoding='utf8').read()
    if cur == t: continue
    h = hashlib.sha1(p.encode()).hexdigest()
    open(f'{out}/{h}.rs', 'w', encoding='utf8').write(cur)
    idx.append(f'{h}.rs\t{os.path.relpath(p, R)}')
for extra in ('🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml',):
    p = f'{R}/{extra}'; h = hashlib.sha1(p.encode()).hexdigest()
    open(f'{out}/{h}.toml', 'w', encoding='utf8').write(open(p, encoding='utf8').read()); idx.append(f'{h}.toml\t{extra}')
open(f'{out}/INDEX.txt', 'w', encoding='utf8').write('\n'.join(idx) + '\n')
print(len(idx), 'snapshots in', out)
