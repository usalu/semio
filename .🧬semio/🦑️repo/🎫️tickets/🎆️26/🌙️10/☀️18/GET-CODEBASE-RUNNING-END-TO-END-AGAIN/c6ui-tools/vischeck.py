"""🔭 C6UI visibility-path check: every top-level item of a snapshot with a restricted-but-wider-than-private visibility (`pub(crate)`, `pub(super)`, `pub(in …)`, and `pub`) must still be nameable at the ORIGINAL path in the live root (or the inner shell) with at least that visibility: defined there, or imported by a `use` that names it. usage: vischeck.py <tag>... (live tree)"""
import importlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import modsplit as M
RANK = {'pub': 4, 'pub(crate)': 3, 'pub(super)': 2}
bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(HERE, 'orig', sp['orig']), encoding='utf-8').read()
    if sp.get('pre'):
        orig = sp['pre'](orig)
    doc = M.Doc(orig)
    base = os.path.join(M.REPO, os.path.dirname(sp['file']))
    root = open(os.path.join(base, '🦀️.rs'), encoding='utf-8').read()
    scopes = [('', doc.items, doc.lines, root)]
    for inn in M.inners_of(sp):
        its = [it for it in doc.items if it['kind'] == 'mod' and it['name'] == inn['mod'] and not doc.lines[it['e']].rstrip().endswith(';')]
        it = its[0]
        pre, head, body, hdr = M.inline_body(doc.lines, it, inn.get('indent', 0), tag)
        d2 = M.Doc('\n'.join(body))
        scopes.append((inn['mod'], d2.items, d2.lines, open(os.path.join(base, inn['folder'], '🦀️.rs'), encoding='utf-8').read()))
    for scope, items, lines, text in scopes:
        nm_uses = {}
        for m in re.finditer(r'((?:pub(?:\([^)]*\))?\s+)?)use\s+([^;]*);', text, re.S):
            vis = m.group(1).strip() or 'private'
            for w in re.findall(r'[A-Za-z_]\w*', m.group(2)):
                nm_uses.setdefault(w, set()).add(vis)
        defined = {}
        for it2 in M.Doc(text).items:
            if it2['name'] and it2['kind'] != 'impl':
                defined[it2['name']] = it2['vis'] or 'private'
        for it in items:
            v = it['vis']
            if it['kind'] not in ('fn', 'const', 'static', 'struct', 'enum', 'trait', 'type', 'union', 'mod') or not it['name'] or it['name'] == '_':
                continue
            if v == '' or v == 'pub(self)':
                continue
            if it['kind'] == 'mod' and doc.lines[it['e']].rstrip().endswith(';'):
                continue
            need = 4 if v == 'pub' else 3 if v == 'pub(crate)' else 2
            def rank(x):
                return 4 if x == 'pub' else 3 if x == 'pub(crate)' else 2 if x in ('pub(super)',) or x.startswith('pub(in') else 1
            have = max([rank(x) for x in nm_uses.get(it['name'], [])] + ([rank(defined[it['name']])] if it['name'] in defined else [0]))
            if have < need:
                print('VIS-LOST', tag, scope or '<root>', v, it['kind'], it['name'], 'have rank', have)
                bad += 1
print('vischeck', 'OK' if not bad else 'FAIL %d' % bad)
