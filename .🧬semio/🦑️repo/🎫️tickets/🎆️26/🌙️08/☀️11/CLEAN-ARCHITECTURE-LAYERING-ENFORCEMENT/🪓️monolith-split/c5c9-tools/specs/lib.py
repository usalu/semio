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


def split_impl(header, boundaries):
    def pre(text):
        lines = text.split('\n')
        hs = [i for i, l in enumerate(lines) if l == header]
        assert len(hs) == 1, ('impl header not unique', header, hs)
        for b in sorted(boundaries, reverse=True):
            assert lines[b - 2].strip() == '' or True
            lines[b - 1:b - 1] = ['}', '', header]
        return '\n'.join(lines)
    return pre


def split_extra(header, boundaries):
    return ['}'] * len(boundaries) + [header] * len(boundaries)


LY = '✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
OD = '🧰️framework/🛍️products/💻️os/🔨️modules'


def split_impl_ln(header_line, boundaries):
    def pre(text):
        lines = text.split('\n')
        header = lines[header_line - 1]
        assert header.startswith('impl') and header.rstrip().endswith('{'), ('impl header', header_line, header)
        for b in sorted(boundaries, reverse=True):
            assert header_line < b, (header_line, b)
            lines[b - 1:b - 1] = ['}', '', header]
        return '\n'.join(lines)
    return pre


def split_extra_ln(orig_name, header_line, boundaries):
    header = open(os.path.join(T, 'orig', orig_name), encoding='utf-8').read().split('\n')[header_line - 1]
    return ['}'] * len(boundaries) + [header] * len(boundaries)


def compose_pre(*fns):
    def pre(text):
        for f in fns:
            text = f(text)
        return text
    return pre


def folders_of(sp):
    return [c['folder'] for c in sp['children']] + [u['folder'] for u in sp.get('unwrap', [])]


def mods_of(sp):
    return [c['mod'] for c in sp['children']] + [u['mod'] for u in sp.get('unwrap', [])]


def unwrap(emoji, kebab, mod, doc):
    return dict(folder=emoji + V + kebab, mod=mod, doc='//! %s%s %s See [`super`].' % (emoji, V, doc))
