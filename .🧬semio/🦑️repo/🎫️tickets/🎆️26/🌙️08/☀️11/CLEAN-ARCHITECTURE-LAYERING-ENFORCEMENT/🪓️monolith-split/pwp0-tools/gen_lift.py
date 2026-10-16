# 🛬️ Creates the lift files of the cross-package item moves (one docstring-only stub per group, in the destination crate's directory). Never overwrites.
# usage: gen_lift.py dry|write
import collections, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C
def main():
    mode = sys.argv[1]
    groups = [g for g in json.load(open(C.GEN + '/pwp0-liftgroups.json')) if g['cross']]
    bydest = collections.defaultdict(list)
    for g in groups: bydest[g['dest']].append(g)
    bad = 0
    for d, gs in bydest.items():
        firsts = collections.Counter(g['folder'].replace('️', '')[:1] for g in gs)
        for g in gs:
            if firsts[g['folder'].replace('️', '')[:1]] > 1: print('SIBLING EMOJI CLASH', d, g['folder']); bad += 1
    written = 0
    for g in groups:
        path = C.PLUG + '/' + g['lift']
        emoji = g['folder'].split('/')[-1][:2] if g['folder'][1:2] == '️' else g['folder'][:1] + '️'
        names = ', '.join(n for n, _, _ in g['items'][:6]) + (', ...' if len(g['items']) > 6 else '')
        text = (f"//! {emoji} Lift file: {len(g['items'])} item(s), about {g['lines']} lines, of `{g['file']}` that belong to crate `{g['dest']}` (WP{g['destWp']}).\n"
                f"//!\n//! WP{g['srcWp']} owns the source file and cuts the items here ({names}); WP{g['destWp']} wires this file with one `mod` line in the crate root. Until `{g['dest']}` exists, the source module re-exports the items with `pub use`.\n")
        if len(path.encode()) > 240: print('PATH TOO LONG', path); bad += 1
        if os.path.exists(path): continue
        if mode == 'write':
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, 'w', encoding='utf-8').write(text)
        written += 1
    print(mode, 'lift files', written, 'of', len(groups), 'bad', bad)
if __name__ == '__main__': main()
