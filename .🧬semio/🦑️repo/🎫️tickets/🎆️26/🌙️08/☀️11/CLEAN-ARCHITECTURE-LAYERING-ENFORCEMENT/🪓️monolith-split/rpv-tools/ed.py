"""✏️ RPV anchored editor: every edit re-reads the file, asserts the anchor occurs exactly once, writes and journals it (GEN/rpv-edits.log). Repo paths only under ROOT, never under 🗑️generated.
usage as a module: from ed import edit, R, T"""
import os, sys, time
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = R + "/🎯️targets/🧊️wgpu"


def edit(rel, old, new, count=1):
    full = os.path.realpath(os.path.join(ROOT, rel))
    assert full.startswith(os.path.realpath(ROOT) + os.sep) and "🗑️generated" not in full, full
    with open(full, "rb") as f:
        tx = f.read().decode("utf-8")
    n = tx.count(old)
    assert n == count, f"anchor count {n} != {count} in {rel[-70:]}: {old[:80]!r}"
    out = tx.replace(old, new)
    with open(full, "wb") as f:
        f.write(out.encode("utf-8"))
    with open(GEN + "/rpv-edits.log", "a", encoding="utf-8") as j:
        j.write(f"{time.strftime('%H:%M:%S')}\t{rel}\t{old[:90]!r}\t=>\t{new[:90]!r}\n")
    print("edited", rel[-80:], repr(old[:60]), "=>", repr(new[:60]))
