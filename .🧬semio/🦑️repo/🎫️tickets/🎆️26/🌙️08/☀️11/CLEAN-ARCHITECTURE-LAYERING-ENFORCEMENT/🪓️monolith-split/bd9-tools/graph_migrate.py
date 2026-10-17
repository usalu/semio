#!/usr/bin/env python3
"""bd9: stdio semio graph subset off the removed `semio_framework_graph`: add graph-engine/-manifest deps (workspace dep, 4 crate manifests, lock). usage: graph_migrate.py [--apply] (cwd = repo root)"""
import re, sys, os
apply = '--apply' in sys.argv
A = '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio'
G = A + '/🏅️standards/🔖️v1/🪆️subsets/🕸️graph'
crates = [G + '/📦️packages/🦀️rust', G + '/🧬️schema/📦️packages/🦀️rust', G + '/🧬️schema/💡️inferences/📦️packages/🦀️rust', G + '/📚️examples/🕸️wires/📦️packages/🦀️rust']
NEW = ['semio-framework-graph-engine', 'semio-framework-graph-manifest']
def write(p, t):
    if apply:
        open(p + '.bd9tmp', 'w', encoding='utf-8').write(t); os.replace(p + '.bd9tmp', p)
ws = open(A + '/Cargo.toml', encoding='utf-8').read()
anchor = '[workspace.dependencies.semio-framework-graph-algorithms]\npath = "../../../../../🧰️framework/🔨️modules/🕸️graph/🧮️algorithms/📦️packages/🦀️rust"\n'
assert ws.count(anchor) == 1
if 'semio-framework-graph-engine]' not in ws:
    ws = ws.replace(anchor, anchor + '\n[workspace.dependencies.semio-framework-graph-engine]\npath = "../../../../../🧰️framework/🔨️modules/🕸️graph/⚙️engine/📦️packages/🦀️rust"\n\n[workspace.dependencies.semio-framework-graph-manifest]\npath = "../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📦️packages/🦀️rust"\n')
    write(A + '/Cargo.toml', ws)
names = []
for c in crates:
    p = c + '/Cargo.toml'
    t = open(p, encoding='utf-8').read()
    name = re.search(r'^name = "([^"]+)"', t, re.M).group(1)
    names.append(name)
    if 'semio-framework-graph-engine' not in t:
        t = t.replace('[dependencies]\n', '[dependencies]\nsemio-framework-graph-engine = { workspace = true }\nsemio-framework-graph-manifest = { workspace = true }\n', 1)
        write(p, t)
lock = open(A + '/Cargo.lock', encoding='utf-8').read()
src = open('✏️s/🔌️plugins/🀄️wfc/Cargo.lock', encoding='utf-8').read() + '\n' + open('✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.lock', encoding='utf-8').read()
def blk(text, n):
    m = re.search(r'(\[\[package\]\]\nname = "' + re.escape(n) + r'"\n.*?)(?=\n\[\[package\]\]|\Z)', text, re.S)
    return m.group(1).rstrip('\n') + '\n' if m else None
head, *bs = re.split(r'\n(?=\[\[package\]\])', lock)
have = {re.search(r'name = "([^"]+)"', b).group(1) for b in bs}
for n in NEW:
    if n not in have:
        b = blk(src, n)
        assert b, n
        bs.append(b)
def adddeps(b):
    m = re.search(r'name = "([^"]+)"', b)
    if m.group(1) in names:
        def fix(mm):
            items = set(re.findall(r'^ "([^"]+)",?$', mm.group(2), re.M)) | set(NEW)
            return mm.group(1) + ''.join(f' "{i}",\n' for i in sorted(items)) + mm.group(3)
        return re.sub(r'(dependencies = \[\n)((?: "[^"]+",?\n)+)(\]\n?)', fix, b)
    return b
bs = [adddeps(b) for b in bs]
bs.sort(key=lambda b: re.search(r'name = "([^"]+)"', b).group(1))
bs = [b if b.endswith('\n') else b + '\n' for b in bs[:-1]] + [bs[-1]]
write(A + '/Cargo.lock', '\n'.join([head] + [b.rstrip('\n') + ('\n' if i < len(bs) - 1 else '') for i, b in enumerate(bs)]) if False else '\n'.join([head] + bs))
print('names', names)
