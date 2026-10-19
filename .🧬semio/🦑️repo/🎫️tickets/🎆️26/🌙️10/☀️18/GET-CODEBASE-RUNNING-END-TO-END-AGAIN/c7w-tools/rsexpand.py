"""📚 c7w repo-text list expander: rsexpand.py — in the renderer test-support `repo-text` crate, every listed split root gains its child files (spec order) right after it."""
import json, os, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import modsplit as P
REPO = '/Users/ueli/Documents/semio'
tags = json.load(open(os.path.dirname(os.path.abspath(__file__)) + '/tags.json', encoding='utf-8'))
ENGINE = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
roots = {}
for tag, p in tags.items():
    if tag == 'bimvalues' or not p.endswith('.rs'):
        continue
    try:
        sp = importlib.import_module('specs.' + tag).SPEC
    except ModuleNotFoundError:
        continue
    roots[p] = [os.path.dirname(p) + '/' + f + '/🦀️.rs' for f in P.rel_folders(sp)]
F = REPO + '/' + ENGINE + '/🎯️targets/🧊️wgpu/🧯️test-support/📜️repo-text/🦀️.rs'
t = open(F, encoding='utf-8').read().split('\n')
out = []
n = 0
for line in t:
    out.append(line)
    s = line.strip()
    if s.startswith('"') and s.endswith('",'):
        rel = s[1:-2]
        full = ENGINE + '/' + rel
        if full in roots:
            for child in roots[full]:
                assert child.startswith(ENGINE + '/')
                ind = line[:len(line) - len(line.lstrip())]
                out.append(f'{ind}"{child[len(ENGINE) + 1:]}",')
                n += 1
open(F, 'w', encoding='utf-8').write('\n'.join(out))
print('added', n)
