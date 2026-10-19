"""🧩 c7w overlay builder: mkoverlay.py <out.json> <tag>... maps each installed tag's root to its pre-split snapshot and hides its children (POS base rev "overlay")."""
import sys, os, json, importlib
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import modsplit as P
out = sys.argv[1]
m, hide = {}, []
for tag in sys.argv[2:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.dirname(sp['file'])
    m[sp['file']] = f"{HERE}/orig/{sp['orig']}"
    for f in P.rel_folders(sp):
        hide.append(f"{base}/{f}/🦀️.rs")
json.dump(dict(map=m, hide=hide), open(out, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('overlay', len(m), 'roots', len(hide), 'hidden')
