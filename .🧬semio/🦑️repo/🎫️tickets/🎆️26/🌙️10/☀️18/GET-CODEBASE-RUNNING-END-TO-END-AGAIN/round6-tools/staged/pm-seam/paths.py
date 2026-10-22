"""🧭️ Shared path vocabulary of the pm-seam tools (standard library only)."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUILD = LIBRARY + "/🏗️build"
DEPENDENCIES = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies"
MANAGERS = DEPENDENCIES + "/🔌️managers"
import sys
def _round():
    if "--round" in sys.argv:
        return int(sys.argv[sys.argv.index("--round") + 1])
    return int(os.environ.get("PM_ROUND", "1"))
ROUND = _round()
STAGE = os.path.join(HERE, "staged" if ROUND == 1 else "staged%d" % ROUND)
NEW = os.path.join(STAGE, "new")

# ---- staged new files: the shared ticket tool `winpath` stores targets whose repository path Windows cannot create as short aliases under staged/new/w with a map
import sys
sys.path.insert(0, C)
import winpath
def new_map():
    return winpath.iter_logical(NEW)
def new_read(relative):
    return open(winpath.resolve(NEW, relative), encoding="utf-8", newline="").read()
def new_write(relative, text):
    return winpath.place(NEW, relative, text.encode("utf-8"))

# ---- round 2 works in a mirror: generators write there when PM_MIRROR is set, and `cut2.py` turns the differences against the round-1 mirror into staged2 rewrites and new files
MIRROR = os.environ.get("PM_MIRROR")
def seam_write(relative, text):
    if MIRROR:
        target = os.path.join(MIRROR, relative)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf-8", newline="") as handle:
            handle.write(text)
        return target
    return new_write(relative, text)
def seam_read(relative):
    if MIRROR:
        return open(os.path.join(MIRROR, relative), encoding="utf-8", newline="").read()
    return new_read(relative)
