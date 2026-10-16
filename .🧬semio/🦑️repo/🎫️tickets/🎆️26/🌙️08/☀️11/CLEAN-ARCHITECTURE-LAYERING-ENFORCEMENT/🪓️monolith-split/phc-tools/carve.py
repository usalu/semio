#!/usr/bin/env python3
"""🪚️ Carve engine for the hub: in-memory item moves, crate models and the global path rewrite (use leaves, qualified paths, glob expansion, free-name imports)."""
import os, re, sys, collections, json
from pathlib import Path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, cfgeval
from hcmod import Mod, Item, Use, Res, Tok
from rsitems import lex

REPO = '/Users/ueli/Documents/semio'

TRAIT_HINTS = {'Write', 'Read', 'BufRead', 'Seek', 'Hasher', 'Hash', 'FromStr', 'TryFrom', 'TryInto', 'Deref', 'DerefMut', 'Borrow', 'AsyncReadExt', 'AsyncWriteExt', 'AsyncBufReadExt', 'StreamExt', 'SinkExt', 'TryStreamExt', 'FutureExt', 'TryFutureExt', 'IntoResponse', 'FromRequest', 'FromRequestParts', 'ToValue', 'FromValue', 'Serialize', 'Deserialize', 'Digest', 'Engine', 'Display', 'Error', 'Add', 'Sub', 'Mul', 'Div', 'Index', 'IntoIterator', 'Iterator', 'FromIterator', 'Extend', 'Future', 'Stream', 'Sink', 'Wake', 'ToString', 'ToOwned', 'AsRef', 'AsMut', 'Into', 'From', 'Default', 'Clone', 'PartialEq', 'Eq', 'PartialOrd', 'Ord', 'Send', 'Sync', 'Sized', 'Unpin', 'Debug', 'Drop', 'Fn', 'FnMut', 'FnOnce'}


class Def:
    __slots__ = ('file', 'chain', 'name')

    def __init__(self, file, chain, name):
        self.file, self.chain, self.name = file, tuple(chain), name

    def key(self):
        return (self.file, self.chain, self.name)

    def __eq__(self, o):
        return isinstance(o, Def) and self.key() == o.key()

    def __hash__(self):
        return hash(self.key())

    def __repr__(self):
        return f"Def({os.path.relpath(self.file, REPO)}::{'::'.join(self.chain + (self.name,))})"


def chain_of(m):
    c = []
    while m.inline:
        c.append(m.name)
        m = m.parent
    return tuple(reversed(c))


def def_of(res):
    if res is None:
        return None
    if res.kind == 'external':
        return None
    if res.kind == 'mod':
        m = res.mod
        if m is None:
            return None
        if m.inline:
            return Def(m.file_root().file, chain_of(m.parent) if m.parent.inline else (), m.name) if False else Def(m.file_root().file, chain_of(m)[:-1], m.name)
        return Def(m.file, (), '')
    if res.kind == 'item':
        m = res.mod
        return Def(m.file_root().file, chain_of(m), res.name)
    if res.kind == 'variant':
        m = res.mod
        return Def(m.file_root().file, chain_of(m), res.item.name)
    return None


class Model:
    def __init__(self, ws, roots, allow_dup=False):
        """roots: ordered dict ident -> abs root path. ident 'crate' denotes the single legacy crate."""
        self.ws = ws
        hcmod.FS = ws
        hcmod.CRATES.clear()
        self.roots = {}
        self.by_file = {}
        self.crate_of_file = {}
        for ident, path in roots.items():
            root, modmap = hcmod.load_crate(path, 'crate' if ident == 'crate' else ident)
            self.roots[ident] = root
            for fm in modmap.values():
                if fm.file in self.by_file:
                    if allow_dup:
                        continue
                    raise RuntimeError(f'file mounted twice: {fm.file} ({self.crate_of_file[fm.file]} and {ident})')
                self.by_file[fm.file] = fm
                self.crate_of_file[fm.file] = ident
            hcmod.CRATES[ident] = root
        # missing mounts
        self.missing = [m for r in self.roots.values() for m in hcmod.walk(r) if getattr(m, 'missing', False)]

    def crate_ident_of(self, m):
        return self.crate_of_file[m.file_root().file]

    def modpath(self, m):
        """List of module names from the crate root down to m (inline included)."""
        names = []
        while m.parent is not None:
            names.append(m.name)
            m = m.parent
        return list(reversed(names))

    def public_mod_chain(self, m):
        while m.parent is not None:
            if m.vis != 'pub':
                return False
            m = m.parent
        return True


def read_tokens(src):
    return hcmod.lex_cached(src)


FMT_ARG = re.compile(r'\{([A-Z][A-Z0-9_]*)(?::[^{}]*)?\}')


def fmt_arg_idents(toks):
    """(token index, name) of every all-caps constant named by an inline format argument `{NAME}` of a string literal."""
    out = []
    for i, t in enumerate(toks):
        if t.k == 'str' and '{' in t.x:
            for m in FMT_ARG.finditer(t.x):
                out.append((i, m.group(1)))
    return out


def free_idents(toks, lo=0, hi=None, skip_ranges=()):
    """Identifiers that start a path (not after '.' and not continuing a '::' path), with the full path segs."""
    hi = len(toks) if hi is None else hi
    out = []
    i = lo
    sk = sorted(skip_ranges)
    while i < hi:
        t = toks[i]
        if t.k != 'id' or (t.x in hcmod.KEYWORDS and t.x not in ('crate', 'self', 'super')):
            i += 1
            continue
        if any(a <= i < b for a, b in sk):
            i += 1
            continue
        prev = toks[i - 1].x if i > 0 else ''
        if prev == '.' and toks[i - 1].k == 'p' and not (i > 1 and toks[i - 2].x == '.' and toks[i - 2].k == 'p'):
            i += 1
            continue
        if prev == ':' and i > 1 and toks[i - 2].x == ':' and toks[i - 2].k == 'p':
            i += 1
            continue
        segs = [t.x]
        j = i + 1
        while j + 2 < hi + 1 and j + 1 < hi and toks[j].x == ':' and toks[j + 1].x == ':' and j + 2 < hi and toks[j + 2].k == 'id':
            segs.append(toks[j + 2].x)
            j += 3
        is_vis = False
        if t.x in ('super', 'crate', 'self') and i >= 2 and toks[i - 1].x == '(' and toks[i - 2].x == 'pub':
            is_vis = True
        if i >= 1 and toks[i - 1].x == 'in' and toks[i - 1].k == 'id' and i >= 3 and toks[i - 2].x == '(' and toks[i - 3].x == 'pub':
            is_vis = True
        if is_vis:
            i = j if j > i + 1 else i + 1
            continue
        out.append((i, j, segs))
        i = j if j > i + 1 else i + 1
    return out


def item_name_set(m):
    return set(m.items.keys())


def scan_regions(src):
    """Return list of (line_index, 'start'|'end', name)."""
    out = []
    for i, line in enumerate(src.split('\n')):
        s = line.strip()
        if s.startswith('//#region'):
            out.append((i, 'start', s[len('//#region'):].strip()))
        elif s.startswith('//#endregion'):
            out.append((i, 'end', s[len('//#endregion'):].strip()))
    return out


# ------------------------------------------------------------------ text surgery
def line_start(src, off):
    return src.rfind('\n', 0, off) + 1


def attached_start(src, item):
    """Start offset of the item including directly attached doc/line comments and attributes (not region markers, not blank-separated)."""
    toks = item.toks_all
    first = None
    for t in toks:
        if t.k != 'comment':
            first = t
            break
    p = first.a if first else item.a
    ls = line_start(src, p)
    # walk upward over comment-only lines
    cur = ls
    while cur > 0:
        prev_end = cur - 1
        prev_ls = line_start(src, prev_end)
        line = src[prev_ls:prev_end].strip()
        if line.startswith('//#region') or line.startswith('//#endregion') or line == '':
            break
        if line.startswith('//') or line.startswith('/*') or line.endswith('*/'):
            cur = prev_ls
            continue
        break
    return cur


def item_full_span(src, item):
    a = attached_start(src, item)
    b = item.b
    # swallow trailing newline
    if b < len(src) and src[b] == '\n':
        b += 1
    return a, b


def impl_self_ident(item):
    toks = [t for t in item.toks if t.k != 'comment']
    i = 0
    while i < len(toks) and toks[i].x != 'impl':
        i += 1
    i += 1
    if i < len(toks) and toks[i].x == '<':
        d = 0
        while i < len(toks):
            if toks[i].x == '<':
                d += 1
            elif toks[i].x == '>':
                d -= 1
                if d == 0:
                    i += 1
                    break
            i += 1
    seg = []
    j = i
    d = 0
    for_idx = None
    while j < len(toks):
        x = toks[j].x
        if x == '<':
            d += 1
        elif x == '>':
            d -= 1
        elif d == 0 and (x == '{' or (x == 'where' and toks[j].k == 'id')):
            break
        elif d == 0 and x == 'for' and toks[j].k == 'id':
            for_idx = j
        j += 1
    rng = toks[(for_idx + 1 if for_idx is not None else i):j]
    for t in rng:
        if t.k == 'id' and t.x not in ('dyn', 'mut', 'const', 'unsafe'):
            return t.x
    return None


def top_items(fm, kinds=None):
    """Top-level items of a file module (not of inline modules), as Item list with toks_all attached."""
    res = []
    for it in fm.all_items:
        res.append(it)
    return res


def parse_items(src):
    """Top-level items of one source text (no mount following)."""
    toks = read_tokens(src)
    out = []
    for a, b in hcmod.split_items(toks, 0, len(toks)):
        attrs, hdr, vis, kind, name = hcmod.classify(toks, a, b)
        if kind == 'none':
            continue
        it = Item(kind, name, vis, attrs, hcmod.cfg_of(attrs), toks[a].a, toks[b - 1].b, toks[hdr:b], toks[a].a)
        it.tok_range = (a, b)
        it.hdr_index = hdr
        it.toks_all = toks[a:b]
        out.append(it)
    return out, toks


def region_stack_at(src, offset):
    line = src.count('\n', 0, offset)
    stack = []
    for i, kind, name in scan_regions(src):
        if i >= line:
            break
        if kind == 'start':
            stack.append(name)
        elif stack:
            stack.pop()
    return stack


PATH_LITERAL = re.compile(r'((?:include_str|include_bytes|include)!\(\s*|#\[\s*path\s*=\s*)"([^"]+)"')


def rebase_paths(text, src_file, dst_file):
    """File-relative literals (`include_str!`, `include_bytes!`, `include!`, `#[path]`) of moved text follow the file that now holds them."""
    sd, dd = os.path.dirname(src_file), os.path.dirname(dst_file)
    if sd == dd:
        return text

    def fix(m):
        lit = m.group(2)
        if lit == '.' or lit.startswith('/') or '$' in lit:
            return m.group(0)
        new = os.path.relpath(os.path.normpath(os.path.join(sd, lit)), dd)
        return f'{m.group(1)}"{new}"'
    return PATH_LITERAL.sub(fix, text)


class Edits:
    """Collects item moves between files of a workspace."""

    def __init__(self, ws):
        self.ws = ws
        self.moved = {}
        self.origins = collections.defaultdict(set)
        self.proxy = {}
        self.inline_map = {}

    def find(self, src_file, names, with_impls=True, kinds=None, impl_idents=()):
        src = self.ws.read(src_file)
        its, toks = parse_items(src)
        sel = []
        wanted = set(names)
        wanted_hit = set()
        for it in its:
            if it.kind == 'impl' and impl_self_ident(it) in set(impl_idents):
                sel.append(it)
        for it in its:
            if it.kind in ('use', 'impl'):
                continue
            if kinds and it.kind not in kinds:
                continue
            if it.kind == 'thread_local':
                for sit in hcmod.thread_local_statics(it, toks, it.hdr_index, it.tok_range[1]):
                    if sit.name in wanted:
                        sel.append(it)
                        wanted_hit.add(sit.name)
                continue
            if it.name in wanted:
                sel.append(it)
        found = {it.name for it in sel} | wanted_hit
        missing = wanted - found
        if missing:
            raise KeyError(f'{src_file}: items not found {sorted(missing)}')
        if with_impls:
            for it in its:
                if it.kind == 'impl' and impl_self_ident(it) in wanted:
                    sel.append(it)
        return src, sel

    def move(self, src_file, names, dst_file, with_impls=True, before=None, region_wrap=True, kinds=None, impl_idents=()):
        src, sel = self.find(src_file, names, with_impls, kinds, impl_idents)
        spans = sorted({(attached_start(src, it), it.b + (1 if it.b < len(src) and src[it.b] == '\n' else 0), it) for it in sel}, key=lambda x: x[0], reverse=False)
        blocks = []
        for a, b, it in spans:
            stack = region_stack_at(src, a)
            blocks.append((stack, rebase_paths(src[a:b].rstrip('\n') + '\n', src_file, dst_file), it))
        new_src = src
        for a, b, it in sorted(spans, key=lambda x: -x[0]):
            new_src = new_src[:a] + new_src[b:]
        new_src = re.sub(r'\n{3,}', '\n\n', new_src)
        self.ws.write(src_file, new_src)
        dst = self.ws.read(dst_file) if self.ws.exists(dst_file) else ''
        out = []
        cur = None
        for stack, text, it in blocks:
            key = tuple(stack) if region_wrap else ()
            if cur is not None and cur != key:
                for name in reversed(cur):
                    out.append(f'//#endregion {name}\n')
                out.append('\n')
            if cur != key:
                for name in key:
                    out.append(f'//#region {name}\n')
            elif out:
                out.append('\n')
            out.append(text)
            cur = key
        if cur:
            for name in reversed(cur):
                out.append(f'//#endregion {name}\n')
        block = ''.join(out)
        if before:
            its, _ = parse_items(dst)
            tgt = [i for i in its if i.name == before and i.kind not in ('use',)]
            if not tgt:
                raise KeyError(f'{dst_file}: anchor {before} not found')
            pos = attached_start(dst, tgt[0])
            dst = dst[:pos] + block + '\n' + dst[pos:]
        else:
            dst = dst.rstrip('\n') + ('\n\n' if dst.strip() else '') + block
        self.ws.write(dst_file, dst)
        for a, b, it in spans:
            if it.kind != 'impl' and it.name:
                self.moved[(src_file, it.name)] = dst_file
        self.origins[dst_file].add(src_file)
        return [it.name for a, b, it in spans if it.kind != 'impl']

    def replace_once(self, file, old, new):
        s = self.ws.read(file)
        if s.count(old) != 1:
            raise ValueError(f'{file}: expected exactly one occurrence of {old[:60]!r}, found {s.count(old)}')
        self.ws.write(file, s.replace(old, new))


# ------------------------------------------------------------------ rewrite engine
class Report:
    def __init__(self):
        self.items = collections.defaultdict(list)

    def add(self, kind, msg):
        self.items[kind].append(msg)

    def dump(self, limit=60):
        for k, v in self.items.items():
            print(f'  [{k}] {len(v)}')
            for m in v[:limit]:
                print('     ', m)


def rel(p):
    return os.path.relpath(p, REPO)


def render_use(attrs, vis, leaves):
    """leaves: list of (segs, alias, glob)."""
    pre = ''.join(a + '\n' for a in attrs) + ((vis + ' ') if vis else '')
    groups = collections.OrderedDict()
    for segs, alias, glob in leaves:
        if glob:
            groups.setdefault(tuple(segs), []).append(('*', None))
        else:
            groups.setdefault(tuple(segs[:-1]), []).append((segs[-1], alias))
    out = []
    for prefix, names in groups.items():
        pstr = '::'.join(prefix)
        rendered = []
        for n, alias in names:
            rendered.append(n + (f' as {alias}' if alias and alias != n else ''))
        if len(rendered) == 1:
            out.append(f"{pre}use {pstr + '::' if pstr else ''}{rendered[0]};")
        else:
            out.append(f"{pre}use {pstr + '::' if pstr else ''}{{{', '.join(rendered)}}};")
    return '\n'.join(out)


def inline_descendants(fm):
    for c in fm.children.values():
        if c.inline:
            yield c
            yield from inline_descendants(c)


def bracket_context(toks):
    """(innermost open bracket per token, identifiers bound locally by let/mut/ref/for/closure/parameter/field patterns)."""
    stack, encl, bound = [], [], set()
    for i, t in enumerate(toks):
        encl.append(stack[-1] if stack else '')
        if t.k == 'p':
            if t.x in '([{':
                stack.append(t.x)
            elif t.x in ')]}' and stack:
                stack.pop()
        elif t.k == 'id':
            nx = toks[i + 1].x if i + 1 < len(toks) else ''
            nx2 = toks[i + 2].x if i + 2 < len(toks) else ''
            pv = toks[i - 1].x if i > 0 else ''
            if (nx == ':' and nx2 != ':' and pv != ':') or pv in ('let', 'mut', 'ref') or (pv == 'for') or (pv == '|' and nx in ('|', ',', ':')):
                bound.add(t.x)
    return encl, bound


class LocalLeaf:
    def __init__(self, path, binding, glob):
        self.path, self.binding, self.glob = path, binding, glob


class Carver:
    def __init__(self, ws, edits, base_model):
        self.ws, self.edits, self.base = ws, edits, base_model
        self.report = Report()
        self.exports = collections.defaultdict(set)
        self.xref = collections.defaultdict(set)
        self.xedges = collections.defaultdict(lambda: collections.defaultdict(set))
        self.xex = {}
        self.pubchain = collections.defaultdict(set)
        self.test_true_crates = set()
        self.new = None

    # --- model handling
    def activate(self, model):
        hcmod.FS = self.ws
        hcmod.CRATES.clear()
        hcmod.CRATES.update(model.roots)

    def build_new(self, roots):
        self.new = Model(self.ws, roots)
        return self.new

    # --- def mapping
    def map_def(self, d):
        im = self.edits.inline_map
        if im:
            hit = im.get((d.file, d.chain + (d.name,)))
            if hit is not None and d.name:
                return Def(hit[0], hit[1][:-1], hit[1][-1])
            for k in range(len(d.chain), 0, -1):
                hit = im.get((d.file, d.chain[:k]))
                if hit is not None:
                    return Def(hit[0], hit[1] + d.chain[k:], d.name)
        top = d.chain[0] if d.chain else d.name
        nf = self.edits.moved.get((d.file, top), d.file)
        return Def(nf, d.chain, d.name)

    def mod_of(self, model, d):
        fm = model.by_file.get(d.file)
        if fm is None:
            return None
        m = fm
        for c in d.chain:
            m = m.children.get(c)
            if m is None:
                return None
        return m

    def def_item(self, model, d):
        m = self.mod_of(model, d)
        if m is None:
            return None
        if d.name == '':
            return None
        if d.name in m.items:
            return m.items[d.name][0]
        return None

    def old_mods_for(self, S):
        """Old-scope modules that correspond to new scope module S (file-level or inline)."""
        chain = []
        m = S
        while m.inline:
            chain.append(m.name)
            m = m.parent
        chain.reverse()
        files = []
        f = m.file
        files.append(f)
        if self.edits.proxy.get(f, f) not in files:
            files.append(self.edits.proxy[f])
        files += [o for o in sorted(self.edits.origins.get(f, ())) if o not in files]
        out = []
        for ff in files:
            om = self.base.by_file.get(ff)
            if om is None:
                continue
            ok = om
            for c in chain:
                ok = ok.children.get(c) if ok is not None else None
            if ok is not None:
                out.append(ok)
        return out

    def old_lookup(self, S, name):
        self.activate(self.base)
        for om in self.old_mods_for(S):
            r = hcmod.lookup(om, name)
            if r is not None:
                return r, om
        return None, None

    def old_resolve(self, S, segs):
        self.activate(self.base)
        for om in self.old_mods_for(S):
            r = hcmod.resolve(om, segs, None, True, exact=True)
            if r is not None:
                return r, om
        return None, None

    def compress_mod(self, tm, nd, assume_pub=False):
        """The nearest ancestor module that re-exports `nd` through `pub use <private file mod>::*`."""
        it = self.def_item(self.new, nd)
        if it is None or (it.vis not in ('pub', 'pub(crate)') and not assume_pub):
            return tm
        x = tm
        while x.parent is not None and not x.inline and x.vis != 'pub':
            p = x.parent
            if any(u.glob and u.path == [x.name] and u.vis.startswith('pub') for u in p.uses):
                x = p
            else:
                break
        return x

    # --- paths
    def newpath(self, F, nd, want_crate=None):
        """Path segments valid from module F (file module of the user) to new def nd. Returns (segs, crate_ident or None)."""
        tm = self.mod_of(self.new, nd)
        if tm is None:
            raise KeyError(f'new def not found: {nd}')
        if nd.name != '':
            tm = self.compress_mod(tm, nd)
        tcrate = self.new.crate_of_file[nd.file]
        fcrate = self.new.crate_ident_of(F)
        mp = self.new.modpath(tm) if nd.name != '' else self.new.modpath(tm)
        if nd.name == '':
            tail = []
            base = mp
        else:
            base = mp
            tail = [nd.name]
        if tcrate == fcrate:
            return ['crate'] + base + tail, None
        self.xref[tcrate].add(nd)
        try:
            import regs
            testy = regs.mod_is_test_only(F)
        except Exception:
            testy = False
        self.xedges[fcrate][tcrate].add(testy)
        self.xex.setdefault((fcrate, tcrate, testy), []).append((F.file, nd))
        if not base:
            return [tcrate] + tail, tcrate
        if nd.name == '':
            return [tcrate] + base, tcrate
        if self.new.public_mod_chain(tm):
            return [tcrate] + base + tail, tcrate
        x = tm
        gated = False
        while x.parent is not None:
            if any(not cfgeval.holds(cf, test=False) for cf in getattr(x, 'cfg', [])):
                gated = True
            x = x.parent
        if gated:
            self.pubchain[tcrate].add(nd)
            return [tcrate] + base + tail, tcrate
        self.exports[tcrate].add(nd)
        return [tcrate] + tail, tcrate

    def def_after(self, res_new):
        return def_of(res_new)

    # --- pass 1
    def pass1(self, only_files=None):
        """Rewrite use leaves, qualified paths and globs. Operates on self.new (already built from ws)."""
        n = 0
        for fm in list(self.new.by_file.values()):
            if only_files and fm.file not in only_files:
                continue
            if getattr(fm, 'missing', False):
                continue
            new_src = self.rewrite_file_pass1(fm)
            if new_src is not None and new_src != fm.src:
                self.ws.write(fm.file, new_src)
                n += 1
        return n

    def scopes_of(self, fm):
        yield fm
        yield from inline_descendants(fm)

    def rewrite_file_pass1(self, fm):
        src = fm.src
        toks = fm.toks
        patches = []
        use_ranges = []
        for S in self.scopes_of(fm):
            by_item = collections.OrderedDict()
            for u in S.uses:
                by_item.setdefault(id(u.item), (u.item, []))[1].append(u)
            for _, (it, leaves) in by_item.items():
                use_ranges.append(it.tok_range)
                out = []
                changed = False
                for u in leaves:
                    r = self.handle_leaf(S, fm, u)
                    if r is None:
                        out.append((u.path, u.binding if u.binding != (u.path[-1] if u.path else None) else None, u.glob))
                        continue
                    action, newleaf = r
                    if action == 'keep':
                        out.append((u.path, self._alias(u), u.glob))
                    elif action == 'drop':
                        changed = True
                    elif action == 'rewrite':
                        changed = True
                        out.append(newleaf)
                if changed:
                    a = attached_start(src, it)
                    b = it.b
                    text = render_use(it.attrs, it.vis, out) if out else ''
                    # keep attached comments (doc lines) above: preserve text from a..hdr start
                    first_code = next((t for t in it.toks_all if t.k != 'comment'), None)
                    lead = src[a:first_code.a] if first_code is not None else ''
                    lead = lead if lead.strip() else ''
                    patches.append((a, b, (lead + text) if text else ''))
        for i, j, S, leaves in self.local_use_statements(fm, use_ranges):
            out = []
            changed = False
            for u in leaves:
                r = self.handle_leaf(S, fm, u)
                if r is None or r[0] == 'keep':
                    out.append((u.path, self._alias(u), u.glob))
                elif r[0] == 'drop':
                    changed = True
                else:
                    changed = True
                    out.append(r[1])
            if changed:
                patches.append((toks[i].a, toks[j].b, render_use([], '', out) if out else ''))
        # qualified paths outside use items
        for S in self.scopes_of(fm):
            lo, hi = (0, len(toks))
            for i, j, segs in free_idents(toks, 0, len(toks), use_ranges):
                if len(segs) < 2:
                    continue
                scope = self.scope_at(fm, i)
                if scope is not S:
                    continue
                if segs[0] not in ('crate', 'self', 'super') and hcmod.lookup(scope, segs[0]) is None:
                    pass
                pr = self.patch_qualified(scope, fm, toks, i, j, segs)
                if pr:
                    patches.append(pr)
        patches += self.vis_path_patches(fm)
        if not patches:
            return None
        patches.sort(key=lambda p: p[0])
        for (a1, b1, _), (a2, b2, _) in zip(patches, patches[1:]):
            if a2 < b1:
                raise RuntimeError(f'overlapping patches in {rel(fm.file)}: {(a1, b1)} {(a2, b2)}')
        out = src
        for a, b, r in sorted(patches, key=lambda p: -p[0]):
            out = out[:a] + r + out[b:]
        return out

    def local_use_statements(self, fm, use_ranges):
        """Function-local `use` statements: (use token index, `;` token index, enclosing scope, leaves); their ranges are added to `use_ranges`."""
        toks = fm.toks
        out = []
        ranges = sorted(use_ranges)
        for i, t in enumerate(toks):
            if t.k != 'id' or t.x != 'use':
                continue
            if any(a <= i < b for a, b in ranges):
                continue
            prev = toks[i - 1].x if i > 0 else ''
            if prev not in ('{', ';', '}', ']'):
                continue
            leaves, j = hcmod.parse_use(toks, i, len(toks))
            if j >= len(toks) or toks[j].x != ';':
                continue
            use_ranges.append((i, j + 1))
            out.append((i, j, self.scope_at(fm, i), [LocalLeaf(p, b, g) for p, b, g in leaves]))
        return out

    def vis_path_patches(self, fm):
        toks = fm.toks
        out = []
        n = len(toks)
        for i in range(n - 3):
            if toks[i].x == 'pub' and toks[i].k == 'id' and toks[i + 1].x == '(' and toks[i + 2].x == 'in' and toks[i + 2].k == 'id':
                j = i + 3
                segs = []
                while j < n and toks[j].x != ')':
                    if toks[j].k == 'id':
                        segs.append(toks[j].x)
                    j += 1
                S = self.scope_at(fm, i)
                r, om = self.old_resolve(S, segs)
                if r is None or r.kind != 'mod':
                    continue
                d = def_of(r)
                nd = self.map_def(d)
                tm = self.mod_of(self.new, nd)
                if tm is None:
                    continue
                same = self.new.crate_of_file[nd.file] == self.new.crate_ident_of(fm)
                if not same:
                    text = 'pub'
                else:
                    mp = self.new.modpath(tm)
                    text = 'pub(crate)' if not mp else 'pub(in crate::' + '::'.join(mp) + ')'
                cur = ''.join(t.x for t in toks[i:j + 1])
                if cur != text.replace(' ', ''):
                    out.append((toks[i].a, toks[j].b, text))
        return out

    def _alias(self, u):
        if u.glob:
            return None
        return u.binding if u.binding != u.path[-1] else None

    def scope_at(self, fm, tok_index):
        best = fm
        for c in inline_descendants(fm):
            a, b = c.item.tok_range
            if a <= tok_index < b:
                best = c
        return best

    def relative_candidate(self, S, first):
        if first in ('crate', 'self', 'super'):
            return True
        self.activate(self.new)
        r = hcmod.lookup(S, first)
        return r is not None and r.kind in ('mod', 'item', 'variant')

    def handle_leaf(self, S, fm, u):
        if not u.path:
            return None
        first = u.path[0]
        old_res, om = self.old_resolve(S, u.path)
        if first not in ('crate', 'self', 'super'):
            self.activate(self.base)
            ol = None
            for o in self.old_mods_for(S):
                ol = hcmod.lookup(o, first)
                if ol is not None:
                    break
            if ol is None and first in hcmod.CRATES:
                pass
            elif ol is None or ol.kind == 'external':
                return ('keep', None)
        if old_res is None:
            self.report.add('unresolved-old-leaf', f'{rel(fm.file)}: use {"::".join(u.path)}{"::*" if u.glob else ""}')
            return ('keep', None)
        if old_res.kind == 'external':
            if first in hcmod.CRATES and not u.glob:
                segs = old_res.ext.split('::') + list(old_res.rest)
                alias = u.binding if u.binding != segs[-1] else None
                return ('rewrite', (segs, alias, False))
            if first in ('super', 'self', 'crate') and not u.glob:
                full = old_res.ext.split('::') + list(old_res.rest)
                self.activate(self.new)
                nr = hcmod.resolve(S, u.path, None, True, exact=True)
                if nr is not None and nr.kind == 'external' and nr.ext.split('::') + list(nr.rest) == full:
                    return ('keep', None)
                alias = u.binding if u.binding != full[-1] else None
                return ('rewrite', (full, alias, False))
            return ('keep', None)
        d = def_of(old_res)
        if d is None:
            return ('keep', None)
        nd = self.map_def(d)
        # still resolves identically in the new tree?
        self.activate(self.new)
        nr = hcmod.resolve(S, u.path, None, True, exact=True)
        if u.glob and nr is not None and nr.kind == 'mod' and nr.mod is S:
            return ('drop', None)
        if nr is not None and nr.kind in ('item', 'mod', 'variant'):
            if def_of(nr) == nd and not u.glob:
                return ('keep', None)
            if u.glob and nr.kind == 'mod' and def_of(nr) == nd:
                return ('keep', None)
        if u.glob:
            return ('drop', None)
        if nd.name and nd.file == S.file_root().file and tuple(nd.chain) == tuple(chain_of(S)):
            return ('drop', None)
        segs, tcrate = self.newpath(S.file_root(), nd)
        if old_res.kind == 'variant':
            segs = segs + [old_res.name]
        alias = u.binding if u.binding != segs[-1] and u.binding != '_' else self._alias(u)
        if alias == segs[-1]:
            alias = None
        return ('rewrite', (segs, alias, False))

    def patch_qualified(self, S, fm, toks, i, j, segs):
        if segs[0] in hcmod.CRATES and hcmod.lookup(S, segs[0]) is None:
            r, om = self.old_resolve(S, segs)
            if r is not None and r.kind == 'external':
                full = r.ext + ''.join('::' + x for x in r.rest)
                return (toks[i].a, toks[i + 3 * (len(segs) - 1)].b, full)
        best = None
        for k in range(len(segs), 0, -1):
            r, om = self.old_resolve(S, segs[:k])
            if r is not None and r.kind in ('item', 'mod', 'variant'):
                best = (k, r)
                break
            if r is not None and r.kind == 'external' and segs[0] in ('super', 'self', 'crate') and k > 1:
                full = r.ext.split('::') + list(r.rest)
                self.activate(self.new)
                nr = hcmod.resolve(S, segs[:k], None, False, exact=True)
                if nr is not None and nr.kind == 'external' and nr.ext.split('::') + list(nr.rest) == full:
                    return None
                return (toks[i].a, toks[i + 3 * (k - 1)].b, '::'.join(full))
        if best is None:
            return None
        k, r = best
        if r.kind == 'mod' and r.mod is not None and r.mod.parent is None and k < len(segs):
            return None
        if segs[0] not in ('crate', 'self', 'super'):
            # plain names go through use bindings handled in the leaf pass; only module-relative paths here
            self.activate(self.base)
            ol = None
            for o in self.old_mods_for(S):
                ol = hcmod.lookup(o, segs[0])
                if ol is not None:
                    break
            if ol is None and segs[0] in hcmod.CRATES:
                pass
            elif ol is None or ol.kind not in ('mod',):
                return None
        d = def_of(r)
        if d is None:
            return None
        nd = self.map_def(d)
        self.activate(self.new)
        nr = hcmod.resolve(S, segs[:k], None, False, exact=True)
        if nr is not None and def_of(nr) == nd:
            return None
        newsegs, tcrate = self.newpath(S.file_root(), nd)
        if r.kind == 'variant':
            newsegs = newsegs + [r.name]
        # token range: first token i .. last token of segs[:k]
        end_tok = i + 3 * (k - 1)
        a = toks[i].a
        b = toks[end_tok].b
        return (a, b, '::'.join(newsegs))

    # --- pass 2: free names
    def pass2(self, only_files=None):
        added = 0
        for fm in list(self.new.by_file.values()):
            if only_files and fm.file not in only_files:
                continue
            if getattr(fm, 'missing', False):
                continue
            new_src = self.rewrite_file_pass2(fm)
            if new_src is not None and new_src != fm.src:
                self.ws.write(fm.file, new_src)
                added += 1
        return added

    def rewrite_file_pass2(self, fm):
        src = fm.src
        toks = fm.toks
        use_ranges = [u.item.tok_range for S in self.scopes_of(fm) for u in S.uses]
        needs = collections.defaultdict(dict)  # scope -> name -> leaf spec
        encl, bound = bracket_context(toks)
        self.activate(self.new)
        for i, j, segs in list(free_idents(toks, 0, len(toks), use_ranges)) + [(i, i + 1, [name]) for i, name in fmt_arg_idents(toks)]:
            first = segs[0]
            if first in hcmod.KEYWORDS or first == 'Self' or first in ('crate', 'self', 'super'):
                continue
            if i > 0 and toks[i - 1].k == 'id' and toks[i - 1].x in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union', 'ref'):
                continue
            if i > 0 and toks[i - 1].k == 'id' and toks[i - 1].x == 'mut' and not (i > 1 and toks[i - 2].x in ('&', '*')):
                continue
            nxt = toks[j].x if j < len(toks) else ''
            nxt2 = toks[j + 1].x if j + 1 < len(toks) else ''
            prv = toks[i - 1].x if i > 0 else ''
            value_arg = len(segs) == 1 and prv in ('(', ',') and nxt in (')', ',') and encl[i] == '(' and first not in bound
            only_value = not first[0].isupper() and not (nxt == '(' or (nxt == ':' and nxt2 == ':') or nxt == '!') and value_arg
            if not first[0].isupper() and not (nxt == '(' or (nxt == ':' and nxt2 == ':') or nxt == '!' or value_arg):
                continue
            S = self.scope_at(fm, i)
            self.activate(self.new)
            nr = hcmod.lookup(S, first)
            if nr is not None:
                continue
            orr, om = self.old_lookup(S, first)
            if orr is None:
                continue
            if only_value and not (orr.kind == 'item' and orr.item.kind in ('fn', 'const', 'static')):
                continue
            if first in needs[S]:
                continue
            if any((not u.glob) and u.binding == first for u in S.uses):
                continue
            if orr.kind == 'external':
                via = orr.via
                cfg = via.cfg if via is not None else []
                needs[S][first] = ('ext', orr.ext, cfg)
            else:
                d = def_of(orr)
                if d is None:
                    continue
                nd = self.map_def(d)
                if nd.file not in self.new.by_file:
                    self.report.add('lost-def', f'{rel(fm.file)}: {first} -> {nd}')
                    continue
                item = self.def_item(self.new, nd)
                cfg = item.cfg if item is not None else []
                needs[S][first] = ('def', nd, cfg, orr.kind == 'variant', orr.name)
        if not needs:
            return None
        patches = []
        always_test = self.new.crate_ident_of(fm) in self.test_true_crates
        for S, names in needs.items():
            bycfg = collections.OrderedDict()
            for name, spec in sorted(names.items()):
                if spec[0] == 'ext':
                    segs = spec[1].split('::')
                    cfgs = tuple(spec[2])
                else:
                    nd, cfg = spec[1], spec[2]
                    segs, tcrate = self.newpath(S.file_root(), nd)
                    if spec[3]:
                        segs = segs + [spec[4]]
                    cfgs = tuple(cfg)
                if always_test:
                    if any(not cfgeval.holds(cf, test=True) for cf in cfgs):
                        continue
                    cfgs = tuple(cf for cf in cfgs if cfgeval.holds(cf, test=False))
                alias = name if segs[-1] != name else None
                bycfg.setdefault(cfgs, []).append((segs, alias, False))
            lines = []
            for cfgs, leaves in bycfg.items():
                attrs = [f'#[cfg({c})]' for c in cfgs]
                lines.append(render_use(attrs, '', leaves))
            text = '\n'.join(lines) + '\n'
            # insertion point: after last use item of S (file-level), else after leading inner docs
            if S.uses:
                last = max((u.item for u in S.uses), key=lambda it: it.b)
                pos = last.b
                while pos < len(src) and src[pos] != '\n':
                    pos += 1
                patches.append((pos + 1, pos + 1, text))
            else:
                if S.inline:
                    a, b = S.item.tok_range
                    body = S.item.toks_all
                    k = next(idx for idx, t in enumerate(body) if t.x == '{')
                    pos = body[k].b
                    patches.append((pos, pos, '\n' + text))
                else:
                    pos = self.header_end(src, toks)
                    patches.append((pos, pos, '\n' + text))
        out = src
        for a, b, r in sorted(patches, key=lambda p: -p[0]):
            out = out[:a] + r + out[b:]
        return out

    def header_end(self, src, toks):
        """Offset after the leading inner doc/attribute block."""
        pos = 0
        lines = src.split('\n')
        off = 0
        last = 0
        for ln in lines:
            s = ln.strip()
            if s.startswith('//!') or s.startswith('#![') or s == '' or s.startswith('//'):
                off += len(ln) + 1
                if s.startswith('//!') or s.startswith('#!['):
                    last = off
                continue
            break
        return last


# ------------------------------------------------------------------ pass 3: tidy
def trait_method_names(item):
    names = set()
    toks = item.toks
    for i, t in enumerate(toks):
        if t.k == 'id' and t.x == 'fn' and i + 1 < len(toks):
            names.add(toks[i + 1].x)
    return names


def dot_methods(toks):
    out = set()
    for i in range(1, len(toks) - 1):
        if toks[i - 1].x == '.' and toks[i - 1].k == 'p' and toks[i].k == 'id':
            out.add(toks[i].x)
    return out


def _tail_ids(fm):
    return {t.x for t in fm.toks if t.k == 'id'}


def descendants_glob_ids(fm, model):
    ids = set()
    for c in fm.children.values():
        if c.inline:
            continue
        if any(u.glob and u.path == ['super'] for u in c.uses):
            ids |= _tail_ids(c)
            ids |= descendants_glob_ids(c, model)
    return ids


def pass3(self, crate_roots):
    n = 0
    self.activate(self.new)
    for fm in list(self.new.by_file.values()):
        if getattr(fm, 'missing', False):
            continue
        src = fm.src
        toks = fm.toks
        patches = []
        is_root = fm.parent is None
        # root-level pub(super) is invalid in a crate root
        if is_root:
            for i, t in enumerate(toks):
                if t.x == 'super' and t.k == 'id' and i >= 2 and toks[i - 1].x == '(' and toks[i - 2].x == 'pub' and i + 1 < len(toks) and toks[i + 1].x == ')':
                    inline = any(c.item.tok_range[0] <= i < c.item.tok_range[1] for c in inline_descendants(fm))
                    if not inline:
                        patches.append((t.a, t.b, 'crate'))
        use_ranges = [u.item.tok_range for S in self.scopes_of(fm) for u in S.uses]
        ids_outside = set()
        for i, t in enumerate(toks):
            if t.k == 'id' and not any(a <= i < b for a, b in use_ranges):
                ids_outside.add(t.x)
        ids_outside |= {name for _, name in fmt_arg_idents(toks)}
        ids_outside |= descendants_glob_ids(fm, self.new)
        methods = dot_methods(toks)
        for c in fm.children.values():
            if not c.inline and any(u.glob and u.path == ['super'] for u in c.uses):
                methods |= dot_methods(c.toks)
        for S in self.scopes_of(fm):
            by_item = collections.OrderedDict()
            for u in S.uses:
                by_item.setdefault(id(u.item), (u.item, []))[1].append(u)
            for _, (it, leaves) in by_item.items():
                keep = []
                changed = False
                new_vis = it.vis
                for u in leaves:
                    if u.glob:
                        keep.append((u.path, None, True))
                        continue
                    r = hcmod.resolve(S, u.path, None, True)
                    first_party = r is not None and r.kind in ('item', 'mod', 'variant') and r.mod is not None
                    if not first_party:
                        keep.append((u.path, self._alias(u), False))
                        continue
                    foreign = self.new.crate_ident_of(r.mod.file_root()) != self.new.crate_ident_of(S.file_root())
                    if it.vis.startswith('pub') and foreign:
                        new_vis = ''
                        changed = True
                    binding = u.binding
                    used = binding in ids_outside or binding == '_'
                    if not used and r.kind == 'item' and r.item.kind == 'trait':
                        used = bool(trait_method_names(r.item) & methods)
                    if not used and it.vis.startswith('pub') and not foreign:
                        used = True
                    if not used:
                        changed = True
                        continue
                    keep.append((u.path, self._alias(u), False))
                if changed:
                    a = attached_start(src, it)
                    first_code = next((t for t in it.toks_all if t.k != 'comment'), None)
                    lead = src[a:first_code.a] if first_code is not None else ''
                    lead = lead if lead.strip() else ''
                    text = render_use(it.attrs, new_vis, keep) if keep else ''
                    patches.append((a, it.b + (1 if it.b < len(src) and src[it.b] == '\n' and not keep else 0), (lead + text) if text else ''))
        if patches:
            patches.sort(key=lambda p: p[0])
            out = src
            for a, b, r in sorted(patches, key=lambda p: -p[0]):
                out = out[:a] + r + out[b:]
            if out != src:
                self.ws.write(fm.file, out)
                n += 1
    return n


Carver.pass3 = pass3


# ------------------------------------------------------------------ exports and visibility
def widen_item_vis(src, item):
    """Return (start, end, replacement) making the item header `pub`."""
    hdr_tok = item.toks[0]
    toks = item.toks
    j = 0
    if toks[0].x == 'pub' and toks[0].k == 'id':
        end = toks[0].b
        if len(toks) > 1 and toks[1].x == '(' :
            k = 1
            while toks[k].x != ')':
                k += 1
            end = toks[k].b
        return (toks[0].a, end, 'pub')
    return (toks[0].a, toks[0].a, 'pub ')


def _mount_item_of(m):
    return getattr(m, 'item', None)


def _testkit_attr(text):
    if 'testkit' in text:
        return text
    m = re.match(r'(#\[\s*cfg\()(.*)(\)\s*\])$', text, re.S)
    if not m:
        return text
    pred = re.sub(r'\btest\b', 'any(test, feature = "testkit")', m.group(2))
    return m.group(1) + pred + m.group(3)


CFG_ATTR = re.compile(r'#\[\s*cfg\(([^\]]*)\)\s*\]')


def open_file_test_attrs(src):
    """Open every remaining cfg(test)-only attribute of a seam file to the testkit feature, except those that gate a module."""
    out = []
    pos = 0
    for m in CFG_ATTR.finditer(src):
        pred = m.group(1)
        if cfgeval.holds(pred, test=False) or 'testkit' in pred:
            continue
        after = src[m.end():m.end() + 400]
        nxt = re.match(r'(?:\s|#\[[^\]]*\]|//[^\n]*\n|/\*.*?\*/)*(pub(?:\([^)]*\))?\s+)?(mod)\b', after, re.S)
        if nxt:
            continue
        out.append((m.start(), m.end(), _testkit_attr(m.group(0))))
    for a, b, r in sorted(out, key=lambda p: -p[0]):
        src = src[:a] + r + src[b:]
    return src


def finalize_exports(self):
    """Widen cross-crate referenced defs to pub, make module chains of module targets pub, open cfg(test) seams to the `testkit` feature and add explicit root `pub use` lines where the module chain is not public."""
    self.activate(self.new)
    import cfgeval
    vis_patches = collections.defaultdict(dict)   # file -> {(a,b): replacement}
    attr_patches = collections.defaultdict(dict)
    root_exports = collections.defaultdict(lambda: collections.OrderedDict())
    collisions = []
    testseams = set()

    def widen(file, it):
        src = self.ws.read(file)
        a, b, r = widen_item_vis(src, it)
        vis_patches[file][(a, b)] = r

    opened = set()
    seam_files = set()

    def open_cfg(file, it, depth=0):
        seam_files.add(file)
        key = (file, it.a)
        if key in opened:
            return
        opened.add(key)
        src = self.ws.read(file)
        # same-file test-gated items named by this item are part of the seam
        if depth < 6:
            fm = self.new.by_file.get(file)
            if fm is not None:
                names = {t.x for t in it.toks_all if t.k == 'id'}
                for sm in [fm] + list(inline_descendants(fm)):
                    for nm, lst in sm.items.items():
                        if nm in names:
                            for other in lst:
                                if other is not it and any(not cfgeval.holds(cf, test=False) for cf in other.cfg):
                                    open_cfg(file, other, depth + 1)
        for at in it.attrs:
            if 'cfg' in at and 'test' in at and 'testkit' not in at:
                preds = at[at.index('(') + 1:at.rindex(')')]
                if not cfgeval.holds(preds, test=False):
                    pos = src.find(at, it.a)
                    # attrs are re-joined tokens: search the raw attribute zone instead
                    zone_end = it.toks[0].a
                    zone = src[it.toks_all[0].a:zone_end]
                    new_zone = re.sub(r'#\[\s*cfg\(([^\]]*)\)\s*\]', lambda mm: _testkit_attr(mm.group(0)) if not cfgeval.holds(mm.group(1), test=False) else mm.group(0), zone)
                    if new_zone != zone:
                        attr_patches[file][(it.toks_all[0].a, zone_end)] = new_zone

    for tcrate, defs in self.xref.items():
        for nd in sorted(defs, key=lambda d: (d.file, d.chain, d.name)):
            m = self.mod_of(self.new, nd)
            if m is None:
                continue
            force_pub = nd in self.pubchain.get(tcrate, ())
            # module chain: every mount on the way up must be pub for module targets; cfg(test) mounts open to testkit
            x = m
            while x.parent is not None:
                it = _mount_item_of(x)
                if it is not None:
                    if (nd.name == '' or force_pub) and it.vis != 'pub':
                        widen(x.parent.file_root().file, it)
                    if any(not cfgeval.holds(cf, test=False) for cf in it.cfg):
                        open_cfg(x.parent.file_root().file, it)
                        for sub in hcmod.walk(x):
                            if not sub.inline and not getattr(sub, 'missing', False):
                                seam_files.add(sub.file)
                x = x.parent
            if nd.name == '':
                continue
            items = [i for i in m.items.get(nd.name, []) if not (i.toks and (i.toks[0].x in hcmod.NAMED_MACROS or i.toks[0].x in hcmod.ENUM_MACROS))]
            for it in items:
                if it.kind == 'extern_crate':
                    continue
                if it.vis != 'pub':
                    widen(m.file_root().file, it)
                if any(not cfgeval.holds(cf, test=False) for cf in it.cfg):
                    open_cfg(m.file_root().file, it)
            mc = self.compress_mod(m, nd, True)
            chain_public = self.new.public_mod_chain(mc)
            if not chain_public and not force_pub:
                mp = self.new.modpath(mc)
                y = mc
                while y.parent is not None and y.parent.parent is not None:
                    yi = _mount_item_of(y)
                    if yi is not None and yi.vis != 'pub':
                        widen(y.parent.file_root().file, yi)
                    y = y.parent
                key = nd.name
                if key in root_exports[tcrate] and root_exports[tcrate][key][0] != (mp):
                    collisions.append((tcrate, nd))
                else:
                    cfgs = items[0].cfg if items else []
                    cfgs = [_testkit_attr('#[cfg(' + cf + ')]')[len('#[cfg('):-len(')]')] if not cfgeval.holds(cf, test=False) else cf for cf in cfgs]
                    root_exports[tcrate][key] = (mp, cfgs)
    files = set(vis_patches) | set(attr_patches)
    for file in files:
        src = self.ws.read(file)
        allp = [(a, b, r) for (a, b), r in vis_patches.get(file, {}).items()] + [(a, b, r) for (a, b), r in attr_patches.get(file, {}).items()]
        for a, b, r in sorted(allp, key=lambda p: -p[0]):
            src = src[:a] + r + src[b:]
        self.ws.write(file, src)
    for file in seam_files:
        self.ws.write(file, open_file_test_attrs(self.ws.read(file)))
    for tcrate, names in root_exports.items():
        root = self.new.roots[tcrate]
        rfile = root.file
        src = self.ws.read(rfile)
        bymod = collections.OrderedDict()
        have = {u.binding for u in root.uses if not u.glob and u.vis.startswith('pub')} | set(root.items)
        for name, (mp, cfgs) in names.items():
            if not mp or name in have:
                continue
            bymod.setdefault((tuple(mp), tuple(cfgs)), []).append(name)
        lines = []
        for (mp, cfgs), ns in bymod.items():
            attrs = ''.join(f'#[cfg({c})]\n' for c in cfgs)
            if len(ns) == 1:
                lines.append(f"{attrs}pub use {'::'.join(mp)}::{ns[0]};")
            else:
                lines.append(f"{attrs}pub use {'::'.join(mp)}::{{{', '.join(ns)}}};")
        if lines:
            src = src.rstrip('\n') + '\n\n' + '\n'.join(lines) + '\n'
            self.ws.write(rfile, src)
    for c in collisions:
        self.report.add('export-collision', f'{c[0]}: {c[1]}')
    return collisions


Carver.finalize_exports = finalize_exports


def widen_members(self, findings):
    """Make the reported non-pub members `pub` (text patches at the recorded offsets)."""
    by_file = collections.defaultdict(dict)
    for f in findings:
        by_file[f['file']][(f['vs'], f['ve'])] = f
    n = 0
    for file, spans in by_file.items():
        src = self.ws.read(file)
        for (vs, ve), f in sorted(spans.items(), key=lambda x: -x[0][0]):
            if vs == ve:
                src = src[:vs] + 'pub ' + src[vs:]
            else:
                src = src[:vs] + 'pub' + src[ve:]
            n += 1
        self.ws.write(file, src)
    return n


Carver.widen_members = widen_members


def open_test_imports(self):
    """Open the cfg(test) imports of defs whose gate was opened to the `testkit` feature."""
    self.activate(self.new)
    n = 0
    for fm in list(self.new.by_file.values()):
        if getattr(fm, 'missing', False):
            continue
        src = fm.src
        patches = {}
        for S in self.scopes_of(fm):
            for u in S.uses:
                it = u.item
                if u.glob or any('testkit' in cf for cf in it.cfg) or all(cfgeval.holds(cf, test=False) for cf in it.cfg):
                    continue
                r = hcmod.resolve(S, u.path, None, True, exact=True)
                if r is None or r.kind not in ('item', 'mod') or r.item is None or not any('testkit' in cf for cf in r.item.cfg):
                    continue
                a, b = it.toks_all[0].a, it.toks[0].a
                zone = src[a:b]
                new_zone = CFG_ATTR.sub(lambda mm: _testkit_attr(mm.group(0)) if not cfgeval.holds(mm.group(1), test=False) else mm.group(0), zone)
                if new_zone != zone:
                    patches[(a, b)] = new_zone
        if patches:
            for (a, b), r in sorted(patches.items(), key=lambda x: -x[0][0]):
                src = src[:a] + r + src[b:]
            self.ws.write(fm.file, src)
            n += len(patches)
    return n


Carver.open_test_imports = open_test_imports

NOT_TEST = re.compile(r'not\(\s*test\s*\)')


def flip_not_test_partners(self):
    """Where a name is bound under the opened `testkit` gate, the `cfg(not(test))` binding of the same name in the same scope follows the same gate."""
    self.activate(self.new)
    n = 0
    for fm in list(self.new.by_file.values()):
        if getattr(fm, 'missing', False):
            continue
        src = fm.src
        patches = {}
        for S in self.scopes_of(fm):
            opened = set()
            for name, lst in S.items.items():
                if any(any('testkit' in cf for cf in it.cfg) for it in lst):
                    opened.add(name)
            for u in S.uses:
                if not u.glob and any('testkit' in cf for cf in u.item.cfg):
                    opened.add(u.binding)
            if not opened:
                continue
            cands = []
            for name, lst in S.items.items():
                if name in opened:
                    cands += [it for it in lst if any(NOT_TEST.search(cf) for cf in it.cfg)]
            for u in S.uses:
                if not u.glob and u.binding in opened and any(NOT_TEST.search(cf) for cf in u.item.cfg):
                    cands.append(u.item)
            for it in cands:
                a, b = it.toks_all[0].a, it.toks[0].a
                zone = src[a:b]
                new_zone = CFG_ATTR.sub(lambda mm: NOT_TEST.sub('not(any(test, feature = "testkit"))', mm.group(0)) if 'testkit' not in mm.group(0) else mm.group(0), zone)
                if new_zone != zone:
                    patches[(a, b)] = new_zone
        if patches:
            for (a, b), r in sorted(patches.items(), key=lambda x: -x[0][0]):
                src = src[:a] + r + src[b:]
            self.ws.write(fm.file, src)
            n += len(patches)
    return n


Carver.flip_not_test_partners = flip_not_test_partners


def crate_dag(self):
    """Return (normal edges, test-only edges, cycles) among carved crates from the recorded cross-crate references."""
    normal = collections.defaultdict(set)
    testonly = collections.defaultdict(set)
    for f, targets in self.xedges.items():
        for t, flags in targets.items():
            if False in flags:
                normal[f].add(t)
            else:
                testonly[f].add(t)
    # tarjan over normal edges
    index, low, onstack, stack, out = {}, {}, set(), [], []
    counter = [0]

    def strong(v):
        index[v] = low[v] = counter[0]
        counter[0] += 1
        stack.append(v)
        onstack.add(v)
        for w in normal.get(v, ()):
            if w not in index:
                strong(w)
                low[v] = min(low[v], low[w])
            elif w in onstack:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop()
                onstack.discard(w)
                comp.append(w)
                if w == v:
                    break
            if len(comp) > 1:
                out.append(comp)

    for v in list(normal):
        if v not in index:
            strong(v)
    return normal, testonly, out


Carver.crate_dag = crate_dag
