"""🎨️ rustfmt of the bin roots written by the rtb tools: mount declarations are swapped for inline stubs so only the root text is formatted. usage: fmt_roots.py [--apply]"""
import re, subprocess, sys, tempfile
from lib import *
APPLY = "--apply" in sys.argv
crates = crate_table()
names = [s for s in crates if s.startswith("shell-tests-") or s in ("engine-attach-tests", "ui-engine-tests", "runtime-mailbox-tests", "frame-tests-1", "scene-ink-tests", "fixtures")]
MOUNT = re.compile(r'((?:[ \t]*#\[cfg[^\n]*\]\n)*)([ \t]*)#\[path = "([^"\n]+)"\]\n[ \t]*((?:pub )?mod [A-Za-z_0-9]+);')
bad = 0
for s in names:
    p = T + "/" + crates[s] + "/🦀️.rs"
    t = read(p)
    store = []
    def stub(m):
        store.append(m.group(0).strip("\n"))
        return f"{m.group(2)}// __MOUNT{len(store) - 1}__"
    inner = MOUNT.sub(stub, t)
    with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False, encoding="utf8", dir=SCRATCH) as f:
        f.write(inner)
        tmp = f.name
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--style-edition", "2024", "--config-path", REPO + "/rustfmt.toml", tmp], capture_output=True, text=True)
    if r.returncode:
        print(s, "rustfmt error", r.stderr[:200])
        continue
    out = read(tmp)
    out = re.sub(r"(?m)^[ \t]*// __MOUNT(\d+)__", lambda m: store[int(m.group(1))], out)
    if out != t:
        bad += 1
        print("DIFF", s)
        if APPLY:
            edit(p, out)
print("roots", len(names), "needing format", bad)
