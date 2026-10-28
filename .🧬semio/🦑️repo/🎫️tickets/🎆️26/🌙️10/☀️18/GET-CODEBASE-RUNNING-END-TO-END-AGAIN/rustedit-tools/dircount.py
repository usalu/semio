"""📏 Counts the directories a whole-repository source-universe observation would visit (Bazel exclusions), streaming, with wall time."""
import os, sys, time, resource
EX = {".git", ".🧬semio", ".cache", ".nx", "node_modules", "target", "dist", "build", "coverage", ".venv", "__pycache__"}
root = sys.argv[1]; t = time.time(); dirs = 0; gi = 0; stack = [root]
while stack:
    d = stack.pop(); dirs += 1
    if dirs % 4096 == 0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 2**20: sys.exit("rss")
    try:
        with os.scandir(d) as it:
            for e in it:
                if e.name == ".gitignore": gi += 1
                if e.is_dir(follow_symlinks=False) and e.name not in EX: stack.append(e.path)
    except OSError: pass
print(f"dirs={dirs} gitignores={gi} wall={time.time()-t:.1f}s")
