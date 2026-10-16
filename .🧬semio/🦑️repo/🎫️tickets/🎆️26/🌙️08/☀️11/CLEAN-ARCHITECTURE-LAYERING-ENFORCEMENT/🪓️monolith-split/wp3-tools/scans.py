"""🔍️ Static scans of the WP3 staging: path resolution, region balance, orphan/inherent ownership and the definition scan (private names used outside their module subtree)."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv, crate_map as C
ROOT = C.ROOT
ALL = [f for fs in list(C.MAP.values()) + list(C.LIFTS.values()) for f in fs]
OWN = C.owners()
rd = lambda p: open(p, encoding='utf8').read()

def mounts(path, text):
    out = []
    for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
        out.append(('path', m.group(1), m.group(2), m.start()))
    for m in re.finditer(r'\binclude(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)', text):
        out.append(('include', m.group(1), None, m.start()))
    return out

def enclosing_mods(text, pos):
    depth = 0; stack = []
    for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{|[{}]', text[:pos]):
        s = m.group(0)
        if s == '{': stack.append(None)
        elif s == '}':
            if stack: stack.pop()
        else: stack.append(m.group(1))
    return [x for x in stack if x]

def pathcheck():
    bad = []; n = 0
    for f in ALL:
        t = rd(f)
        for kind, rel, mod, pos in mounts(f, t):
            n += 1
            base = os.path.dirname(f)
            mods = enclosing_mods(t, pos) if kind == 'path' else []
            cand = os.path.normpath(os.path.join(base, *mods, rel)) if kind == 'path' else os.path.normpath(os.path.join(base, rel))
            if not os.path.exists(cand): bad.append((os.path.relpath(f, C.T), rel))
            if len(os.path.relpath(cand, ROOT).encode()) > 240: bad.append(('LONG', cand))
    return n, bad

def regions():
    bad = []
    for f in ALL:
        c = collections.Counter()
        for m in re.finditer(r'//#(end)?region (.*)', rd(f)):
            c[m.group(2).strip()] += -1 if m.group(1) else 1
        for k, v in c.items():
            if v != 0: bad.append((os.path.relpath(f, C.T), k, v))
    return bad

TYPE_DEF = re.compile(r'\b(?:struct|enum|union|trait|type)\s+([A-Za-z_]\w*)')
def orphan():
    tdef = {}
    for crate, fs in list(C.MAP.items()) + list(C.LIFTS.items()):
        for f in fs:
            for it in mv.split_items(rd(f)):
                if it.kind in ('struct', 'enum', 'union', 'type', 'trait'):
                    m = TYPE_DEF.search(mv.key(it))
                    if m: tdef.setdefault(m.group(1), set()).add(crate)
    bad = []
    for crate, fs in list(C.MAP.items()) + list(C.LIFTS.items()):
        for f in fs:
            for it in mv.split_items(rd(f)):
                if it.kind != 'impl': continue
                line = mv.key(it)
                if re.match(r'^(?:unsafe\s+)?impl\b[^{]*\sfor\s', line): continue
                m = re.match(r'^(?:unsafe\s+)?impl(?:<[^>]*>)?\s+(?:[\w:]+::)?([A-Za-z_]\w*)', line)
                if m and m.group(1) in tdef and crate not in tdef[m.group(1)]:
                    bad.append((crate, m.group(1), sorted(tdef[m.group(1)])))
    return bad

def children_of():
    kids = collections.defaultdict(set)
    for f in ALL:
        t = rd(f)
        for kind, rel, mod, pos in mounts(f, t):
            if kind == 'path':
                p = os.path.normpath(os.path.join(os.path.dirname(f), *enclosing_mods(t, pos), rel))
                if p in OWN: kids[f].add(p)
    return kids

def subtree(f, kids):
    out = {f}; stack = [f]
    while stack:
        x = stack.pop()
        for k in kids.get(x, ()):
            if k not in out: out.add(k); stack.append(k)
    return out

def defscan():
    kids = children_of()
    priv_items = collections.defaultdict(dict)  # file -> name -> kind  (private top-level items, fields, methods)
    pubnames = set()
    for f in ALL:
        t = rd(f)
        pv = mv.names_private(t)
        priv_items[f] = pv
        for it in mv.split_items(t):
            line = mv.key(it)
            if re.match(r'^\s*pub\b', line):
                m = re.search(r'\b(?:fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)', line)
                if m: pubnames.add(m.group(1))
            if it.kind == 'impl':
                for m in re.finditer(r'^    pub(?:\([^)]*\))?\s+(?:async\s+|const\s+|unsafe\s+)*fn\s+(\w+)', it.text, re.M): pubnames.add(m.group(1))
            if it.kind == 'struct':
                for m in re.finditer(r'^    pub(?:\([^)]*\))?\s+(\w+)\s*:', it.text, re.M): pubnames.add(m.group(1))
    hits = []
    ids = {f: set(re.findall(r'[A-Za-z_]\w*', re.sub(r'//[^\n]*', '', rd(f)))) for f in ALL}
    for f, names in priv_items.items():
        sub = subtree(f, kids)
        for n, kind in names.items():
            if n in pubnames or len(n) < 5: continue
            for g in ALL:
                if g in sub or n not in ids[g]: continue
                hits.append((os.path.relpath(f, C.T), os.path.relpath(g, C.T), n, kind))
    return hits

if __name__ == '__main__':
    n, bad = pathcheck(); print('paths', n, 'bad', bad)
    print('regions bad', regions())
    print('orphan', orphan())
    h = defscan(); print('defscan hits', len(h))
    for x in h[:60]: print('  ', x)
