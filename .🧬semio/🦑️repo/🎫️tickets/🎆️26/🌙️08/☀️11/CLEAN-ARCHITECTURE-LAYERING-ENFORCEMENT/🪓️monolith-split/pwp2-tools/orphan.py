#!/usr/bin/env python3
# 🧿️ Orphan-rule scan of the carved WP2 crates: every `impl Trait for Type` must have its trait or its (outermost) self type defined in the same crate.
# The union oracle cannot see E0117 (all crates are one crate there), so this token scan stands in. Reads the OUT tree (falls back to the snapshot).
import re, os, sys, collections
from common import *
import model as M
import modtree
sys.argv_saved = list(sys.argv); sys.argv = ['x']
import plan_tools as pt
from rs import blank

def tree_files(key, base):
    root = base + '/' + M.ROOT[key]
    return [f for f in modtree.tree(root)]

def defined(files):
    types, traits = set(), set()
    for f in files:
        t = read(f)
        its, L, R = pt.outline(f)
        for it in its:
            nm = re.match(r'[A-Za-z_]\w*', it['name']) if it.get('name') else None
            if not nm:
                continue
            if it['kind'] in ('struct', 'enum', 'union', 'type'):
                types.add(nm.group(0))
            elif it['kind'] == 'trait':
                traits.add(nm.group(0))
        for m in re.finditer(r'macro_rules!\s*(\w+)', blank(t)):
            pass
    return types, traits

def main(base):
    bad = 0
    total = [0]
    for key in M.WP2:
        files = [f for f in tree_files(key, base) if '/🧪️tests/' not in f]
        types, traits = defined(files)
        for f in files:
            its, L, R = pt.outline(f)
            for it in its:
                if it['kind'] != 'impl':
                    continue
                ty, tr = pt.impl_target(it['head'])
                if not tr:
                    continue
                total[0] += 1
                if tr in traits or ty in types:
                    continue
                args = re.search(r'impl\s*(?:<.*?>\s*)?[A-Za-z_:]+<([^{]*?)>\s*for\b', it['head'])
                if args and any(x in types for x in re.findall(r'[A-Za-z_]\w*', args.group(1))):
                    continue
                head = it['head']
                if re.search(r'impl\s*<[^>]*>\s*' + re.escape(tr) + r'[^{]*\bfor\s+(?:&\s*)?[A-Z]\b', head):
                    pass
                bad += 1
                print(f'{key}: {os.path.relpath(f, base)}:{it["s"]}: impl {tr} for {ty}')
    print('trait impls scanned:', total[0], '; orphan candidates:', bad)

if __name__ == '__main__':
    main(sys.argv_saved[1] if len(sys.argv_saved) > 1 else WS + '/out')
