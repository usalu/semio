import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
FILES = dict(l.rstrip('\n').split('\t') for l in open(os.path.join(T, 'files.tsv'), encoding='utf-8'))
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


def empties(tag):
    lines = open(os.path.join(T, 'orig', tag + '.rs'), encoding='utf-8').read().split('\n')
    out = []
    for i in range(len(lines) - 1):
        m = _re.match(r'^//\s?#region (.*)$', lines[i].strip())
        if not m:
            continue
        j = i + 1
        while j < len(lines) and lines[j].strip() == '':
            j += 1
        if j < len(lines) and _re.match(r'^//\s?#endregion ' + _re.escape(m.group(1)) + r'$', lines[j].strip()):
            out.append(m.group(1))
    return tuple(out)
