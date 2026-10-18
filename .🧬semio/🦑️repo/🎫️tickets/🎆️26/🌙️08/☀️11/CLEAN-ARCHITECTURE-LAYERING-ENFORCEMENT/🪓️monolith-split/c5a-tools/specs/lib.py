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


def cut_impl(header_line, cuts):
    """pre-edit factory: splits the inherent `impl` whose header is at 1-based `header_line` into several `impl` blocks of the same header, closing and re-opening it before each 1-based line in `cuts` (the first line of a method's doc block)."""
    def pre(text):
        ls = text.split('\n')
        header = ls[header_line - 1]
        assert header.startswith('impl<') and header.rstrip().endswith('{'), header
        for c in sorted(cuts, reverse=True):
            assert ls[c - 1].startswith('    '), (c, ls[c - 1])
            ls[c - 1:c - 1] = ['}', '', header]
        return '\n'.join(ls)
    return pre


def cut_extra(header_line, cuts, text):
    header = text.split('\n')[header_line - 1]
    return ['}'] * len(cuts) + [header] * len(cuts)
