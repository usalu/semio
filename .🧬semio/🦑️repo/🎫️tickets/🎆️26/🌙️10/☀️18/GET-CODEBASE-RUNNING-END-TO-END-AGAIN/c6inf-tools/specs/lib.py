import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
FLOW = '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow'
MOD = '🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board'
NRM = MOD + '/🔌️ports/➡️directed/➕️normal'
DAG = MOD + '/🔌️ports/➡️directed/🕸️dag'
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


_MARK = _re.compile(r'^\s*//\s?#(end)?region\b')
_END = _re.compile(r'^\s*//\s?#endregion\b')


def cut_impls(text, cuts):
    lines = text.split('\n')
    extra = []
    for cut in cuts:
        anchor, header = cut[0], cut[1]
        keep = len(cut) > 2 and cut[2] == 'keep'
        hits = [i for i, l in enumerate(lines) if l.strip().startswith(anchor)]
        assert len(hits) == 1, ('anchor not unique', anchor, len(hits))
        L = hits[0]
        j = L
        while j - 1 >= 0 and lines[j - 1].strip() != '' and (lines[j - 1].lstrip().startswith(('//', '#[')) ):
            j -= 1
        block = lines[j:L]
        marks = [b.strip() for b in block if _MARK.match(b)]
        rest = [b for b in block if not _MARK.match(b)]
        k = j
        endmarks = []
        while True:
            while k - 1 >= 0 and lines[k - 1].strip() == '':
                k -= 1
            if not keep and k - 1 >= 0 and _END.match(lines[k - 1]):
                endmarks.insert(0, lines[k - 1].strip())
                k -= 1
                continue
            break
        new = ['}'] + endmarks + [''] + marks + [header] + rest
        lines[k:L] = new
        extra += ['}', header]
    return '\n'.join(lines), extra
