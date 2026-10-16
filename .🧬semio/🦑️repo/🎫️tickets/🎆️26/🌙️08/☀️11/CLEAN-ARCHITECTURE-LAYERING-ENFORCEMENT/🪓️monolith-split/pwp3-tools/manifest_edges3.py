"""📋️ First-party edges of the WP3 crates on the staged tree (item-level token references, file-based crate assignment) against the edges their Cargo.toml declares. usage: CLOSURE=<closure.json> python3 manifest_edges3.py"""
import collections, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g
pt = g.pt
PLUG = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
ROOTS = {'child': '🪆️child', 'composition-emission': '🧩️composition', 'composition-publication': '🧩️composition/📬️publication', 'publication-mounted': '🧩️composition/📬️publication/🤝️group/🪟️mounted', 'document-media': '🗃️document', 'job': '👷️job', 'command': '🎮️command', 'peer': '👥️peer', 'retained-command': '🧵️retained-command', 'extension': '🧩️extension'}
MAN = json.load(open(HERE + '/../🗑️generated/pwp3-manifests.json'))
def declared(crate):
    key = [k for k in MAN if k.endswith('/📦️packages/🦀️rust') and k.startswith('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/' + ROOTS[crate] + '/📦️packages')][0]
    t = MAN[key]
    return {k.replace('semio-framework-plugin-', '') for k in t.get('dependencies', {}) if k.startswith('semio-framework-plugin-')}
items, size, adj, crate_of = g.run()
edges = collections.defaultdict(collections.Counter)
for u, vs in adj.items():
    cu = crate_of[u]
    if cu not in g.WP3: continue
    for v in vs:
        cv = crate_of[v]
        if cv != cu and not cv.startswith(('TOP', 'UNMAPPED')): edges[cu][cv] += 1
for c in sorted(g.WP3, key=lambda c: g.RANK[c]):
    try: dec = declared(c)
    except Exception as e: dec = set(); print('declared failed', c, e)
    used = set(edges[c])
    print(f'{c}: used {sorted(used, key=lambda x: g.RANK.get(x, 99))}')
    print(f'    declared-not-used {sorted(dec - used)}   used-not-declared {sorted(used - dec)}   upward {sorted(x for x in used if g.RANK.get(x, -1) > g.RANK[c])}')
