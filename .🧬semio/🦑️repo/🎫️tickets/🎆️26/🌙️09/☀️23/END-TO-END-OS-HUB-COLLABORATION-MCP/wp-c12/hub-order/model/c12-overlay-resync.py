"""🔁️ C12: re-syncs overlay `s14-c12-overlay-order` to the CURRENT hub-order set without touching any other overlay file: for every
file the current set changes, checks overlay == live + <previous set> (no drift since the overlay was cut) and writes live +
<current set>; a drifted file the amendment does not change stays as it is. usage: python3 c12-overlay-resync.py <previous sections dir> [--write]"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
DRIVER = os.path.join(HERE, "..", "c12-hub-order-patch.py")
CURRENT = os.path.join(HERE, "..", "sections")
LIVE = "/Users/ueli/Documents/semio"
OVERLAY = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c12-overlay-order"


def compose(sections_dir):
    source = open(DRIVER, encoding="utf-8").read()
    source = source.replace('os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections"', 'os.path.join(' + repr(sections_dir) + ', ""')
    source = source.replace('for line in plan:', 'for line in []:')
    scope = {"__file__": DRIVER, "__name__": "c12_compose"}
    os.environ["C12_REPO"] = LIVE
    argv, sys.argv = sys.argv, ["compose"]
    exec(compile(source, "compose", "exec"), scope)
    sys.argv = argv
    return {rel: text for rel, text in scope["files"].items() if text is not None}, scope["problems"]


previous_dir = sys.argv[1]
previous, previous_problems = compose(previous_dir)
current, current_problems = compose(CURRENT)
print("previous problems", previous_problems, "current problems", current_problems)
drift, changed = [], []
for rel, text in current.items():
    overlay_path = os.path.join(OVERLAY, rel)
    overlay_text = open(overlay_path, encoding="utf-8").read() if os.path.exists(overlay_path) else None
    if overlay_text != previous.get(rel):
        drift.append(rel)
    elif overlay_text != text:
        changed.append(rel)
print("drift (overlay != live + previous set):", drift)
print("changed by the current set:", changed)
if "--write" in sys.argv and not set(changed) & set(drift) and not previous_problems and not current_problems:
    for rel in changed:
        open(os.path.join(OVERLAY, rel), "w", encoding="utf-8").write(current[rel])
    print("written", len(changed))
