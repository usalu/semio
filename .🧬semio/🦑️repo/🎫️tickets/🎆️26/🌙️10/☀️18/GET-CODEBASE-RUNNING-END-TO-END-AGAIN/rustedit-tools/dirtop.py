"""🗺 Streams a directory walk with an exclusion set and prints directory counts per top-level / second-level entry (largest first)."""
import os, sys, time, resource
from collections import Counter
root = sys.argv[1]; mode = sys.argv[2]
BAZEL = {".git", ".🧬semio", ".cache", ".nx", "node_modules", "target", "dist", "build", "coverage", ".venv", "__pycache__"}
PAYLOAD = {"target", "node_modules", "dist", "🗑️generated", "📦️packages", "⚡️cache", "bin", "obj", "out", "pkg", "coverage", "build"}
def skip(n): return n in BAZEL or (mode == "payload" and (n in PAYLOAD or n.startswith(".")))
t = time.time(); c = Counter(); n = 0; stack = [(root, "")]
while stack:
    d, key = stack.pop(); n += 1; c[key] += 1
    if n % 4096 == 0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 2**20: sys.exit("rss")
    try:
        with os.scandir(d) as it:
            for e in it:
                if e.is_dir(follow_symlinks=False) and not skip(e.name):
                    parts = key.split("/") if key else []
                    stack.append((e.path, key if len(parts) >= 2 else (key + "/" + e.name if key else e.name)))
    except OSError: pass
print(f"mode={mode} dirs={n} wall={time.time()-t:.1f}s")
for k, v in c.most_common(25): print(f"{v:8d} {k}")
