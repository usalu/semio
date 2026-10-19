import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
U = '🧰️framework/🔨️modules/🖱️ui'
REPO = '/Users/ueli/Documents/semio'
import re as _re
_PATH = _re.compile(r'#\[path\s*=\s*"([^"]+)"\]|\binclude!\(\s*"([^"]+)"\s*\)')


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
            rel = m.group(1) or m.group(2)
            if rel == '.' or not rel.endswith('.rs'):
                continue
            nxt = os.path.normpath(os.path.join(os.path.dirname(cur), rel))
            if nxt not in seen and nxt != file_rel:
                todo.append(nxt)
    out = list(seen)
    if glue:
        out.append(glue)
    return out


def split_impl(header, boundaries, nlines=1, ind=''):
    def pre(text):
        lines = text.split('\n')
        hs = [i for i, l in enumerate(lines) if l == header]
        assert len(hs) == 1, ('impl header not unique', header, hs)
        h = hs[0]
        hdr = lines[h:h + nlines]
        for b in sorted(boundaries, reverse=True):
            assert h < b - 1, ('boundary before the impl header', b)
            lines[b - 1:b - 1] = [ind + '}', ''] + hdr
        return '\n'.join(lines)
    return pre


def split_extra(header, boundaries, nlines=1, hdr=None):
    hl = hdr if hdr is not None else [header]
    return ['}'] * len(boundaries) + [l for l in hl for _ in boundaries]


class Cut:
    """✂️ c7w spec builder: ranges are given in ORIGINAL line numbers; impl blocks are cut at method boundaries and ranges are mapped onto the pre-edited text. `at` pins a non-unique impl header to its (1-based) line."""

    def __init__(self, file_rel, orig):
        self.file = file_rel
        self.orig = orig
        self.ins = []
        self.children = []
        self._lines = open(os.path.join(T, 'orig', orig), encoding='utf-8').read().split('\n')

    def impl(self, header, boundaries, nlines=1, ind='', at=None):
        if at is None:
            hs = [k + 1 for k, l in enumerate(self._lines) if l == header]
            assert len(hs) == 1, ('impl header not unique, pass at=', header, hs)
            at = hs[0]
        assert self._lines[at - 1] == header, ('header mismatch', at, self._lines[at - 1])
        for b in boundaries:
            assert b > at, ('boundary before header', b, at)
            self.ins.append((b, header, nlines, ind, at))
        return self

    def _shift(self, x):
        return sum(2 + e[2] for e in self.ins if e[0] <= x)

    def start(self, a):
        s = a + self._shift(a)
        for e in self.ins:
            if e[0] == a:
                s -= e[2]
        return s

    def end(self, b):
        e = b + self._shift(b)
        for bb in sorted(x[0] for x in self.ins):
            if bb > b and all(self._lines[k - 1].strip() == '' for k in range(b + 1, bb)):
                return bb + self._shift(b)
        return e

    def _snap(self, a):
        if any(e[0] == a for e in self.ins):
            return a
        while a > 1:
            prev = self._lines[a - 2].lstrip()
            if (prev == '' or prev.startswith(('///', '//', '#['))) and not _re.match(r'//\s?#(end)?region', prev):
                a -= 1
            else:
                break
        return a

    def child(self, emoji, kebab, mod, ranges, doc, **kw):
        base = len(os.path.dirname(self.file).encode('utf-8')) + 1
        fits = 240 - base - len((emoji + V).encode('utf-8')) - len('/🦀️.rs'.encode('utf-8'))
        assert len(kebab.encode('utf-8')) <= fits and len(REPO) + base + len((emoji + V + kebab + '/🦀️.rs')) <= 256, ('kebab too long, at most %d bytes' % fits, kebab)
        mapped = [(self.start(self._snap(a)), self.end(b)) for a, b in ranges]
        self.children.append(ch(emoji, kebab, mod, mapped, doc, **kw))
        return self

    def pre(self):
        ins = list(self.ins)

        def run(text):
            lines = text.split('\n')
            for b, h, nl, ind, at in sorted(ins, key=lambda t: -t[0]):
                assert lines[at - 1] == h or b < at, ('header moved', h)
                hdr = self._lines[at - 1:at - 1 + nl]
                lines[b - 1 + self._shift_before(b, ins, b):b - 1 + self._shift_before(b, ins, b)] = [ind + '}', ''] + hdr
            return '\n'.join(lines)
        return run

    @staticmethod
    def _shift_before(b, ins, _):
        return 0

    def extra(self):
        out = [e[3] + '}' for e in self.ins]
        for e in self.ins:
            out.extend(self._lines[e[4] - 1:e[4] - 1 + e[2]])
        return out

    def spec(self, **kw):
        ext = mounted(self.file, orig=self.orig)
        for s in crate_siblings(self.file):
            if s not in ext:
                ext.append(s)
        sp = dict(orig=self.orig, file=self.file, children=self.children, wiring='block', external=ext)
        if self.ins:
            sp['pre'] = self.pre()
            sp['expect_extra'] = self.extra()
        sp.update(kw)
        return sp


W = U + '/🎯️targets/🧊️wgpu'

R = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
RW = R + '/🎯️targets/🧊️wgpu'


def crate_siblings(file_rel):
    """📦 c7w: every other source file of the owning crates' #[path]/include! closures (text readers of the shared module scope), excluding the split file's own directory subtree."""
    import json
    sys_path = os.path.join(T, 'owners.json')
    if not os.path.exists(sys_path):
        return []
    owners = json.load(open(sys_path, encoding='utf-8'))
    roots = json.load(open(os.path.join(T, 'crateroots.json'), encoding='utf-8'))
    here = None
    tags = json.load(open(os.path.join(T, 'tags.json'), encoding='utf-8'))
    for tag, p in tags.items():
        if p == file_rel:
            here = tag
    if here is None:
        return []
    out = []
    sub = os.path.dirname(file_rel) + '/'
    import crateof
    for name in owners.get(here, []):
        for f in sorted(crateof.closure(roots[name])):
            rel = os.path.relpath(f, REPO)
            if rel == file_rel or rel.startswith(sub) or rel in out:
                continue
            out.append(rel)
    return out
