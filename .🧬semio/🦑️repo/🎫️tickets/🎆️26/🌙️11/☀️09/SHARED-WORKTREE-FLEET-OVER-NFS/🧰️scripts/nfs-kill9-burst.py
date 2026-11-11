#!/usr/bin/env python3
"""usage: nfs-kill9-burst.py TREE ROUND NSMALL NLARGE -- create TREE/nfs-kill9/rROUND with NSMALL small files and NLARGE random 2 MiB files (all names and bytes derive from ROUND, so the expected tree is reproducible); prints the epoch when it finished."""
import os, random, sys, time

tree, rnd, nsmall, nlarge = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
d = os.path.join(tree, "nfs-kill9", "r%d" % rnd); os.makedirs(d, exist_ok=True)
rng = random.Random(rnd)
for i in range(nlarge):
    with open(os.path.join(d, "large%02d.bin" % i), "wb") as f: f.write(rng.randbytes(2 * 1024 * 1024))
for i in range(nsmall):
    with open(os.path.join(d, "s%04d.txt" % i), "w") as f: f.write("kill9 %d %d %s\n" % (rnd, i, "y" * rng.randrange(1, 300)))
print(time.time())
