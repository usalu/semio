# 🔎 Extern-path existence check: every `ext_crate::seg1::seg2...` path in the carved crates must name an item the target crate exposes publicly (name index over the target source, cfg-agnostic).
# The union differential cannot see these paths (extern crates are unresolved in the sandbox), so this is the compensating scan. usage: extcheck.py <crate-key>...
import os, re, sys, json, functools, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import tokenize, items
import okusetree as U
import okcrates as C
R = C.R
DEFAULT_KEYS = {}

@functools.lru_cache(maxsize=None)
def manifest_json(pkgdir):
    import subprocess
    out = subprocess.run(['bun', '-e', 'import {readFileSync} from "node:fs"; console.log(JSON.stringify(Bun.TOML.parse(readFileSync(process.argv[1],"utf8"))))', pkgdir + '/Cargo.toml'], capture_output=True, text=True)
    return json.loads(out.stdout)

def lib_root(pkgdir):
    m = manifest_json(pkgdir)
    return os.path.normpath(os.path.join(pkgdir, m.get('lib', {}).get('path', 'src/lib.rs')))

def lib_name(pkgdir):
    m = manifest_json(pkgdir)
    return m.get('lib', {}).get('name') or m['package']['name'].replace('-', '_')

def deps_of(pkgdir):
    """ident -> package dir, for every dependency table of the manifest."""
    m = manifest_json(pkgdir); out = {}
    tables = [m.get('dependencies', {}), m.get('dev-dependencies', {})] + [t.get('dependencies', {}) for t in m.get('target', {}).values()] + [t.get('dev-dependencies', {}) for t in m.get('target', {}).values()]
    for tb in tables:
        for key, spec in tb.items():
            if isinstance(spec, dict) and 'path' in spec:
                d = os.path.normpath(os.path.join(pkgdir, spec['path']))
                out[(spec.get('package') and key) or None] = None
                try: ident = lib_name(d) if not spec.get('package') or True else key
                except Exception: continue
                out[key.replace('-', '_') if '-' in key else key] = d
                out[ident] = d
            elif isinstance(spec, dict) and spec.get('workspace'):
                ws = workspace_deps().get(key)
                if ws: d = os.path.normpath(os.path.join(R, ws['path'])); out[key.replace('-', '_')] = d; out[lib_name(d)] = d
    out.pop(None, None)
    return out

@functools.lru_cache(maxsize=None)
def workspace_deps():
    import subprocess
    out = subprocess.run(['bun', '-e', 'import {readFileSync} from "node:fs"; console.log(JSON.stringify(Bun.TOML.parse(readFileSync(process.argv[1],"utf8")).workspace.dependencies))', R + '/Cargo.toml'], capture_output=True, text=True)
    return {k: v for k, v in json.loads(out.stdout).items() if isinstance(v, dict) and 'path' in v}

KINDS = ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'mod', 'union', 'macro_rules')
class Index:
    """Public name index of one module (file or inline body)."""
    def __init__(self, text, dirpath, owner):
        self.names = {}      # name -> ('item', kind) | ('mod', Index) | ('use', path-list)
        self.globs = []      # glob target paths
        self.owner = owner; self.dir = dirpath; self.deps = None; self.priv = {}; self.multi = collections.defaultdict(list)
        self.text = text
        for it in items(text):
            vis = it['vis']
            if it['kind'] == 'fn':
                at = ' '.join(it['attrs'])
                for m in re.finditer(r'proc_macro_derive\(\s*([A-Za-z_0-9]+)', at): self.names[m.group(1)] = ('macro', None)
                if re.search(r'#\[\s*proc_macro(_attribute)?\s*\]', at): self.names[it['name']] = ('macro', None)
            if it['kind'] == 'macro_rules':
                if any('macro_export' in a for a in it['attrs']): self.names[it['name']] = ('macro', None)
                continue
            if vis != 'pub':
                if it['kind'] == 'mod': self.priv[it['name']] = ('mod', self.module(it, dirpath))
                continue
            if it['kind'] == 'use':
                for path, alias, glob in U.parse(text_of_use(it['text'])):
                    if glob: self.globs.append(path)
                    else:
                        nm = U.local_name((path, alias, glob))
                        self.multi[nm].append(('use', path))
                        self.names.setdefault(nm, ('use', path))
            elif it['kind'] == 'mod':
                self.names[it['name']] = ('mod', self.module(it, dirpath))
            elif it['kind'] in KINDS: self.names[it['name']] = ('item', it['kind'])
    def module(self, it, dirpath):
        t = it['text']
        pm = re.search(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', ''.join(it['attrs']))
        if '{' in t.split('mod ' + it['name'], 1)[1][:30] and not re.match(r'\s*;', t.split('mod ' + it['name'], 1)[1]):
            body = t[t.index('{', t.index('mod ' + it['name'])) + 1: t.rindex('}')]
            sub = dirpath if not pm else os.path.normpath(os.path.join(dirpath, pm.group(1)))
            return Index(body, os.path.join(sub, it['name']) if not pm else sub, self.owner)
        if pm:
            f = os.path.normpath(os.path.join(dirpath, pm.group(1)))
        else:
            cand = [os.path.join(dirpath, it['name'] + '.rs'), os.path.join(dirpath, it['name'], 'mod.rs')]
            f = next((c for c in cand if os.path.exists(c)), None)
        if f and os.path.exists(f): return Index(open(f, encoding='utf8').read(), os.path.dirname(f) if pm else os.path.join(dirpath, it['name']), self.owner)
        return None

def text_of_use(t):
    code = ''.join(x[1] for x in tokenize(t) if x[0] != 'comment')
    m = re.search(r'\buse\b(.*);', code, re.S)
    return m.group(1) if m else ''

_root_cache = {}
def crate_index(pkgdir):
    if pkgdir not in _root_cache:
        rf = lib_root(pkgdir)
        _root_cache[pkgdir] = (Index(open(rf, encoding='utf8').read(), os.path.dirname(rf), pkgdir), deps_of(pkgdir))
        _root_cache[pkgdir][0].deps = _root_cache[pkgdir][1]
        ROOT_OF[pkgdir] = _root_cache[pkgdir][0]
    return _root_cache[pkgdir]

def has_name(idx, name, deps, depth=0):
    """True if `name` is publicly nameable in idx (own items, explicit pub use, glob re-exports)."""
    if depth > 6: return None
    if name in idx.names: return True
    unknown = False
    for g in idx.globs:
        r = glob_has(idx, g, name, deps, depth + 1)
        if r: return True
        if r is None: unknown = True
    return None if unknown else False

def resolve_path(idx, path, deps, depth=0):
    """Resolves a use path (list of segments) to ('mod', Index) | ('item',...) | None(unknown)."""
    if depth > 6 or not path: return None
    head = path[0]
    if head in ('crate', 'self'):
        rest = path[1:]; cur = idx if head == 'self' else ROOT_OF.get(idx.owner, idx)
    elif head in deps:
        cur = crate_index(deps[head])[0]; rest = path[1:]
    elif head in idx.names or head in idx.priv:
        cur = idx; rest = path
    else: return None
    for i, seg in enumerate(rest):
        e = cur.names.get(seg) or (cur.priv.get(seg) if i == 0 else None)
        if e is None:
            hit = None
            for g in cur.globs:
                gi = resolve_path(cur, g, depsof(cur, deps), depth + 1)
                if gi and gi[0] == 'mod' and gi[1] and seg in gi[1].names: hit = gi[1].names[seg]; break
            if hit is None: return None
            e = hit
        if e[0] == 'mod':
            if e[1] is None: return None
            cur = e[1]
            if i == len(rest) - 1: return ('mod', cur)
        elif e[0] == 'use':
            r = resolve_path(cur, e[1], depsof(cur, deps), depth + 1)
            if r is None: return None
            if r[0] == 'mod' and i < len(rest) - 1: cur = r[1]
            elif i == len(rest) - 1: return r
            else: return None
        else:
            return e if i == len(rest) - 1 else None
    return ('mod', cur)

def glob_has(idx, gpath, name, deps, depth):
    r = resolve_path(idx, gpath, depsof(idx, deps), depth)
    if r is None: return None
    if r[0] != 'mod' or r[1] is None: return False
    return has_name(r[1], name, getattr(r[1], 'deps', deps), depth + 1)

ROOT_OF = {}
def depsof(idx, fallback):
    if idx.deps: return idx.deps
    r = ROOT_OF.get(idx.owner)
    return (r.deps if r is not None and r.deps else None) or fallback

def check_path(ext_ident, segs, extdeps):
    """segs after the extern ident. Returns (ok: True/False/None, detail)."""
    pkg = extdeps[ext_ident]
    idx, d = crate_index(pkg)
    ROOT_OF[pkg] = idx
    cur = idx; trail = [ext_ident]
    for i, seg in enumerate(segs):
        ok = has_name(cur, seg, depsof(cur, d))
        if ok is False: return False, '::'.join(trail + [seg])
        if ok is None: return None, '::'.join(trail + [seg])
        e = cur.names.get(seg)
        if e is None:
            # via glob: find which module provides it to descend
            nxt = None
            for g in cur.globs:
                gi = resolve_path(cur, g, depsof(cur, d))
                if gi and gi[0] == 'mod' and gi[1] and has_name(gi[1], seg, getattr(gi[1], 'deps', d)):
                    nxt = gi[1].names.get(seg); cur_owner = gi[1]; break
            e = nxt
            if e is None:
                if i == len(segs) - 1: return True, ''
                return None, '::'.join(trail + [seg]) + ' (provided by a glob)'
        trail.append(seg)
        if e[0] == 'mod' and e[1] is not None: cur = e[1]
        elif e[0] == 'use':
            r = resolve_path(cur, e[1], depsof(cur, d))
            if r and r[0] == 'mod' and r[1] is not None: cur = r[1]
            elif r and r[0] == 'item': return True, ''
            elif i < len(segs) - 1: return None, '::'.join(trail) + ' (re-export, not followed)'
        elif i < len(segs) - 1:
            return (True, '') if e[0] in ('item', 'macro') else (None, '::'.join(trail) + ' (not a module)')
    return True, ''

def scan_file(path, extdeps):
    src = open(path, encoding='utf8').read()
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    res = collections.Counter(); bad = []
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] in extdeps and i + 2 < len(toks) and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            if i and toks[i - 1][1] == ':' and i >= 2 and toks[i - 2][1] == ':' : continue
            if i and toks[i - 1][0] == 'id' and toks[i - 1][1] not in ('use', 'pub', 'as', 'in', 'crate'): continue
            j = i + 3; segs = []
            while j < len(toks) and toks[j][0] == 'id':
                segs.append(toks[j][1]); j += 1
                if j + 1 < len(toks) and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                else: break
            if j < len(toks) and toks[j][1] == '{' and toks[j - 1][1] == ':':
                # use tree: expand leaves
                d = 0; k = j; buf = []
                while k < len(toks):
                    buf.append(toks[k][1])
                    if toks[k][1] == '{': d += 1
                    if toks[k][1] == '}':
                        d -= 1
                        if d == 0: break
                    k += 1
                for leaf, alias, glob in U.parse(''.join(buf)): 
                    full = segs + [p for p in leaf if p != 'self']
                    if glob: continue
                    ok, detail = check_path(t[1], full[:3], extdeps)
                    res[ok] += 1
                    if ok is False: bad.append((path, t[1] + '::' + '::'.join(full)))
                continue
            if not segs: continue
            ok, detail = check_path(t[1], segs[:3], extdeps)
            res[ok] += 1
            if ok is False: bad.append((path, t[1] + '::' + '::'.join(segs), detail))
    return res, bad
