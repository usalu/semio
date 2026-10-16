import sys; sys.path.insert(0, '.')
from comps6 import *

def split_top(text_masked, text_raw, start, end, seps=','):
    """split text_raw[start:end] at top-level separators (depth over ( [ { and angle brackets in type position are ignored: separators inside <> are not split because init expressions rarely use top-level commas inside generics)"""
    parts = []; d = 0; s = start; prev = ''
    for i in range(start, end):
        c = text_masked[i]
        if c in '([{': d += 1
        elif c in ')]}': d -= 1
        elif c == ',' and d == 0:
            parts.append((s, i)); s = i + 1
        prev = c
    parts.append((s, end))
    return [(a, b) for a, b in parts if text_raw[a:b].strip()]

def find_literal(raw, head='let mut this = Self {'):
    M = mask(raw)
    i = raw.index(head)
    ob = i + len(head) - 1
    d = 0; j = ob
    while j < len(M):
        if M[j] == '{': d += 1
        elif M[j] == '}':
            d -= 1
            if d == 0: break
        j += 1
    return M, ob, j

def parse_literal(raw, head='let mut this = Self {'):
    M, ob, cb = find_literal(raw, head)
    ents = []
    for a, b in split_top(M, raw, ob + 1, cb):
        seg = raw[a:b]; sm = M[a:b]
        # leading attrs / comments
        m = re.match(r'((?:\s*(?:#\[[^\]]*\]|//[^\n]*))*)\s*(\w+)\s*(:\s*)?', seg, re.S)
        attrs = m.group(1).strip(); name = m.group(2)
        init = seg[m.end():].strip() if m.group(3) else None
        ents.append(dict(attrs=attrs, name=name, init=init, raw=seg.strip()))
    return ents, ob, cb

if __name__ == '__main__':
    raw = open(PLUG + '🧵️instance/📥️staging/🚪️ingress/🦀️.rs', encoding='utf-8').read()
    ents, ob, cb = parse_literal(raw)
    print(len(ents), 'entries; first', ents[:3])
    ft = field_table()
    names = [e['name'] for e in ents]
    missing = set(ft) - set(names); extra = set(names) - set(ft)
    print('missing', missing, 'extra', extra)
