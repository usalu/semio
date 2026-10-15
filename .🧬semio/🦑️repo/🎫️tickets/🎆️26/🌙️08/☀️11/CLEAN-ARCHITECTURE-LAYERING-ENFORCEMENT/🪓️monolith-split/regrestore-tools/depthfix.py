import sys, os, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from touched import *

def fix_file(path, apply):
    t = read(path)
    T = Tfile(path, t)
    lang = 'rs' if path.endswith('.rs') else 'ts'
    grp, rel, ds, de = gapinfo(T.code, lang)
    if not rel:
        return [('unreliable', path)]
    def depth_at(n):
        k = bisect.bisect_left(T.fidx, n)
        return 0 if k >= len(T.fidx) else ds[k], k
    stack = []
    moves = []
    notes = []
    for n, kind, name in T.marks:
        if kind == 'open':
            stack.append((n, name, depth_at(n)[0]))
            continue
        k = None
        for q in range(len(stack) - 1, -1, -1):
            if name == '' or stack[q][1] == name:
                k = q
                break
        if k is None:
            continue
        on, onm, od = stack[k]
        cd, ci = depth_at(n)
        del stack[k:]
        if cd > od:
            c = ci
            while c < len(T.code) and not (de[c] == od):
                c += 1
            if c >= len(T.code):
                notes.append(('no target', onm, on + 1, n + 1))
                continue
            target = T.fidx[c] + 1
            between = [m for m in T.mark_lines if n < m < target]
            if between:
                notes.append(('markers between', onm, on + 1, n + 1, len(between)))
                continue
            moves.append((n, target, T.lines[n]))
            notes.append(('move close', onm, on + 1, n + 1, '->', target + 1))
    if apply and moves:
        lines = list(T.lines)
        ins = {}
        dele = set()
        for n, target, text in moves:
            dele.add(n)
            ins.setdefault(target, []).append(text)
        out = []
        for i, l in enumerate(lines):
            if i in ins:
                out.extend(ins[i])
            if i in dele:
                continue
            out.append(l)
        if len(lines) in ins:
            out.extend(ins[len(lines)])
        new = '\n'.join(out)
        assert strip_markers(new) == strip_markers(t)
        assert read(path) == t
        open(path, 'w', encoding='utf-8', newline='').write(new)
    return notes

if __name__ == '__main__':
    apply = sys.argv[1] == 'apply'
    ids = sys.argv[2:]
    for sid in ids:
        for p in touched(sid):
            notes = fix_file(os.path.join(ROOT, p), apply)
            for nn in notes:
                print(sid, p[-60:], nn)
