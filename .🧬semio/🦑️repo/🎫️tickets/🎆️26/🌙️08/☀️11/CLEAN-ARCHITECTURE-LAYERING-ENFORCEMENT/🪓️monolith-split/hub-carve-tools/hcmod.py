#!/usr/bin/env python3
"""🧱️ Rust module-tree model for the hub carve: items with source spans, use leaves, inline modules, #[path] mounts and a scope-based name resolver."""
import os, re, sys, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex

KEYWORDS = {'as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct', 'super', 'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while', 'async', 'await', 'dyn'}
NAMED_MACROS = {'capability_type'}
ENUM_MACROS = {'app_commands': 'enum', 'app_labels': 'struct'}
ITEM_KW = ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union')


class Tok:
    __slots__ = ('k', 'x', 'sl', 'el', 'a', 'b')

    def __init__(self, t):
        self.k, self.x, self.sl, self.el, self.a = t
        self.b = self.a + len(self.x)


class Item:
    def __init__(self, kind, name, vis, attrs, cfg, a, b, toks, hdr):
        self.kind, self.name, self.vis, self.attrs, self.cfg = kind, name, vis, attrs, cfg
        self.a, self.b, self.toks, self.hdr = a, b, toks, hdr

    def __repr__(self):
        return f"Item({self.kind} {self.name} {self.vis!r} [{self.a}:{self.b}])"


class Use:
    def __init__(self, path, binding, glob, vis, cfg, item):
        self.path, self.binding, self.glob, self.vis, self.cfg, self.item = path, binding, glob, vis, cfg, item

    def __repr__(self):
        return f"Use({'::'.join(self.path)}{'::*' if self.glob else ''} as {self.binding} {self.vis!r})"


class Mod:
    def __init__(self, name, file, parent, src, toks, inline_of=None):
        self.name, self.file, self.parent, self.src, self.toks = name, file, parent, src, toks
        self.items = collections.defaultdict(list)
        self.all_items = []
        self.uses = []
        self.children = {}
        self.cfg = []
        self.vis = ''
        self.inline = inline_of is not None
        self.file_mod = None

    @property
    def path(self):
        names = []
        m = self
        while m is not None:
            names.append(m.name)
            m = m.parent
        return '::'.join(reversed(names))

    def file_root(self):
        m = self
        while m.inline:
            m = m.parent
        return m

    def __repr__(self):
        return f"Mod({self.path})"


def cfg_of(attrs):
    out = []
    for a in attrs:
        m = re.match(r'#\[\s*cfg\((.*)\)\s*\]$', a.replace('\n', ' '))
        if m:
            out.append(m.group(1).strip())
    return out


def split_items(toks, lo, hi):
    """Split the token slice [lo,hi) into item token ranges at depth 0 ('}' not followed by ';' or ';')."""
    out = []
    depth = 0
    start = lo
    i = lo
    while i < hi:
        t = toks[i]
        if t.k == 'p':
            if t.x in '{([':
                depth += 1
            elif t.x in '})]':
                depth -= 1
                if depth == 0 and t.x == '}':
                    nxt = toks[i + 1].x if i + 1 < hi else ''
                    nk = toks[i + 1].k if i + 1 < hi else ''
                    if not (nxt == ';' and nk == 'p'):
                        out.append((start, i + 1))
                        start = i + 1
            elif t.x == ';' and depth == 0:
                out.append((start, i + 1))
                start = i + 1
        i += 1
    if start < hi and any(toks[j].k != 'comment' for j in range(start, hi)):
        out.append((start, hi))
    return out


def classify(toks, lo, hi):
    """Return (attrs, hdr_index, vis, kind, name) for the item in toks[lo:hi]."""
    j = lo
    attrs = []
    while j < hi:
        t = toks[j]
        if t.k == 'comment':
            j += 1
            continue
        if t.x == '#' and t.k == 'p':
            s = j
            j += 1
            if toks[j].x == '!':
                j += 1
            d = 0
            while j < hi:
                if toks[j].x == '[' and toks[j].k == 'p':
                    d += 1
                elif toks[j].x == ']' and toks[j].k == 'p':
                    d -= 1
                    if d == 0:
                        break
                j += 1
            attrs.append(''.join(x.x for x in toks[s:j + 1]))
            j += 1
            continue
        break
    hdr = j
    if hdr >= hi:
        return attrs, hdr, '', 'none', ''
    i = hdr
    vis = ''
    if toks[i].x == 'pub' and toks[i].k == 'id':
        vis = 'pub'
        i += 1
        if i < hi and toks[i].x == '(' and toks[i].k == 'p':
            k = i
            while toks[k].x != ')':
                k += 1
            vis = 'pub' + ''.join(x.x for x in toks[i:k + 1])
            i = k + 1
    while i < hi and (toks[i].x in ('async', 'unsafe', 'extern', 'default') and toks[i].k == 'id' or toks[i].k == 'str' or (toks[i].x == 'const' and i + 1 < hi and toks[i + 1].x in ('fn', 'unsafe', 'async', 'extern'))):
        if toks[i].x == 'extern' and i + 1 < hi and toks[i + 1].x == 'crate':
            break
        i += 1
    kw = toks[i].x if i < hi else ''
    if kw in ITEM_KW:
        nm = toks[i + 1].x
        if nm == 'mut':
            nm = toks[i + 2].x
        return attrs, hdr, vis, kw, nm
    if kw == 'use':
        return attrs, hdr, vis, 'use', ''
    if kw == 'impl':
        return attrs, hdr, vis, 'impl', ''
    if kw == 'macro_rules':
        return attrs, hdr, vis, 'macro', toks[i + 2].x
    if kw == 'extern' and i + 1 < hi and toks[i + 1].x == 'crate':
        k = i + 2
        name = toks[k].x
        binding = name
        if k + 1 < hi and toks[k + 1].x == 'as':
            binding = toks[k + 2].x
        return attrs, hdr, vis, 'extern_crate', binding
    if kw == 'include':
        return attrs, hdr, vis, 'include', ''
    if kw == 'thread_local' and i + 1 < hi and toks[i + 1].x == '!':
        return attrs, hdr, vis, 'thread_local', ''
    if kw == 'std' and i + 4 < hi and toks[i + 1].x == ':' and toks[i + 2].x == ':' and toks[i + 3].x == 'thread_local' and toks[i + 4].x == '!':
        return attrs, hdr, vis, 'thread_local', ''
    if kw in ENUM_MACROS and i + 3 < hi and toks[i + 1].x == '!' and toks[i + 2].x in '({[':
        j = i + 3
        want = ENUM_MACROS[kw]
        while j + 1 < hi and not (toks[j].x == want and toks[j].k == 'id'):
            j += 1
        if j + 1 < hi:
            return attrs, hdr, 'pub', want, toks[j + 1].x
    if kw in NAMED_MACROS and i + 3 < hi and toks[i + 1].x == '!' and toks[i + 2].x in '({[':
        return attrs, hdr, 'pub', 'struct', toks[i + 3].x
    return attrs, hdr, vis, kw, ''


def parse_use(toks, i, hi):
    """toks[i] is 'use'; returns list of (path segs, binding, glob) and the next index."""
    leaves = []

    def tree(i, prefix):
        segs = list(prefix)
        while i < hi:
            t = toks[i]
            if t.x == '{' and t.k == 'p':
                i += 1
                while i < hi and toks[i].x != '}':
                    i = tree(i, segs)
                    if i < hi and toks[i].x == ',':
                        i += 1
                return i + 1
            if t.x == '*' and t.k == 'p':
                leaves.append((segs, None, True))
                return i + 1
            if t.x == ':' and t.k == 'p':
                i += 1
                continue
            if t.k == 'id':
                segs.append(t.x)
                i += 1
                if i + 1 < hi and toks[i].x == ':' and toks[i + 1].x == ':':
                    i += 2
                    continue
                binding = segs[-1]
                if i < hi and toks[i].x == 'as' and toks[i].k == 'id':
                    binding = toks[i + 1].x
                    i += 2
                if segs[-1] == 'self' and len(segs) > 1:
                    segs = segs[:-1]
                    if binding == 'self':
                        binding = segs[-1]
                leaves.append((list(segs), binding, False))
                return i
            i += 1
        return i

    j = tree(i + 1, [])
    return leaves, j


class DiskFS:
    def read(self, path):
        return Path(path).read_text()

    def exists(self, path):
        return Path(path).exists()


FS = DiskFS()


_LEX_CACHE = {}


def lex_cached(src):
    h = hash(src)
    hit = _LEX_CACHE.get(h)
    if hit is not None and hit[0] == src:
        return hit[1]
    toks = [Tok(t) for t in lex(src)]
    _LEX_CACHE[h] = (src, toks)
    return toks


def load_file(path, parent, name, modmap, mounted_dir=None):
    src = FS.read(path)
    toks = lex_cached(src)
    m = Mod(name, str(path), parent, src, toks)
    modmap[str(path)] = m
    fill(m, toks, 0, len(toks), Path(path).parent, modmap)
    return m


def fill(m, toks, lo, hi, basedir, modmap):
    for a, b in split_items(toks, lo, hi):
        attrs, hdr, vis, kind, name = classify(toks, a, b)
        if kind == 'none':
            continue
        sa = toks[a].a
        sb = toks[b - 1].b
        it = Item(kind, name, vis, attrs, cfg_of(attrs), sa, sb, toks[hdr:b], toks[a].a)
        it.tok_range = (a, b)
        it.hdr_index = hdr
        it.toks_all = toks[a:b]
        m.all_items.append(it)
        if kind == 'use':
            leaves, _ = parse_use(toks, find_kw(toks, hdr, b, 'use'), b)
            for segs, binding, glob in leaves:
                m.uses.append(Use(segs, binding, glob, vis, it.cfg, it))
        elif kind in ('mod',):
            m.items[name].append(it)
            pm = None
            for at in attrs:
                mm = re.match(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]$', at.replace('\n', ' '))
                if mm:
                    pm = mm.group(1)
            # detect inline body
            body = None
            k = find_kw(toks, hdr, b, 'mod') + 2
            if k < b and toks[k].x == '{':
                body = (k + 1, b - 1)
            if body is not None:
                child = Mod(name, m.file, m, m.src, toks, inline_of=m)
                child.cfg = it.cfg
                child.vis = vis
                child.item = it
                sub = basedir if pm is None else (basedir / pm)
                if pm is None:
                    sub = basedir / name
                fill(child, toks, body[0], body[1], sub, modmap)
                key = name if name not in m.children else f'{name}#{len(m.children)}'
                m.children[key] = child
            else:
                if pm is None:
                    cand1 = basedir / (name + '.rs')
                    cand2 = basedir / name / 'mod.rs'
                    target = cand1 if FS.exists(cand1) else cand2
                else:
                    target = Path(os.path.normpath(basedir / pm))
                if not FS.exists(target):
                    child = Mod(name, str(target), m, '', [])
                    child.missing = True
                else:
                    child = load_file(Path(os.path.normpath(target)), m, name, modmap)
                child.cfg = it.cfg
                child.vis = vis
                child.item = it
                key = name if name not in m.children else f'{name}#{len(m.children)}'
                m.children[key] = child
        elif kind == 'include':
            try:
                k = find_kw(toks, hdr, b, 'include')
                lit = next(t.x for t in toks[k:b] if t.k == 'str')
                target = Path(os.path.normpath(basedir / lit.strip('"')))
                if FS.exists(target):
                    inc_src = FS.read(target)
                    inc_toks = lex_cached(inc_src)
                    fill(m, inc_toks, 0, len(inc_toks), basedir, modmap)
            except (StopIteration, ValueError):
                pass
        elif kind == 'thread_local':
            for sit in thread_local_statics(it, toks, hdr, b):
                m.items[sit.name].append(sit)
                m.all_items.append(sit)
        elif kind in ('extern_crate',):
            m.items[name].append(it)
        elif kind in ('impl', 'include', 'none'):
            m.all_items.append(it) if False else None
        elif name:
            m.items[name].append(it)


def thread_local_statics(it, toks, hdr, hi):
    out = []
    k = hdr
    while k < hi and toks[k].x != '{' and toks[k].x != '(':
        k += 1
    depth = 0
    i = k
    while i < hi:
        t = toks[i]
        if t.k == 'p' and t.x in '({[':
            depth += 1
        elif t.k == 'p' and t.x in ')}]':
            depth -= 1
        elif depth == 1 and t.x == 'static' and t.k == 'id':
            j = i + 1
            if toks[j].x == 'mut':
                j += 1
            name = toks[j].x
            s = i
            vis = ''
            if i >= 1 and toks[i - 1].x == 'pub' and toks[i - 1].k == 'id':
                s, vis = i - 1, 'pub'
            elif i >= 1 and toks[i - 1].x == ')':
                m2 = i - 1
                while m2 > 0 and toks[m2].x != '(':
                    m2 -= 1
                if m2 >= 1 and toks[m2 - 1].x == 'pub':
                    s, vis = m2 - 1, 'pub' + ''.join(x.x for x in toks[m2:i])
            sit = Item('static', name, vis, it.attrs, it.cfg, it.a, it.b, toks[s:hi], it.hdr)
            sit.tok_range = it.tok_range
            sit.hdr_index = s
            sit.toks_all = it.toks_all
            out.append(sit)
        i += 1
    return out


def find_kw(toks, lo, hi, kw):
    for i in range(lo, hi):
        if toks[i].x == kw and toks[i].k == 'id':
            return i
    raise ValueError(kw)


def load_crate(root, name='crate'):
    modmap = {}
    root = Path(os.path.normpath(root))
    m = load_file(root, None, name, modmap)
    return m, modmap


def walk(m):
    yield m
    for c in m.children.values():
        yield from walk(c)


# ---------------------------------------------------------------- resolver
class Res:
    def __init__(self, kind, mod=None, name=None, item=None, ext=None):
        self.kind, self.mod, self.name, self.item, self.ext = kind, mod, name, item, ext
        self.via = None
        self.via_mod = None
        self.rest = []

    def __repr__(self):
        if self.kind == 'external':
            return f"Res(external {self.ext})"
        return f"Res({self.kind} {self.mod.path if self.mod else None}::{self.name})"


EXTERNALS = set()
CRATES = {}


def crate_root(m):
    while m.parent is not None:
        m = m.parent
    return m


def within(asker, t):
    x = asker
    while x is not None:
        if x is t:
            return True
        x = x.parent
    return False


def visible(vis, t, asker):
    """Whether a binding with visibility `vis` declared in module `t` is visible from module `asker`."""
    if vis == 'pub' or vis.startswith('pub(crate') or vis.startswith('pub(in'):
        return True
    if vis == 'pub(super)':
        return within(asker, t.parent if t.parent is not None else t)
    return within(asker, t)


def lookup(m, name, seen=None, want_pub=False):
    r = lookup_vis(m, name, seen)
    return r[0] if r else None


def lookup_vis(m, name, seen=None):
    """Name lookup inside module m scope: own items/mods, named uses, globs. Returns (Res, binding visibility) or None."""
    if seen is None:
        seen = set()
    key = (id(m), name)
    if key in seen:
        return None
    seen = seen | {key}
    if name in m.items:
        it = m.items[name][0]
        if it.kind == 'mod':
            return Res('mod', m.children.get(name), name, it), it.vis
        if it.kind == 'extern_crate':
            return Res('external', ext=name), it.vis
        return Res('item', m, name, it), it.vis
    for u in m.uses:
        if not u.glob and u.binding == name:
            r = resolve_use_path(m, u.path, seen)
            if r is not None:
                if getattr(r, 'via', None) is None:
                    r = Res(r.kind, r.mod, r.name, r.item, r.ext)
                    r.via = u
                    r.via_mod = m
                return r, u.vis
    for u in m.uses:
        if u.glob:
            r = resolve_use_path(m, u.path, seen)
            if r is not None and r.kind == 'mod' and r.mod is not None:
                tgt = r.mod
                got = lookup_vis(tgt, name, seen)
                if got is not None:
                    rr, vis = got
                    if not visible(vis, tgt, m):
                        continue
                    if getattr(rr, 'via', None) is None:
                        rr = Res(rr.kind, rr.mod, rr.name, rr.item, rr.ext)
                        rr.via = u
                        rr.via_mod = m
                    return rr, u.vis
    return None


def resolve_use_path(m, segs, seen=None):
    return resolve(m, segs, seen, use_mode=True)


def resolve(m, segs, seen=None, use_mode=False, exact=False):
    """Resolve a path (list of segments) relative to module m. Returns Res (item/mod/external/variant) or None."""
    if not segs:
        return None
    first = segs[0]
    cur = None
    i = 1
    if first == 'crate':
        cur = Res('mod', crate_root(m), 'crate')
    elif first == 'self':
        cur = Res('mod', m, m.name)
    elif first == 'super':
        cm = m
        i = 0
        while i < len(segs) and segs[i] == 'super':
            if cm.parent is None:
                return None
            cm = cm.parent
            i += 1
        cur = Res('mod', cm, cm.name)
    elif first == '':
        return Res('external', ext='::'.join(segs))
    else:
        r = lookup(m, first, seen)
        if r is None and first in CRATES:
            r = Res('mod', CRATES[first], first)
        if r is None:
            if use_mode:
                return Res('external', ext='::'.join(segs))
            return None
        cur = r
    while i < len(segs):
        if cur.kind == 'external':
            c2 = Res('external', ext=cur.ext)
            c2.via, c2.via_mod, c2.rest = cur.via, cur.via_mod, list(segs[i:])
            return c2
        if cur.kind == 'mod' and cur.mod is not None:
            r = lookup(cur.mod, segs[i], seen)
            if r is None:
                return None
            cur = r
            i += 1
            continue
        if cur.kind == 'item' and cur.item.kind == 'enum':
            return Res('variant', cur.mod, segs[i], cur.item)
        return None if exact else cur
    return cur


def lookup_prelude(m, name, seen):
    return None


def modpath_chain(m):
    c = []
    while m is not None:
        c.append(m)
        m = m.parent
    return list(reversed(c))
