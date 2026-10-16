"""📦️ Adds the WP3-to-WP3 first-party dependency edges the staged sources really use (downward only, plan 10.4.5) to the crate manifests; prints what it added."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
src = open(os.path.dirname(os.path.abspath(__file__)) + '/edges.py', encoding='utf8').read().split("for crate in sorted(RANK")[0]
ns = {'__file__': os.path.dirname(os.path.abspath(__file__)) + '/edges.py'}
exec(src, ns)
fwd, RANK = ns['fwd'], ns['RANK']; C = ns['C']
NOISE = {'physical', 'pointer', '_', 'now_us', 'candidate', 'active'}
V = '️'
added = {}
for crate, deps in fwd.items():
    folder = C.SUFFIX_FOLDER[crate].replace('@', V)
    path = f'{C.T}/{folder}/📦{V}packages/🦀{V}rust/Cargo.toml'
    text = open(path, encoding='utf8').read()
    i = text.index('\n[dependencies]\n') + len('\n[dependencies]\n')
    j = text.index('\n\n', i) if '\n\n' in text[i:] else len(text)
    block = text[i:j].split('\n')
    have = {l.split(' = ')[0] for l in block}
    for d, names in deps.items():
        if set(names) <= NOISE or RANK[d] >= RANK[crate]: continue
        pkg = 'semio-framework-os-renderer-wgpu-' + d
        if pkg in have: continue
        dfolder = C.SUFFIX_FOLDER[d].replace('@', V)
        block.append(f'{pkg} = {{ path = "../../../{dfolder}/📦{V}packages/🦀{V}rust" }}')
        added.setdefault(crate, []).append(d)
    first = sorted([l for l in block if l.startswith('semio-framework-os-renderer-wgpu-')])
    rest = [l for l in block if l and not l.startswith('semio-framework-os-renderer-wgpu-')]
    new = '\n'.join(first + rest)
    if new != text[i:j]:
        open(path, 'w', encoding='utf8').write(text[:i] + new + text[j:])
print(added)
