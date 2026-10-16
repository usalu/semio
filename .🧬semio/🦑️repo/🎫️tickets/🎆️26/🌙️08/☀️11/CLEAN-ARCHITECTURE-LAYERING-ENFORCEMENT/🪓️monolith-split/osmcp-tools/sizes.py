import sys
sys.path.insert(0, ".")
from oracle2 import *
lay, em, files = generate(sys.argv[1], False)
tot = collections.defaultdict(lambda: [0, 0, 0])
for o in lay.outs:
    t = files[o.path]
    r = tot[o.crate]; r[0] += t.count("\n") + 1; r[1] += len(t.encode()); r[2] += 1
for ci, n in enumerate(lay.names):
    r = tot[ci]
    lr = files[(node_dir(lay, ci) + "/🦀️.rs") if node_dir(lay, ci) else os.path.join(MCP, "📦️packages", "🦀️rust", "🦀️.rs")]
    print(f"{ci:2d} {n:16s} files={r[2]+1:3d} lines={r[0]+lr.count(chr(10)):6d} bytes={r[1]+len(lr.encode()):7d}")
