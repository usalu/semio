"""🔗 C4P28 non-Rust consumer re-points (source-text readers and pins of the puzzle 3d split files). Every edit re-reads the file, asserts the exact anchor count and writes atomically.
usage: consumers_ts.py plan|apply   (plan prints what would change)"""
import hashlib, json, os, sys, time

REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LOG = F + "/🗑️generated/c4p28-edits.log"
mode = sys.argv[1]
E = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ROOTS = f"{E}/🦀️.rs"


def edit(rel, old, new, count=1):
    p = f"{REPO}/{rel}"
    raw = open(p, "rb").read()
    text = raw.decode("utf-8")
    assert text.count(old) == count, (rel, "anchor count", text.count(old), old[:90])
    out = text.replace(old, new)
    line = f"{time.strftime('%H:%M:%S')} {rel.split('/')[-2]}/{rel.split('/')[-1]}: {old[:70]!r} -> {new[:70]!r} (x{count})"
    print(("APPLY " if mode == "apply" else "PLAN  ") + line)
    if mode == "apply":
        again = open(p, "rb").read()
        assert hashlib.sha256(again).hexdigest() == hashlib.sha256(raw).hexdigest(), f"{rel} changed meanwhile"
        tmp = p + ".c4p28-tmp"
        open(tmp, "w", encoding="utf-8").write(out)
        os.replace(tmp, p)
        open(LOG, "a", encoding="utf-8").write(line + "\n")


def lines_of(rel):
    return open(f"{REPO}/{rel}", encoding="utf-8").read().split("\n")


# 1. policy split roots: the readers of the tool-job census, the interactivity audit and the rust evidence read a declared split root as the one text the unsplit file held
src = f"{LIB}/🔍️discovery/📖️source-access/🟦️.ts"
anchor = '  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n'
adds = "".join(f'  "{E}/{x}/🦀️.rs",\n'.replace("/./", "/") if x else f'  "{E}/🦀️.rs",\n' for x in ["", "⏳️precompute", "⏳️precompute/🪣️fill", "🎟️session"])
edit(src, anchor, anchor + adds)

# 2. publication authority fixture: the 3d owner declares its mounted children
fx = "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json"
old = '      "owner": "Puzzle3dPlayApp",\n      "source": "🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n'
new = old + '      "mounted": [\n        "🧰️retained-commands/🦀️.rs",\n        "📬️store-preparation/🦀️.rs",\n        "✂️clipboard-job/🦀️.rs",\n        "🧩️play-app/🦀️.rs",\n        "📋️manifest/🦀️.rs"\n      ],\n'
edit(fx, old, new)

# 3. completion fixture: the 3d retained command job lives in the retained-commands child
comp = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⏳️completion/🔣️.json"
old = f'            "source": "{E}/🦀️.rs",\n            "tokens": [\n              "type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle3dPlayApp>>;",'
new = f'            "source": "{E}/🧰️retained-commands/🦀️.rs",\n            "tokens": [\n              "type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle3dPlayApp>>;",'
edit(comp, old, new)

# 4. compute ownership: the compute engine binding lives in the brush-meshes child of the precompute root
pc = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏛️ownership/🧮️compute"
old_src = f"{E}/⏳️precompute/🦀️.rs"
new_src = f"{E}/⏳️precompute/🥽️brush-meshes/🦀️.rs"
edit(f"{pc}/🧪️tests/🧫️fixtures/🔣️.json", f'"source": "{old_src}"', f'"source": "{new_src}"')
edit(f"{pc}/🧬️schema/🔣️.json", f'"const": "{old_src}"', f'"const": "{new_src}"')
edit(f"{pc}/📋️project.json", f'"{{workspaceRoot}}/{old_src}"', f'"{{workspaceRoot}}/{new_src}"')

# 5. lib wiring ratchet: roots at most 150 lines lose their pin, the editor root is pinned at its new size
rat = f"{LIB}/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
size = lambda rel: len(open(f"{REPO}/{rel}", encoding="utf-8").read().split("\n"))
for rel in [f"{E}/⏳️precompute/🦀️.rs", f"{E}/⏳️precompute/🪣️fill/🦀️.rs", f"{E}/🎟️session/🦀️.rs", f"{E}/🦀️.rs"]:
    n = size(rel)
    pinned = [l for l in lines_of(rat) if l.strip().startswith(f'"{rel}":')]
    assert len(pinned) == 1, (rel, pinned)
    if n <= 150:
        edit(rat, pinned[0] + "\n", "")
    else:
        edit(rat, pinned[0], f'    "{rel}": {n},' if pinned[0].rstrip().endswith(",") else f'    "{rel}": {n}')
