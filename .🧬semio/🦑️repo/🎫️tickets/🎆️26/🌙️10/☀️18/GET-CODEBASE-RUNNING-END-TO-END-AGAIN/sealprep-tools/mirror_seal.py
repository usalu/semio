#!/usr/bin/env python3
"""🪞 [sealprep] `mirror.py run` defaults SEMIO_REPO_SEAL=live (audit11's patch, as one anchored edit instead of a hashed whole file, because other lanes keep editing mirror-tools/mirror.py): a mirror carries no sealed store, so its first `bun repo` promotes its own snapshot and, in sealed mode, a batch applied to the mirror AFTER that first command would never be the code that runs (a silent false green). An explicit SEMIO_REPO_SEAL in the caller's environment still wins.
usage: mirror_seal.py apply|revert|status [--file PATH]   (default file: ../mirror-tools/mirror.py next to this script; exit 0 done/ok, 3 already in the wanted state, 2 anchor problem; keeps the mode; atomic)"""
import os, stat, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
path = os.path.abspath(sys.argv[sys.argv.index("--file") + 1]) if "--file" in sys.argv else os.path.join(os.path.dirname(HERE), "mirror-tools", "mirror.py")
verb = next((a for a in sys.argv[1:] if a in ("apply", "revert", "status")), "status")
ANCHOR = "    env = dict(os.environ, SEMIO_MIRROR_ROOT=root, SEMIO_MIRROR_NAME=name)\n"; LINE = '    env.setdefault("SEMIO_REPO_SEAL", "live")\n'
text = open(path, encoding="utf8").read(); has = ANCHOR + LINE in text
if verb == "status": print("applied" if has else "base"); sys.exit(0 if has else 1)
if verb == "apply" and has or verb == "revert" and not has: print("already", "applied" if has else "base"); sys.exit(3)
if text.count(ANCHOR) != 1: print("anchor occurs %d times" % text.count(ANCHOR)); sys.exit(2)
new = text.replace(ANCHOR, ANCHOR + LINE) if verb == "apply" else text.replace(ANCHOR + LINE, ANCHOR)
mode = stat.S_IMODE(os.stat(path).st_mode); fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".seal-")
with os.fdopen(fd, "w", encoding="utf8") as handle: handle.write(new)
os.chmod(tmp, mode); os.replace(tmp, path); print("ok", verb, path)
