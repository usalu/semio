import sys, av, numpy as np

src = sys.argv[1]
x0, y0, x1, y1 = map(int, sys.argv[2:6])
counts = []
with av.open(src) as c:
    for f in c.decode(video=0):
        a = f.to_ndarray(format="rgb24")[y0:y1, x0:x1].astype(int)
        spread = a.max(2) - a.min(2)
        counts.append(int(((spread < 18) & (a.mean(2) > 70) & (a.mean(2) < 215)).sum()))
n = np.array(counts)
print("frames", len(n), "min", n.min(), "mean", int(n.mean()), "max", n.max())
d = np.abs(np.diff(n))
idx = np.argsort(d)[-5:][::-1]
print("largest jumps", [(int(i), int(n[i]), int(n[i + 1])) for i in idx])
nz = np.nonzero(n)[0]; print("first nonzero", int(nz[0]), "zeros after", int((n[nz[0]:] == 0).sum()), "min after", int(n[nz[0]:].min()))
print("low", [int(i) for i in np.nonzero(n < 40)[0] if i > nz[0] + 30][:30])
