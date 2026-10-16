# 🧪️ Test-only files of the plugin crate and the crate that hosts each (directory map first, then the crate whose definitions the file names most, highest rank winning).
# usage: CLOSURE=../🗑️generated/pwp0-closure.json python3 testhosts.py [json-out]
import collections, json, os, re, sys
import plan_tools as pt
EMB = ('.json', '.csv', '.txt', '.md')
def main():
    clo = pt.CLOSURE
    lib = {x['path'] for k in ('lib native', 'lib wasm32-wasip2', 'lib wasm32-unknown-unknown') for x in clo[k]}
    tests = {}
    for k in ('test native', 'test wasm32-wasip2', 'test native all-features'):
        for x in clo[k]:
            if x['path'] not in lib: tests.setdefault(x['path'], x)
    files, items, defs, owner = pt.build(); size, adj, crate_of = pt.graph(items, defs, owner)
    final, moves, _ = pt.simulate(size, adj, crate_of, items)
    unit_crate = {u: c for u, c in final.items()}
    def_crate = collections.defaultdict(collections.Counter)
    for name, ids in defs.items():
        for j in ids:
            u = owner.get(j, j)
            c = unit_crate.get(u) or crate_of.get(u)
            if c: def_crate[name][c] += 1
    order = {c: i for i, c in enumerate(pt.ORDER)}
    rows = []
    for p, x in sorted(tests.items()):
        f = p.replace(pt.P, '')
        direct = pt.file_crate(f)
        host = None; how = None
        if x['path'].endswith('.rs') and not direct.startswith('UNMAPPED') and not direct.startswith('TOP'):
            host, how = direct, 'dir'
        elif direct.startswith('TOP'):
            host, how = 'TOP', 'dir'
        if host is None and x['path'].endswith('.rs'):
            txt = pt.strip(open(pt.ROOT + p, encoding='utf-8').read()); score = collections.Counter()
            for t in pt.tokens(txt):
                if t in pt.GENERIC: continue
                cs = def_crate.get(t)
                if not cs or sum(cs.values()) > 3: continue
                for c in cs: score[c] += 1
            if score:
                best = max(score.values()); cands = [c for c, v in score.items() if v == best]
                host = max(cands, key=lambda c: order.get(c, 999) if not c.startswith('TOP') else 1000); how = 'mention'
            else: host, how = 'none', 'none'
        rows.append(dict(file=f, kind=x['kind'], lines=x['lines'], bytes=x['bytes'], host=host or 'embedded', how=how or 'embedded'))
    agg = collections.defaultdict(lambda: [0, 0, 0])
    for r in rows:
        a = agg[r['host']]; a[0] += 1; a[1] += r['lines']; a[2] += r['bytes']
    for h, a in sorted(agg.items(), key=lambda kv: order.get(kv[0], 999)): print(f'{h:26s} files {a[0]:4d} lines {a[1]:6d} KB {a[2]//1024:5d}')
    if len(sys.argv) > 1: json.dump(rows, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
if __name__ == '__main__': main()
