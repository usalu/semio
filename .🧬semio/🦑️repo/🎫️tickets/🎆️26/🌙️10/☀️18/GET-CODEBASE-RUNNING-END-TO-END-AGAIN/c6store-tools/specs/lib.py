import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
PL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
REPO = '/Users/ueli/Documents/semio'
import re as _re
_PATH = _re.compile(r'#\[path\s*=\s*"([^"]+)"\]')


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def mounted(file_rel, glue=None, orig=None):
    seen = []
    todo = [file_rel]
    first = True
    while todo:
        cur = todo.pop(0)
        p = os.path.join(REPO, cur)
        if not os.path.isfile(p):
            continue
        is_root = first
        if not first:
            if cur in seen:
                continue
            seen.append(cur)
        first = False
        text = open(os.path.join(T, 'orig', orig) if (is_root and orig) else p, encoding='utf-8').read()
        for m in _PATH.finditer(text):
            rel = m.group(1)
            if rel == '.' or not rel.endswith('.rs'):
                continue
            nxt = os.path.normpath(os.path.join(os.path.dirname(cur), rel))
            if nxt not in seen and nxt != file_rel:
                todo.append(nxt)
    out = list(seen)
    if glue:
        out.append(glue)
    return out


def split_impl(header, boundaries, nlines=1):
    def pre(text):
        lines = text.split('\n')
        hs = [i for i, l in enumerate(lines) if l == header]
        assert len(hs) == 1, ('impl header not unique', header, hs)
        h = hs[0]
        hdr = lines[h:h + nlines]
        for b in sorted(boundaries, reverse=True):
            assert h < b - 1, ('boundary before the impl header', b)
            lines[b - 1:b - 1] = ['}', ''] + hdr
        return '\n'.join(lines)
    return pre


def split_extra(header, boundaries, nlines=1, hdr=None):
    hl = hdr if hdr is not None else [header]
    return ['}'] * len(boundaries) + [l for l in hl for _ in boundaries]
DB = '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db'


STORE = '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store'


class IS:
    """impl splitter: impls = [(header_line, header_lines, [boundary | (boundary, [region open lines])])], boundaries are 1-based lines of the original text."""

    def __init__(self, impls):
        self.impls = impls
        self.bs = []
        for (_, n, bl) in impls:
            for x in bl:
                b, rg = (x, []) if isinstance(x, int) else x
                self.bs.append((b, n, len(rg)))
        self.bs.sort()

    def new(self, L):
        return L + sum(2 + n + 2 * k for b, n, k in self.bs if b <= L)

    def lo(self, a):
        for b, n, k in self.bs:
            if b == a:
                return self.new(a) - n - k
        return self.new(a)

    def hi(self, b):
        for bb, n, k in self.bs:
            if bb == b + 1:
                return self.new(b) + k + 1
        return self.new(b)

    def rng(self, a, b):
        return (self.lo(a), self.hi(b))

    def pre(self, text):
        lines = text.split('\n')
        ins = []
        for h, n, bl in self.impls:
            hdr = lines[h - 1:h - 1 + n]
            assert hdr[-1].rstrip().endswith('{'), ('impl header does not end with a brace', h, hdr)
            for x in bl:
                b, rg = (x, []) if isinstance(x, int) else x
                assert b > h + n, ('boundary inside the header', b)
                closers = [r.replace('#region', '#endregion', 1) for r in reversed(rg)]
                ins.append((b, closers + ['}', ''] + hdr + list(rg)))
        for b, add in sorted(ins, key=lambda x: -x[0]):
            lines[b - 1:b - 1] = add
        return '\n'.join(lines)

    def extra_full(self, text):
        lines = text.split('\n')
        out = []
        for h, n, bl in self.impls:
            out.extend(['}'] * len(bl))
            for _ in bl:
                out.extend(l for l in lines[h - 1:h - 1 + n])
        return out


def crate_ext(root_rel, self_rel, orig=None, children=()):
    kids = {os.path.join(os.path.dirname(self_rel), c['folder'], '🦀️.rs') for c in children}
    files = [root_rel] + mounted(root_rel)
    out = [f for f in files if f != self_rel and f not in kids]
    return out + [f for f in mounted(self_rel, orig=orig) if f not in kids]
