import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from touched import *

def scan(path):
    T = Tfile(path)
    lang = 'rs' if path.endswith('.rs') else 'ts'
    grp, rel, ds, de = gapinfo(T.code, lang)
    if not rel:
        return None, []
    # map marker -> depth at gap: depth before the next code line
    import bisect
    def depth_at(n):
        k = bisect.bisect_left(T.fidx, n)
        if k >= len(T.fidx):
            return 0
        return ds[k]
    stack = []
    bad = []
    for n, kind, name in T.marks:
        if kind == 'open':
            stack.append((n, name, depth_at(n)))
        else:
            k = None
            for q in range(len(stack) - 1, -1, -1):
                if name == '' or stack[q][1] == name:
                    k = q
                    break
            if k is None:
                continue
            on, onm, od = stack[k]
            cd = depth_at(n)
            if od != cd:
                bad.append((onm[:30], on + 1, od, n + 1, cd))
            del stack[k:]
    return T, bad

if __name__ == '__main__':
    ids = sys.argv[1:] or list(SPECS)
    tot = 0
    for sid in ids:
        files = touched(sid)
        nb = 0
        out = []
        for p in files:
            T, bad = scan(os.path.join(ROOT, p))
            if T is None:
                out.append((p[-60:], 'lexer unreliable'))
            for b in bad:
                nb += 1
                out.append((p[-60:], b))
        tot += nb
        print(sid, 'touched', len(files), 'depth-mismatched pairs', nb)
        for o in out[:10]:
            print('   ', o)
    print('total', tot)
