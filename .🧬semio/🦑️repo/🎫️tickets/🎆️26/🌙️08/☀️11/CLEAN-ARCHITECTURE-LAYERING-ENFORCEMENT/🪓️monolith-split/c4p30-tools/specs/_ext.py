import os, re
R = '/Users/ueli/Documents/semio'
RX = re.compile(r'#\[path\s*=\s*"([^"]+)"\s*\]')


def mounted(rel, exclude=()):
    seen = []
    todo = [rel]
    while todo:
        cur = todo.pop()
        if cur in seen or not os.path.isfile(os.path.join(R, cur)):
            continue
        seen.append(cur)
        text = open(os.path.join(R, cur), encoding='utf-8', errors='ignore').read()
        base = os.path.dirname(cur)
        for m in RX.finditer(text):
            p = m.group(1)
            if p == '.' or not p.endswith('.rs'):
                continue
            todo.append(os.path.normpath(os.path.join(base, p)))
    return [s for s in sorted(seen) if s != rel and s not in exclude]
