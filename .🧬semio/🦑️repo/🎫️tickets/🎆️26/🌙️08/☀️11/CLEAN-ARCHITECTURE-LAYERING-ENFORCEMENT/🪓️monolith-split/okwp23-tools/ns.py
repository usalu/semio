# 🧭 Namespace model of the OS kernel module tree for the WP2/WP3 carve: modules, definitions, imports, globs; resolves `super::`/`crate::`/bare paths to the defining module (cfg-agnostic, visibility-agnostic).
import os, re, sys, functools, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import items, tokenize
import okusetree as U
R = '/Users/ueli/Documents/semio'
K = R + '/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules'
S = M + '/🏪️store'
DEF_KINDS = ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'union', 'macro_rules')
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')

class Mod:
    def __init__(self, key, file, parent, name, text, base_dir, offset=0):
        self.key = key; self.file = file; self.parent = parent; self.name = name
        self.defs = {}; self.uses = {}; self.globs = []; self.children = {}; self.impls = []; self.base_dir = base_dir
        self.vis = ''; self.cfg = []; self.text = text; self.offset = offset
        self.tokens = None
        self.cw = None
    def file_module(self):
        m = self
        while m.parent is not None and m.parent.file == m.file and m.key != m.file: m = m.parent
        return m
    def inline_chain(self):
        names = []; m = self
        while m.key != m.file and m.parent is not None:
            names.append(m.name); m = m.parent
        return list(reversed(names))
    def path(self):
        p = []; m = self
        while m.parent is not None: p.append(m.name); m = m.parent
        return list(reversed(p))
    def __repr__(self): return 'Mod(' + '::'.join(self.path()) + ' @' + os.path.relpath(self.file, R)[-40:] + ')'

class World:
    """One crate: the module tree reachable from its lib root."""
    def __init__(self, root_file, ident=None):
        self.files = {}      # file -> Mod (module whose body is the file)
        self.root = None
        self.mods = []
        self.root_file = root_file
        self.ident = ident
    def load_file(self, f, parent, name, vis, cfg, key=None):
        if f in self.files: return self.files[f]
        text = open(f, encoding='utf8').read()
        m = Mod(key or f, f, parent, name, text, os.path.dirname(f))
        m.vis = vis; m.cfg = cfg; m.cw = self
        self.files[f] = m; self.mods.append(m)
        self.fill(m, text)
        return m
    def fill(self, m, text):
        for it in items(text):
            kind = it['kind']
            cfgs = [re.sub(r'\s+', ' ', a) for a in it['attrs'] if re.match(r'#\s*\[\s*cfg', a)]
            if kind in DEF_KINDS and it['name']:
                if it['name'] not in m.defs or not cfgs: m.defs[it['name']] = (kind, it['vis'], it)
                else: m.defs.setdefault('\0' + it['name'], (kind, it['vis'], it))
            elif kind == 'impl':
                m.impls.append(it)
            elif kind == 'use':
                body = re.search(r'\buse\b(.*);', ''.join(t[1] for t in tokenize(it['text']) if t[0] != 'comment'), re.S)
                if not body: continue
                for path, alias, glob in U.parse(body.group(1)):
                    if glob: m.globs.append((path, it['vis'], cfgs))
                    else:
                        nm = U.local_name((path, alias, glob))
                        p2 = path[:-1] if path and path[-1] == 'self' else path
                        m.uses.setdefault(nm, []).append((p2, it['vis'], cfgs))
            elif kind == 'extern':
                mm = re.match(r'(?:pub\s+)?extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?', it['text'].lstrip())
                if mm and mm.group(1) == 'self' and mm.group(2): m.uses.setdefault(mm.group(2), []).append((['crate'], it['vis'], cfgs))
            elif kind == 'mod':
                name = it['name']
                at = ' '.join(it['attrs'])
                pm = PATH_RX.search(at)
                head = it['text'][it['text'].index('mod ' + name) + 4 + len(name):]
                inline = head.lstrip().startswith('{')
                if inline:
                    body = it['text'][it['text'].index('{', it['text'].index('mod ' + name)) + 1: it['text'].rindex('}')]
                    child = Mod(m.key + '#' + name, m.file, m, name, body, m.base_dir if (pm and pm.group(1) == '.') else os.path.join(m.base_dir, name))
                    child.vis = it['vis']; child.cfg = cfgs; child.cw = self
                    self.mods.append(child)
                    self.fill(child, body)
                    m.children[name] = child
                else:
                    cand = None
                    if pm: cand = os.path.normpath(os.path.join(m.base_dir, pm.group(1)))
                    else:
                        for c in (os.path.join(m.base_dir, name + '.rs'), os.path.join(m.base_dir, name, 'mod.rs')):
                            if os.path.exists(c): cand = c; break
                    if cand and os.path.exists(cand):
                        if cand in self.files: m.children[name] = self.files[cand]
                        else:
                            child = self.load_file(cand, m, name, it['vis'], cfgs)
                            m.children[name] = child
            elif kind is None and 'include!' in it['text']:
                code = ''.join(x[1] for x in tokenize(it['text']) if x[0] != 'comment').strip()
                mm = re.match(r'include!\(\s*"([^"]*)"\s*\)', code)
                if mm:
                    p = os.path.normpath(os.path.join(m.base_dir, mm.group(1)))
                    if os.path.exists(p): self.fill(m, open(p, encoding='utf8').read())

_world = None
def world():
    global _world
    if _world is None:
        w = World(K + '/🦀️.rs', 'semio_framework_os_kernel')
        w.root = w.load_file(K + '/🦀️.rs', None, '', '', [], key='<crate>')
        _world = w
    return _world

_ident_roots = {}     # ident -> lib root file (first-party crates the resolver may enter)
_ext_worlds = {}
def register_crate(ident, root_file): _ident_roots[ident] = root_file
def ext_world(ident):
    if ident in _ext_worlds: return _ext_worlds[ident]
    rf = _ident_roots.get(ident)
    if not rf or not os.path.exists(rf):
        _ext_worlds[ident] = None; return None
    w = World(rf, ident)
    w.root = w.load_file(rf, None, '', '', [], key='<crate ' + ident + '>')
    _ext_worlds[ident] = w
    return w

def forget_worlds():
    global _world
    _world = None; _ext_worlds.clear(); _cache.clear()

class Res:
    """Resolution result."""
    def __init__(self, kind, mod=None, name=None, item=None, extern=None, rest=()):
        self.kind = kind; self.mod = mod; self.name = name; self.item = item; self.extern = extern; self.rest = list(rest)
    def __repr__(self):
        if self.kind == 'def': return f'Def({self.name} in {self.mod}, rest={self.rest})'
        if self.kind == 'mod': return f'Mod({self.mod}, rest={self.rest})'
        if self.kind == 'extern': return 'Extern(' + '::'.join(self.extern) + ')'
        return f'{self.kind}({self.extern})'

_cache = {}
_progress = set()
_cuts = [0]
EXTERN_GLOB = 'extern-glob'

def is_public(vis):
    return vis == 'pub'

def lookup(m, name, depth=0, seen=None, via_glob=False, pub_only=False):
    """Name `name` in the namespace of module m -> Res | None. Order: defs, child modules, explicit uses, globs. pub_only: the lookup crosses a crate boundary through a glob, so only `pub` entries are visible."""
    if depth > 40: return None
    k = (m.key, name, pub_only)
    if k in _cache: return _cache[k]
    if k in _progress:
        _cuts[0] += 1
        return None
    _progress.add(k)
    c0 = _cuts[0]
    try:
        r = _lookup(m, name, depth, pub_only)
    finally:
        _progress.discard(k)
    if r is not None or _cuts[0] == c0: _cache[k] = r
    return r

def _lookup(m, name, depth, pub_only=False):
    if name in m.defs and not name.startswith('\0'):
        kind, vis, it = m.defs[name]
        if not pub_only or is_public(vis): return Res('def', mod=m, name=name, item=it)
    if name in m.children and (not pub_only or is_public(getattr(m.children[name], 'vis', 'pub'))): return Res('mod', mod=m.children[name])
    for path, vis, cfgs in m.uses.get(name, ()):
        if pub_only and not is_public(vis): continue
        r = resolve_use_path(m, path, depth + 1)
        if r is not None: return r
    ext = []
    for gpath, vis, cfgs in m.globs:
        if pub_only and not is_public(vis): continue
        r = resolve_use_path(m, gpath, depth + 1)
        if r is not None and r.kind == 'mod' and not r.rest:
            t = lookup(r.mod, name, depth + 1, pub_only=pub_only or r.mod.cw is not m.cw)
            if t is not None:
                if t.kind == EXTERN_GLOB: ext += [e[:-1] for e in t.extern]
                else: return t
        elif r is not None and r.kind == 'extern':
            ext.append(r.extern)
    if ext:
        return Res(EXTERN_GLOB, extern=[e + [name] for e in ext])
    return None

def resolve_use_path(m, path, depth=0):
    return resolve(m, path, depth)

def resolve(m, segs, depth=0):
    """Resolve a path as written in module m. Returns Res (def/mod/extern/unresolved)."""
    if depth > 24 or not segs: return Res('unresolved', extern=list(segs))
    w = m.cw or world()
    s0 = segs[0]; rest = list(segs[1:])
    cur = None
    if s0 == 'crate': cur = Res('mod', mod=w.root)
    elif s0 == 'self': cur = Res('mod', mod=m)
    elif s0 == 'super':
        p = m
        i = 0
        while i < len(segs) and segs[i] == 'super':
            p = p.parent
            if p is None: return Res('unresolved', extern=list(segs))
            i += 1
        cur = Res('mod', mod=p); rest = list(segs[i:])
    else:
        cur = lookup(m, s0, depth + 1)
        if cur is None or cur.kind == EXTERN_GLOB: return Res('extern', extern=list(segs))
    while rest:
        if cur.kind == 'mod':
            nxt = lookup(cur.mod, rest[0], depth + 1)
            if nxt is None:
                return Res('unresolved', mod=cur.mod, extern=list(rest), rest=rest)
            cur = nxt; rest = rest[1:]
        elif cur.kind == 'extern':
            return Res('extern', extern=cur.extern + rest)
        elif cur.kind == EXTERN_GLOB:
            return Res(EXTERN_GLOB, extern=[c + rest for c in cur.extern])
        else:
            return Res(cur.kind, cur.mod, cur.name, cur.item, cur.extern, list(cur.rest) + list(rest))
    return Res(cur.kind, cur.mod, cur.name, cur.item, cur.extern, list(cur.rest))

def mod_of_file(path):
    return world().files.get(path)

def extern_exists(segs, inner=False):
    """True/False when the first segment names a registered first-party crate; None when unknown (std, third party) at the top level. Candidates of extern globs that cannot be followed count as absent."""
    if not segs: return None
    w = ext_world(segs[0])
    if w is None: return False if inner else None
    cur = Res('mod', mod=w.root)
    for i, s in enumerate(segs[1:]):
        if cur.kind != 'mod': return True
        nxt = lookup(cur.mod, s, pub_only=True)
        if nxt is None: return False
        if nxt.kind == EXTERN_GLOB:
            return any(extern_exists(c + list(segs[i + 2:]), True) is True for c in nxt.extern)
        if nxt.kind == 'extern':
            return extern_exists(nxt.extern + list(segs[i + 2:]), inner) 
        cur = nxt
    return True

def disambiguate(res):
    """EXTERN_GLOB -> list of candidate path lists that really export the name (or all when unknown)."""
    if res.kind == 'extern': return [res.extern]
    if res.kind != EXTERN_GLOB: return []
    good = [c for c in res.extern if extern_exists(c) is True]
    if good: return good
    unk = [c for c in res.extern if extern_exists(c) is None]
    return unk
