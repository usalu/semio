import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
M = '🧰️framework/🔨️modules'
B3 = M + '/🧊️3d/📐️brep'
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


def split_impl(header, boundaries, nlines=1, at=None):
    def pre(text):
        lines = text.split('\n')
        hs = [i for i, l in enumerate(lines) if l == header]
        if at is not None:
            assert lines[at - 1] == header, ('impl header not at line', at, lines[at - 1])
            hs = [at - 1]
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


def crate_files(crate_root, own):
    base = os.path.dirname(own) + '/'
    return [f for f in mounted(crate_root) if f != own and not f.startswith(base)]


def multi_pre(*pres):
    def pre(text):
        for p in pres:
            text = p(text)
        return text
    return pre


def close_regions(before_lines, at_eof=()):
    def pre(text):
        lines = text.split('\n')
        for ln, close_lines in sorted(before_lines.items(), reverse=True):
            lines[ln - 1:ln - 1] = list(close_lines) + ['']
        if at_eof:
            assert lines[-1] == '', 'file must end with a newline'
            lines[-1:-1] = list(at_eof)
        return '\n'.join(lines)
    return pre
