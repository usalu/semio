"""🧷 E51 anchored editor: every edit re-reads the file, asserts the anchor occurs exactly once, writes it, journals it (GEN/e51-edits.log, short) and appends the full old/new pair to GEN/e51-edits.jsonl (the oracle's before-run replays the inverse). Repo paths only under ROOT, never under 🗑️generated.
usage as a module: from ed import edit"""
import json, os, sys, time
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"


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
    with open(GEN + "/e51-edits.log", "a", encoding="utf-8") as j:
        j.write(f"{time.strftime('%H:%M:%S')}\t{rel}\t{old[:90]!r}\t=>\t{new[:90]!r}\n")
    with open(GEN + "/e51-edits.jsonl", "a", encoding="utf-8") as j:
        j.write(json.dumps(dict(rel=rel, old=old, new=new, count=count), ensure_ascii=False) + "\n")
    print("edited", rel[-80:], repr(old[:60]), "=>", repr(new[:60]))
