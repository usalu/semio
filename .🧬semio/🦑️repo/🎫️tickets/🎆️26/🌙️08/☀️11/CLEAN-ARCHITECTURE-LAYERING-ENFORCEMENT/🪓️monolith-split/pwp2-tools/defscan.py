#!/usr/bin/env python3
# 🔎️ Definition scan of PLUGIN-WP2: private fields, methods and column-0 items of a carved crate that a file outside the crate mentions (the union oracle under-reports E0616 / E0624).
# Names also defined `pub` elsewhere in the plugin tree, and names of up to 3 letters, are skipped as ambiguous. `--seed` plants a private field in decl-views and expects the scan to find it.
import collections, json, os, re, sys
ARGS = list(sys.argv[1:])
from common import *
import model as M
import modtree
from rs import blank
import widen as W

FIELD = re.compile(r'(?m)^[ \t]+((?:pub(?:\([^)]*\))?[ \t]+)?)([a-z_][A-Za-z0-9_]*)[ \t]*:[^:]')
FN = re.compile(r'(?m)^[ \t]+((?:pub(?:\([^)]*\))?[ \t]+)?)(?:async[ \t]+|const[ \t]+|unsafe[ \t]+)*fn[ \t]+([a-z_][A-Za-z0-9_]*)')
ITEM = re.compile(r'(?m)^((?:pub(?:\([^)]*\))?[ \t]+)?)(?:async[ \t]+|const[ \t]+|unsafe[ \t]+)*(?:fn|const|static|struct|enum|type|trait)[ \t]+(?:mut[ \t]+)?([A-Za-z_][A-Za-z0-9_]*)')

def members(text):
    """private fields of structs, private methods of inherent impls, private column-0 items (trait bodies and trait impls are public by definition)"""
    import plan_tools as pt
    tmp = WS + '/tmp-defscan.rs'
    write(tmp, text)
    its, L, R = pt.outline(tmp)
    private = set()
    for it in its:
        body = '\n'.join(L[it['s'] - 1:it['e']])
        head = it['head']
        if it['kind'] == 'struct' and '{' in body:
            i = body.find('{')
            for m in FIELD.finditer(body[i:]):
                if not m.group(1).strip():
                    private.add(m.group(2))
            if not head.startswith('pub'):
                nm = re.match(r'struct\s+([A-Za-z_]\w*)', head.replace('pub(crate) ', '').replace('pub(super) ', ''))
                if nm:
                    private.add(nm.group(1))
        elif it['kind'] == 'impl':
            ty, tr = pt.impl_target(head)
            if tr:
                continue
            for m in FN.finditer(body):
                if not m.group(1).strip():
                    private.add(m.group(2))
        elif it['kind'] in ('fn', 'const', 'static', 'enum', 'type', 'trait', 'union') and not head.startswith('pub'):
            nm = re.search(r'(?:fn|const|static|enum|type|trait|union)\s+(?:mut\s+)?([A-Za-z_]\w*)', head)
            if nm:
                private.add(nm.group(1))
    return private

def members_all(text):
    clean = blank(text)
    out = set()
    for rx in (FIELD, FN, ITEM):
        for m in rx.finditer(clean):
            out.add(m.group(2))
    return out

def main(seed):
    W.make_view()
    if seed:
        p = W.VIEW + '/' + M.ROOT['decl-views']
        t = read(p)
        write(p, t + '\nstruct DefscanSeed { seeded_private_field_for_defscan: u8 }\n')
        q = W.VIEW + '/🧪️testing/🦀️.rs'
        write(q, read(q) + '\nfn defscan_seed_user(x: &DefscanSeed) -> u8 { x.seeded_private_field_for_defscan }\n')
    allf = []
    for dp, dn, fn in os.walk(W.VIEW):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for f in fn:
            if f.endswith('.rs'):
                allf.append(os.path.join(dp, f))
    toks = {f: set(re.findall(r'(?<![A-Za-z0-9_$])[A-Za-z_][A-Za-z0-9_]*', blank(read(f)))) for f in allf}
    pubnames = collections.Counter()
    for f in allf:
        clean = blank(read(f))
        for rx in (FIELD, FN, ITEM):
            for m in rx.finditer(clean):
                if m.group(1).strip().startswith('pub') and not m.group(1).strip().startswith('pub('):
                    pubnames[m.group(2)] += 1
    total = 0
    for key in M.WP2:
        own = set(modtree.tree(W.VIEW + '/' + M.ROOT[key]))
        priv = set()
        for f in own:
            if '/🧪️tests/' in f:
                continue
            priv |= members(read(f))
        priv = {n for n in priv if len(n) > 3 and pubnames[n] == 0}
        hits = collections.defaultdict(set)
        for f, ts in toks.items():
            if f in own:
                continue
            cand = priv & ts
            if not cand:
                continue
            defs = members_all(read(f))
            for n in cand:
                if n in defs:
                    continue
                if re.search(r'(?<![A-Za-z0-9_$])(?:let\s+(?:mut\s+)?|\|\s*|for\s+)' + re.escape(n) + r'\b', blank(read(f))):
                    continue
                hits[n].add(os.path.relpath(f, W.VIEW))
        for n, fs in sorted(hits.items()):
            total += 1
            print(f'{key}: {n}: {sorted(fs)[:2]}')
    print('private definitions mentioned outside their crate:', total)

if __name__ == '__main__':
    main('--seed' in ARGS)
