"""🧮️ Per-unit crate assignment of the WP5 files by the plan's min-cost back-edge simulation (section 5.2) on the CURRENT tree: prints every unit of the time-travel, tool-run, tool-machine and command files whose final crate differs from its file/region crate. usage: CLOSURE=<closure.json> python3 moves5.py"""
import collections, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g
pt = g.pt
files, items, defs, owner = pt.build()
size, adj, crate_of = pt.graph(items, defs, owner)
final, moves, sccs = pt.simulate(size, adj, crate_of, items)
WATCH = ('⏪️time-travel/🦀️.rs', '⏯️tool-run/', '🛠️tool-machine/', '🎮️command/🦀️.rs', '⏪️time-travel/🪟️panel/', '🎮️command/🛬️lifted/', '👷️job/🦀️.rs')
rows = []
for u, c in final.items():
    it = items[u]; f = it['file']
    if not any(f.startswith(w) for w in WATCH): continue
    c0 = crate_of[u]
    if c != c0: rows.append((f, it['s'], it['e'], it['defname'] or it['head'][:50], c0, c, size[u]))
for r in sorted(rows): print(*r)
print('moves total', len(moves), 'lines', sum(m[0] for m in moves))
agg = collections.defaultdict(list)
for l, n, f, t, k in moves: agg[(f, t)].append((l, n))
for (f, t), v in sorted(agg.items(), key=lambda kv: -sum(x[0] for x in kv[1])):
    if {f, t} & {'time-travel', 'time-travel-panel', 'tool-run', 'command'}: print(f'{sum(x[0] for x in v):5d} {f} -> {t}: ' + ', '.join(f'{n[:40]}({l})' for l, n in sorted(v, reverse=True)[:25]))
