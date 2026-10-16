# definition scan: every method, associated function and top-level item that a module of the instance family uses from ANOTHER module
# must be visible there. Name based and conservative (a false positive is a private same-name definition, never a miss of a real use).
import sys; sys.path.insert(0, '.')
from lib6 import *
from methods6 import FN
FN_HEAD = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+))')
ITEM_HEAD = re.compile(r'^((?:pub(?:\([^)]*\))?\s+)?)(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|trait|type|static|const|union)\s+(\w+)')

def family_files(root=ROOT, touched=None):
    t = touched or json.load(open(GEN + '/pwp6-touched.json', encoding='utf-8'))
    files = sorted(set(t['created'] + t['modified']) - set(t['deleted']))
    return [f for f in files if f.endswith('.rs') and os.path.exists(root + PLUGP + f) and ('🧵️instance' in f or '🪜️' in f or '⏪️time-travel/🧵️instance' in f or '⏯️tool-run/🧵️instance' in f)]

def tree_of(rel):
    if rel.startswith('⏪️time-travel/🧵️instance/') and '📥️staging' not in rel: return 'tt'
    if rel.startswith('⏯️tool-run/🧵️instance/') and '📥️staging' not in rel: return 'tr'
    if rel.endswith('📮️action-ingress/🦀️.rs'): return 'ai'
    return 'instance'

def parse_module(root, rel):
    raw = open(root + PLUGP + rel, encoding='utf-8').read()
    M = mask(raw); R = raw.split('\n'); L = M.split('\n')
    defs = []   # (kind, name, vis, line)
    depth = 0; impl_stack = []
    cur_impl = None
    for i, l in enumerate(L):
        before = depth
        depth = before + l.count('{') - l.count('}')
        s = l.strip()
        if before == 0 and l[:1] not in (' ', '\t', '') and not s.startswith(('#', '//', '}')):
            m = ITEM_HEAD.match(s)
            if m: defs.append(('item', m.group(3), m.group(1).strip(), i + 1, m.group(2)))
            im = re.match(r'(?:unsafe\s+)?impl\b', s)
            if im:
                tgt, tr = pt.impl_target(s) if '{' in s or True else (None, None)
                cur_impl = (tgt, tr)
            else: cur_impl = None
        elif before == 1 and cur_impl is not None:
            m = FN_HEAD.match(l)
            if m:
                vis = re.match(r'\s*(pub(?:\([^)]*\))?)\s', l); vis = vis.group(1) if vis else ''
                defs.append(('method', m.group(3), vis if cur_impl[1] is None else 'pub', i + 1, cur_impl))
    return dict(rel=rel, raw=raw, M=M, defs=defs)

def token_sets(M):
    words = set(re.findall(r'(?<![\w])(?<![^.]\.)([A-Za-z_]\w*)\b', M))
    dotted = set(re.findall(r'(?:\.|::)\s*([A-Za-z_]\w*)\s*(?:\(|::<)', M))
    bare = set(re.findall(r'(?<![\w:])(?<![^.]\.)([A-Za-z_]\w*)\s*(?:\(|::<)', M))
    return words, dotted | bare

def scan(root=ROOT, touched=None, quiet=False):
    files = family_files(root, touched)
    mods = {f: parse_module(root, f) for f in files}
    for f, m in mods.items(): m['words'], m['calls'] = token_sets(m['M'])
    byname = collections.defaultdict(list)
    for f, m in mods.items():
        for d in m['defs']: byname[d[1]].append((f, d))
    viol = []
    for uf, um in mods.items():
        for name, ds in byname.items():
            if any(f == uf for f, d in ds): continue
            used_call = name in um['calls']; used_word = name in um['words']
            if not used_call and not used_word: continue
            visible = []
            for f, d in ds:
                kind, nm, vis = d[0], d[1], d[2]
                if kind == 'method' and not used_call: continue
                if kind == 'item' and not used_word: continue
                ok = vis in ('pub', 'pub(crate)') or vis.startswith('pub(in') or (vis == 'pub(super)' and tree_of(f) == tree_of(uf) == 'instance')
                visible.append(ok)
            if visible and not any(visible):
                viol.append((uf, name, [(f, d[2] or 'private', d[0]) for f, d in ds]))
    if not quiet:
        print('definition scan: modules', len(mods), 'names', len(byname), 'violations', len(viol))
        for v in viol[:80]: print('  ', '/'.join(v[0].split('/')[-3:-1]), 'uses', v[1], 'defined private in', [('/'.join(x[0].split('/')[-3:-1]), x[1]) for x in v[2]])
    return viol

if __name__ == '__main__':
    scan()
