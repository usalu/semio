"""✍️ C4P28 non-Rust edits that go with the seven splits: the policy split-root registry, the block publication-authority script, the composition-admission caller corpus and the lib-wiring ratchet.
Every edit is an anchored replace (the anchor must occur exactly once), written atomically (tmp + rename), and logged to 🗑️generated/c4p28-edits.log. Run it only after the Rust install (it reads the installed roots to pin the ratchet).
usage: edits.py check|apply"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
B = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
S = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
mode = sys.argv[1]
log = []


def read(rel):
    return open(f"{REPO}/{rel}", encoding="utf-8").read()


def write(rel, text):
    path = f"{REPO}/{rel}"
    tmp = path + ".c4p28-tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(text)
    os.replace(tmp, path)


def once(text, anchor, new, rel):
    assert text.count(anchor) == 1, f"{rel}: anchor occurs {text.count(anchor)} times: {anchor[:80]}"
    return text.replace(anchor, new)


edits = {}

rel = f"{LIB}/🔍️discovery/📖️source-access/🟦️.ts"
t = read(rel)
session = f'  "{P}/✏️editor/🎟️session/🦀️.rs",\n'
added = "".join(f'  "{p}",\n' for p in [
    f"{P}/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    f"{P}/✏️editor/🧵️retained/🎬️action/🦀️.rs",
    f"{P}/✏️editor/🧵️retained/🗂️document/🦀️.rs",
    f"{P}/🧬️schema/🦀️.rs",
    f"{B}/✏️editor/🦀️.rs",
    f"{S}/✏️editor/🦀️.rs",
])
if added.split("\n")[0] in t:
    log.append(f"{rel}: already registered")
else:
    edits[rel] = once(t, session, session + added, rel)
    log.append(f"{rel}: +6 split source roots after the puzzle 3d session root")

rel = "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts"
t = read(rel)
if "readModuleTreeSource" in t:
    log.append(f"{rel}: already re-pointed")
else:
    imp_anchor = 'import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";\n'
    imp_new = imp_anchor + 'import { readModuleTreeSource } from "../../../../../🧰️framework/📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";\n'
    t = once(t, imp_anchor, imp_new, rel)
    read_anchor = "sources.set(app.owner, await Bun.file(resolve(plugin, app.source)).text());"
    t = once(t, read_anchor, "sources.set(app.owner, readModuleTreeSource(resolve(plugin, app.source)));", rel)
    edits[rel] = t
    log.append(f"{rel}: the publication-authority source is read through readModuleTreeSource (editor root plus mounted children)")

rel = "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json"
t = read(rel)
root_row = f'"source": "{S}/✏️editor/🦀️.rs"'
child_row = f'"source": "{S}/✏️editor/🧩️sourcing-app/🦀️.rs"'
if child_row in t:
    log.append(f"{rel}: already re-pointed")
else:
    edits[rel] = once(t, root_row, child_row, rel)
    log.append(f"{rel}: the sourcing editor caller of ChildRestoreProjection is now the sourcing-app child")

rel = f"{LIB}/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
t = read(rel)
data = json.loads(t)
budget = data["budget"]
roots = {
    f"{P}/✏️editor/🧵️retained/🎬️action/🦀️.rs": 3,
    f"{P}/✏️editor/🧵️retained/🗂️document/🦀️.rs": 4,
    f"{P}/🧬️schema/🦀️.rs": 5,
    f"{B}/✏️editor/🦀️.rs": 6,
}
for path, n in roots.items():
    lines = len(read(path).split("\n"))
    key = f"✏️s/🔌️plugins/{path[len('✏️s/🔌️plugins/'):]}"
    old = re.search(r'^\s*"' + re.escape(path) + r'": (\d+),?$', t, re.M)
    assert old, f"ratchet row missing for {path}"
    if lines <= budget:
        t = re.sub(r'^\s*"' + re.escape(path) + r'": \d+,?\n', "", t, count=1, flags=re.M)
        log.append(f"{rel}: pin removed for root {n} ({old.group(1)} -> {lines} lines, within the {budget}-line budget)")
    else:
        t = once(t, old.group(0), old.group(0).replace(old.group(1), str(lines)), rel)
        log.append(f"{rel}: pin of root {n} {old.group(1)} -> {lines}")
edits[rel] = t
json.loads(t)

for rel, text in edits.items():
    if mode == "apply":
        write(rel, text)
for l in log:
    print(l)
if mode == "apply":
    with open(F + "/🗑️generated/c4p28-edits.log", "a", encoding="utf-8") as f:
        f.write("\n".join(log) + "\n")
