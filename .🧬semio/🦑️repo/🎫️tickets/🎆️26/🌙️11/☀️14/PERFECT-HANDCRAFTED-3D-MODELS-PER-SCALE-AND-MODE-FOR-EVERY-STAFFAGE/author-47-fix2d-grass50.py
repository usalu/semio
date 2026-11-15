"""Author 47: miscanthus 1:50 elevation plumes were drawn 0.30 m long (bone length) while front, plan, 1:100-1:500 and `dimensions.heightM` use 0.403 m; scale the elevation plume path along the bone by 0.403/0.3."""
import importlib.util, re, sys
spec = importlib.util.spec_from_file_location("l", "author-47-lib2d.py"); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
P = l.path("vegetation", "miscanthus-grass")
t = open(P).read()
K = 0.403 / 0.3
for n in (1, 2, 3, 4):
    s, a, b = l.get_shape(t, "50", "elevation", "plume-%d" % n)
    old = s["d"]
    toks = re.findall(r"[A-Za-z]|-?\d+\.?\d*", old)
    out, xs, nums = [], True, 0
    for tk in toks:
        if re.match(r"[A-Za-z]", tk):
            out.append(tk); nums = 0; continue
        v = float(tk)
        if nums % 2 == 0: v = v * K
        out.append(("%.4f" % v).rstrip("0").rstrip("."))
        nums += 1
    new = " ".join(out)
    if new == old: print("unchanged", n); continue
    seg = t[a:b]
    assert seg.count(old) == 1
    t = t[:a] + seg.replace(old, new) + t[b:]
    print("plume", n, "->", new[:60])
open(P, "w").write(t)
