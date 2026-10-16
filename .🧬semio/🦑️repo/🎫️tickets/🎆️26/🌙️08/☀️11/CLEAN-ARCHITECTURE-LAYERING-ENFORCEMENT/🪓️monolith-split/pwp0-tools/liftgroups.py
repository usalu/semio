# 🛬️ Item-move groups of section 5.2 by (source module, destination crate), with the owning work packages and the lift file each cross-package group lands in.
# usage: CLOSURE=../🗑️generated/pwp0-closure.json python3 liftgroups.py <out.json>
import collections, json, os, re, sys
import plan_tools as pt, crates as C
def main():
    crates, order = C.build()
    files, items, defs, owner = pt.build(); size, adj, crate_of = pt.graph(items, defs, owner)
    final, moves, _ = pt.simulate(size, adj, crate_of, items)
    wp = {k: c['wp'] for k, c in crates.items()}
    import ownership as ow
    groups = collections.defaultdict(lambda: dict(items=[], lines=0))
    for u, dest in final.items():
        it = items[u]; f = it['file']
        sw = ow.owner_wp(f)
        if sw is None or f.startswith(('🧵️instance/', '🪜️')) or crate_of[u].startswith('TOP'): continue
        src = crate_of[u]
        if src == dest and sw == wp[dest]: continue
        if src != dest or sw != wp[dest]:
            key = (dest, os.path.dirname(f), f, sw)
            g = groups[key]; g['items'].append((it['defname'] or it['head'][:30], size[u], it['s'])); g['lines'] += size[u]; g.setdefault('srcs', set()).add(src)
    out = []
    for (dest, mod, f, sw), g in groups.items():
        src = '+'.join(sorted(g['srcs']))
        cross = sw != wp[dest]
        base = mod.split('/')[-1] if mod else '🦀️'
        if f.startswith('🧵️task/'): base = '🧵️task'
        out.append(dict(src=src, dest=dest, srcWp=sw, destWp=wp[dest], file=f, folder=base, lines=g['lines'], items=sorted(g['items'], key=lambda x: -x[1]), cross=cross))
    out.append(dict(src='decl-registry', dest='instance-app', srcWp=2, destWp=7, file='📜️declaration/🌳️tree/🦀️.rs', folder='🌳️tree', lines=63, items=[['editor_surface', 0, 0], ['viewer_surface', 0, 0], ['EditorSurfaceApp', 0, 0], ['ViewerSurfaceApp', 0, 0]], cross=True, note='I3: the four items name VcsArtifactApp (63 lines); re-exported from the facade at the same path'))
    out.append(dict(src='reactor-jobs', dest='job', srcWp=4, destWp=3, file='⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs', folder='🏛️reserved', lines=145, items=[['drive_framework_reserved_registry_job', 0, 0], ['drive_framework_reserved_registry_job_counted', 0, 0]], cross=True, note='I6: the registry driver the instance runs the reserved job through'))
    byd = collections.defaultdict(list)
    for r in out:
        if r['cross']: byd[r['dest']].append(r)
    for d, rs in byd.items():
        seen = {}
        for r in sorted(rs, key=lambda x: -x['lines']):
            k = r['folder'].replace('\ufe0f', '')[:1]
            if k in seen:
                parts = r['file'].split('/')[:-1]
                r['folder'] = parts[-2] + '-' + parts[-1].replace('\ufe0f', '')[1:] if len(parts) > 1 else r['folder']
            seen[r['folder'].replace('\ufe0f', '')[:1]] = 1
    for r in out:
        r['dest_root'] = crates[r['dest']]['root']
        r['lift'] = r['dest_root'] + '/' + C.LIFT + '/' + r['folder'] + '/🦀️.rs'
    seen = collections.Counter((r['dest'], r['folder']) for r in out if r['cross'])
    for r in out:
        if r['cross'] and seen[(r['dest'], r['folder'])] > 1: print('LIFT FOLDER COLLISION', r['dest'], r['folder'], r['file'])
    out.sort(key=lambda r: (-r['cross'], -r['lines']))
    json.dump(out, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
    print(len(out), 'groups;', sum(1 for r in out if r['cross']), 'cross-package;', sum(r['lines'] for r in out), 'lines;', sum(r['lines'] for r in out if r['cross']), 'cross lines')
    for r in out:
        print(('X ' if r['cross'] else '  ') + f"{r['lines']:4d} {r['src']}(WP{r['srcWp']}) -> {r['dest']}(WP{r['destWp']})  {r['file']}  [{len(r['items'])}] {', '.join(n for n,_,_ in r['items'][:3])}")
if __name__ == '__main__': main()
