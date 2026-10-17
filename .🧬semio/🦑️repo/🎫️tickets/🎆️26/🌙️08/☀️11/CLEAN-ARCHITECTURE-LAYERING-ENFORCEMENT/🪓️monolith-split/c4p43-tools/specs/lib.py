import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
M = '🧰️framework/🛍️products/💻️os/🔨️modules'
INF = M + '/💡️inference'
DIR = M + '/📇️directory/🧬️schema'
HCD = M + '/📡️spr/📜️history/⚙️codec'
PLW = M + '/📡️spr/🧪️tests/⚖️protocol-laws'
CCD = M + '/📡️spr/🧵️channel/🗜️codec'
CDC = M + '/📡️spr/🧵️channel/🧩️command-decode'
CUT = M + '/📡️spr/🧵️channel/🧪️tests/🔬️unit'


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
