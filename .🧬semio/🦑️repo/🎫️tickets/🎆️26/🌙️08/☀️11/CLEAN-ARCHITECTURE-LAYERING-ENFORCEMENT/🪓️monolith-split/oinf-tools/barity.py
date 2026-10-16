"""📐 Arity scan of the board carve: new crates plus rewritten external files against their definitions, compared with the baseline tree."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import warity

def rs(root):
    out = []
    for dp, dn, fn in os.walk(root):
        if 'node_modules' in dp: continue
        for f in fn:
            if f.endswith('.rs'): out.append(os.path.join(dp, f))
    return out

def run(files):
    return warity.check(files)

if __name__ == '__main__':
    new_inf, ext, snap = sys.argv[1:4]
    nf = [p for p in rs(os.path.join(new_inf, '🎲️board'))] + rs(ext)
    nb = run(nf)
    sf = rs(os.path.join(snap, 'INF', '🎲️board')) if os.path.isdir(os.path.join(snap, 'INF')) else rs(snap)
    print('new tree files', len(nf), 'mismatches', len(nb))
    for p, line, name, a, exp in nb[:60]: print(p[-90:], line, name, 'args', a, 'expected', exp)

def key(b):
    return (b[2], b[3], tuple(b[4]))

def compare(new_inf, ext, snap_board, live_root):
    import collections
    nf = rs(os.path.join(new_inf, '🎲️board')) + rs(ext)
    sf = rs(snap_board)
    for dp, dn, fn in os.walk(ext):
        for f in fn:
            rel = os.path.relpath(os.path.join(dp, f), ext)
            sf.append(os.path.join(live_root, rel))
    nb = collections.Counter(key(b) for b in run(nf))
    sb = collections.Counter(key(b) for b in run(sf))
    extra = nb - sb
    gone = sb - nb
    print('new', sum(nb.values()), 'base', sum(sb.values()), 'extra', sum(extra.values()), 'gone', sum(gone.values()))
    for k, v in sorted(extra.items(), key=lambda x: str(x[0])): print('EXTRA', k, v)
    for k, v in sorted(gone.items(), key=lambda x: str(x[0])): print('GONE', k, v)
    return extra
