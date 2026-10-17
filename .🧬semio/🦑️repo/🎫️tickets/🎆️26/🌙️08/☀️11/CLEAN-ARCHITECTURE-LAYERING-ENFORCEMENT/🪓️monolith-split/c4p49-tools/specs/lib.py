import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
R = '🧰️framework/🔨️modules'
TML = R + '/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit'
ACT = R + '/🛂️manifest/🕹️actions'
ARG = R + '/🛂️manifest/🧮️arg-schema'
APP = R + '/🛂️manifest/🧱️app-surface'
TMU = R + '/🛠️tool-machine/🧪️tests/🔬️unit'
CFG = R + '/🧮️math/🎯️sampling/🎚️config'
VOC = R + '/🧮️math/🎯️sampling/🔤️vocabulary'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)

REPO = '/Users/ueli/Documents/semio'
import re as _re
_PATH = _re.compile(r'#\[path\s*=\s*"([^"]+)"\]')


def mounted(file_rel, glue=None):
    seen = []
    todo = [file_rel]
    first = True
    while todo:
        cur = todo.pop(0)
        p = os.path.join(REPO, cur)
        if not os.path.isfile(p):
            continue
        if not first:
            if cur in seen:
                continue
            seen.append(cur)
        first = False
        text = open(p, encoding='utf-8').read()
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


def split_impls(cuts):
    """cuts: [(first_line, open_line, [member_first_line, ...])] 1-based lines of the ORIGINAL text; returns (pre, extras).
    pre(text) closes the impl block before every listed member and reopens it with the verbatim header (attributes included)."""
    def pre(text):
        ls = text.split('\n')
        edits = []
        for first, opn, members in cuts:
            assert ls[opn - 1].rstrip().endswith('{'), ('impl header does not end with a brace', opn, ls[opn - 1][:80])
            header = ls[first - 1:opn]
            for m in members:
                edits.append((m, header))
        for m, header in sorted(edits, key=lambda e: -e[0]):
            idx = m - 1
            if ls[idx - 1].strip() == '':
                del ls[idx - 1]
                idx -= 1
            ls[idx:idx] = ['}', ''] + header
        return '\n'.join(ls)
    return pre


def impl_extras(orig_text, cuts):
    ls = orig_text.split('\n')
    out = []
    for first, opn, members in cuts:
        header = [l.strip() for l in ls[first - 1:opn]]
        for _ in members:
            out.append('}')
            out.extend(h for h in header if h)
    return out
