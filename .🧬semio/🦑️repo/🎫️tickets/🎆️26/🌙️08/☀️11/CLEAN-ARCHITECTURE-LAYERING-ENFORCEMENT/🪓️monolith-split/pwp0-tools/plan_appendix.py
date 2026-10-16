# 📋️ Generates the markdown of Appendices H (crate registry), I (hub units), J (lift groups) and K (cross-package mount edges) of PLAN-PLUGIN-CRATES.md from the frozen tables.
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C, rootunits as ru, hubsplit as H
def esc(s): return str(s).replace('|', '\\|')
def appendix_h():
    crates = json.load(open(C.GEN + '/pwp0-crates.json'))
    rows = ['| rank | crate | kind | WP | root | stub | first-party dependencies | features |', '|---:|---|---|---:|---|---|---|---|']
    for k, c in sorted(crates.items(), key=lambda kv: kv[1]['rank']):
        deps = ', '.join('`%s`' % d for d in c['deps']) if c['kind'] != 'companion' else '%d crates (hosts %s and their closure, `test-support`)' % (len(c['deps']), ', '.join(c['hosts']))
        stub = 'A: `' + c['root'].split('/')[-1] + '/🦀️.rs`' if c['class'] == 'A' else 'B: `📦️packages/🦀️rust/🦀️.rs`'
        rows.append(f"| {c['rank']} | `{k}` | {c['kind']} | {c['wp']} | `{esc(c['root'])}` | {stub} | {deps or '-'} | {', '.join('`%s`' % f for f in c['features']) or '-'} |")
    return '\n'.join(rows)
def appendix_i():
    rows = ['| hub | module | target (relative to `PLUG/`) | cfg | region |', '|---|---|---|---|---|']
    n = 0
    for hub, (folder, emoji, theme) in H.HUBS.items():
        p = H.HUBDIR + '/' + folder + '/🦀️.rs'
        for u in ru.units(open(p, encoding='utf-8').read()):
            n += 1
            rows.append(f"| WP{hub} `{folder}` | `{u['name']}` | `{esc(re.sub('^../../', '', u['path']))}` | {esc(', '.join(u['cfg'])) or '-'} | {u['region'] or '-'} |")
    for u in ru.units(open(ru.ROOT_RS, encoding='utf-8').read()):
        n += 1
        rows.append(f"| root | `{u['name']}` | `{esc(u['path'])}` | {esc(', '.join(u['cfg'])) or '-'} | {u['region'] or '-'} |")
    return '\n'.join(rows), n
def appendix_j():
    groups = json.load(open(C.GEN + '/pwp0-liftgroups.json'))
    cross = [g for g in groups if g['cross']]; internal = [g for g in groups if not g['cross']]
    rows = ['| lift file (create-if-missing; stub exists) | from WP | to WP | lines | items (largest first) |', '|---|---:|---:|---:|---|']
    for g in cross:
        items = ', '.join(n for n, _, _ in g['items'][:5]) + (', +%d more' % (len(g['items']) - 5) if len(g['items']) > 5 else '')
        rows.append(f"| `{esc(g['lift'])}` (from `{esc(g['file'])}`) | {g['srcWp']} | {g['destWp']} | {g['lines']} | {esc(items)} |")
    rows2 = ['| source file | crate (as cut) -> crate | WP | lines | items |', '|---|---|---:|---:|---|']
    for g in internal:
        items = ', '.join(n for n, _, _ in g['items'][:4]) + (', +%d more' % (len(g['items']) - 4) if len(g['items']) > 4 else '')
        rows2.append(f"| `{esc(g['file'])}` | `{g['src']}` -> `{g['dest']}` | {g['destWp']} | {g['lines']} | {esc(items)} |")
    return '\n'.join(rows), '\n'.join(rows2), len(cross), sum(g['lines'] for g in cross), len(internal), sum(g['lines'] for g in internal)
def appendix_k():
    edges = json.load(open(C.GEN + '/pwp0-crossmounts.json'))
    edges = [e for e in edges if e['parentCrate'] != 'TOP-root']
    rows = ['| parent file (owner WP) | child file (owner WP) | mount | who edits the parent line |', '|---|---|---|---|']
    for e in edges:
        who = 'WP%d (child owner), one line, after re-reading' % e['childWp']
        rows.append(f"| `{esc(e['parent'])}` (WP{e['parentWp']}) | `{esc(e['child'])}` (WP{e['childWp']}) | `{e['mod']}` | {who} |")
    return '\n'.join(rows), len(edges)
if __name__ == '__main__':
    print(appendix_h()[:500]); print(appendix_i()[1]); j = appendix_j(); print(j[2:]); print(appendix_k()[1])
