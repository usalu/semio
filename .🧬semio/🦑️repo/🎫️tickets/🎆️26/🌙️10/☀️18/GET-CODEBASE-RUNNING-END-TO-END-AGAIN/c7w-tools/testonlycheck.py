"""🧪 C6UI test-only re-export check: every `#[cfg(test)] use child::{names}` line the splitter emitted in a root (or shell) is checked against the PRODUCTION files mounted from that scope (spec `external` without a `🧪️tests` path component): a name used there needs an unconditional use.
usage: testonlycheck.py <tag>... (live tree)"""
import importlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import modsplit as M
bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.join(M.REPO, os.path.dirname(sp['file']))
    levels = [os.path.join(base, '🦀️.rs')] + [os.path.join(base, i['folder'], '🦀️.rs') for i in M.inners_of(sp)]
    prod = [p for p in sp.get('external', []) if '🧪️tests' not in p.split('/') and '🧪️' not in p.split('/')[-2]]
    prod_ids = {}
    for p in prod:
        f = os.path.join(M.REPO, p)
        if os.path.isfile(f):
            prod_ids[p] = M.ident_set(open(f, encoding='utf-8').read())
    for lv in levels:
        if not os.path.isfile(lv):
            continue
        lines = open(lv, encoding='utf-8').read().split('\n')
        i = 0
        while i < len(lines):
            if lines[i].strip() == '#[cfg(test)]' and i + 1 < len(lines) and re.match(r'^(pub(\([^)]*\))? )?use ', lines[i + 1]):
                j = i + 1
                stmt = lines[j]
                while not stmt.rstrip().endswith(';'):
                    j += 1
                    stmt += lines[j]
                names = set(re.findall(r'\b([A-Za-z_]\w*)\b', stmt.split('::', 1)[1])) - {'pub', 'use', 'crate', 'super', 'self'}
                for p, ids in prod_ids.items():
                    hit = names & ids
                    if hit:
                        print('TESTONLY-USED-IN-PRODUCTION', tag, os.path.relpath(lv, M.REPO)[-60:], sorted(hit), 'used by', p[-70:])
                        bad += 1
                i = j
            i += 1
print('testonlycheck', 'OK' if not bad else 'FAIL %d' % bad)
