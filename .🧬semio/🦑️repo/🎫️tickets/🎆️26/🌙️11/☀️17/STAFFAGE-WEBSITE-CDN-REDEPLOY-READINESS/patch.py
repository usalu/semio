import sys
def patch_plan(path, scales, old, new, view="plan", stop='"canvas'):
    t = open(path).read()
    for sc in scales:
        s = t.index(f'      "{sc}": {{')
        p = t.index(f'"{view}": [', s)
        e = t.index(stop, p)
        seg = t[p:e]
        assert old in seg, (sc, old)
        t = t[:p] + seg.replace(old, new) + t[e:]
    open(path, "w").write(t)
if __name__ == "__main__":
    patch_plan(sys.argv[1], sys.argv[2].split(","), sys.argv[3], sys.argv[4], sys.argv[5], sys.argv[6])
