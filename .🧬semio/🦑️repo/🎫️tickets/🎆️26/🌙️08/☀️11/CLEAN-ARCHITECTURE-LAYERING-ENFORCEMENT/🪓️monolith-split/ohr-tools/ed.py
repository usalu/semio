"""✏️ OHR anchored editor: each edit re-reads the file, asserts the anchor count, writes and journals it (GEN/ohr-edits.log). Repo paths only under ROOT, never under 🗑️generated.
usage as a module: from ed import edit, rx, R, T"""
import os, re, sys, time
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = R + "/🎯️targets/🧊️wgpu"


def _path(rel):
    full = os.path.realpath(os.path.join(ROOT, rel))
    assert full.startswith(os.path.realpath(ROOT) + os.sep) and "🗑️generated" not in full, full
    return full


def _log(rel, a, b):
    with open(GEN + "/ohr-edits.log", "a", encoding="utf-8") as j:
        j.write(f"{time.strftime('%H:%M:%S')}\t{rel}\t{a[:90]!r}\t=>\t{b[:90]!r}\n")
    print("edited", rel[-70:], repr(a[:55]), "=>", repr(b[:55]))


def edit(rel, old, new, count=1):
    full = _path(rel)
    tx = open(full, "rb").read().decode("utf-8")
    n = tx.count(old)
    assert n == count, f"anchor count {n} != {count} in {rel[-70:]}: {old[:80]!r}"
    open(full, "wb").write(tx.replace(old, new).encode("utf-8"))
    _log(rel, old, new)


def rx(rel, pattern, repl, count):
    full = _path(rel)
    tx = open(full, "rb").read().decode("utf-8")
    out, n = re.subn(pattern, repl, tx)
    assert n == count, f"regex count {n} != {count} in {rel[-70:]}: {pattern[:80]!r}"
    open(full, "wb").write(out.encode("utf-8"))
    _log(rel, pattern, repl)
