import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex

REPO = '/Users/ueli/Documents/semio'


def children(path, text):
    toks = [t for t in lex(text) if t[0] != 'comment']
    out = []
    stack = []
    depth = 0
    pending = None
    i = 0
    n = len(toks)
    base = os.path.dirname(path)
    while i < n:
        k, x, sl, el, si = toks[i]
        if k == 'p' and x == '#' and i + 1 < n and toks[i + 1][1] in ('[', '!'):
            j = i + 1
            if toks[j][1] == '!':
                j += 1
            d = 0
            start = j
            while j < n:
                if toks[j][1] == '[':
                    d += 1
                elif toks[j][1] == ']':
                    d -= 1
                    if d == 0:
                        break
                j += 1
            seg = toks[start:j + 1]
            if len(seg) >= 4 and seg[1][1] == 'path' and seg[2][1] == '=' and seg[3][0] == 'str':
                pending = seg[3][1][1:-1]
            i = j + 1
            continue
        if k == 'p' and x == '{':
            depth += 1
        elif k == 'p' and x == '}':
            depth -= 1
            while stack and stack[-1][1] > depth:
                stack.pop()
        elif k == 'id' and x == 'mod' and i + 2 < n and toks[i + 1][0] == 'id':
            name = toks[i + 1][1]
            nxt = toks[i + 2][1]
            if nxt == ';':
                if pending is not None:
                    parts = [s[0] for s in stack]
                    out.append(os.path.normpath(os.path.join(base, *parts, pending)))
                pending = None
            elif nxt == '{':
                stack.append((pending if pending is not None else name, depth + 1))
                pending = None
            else:
                pending = None
        elif k == 'id' and x in ('fn', 'struct', 'enum', 'impl', 'trait', 'const', 'static', 'type'):
            pending = None
        i += 1
    return out


def tree(path, seen=None):
    seen = seen if seen is not None else []
    if path in seen or not os.path.isfile(path) or not path.endswith('.rs'):
        return seen
    seen.append(path)
    text = open(path, encoding='utf-8').read()
    for c in children(path, text):
        tree(c, seen)
    return seen


if __name__ == '__main__':
    for f in tree(sys.argv[1]):
        print(os.path.relpath(f, REPO))
