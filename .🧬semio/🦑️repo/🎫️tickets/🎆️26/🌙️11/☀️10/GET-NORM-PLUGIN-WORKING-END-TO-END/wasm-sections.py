"""📦️ Sums a wasm component's section bytes by kind, descending into nested core modules, and names custom sections."""
import sys
from collections import Counter

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7f) << s; s += 7
        if x < 0x80: return r, i

def walk(b, start, end, acc, depth):
    i = start + 8
    while i < end:
        sid = b[i]; size, j = leb(b, i + 1); k = j + size
        if sid == 0:
            n, p = leb(b, j); acc[f"custom:{b[p:p+n].decode(errors='replace')}"] += size
        elif depth == 0 and sid == 1:
            walk(b, j, k, acc, 1)
        else:
            acc[f"{'core' if depth else 'component'}:{sid}"] += size
        i = k

data = open(sys.argv[1], "rb").read()
acc = Counter(); walk(data, 0, len(data), acc, 0)
print(len(data))
for k, v in acc.most_common(12): print(v, k)
