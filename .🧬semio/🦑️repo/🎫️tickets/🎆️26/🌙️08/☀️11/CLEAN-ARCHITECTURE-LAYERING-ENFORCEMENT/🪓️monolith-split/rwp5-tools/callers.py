# 📞️ External callers of WP5 methods: which non-Shell source files call a method homed in a WP5 crate (they must switch to the bundle API).
import os, re, subprocess, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import model as M
home, counts = M.appendix_b(); ms = M.scan()
WP5 = {"shell-session", "shell-window-actions", "shell-actions", "shell-pointer", "shell-view-paint", "shell-view-chrome", "shell-view-overlays", "shell-view-panels"}
names = sorted({m["name"] for m in ms if home.get(m["name"], ["?"])[0] in WP5} - {"new", "boot", "debug_log"})
ENG = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
paths = [ENG + "/🎯️targets", ENG + "/🧱️elements", ENG + "/🧪️tests"]
rx = "[.:](" + "|".join(names) + ")[[:space:]]*(::<[^>]*>)?[(]"
out = subprocess.run(["git", "-c", "core.quotepath=false", "grep", "-n", "-E", rx, "--"] + paths, cwd=M.S.ROOT, capture_output=True, text=True).stdout
by = collections.defaultdict(list)
for line in out.split("\n"):
    if not line: continue
    if "error: daemon" in line: continue
    f, ln, txt = line.split(":", 2)
    if "/🐚️Shell/🎯️targets/" in f or txt.lstrip().startswith("//"): continue
    for m in re.finditer(r"[.:](" + "|".join(names) + r")\b *(::<[^>]*>)?\(", txt):
        by[f].append((int(ln), m.group(1)))
tot = 0
for f, v in sorted(by.items()):
    print(len(v), f.replace(ENG + "/", "E/"), sorted({n for _, n in v}))
    tot += len(v)
print(tot, "call sites in", len(by), "files")
