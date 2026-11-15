import json, sys, subprocess, os, re
slug = sys.argv[1]; cat = sys.argv[2] if len(sys.argv) > 2 else 'humans'
T = os.path.dirname(os.path.abspath(__file__))
bun = subprocess.check_output('ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1', shell=True, text=True).strip()
out = subprocess.run([bun, 'author-10-exposed.ts', f'{cat}/{slug}'], cwd=T, capture_output=True, text=True).stdout
line = [l for l in out.splitlines() if l.startswith('50 ')][0]
vals = {m.group(1): (float(m.group(2)), float(m.group(3))) for m in re.finditer(r'([\w-]+)=([0-9.]+)\(([0-9.]+)\)', line)}
S = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/'
import glob
path = glob.glob(S + '*/' + slug + '/🔣️.json')[0]
txt = open(path).read()
d = json.loads(txt)
changed = []
for r in d['asset']['thermal']['regions']:
    if r['id'] in vals:
        new, old = vals[r['id']]
        if abs(new / old - 1) > 0.10:
            n = float(f'{new:.3g}')
            changed.append((r['id'], r['areaM2'], n))
            if '--apply' in sys.argv:
                pass
print(slug, changed)
if '--apply' in sys.argv and changed:
    txt = open(path).read()
    base = txt.index('"regions": [', txt.index('"thermal": {'))
    for rid, old, new in changed:
        m = re.compile(r'"id": "' + re.escape(rid) + r'",.*?"areaM2": ([0-9.eE+-]+)', re.S).search(txt, base)
        assert m and float(m.group(1)) == old, rid
        txt = txt[:m.start(1)] + repr(new) + txt[m.end(1):]
    open(path, 'w').write(txt)
