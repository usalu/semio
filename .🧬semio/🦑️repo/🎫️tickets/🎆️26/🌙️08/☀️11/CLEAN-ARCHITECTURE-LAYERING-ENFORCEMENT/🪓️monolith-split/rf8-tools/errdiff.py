"""Shows the symmetric difference of two oracle err files: errdiff.py <A.err> <B.err>."""
import collections, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
import oracle
ka, kb = oracle.parse(read(sys.argv[1])), oracle.parse(read(sys.argv[2]))
ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
for (h, m, l), n in sorted((cb - ca).items()):
    print(f"+B {n}x {h}: {m[:300]} | {l[:200]}")
for (h, m, l), n in sorted((ca - cb).items()):
    print(f"-B {n}x {h}: {m[:300]} | {l[:200]}")
