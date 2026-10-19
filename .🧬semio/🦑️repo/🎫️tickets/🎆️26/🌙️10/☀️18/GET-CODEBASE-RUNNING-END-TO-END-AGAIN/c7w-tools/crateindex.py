"""📦 c7w owner index: builds closure(lib root) for every crate under the renderer and ui trees and prints, per tag, the crates whose #[path]/include! closure contains the tag's root file. usage: crateindex.py <tag>..."""
import os, re, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from crateof import closure, REPO, PK
tags = json.load(open(HERE + '/tags.json'))
roots = ['🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer', '🧰️framework/🔨️modules/🖱️ui', '✏️s/🔌️plugins/🏙️bim', '🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite']
crates = {}
for r in roots:
    for dp, dn, fn in os.walk(os.path.join(REPO, r)):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target', 'dist')]
        if dp.endswith('📦️packages/🦀️rust') and 'Cargo.toml' in fn:
            t = open(os.path.join(dp, 'Cargo.toml'), encoding='utf-8').read()
            m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
            lib = re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"', t, re.M | re.S)
            root = os.path.normpath(os.path.join(dp, lib.group(1) if lib else 'src/lib.rs'))
            crates[m.group(1)] = root
out = {}
for name, root in crates.items():
    cl = closure(root)
    for tag, p in tags.items():
        if os.path.join(REPO, p) in cl:
            out.setdefault(tag, []).append((name, os.path.relpath(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(root)))) if root.endswith('🦀️.rs') and False else root, REPO)))
for tag in (sys.argv[1:] or sorted(tags)):
    print(tag, [n for n, _ in out.get(tag, [])])
json.dump({t: [n for n, _ in v] for t, v in out.items()}, open(HERE + '/owners.json', 'w'), indent=1)
json.dump(crates, open(HERE + '/crateroots.json', 'w'), ensure_ascii=False, indent=1)
