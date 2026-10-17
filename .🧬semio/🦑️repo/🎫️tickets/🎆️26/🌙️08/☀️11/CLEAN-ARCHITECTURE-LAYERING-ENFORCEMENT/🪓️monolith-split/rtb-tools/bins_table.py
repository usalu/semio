"""📋️ Markdown table of the test files each bin mounts. usage: bins_table.py"""
import re, os
from lib import *
crates = crate_table()
names = [s for s in crates if s.startswith("shell-tests-")] + ["engine-attach-tests", "ui-engine-tests", "runtime-mailbox-tests", "frame-tests-1", "scene-ink-tests"]
MOUNT = re.compile(r'#\[path = "([^"\n]+)"\]\n\s*(?:pub )?mod ([A-Za-z_0-9]+);|include!\("([^"\n]+)"\);')
print("| bin | mounted test files |")
print("|---|---|")
for s in sorted(names):
    root = T + "/" + crates[s] + "/🦀️.rs"
    t = read(root)
    items = []
    for m in MOUNT.finditer(t):
        rel_ = m.group(1) or m.group(3)
        d = os.path.basename(os.path.dirname(rel_)) if os.path.basename(rel_) == "🦀️.rs" else rel_
        items.append(d.replace("🧊️wgpu-standalone", "wgpu-standalone"))
    print(f"| `{s}` | " + ", ".join(f"`{i}`" for i in items) + " |" if items else f"| `{s}` | (skeleton) |")
