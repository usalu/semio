"""📏 Audits the tracked paths against the 256 limit of AGENTS.md in the unit Windows counts (UTF-16 code units: an emoji outside the Basic Multilingual Plane is two): python3 -I long-paths.py [--root DIR] [--limit 256]. Exit 1 when a tracked path is longer; prints the paths, the longest source path and the room left for a checkout prefix under MAX_PATH (260 including the terminating NUL). Read-only (`git ls-files`)."""
import os, subprocess, sys
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.getcwd()
limit = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else 256
units = lambda text: len(text.encode("utf-16-le", "surrogatepass")) // 2
paths = [row.decode("utf8", "surrogateescape") for row in subprocess.run(["git", "-C", root, "ls-files", "-z"], capture_output=True, check=True).stdout.split(b"\0") if row]
over = sorted(((units(path), path) for path in paths if units(path) > limit), reverse=True)
source = max((units(path) for path in paths if not path.startswith(".🧬semio/")), default=0)
for count, path in over: print(count, path)
print("tracked paths: %d, over %d: %d, longest source path: %d units, room for the checkout prefix under MAX_PATH 260: %d units" % (len(paths), limit, len(over), source, 259 - source))
sys.exit(1 if over else 0)
