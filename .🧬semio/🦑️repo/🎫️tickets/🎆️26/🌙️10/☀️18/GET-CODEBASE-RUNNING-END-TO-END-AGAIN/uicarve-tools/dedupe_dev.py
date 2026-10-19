import re, glob, sys
sys.path.insert(0, 'lib')
import carve
UI = carve.UI
files = glob.glob(UI + '/⌨️tui/*/📦️packages/🦀️rust/Cargo.toml') + glob.glob(UI + '/⌨️tui/*/*/📦️packages/🦀️rust/Cargo.toml') + glob.glob(UI + '/🎯️targets/🧊️wgpu/**/📦️packages/🦀️rust/Cargo.toml', recursive=True)
n = 0
for p in files:
    s = open(p, encoding='utf8').read()
    m = re.search(r'\[dependencies\]\n(.*?)(?=\n\[|\Z)', s, re.S)
    nm = set(l.split(' ')[0] for l in (m.group(1).split('\n') if m else []) if l and '=' in l)
    m2 = re.search(r'\[dev-dependencies\]\n(.*?)(?=\n\[|\Z)', s, re.S)
    if not m2: continue
    lines = [l for l in m2.group(1).split('\n') if l]
    keep = [l for l in lines if not (l.split(' ')[0] in nm and l == next((x for x in m.group(1).split('\n') if x.startswith(l.split(' ')[0] + ' ')), None))]
    if len(keep) != len(lines):
        s = s[:m2.start(1)] + '\n'.join(keep) + ('\n' if keep else '') + s[m2.end(1):]
        if not keep: s = s.replace('\n[dev-dependencies]\n\n', '\n').replace('\n[dev-dependencies]\n', '\n')
        open(p, 'w', encoding='utf8').write(s); n += 1
print('deduped', n)
