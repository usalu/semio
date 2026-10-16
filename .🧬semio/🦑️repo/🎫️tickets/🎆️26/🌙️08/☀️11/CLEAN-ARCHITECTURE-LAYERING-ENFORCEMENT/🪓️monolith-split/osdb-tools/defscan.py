"""🔎 Definition scan over the staged crates, with a seeded sensitivity check.

1. every item reached from another crate through a use leaf or qualified path must be `pub` (items, nested modules);
2. restricted-visibility members (`pub(crate)` / `pub(super)` / `pub(in ..)`) of the staged crates whose name another crate uses.
The sensitivity check seeds a private item into an in-memory copy and requires the scan to flag it.
"""
import collections, json, os, re, sys
from lib import *
from rs import idents
from widen import Crate, refs_of, MEMBER_FN, FIELD

def scan(crates):
    problems = []
    for x in crates.values():
        users = collections.defaultdict(set)
        for y in crates.values():
            if y is x: continue
            for f in y.files:
                for rest in refs_of(x.lib, y, f):
                    users[rest].add(y.lib)
        for rest, ys in users.items():
            f = x.root
            for seg in rest:
                if seg == '*': break
                it = next((i for i in x.items(f) if i.name == seg and i.kind not in ('impl', 'use')), None)
                if it is None:
                    reex = next((i for i in x.items(f) if i.kind == 'use' and re.search(r'\b' + re.escape(seg) + r'\b', i.name or '')), None)
                    if reex is None:
                        problems.append(('unresolved', x.lib, '::'.join(rest), sorted(ys)))
                    break
                lines_ = x.lines(f)
                is_pub = it.vis == 'pub' or (it.kind == 'static' and any(re.match(r'\s*pub\s+static\s', l) for l in lines_[it.header:it.end + 1]))
                if not is_pub:
                    problems.append(('not-pub', x.lib, seg, sorted(ys)))
                if it.kind == 'mod':
                    tgt = x.mounts(f).get(seg)
                    if tgt is None: break
                    f = tgt; continue
                break
    return problems

def restricted_members(crates):
    out = []
    idset = {y.lib: set().union(*[idents(y.stripped[f]) for f in y.files]) for y in crates.values()}
    for x in crates.values():
        for f in x.files:
            lines = x.lines(f)
            for it in x.items(f):
                if it.kind == 'impl' and not it.trait:
                    for ln in range(it.header + 1, it.end):
                        m = MEMBER_FN.match(lines[ln])
                        if m and (m.group('vis') or '').strip().startswith('pub(') and any(m.group('name') in idset[y.lib] for y in crates.values() if y is not x):
                            out.append((x.lib, it.self_type, m.group('name')))
    return out

def main(stage, cj):
    meta = json.load(open(cj))
    crates = {c['lib']: Crate(stage, c['lib'], c['root'], c['files']) for c in meta}
    base = scan(crates)
    print('not-pub / unresolved cross-crate references:', len(base))
    for p in base[:30]: print('  ', p)
    # sensitivity: seed a private item
    x = next(c for c in crates.values() if c.lib == 'db_ids')
    name = next(i.name for i in x.items(x.root) if i.kind == 'struct' and i.vis == 'pub')
    ln = next(i.header for i in x.items(x.root) if i.name == name)
    saved = x.lines(x.root)[ln]
    from model import parse_items
    x.set_line(x.root, ln, re.sub(r'^(\s*)pub\s+', r'\1', saved))
    x.parsed[x.root] = parse_items(x.text[x.root])
    seeded = scan(crates)
    x.set_line(x.root, ln, saved)
    x.parsed[x.root] = parse_items(x.text[x.root])
    assert len(seeded) > len(base), 'sensitivity check failed: the seeded private item was not flagged'
    print('sensitivity check: seeded private', name, 'flagged', len(seeded) - len(base), 'new problem(s)')
    rm = restricted_members(crates)
    print('restricted-visibility inherent methods used by other crates:', len(rm))
    for r in rm[:30]: print('  ', r)
    return 1 if base else 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1], sys.argv[2]))
