"""x-release in process on an overlay root: the anchored adapter edits of mode_patch.py, the dry-run plan edit, the README wording and the 📥️mode suite."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

STAGED = os.path.join(TICKET, "x-release-tools", "staged")
README = "🧰️framework/🛍️products/🦑️repo/README.md"


def run(root):
    sys.path.insert(0, STAGED)
    import mode_patch as m
    adapter, script, readme = f"{BAZEL}/📜️script.ts", f"{BUILD}/📜️script.ts", README
    changed = []
    for rel, edits, fn in ((adapter, None, m.patched), (script, m.SCRIPT_EDITS, lambda text, e: m.patched(text, e))):
        before = read(root, rel)
        after = fn(before, edits) if edits is not None else m.patched(before)
        if before != after:
            write(root, rel, after)
            changed.append(rel)
    before = read(root, readme)
    after = m.readme(before, "full")
    if before != after:
        write(root, readme, after)
        changed.append(readme)
    for name in sorted(os.listdir(m.STAGED_TESTS)):
        source = os.path.join(m.STAGED_TESTS, name)
        if not os.path.isfile(source):
            continue
        rel = f"{BAZEL}/🧪️tests/📥️mode/{name}"
        data = open(source, encoding="utf8").read()
        if not os.path.exists(os.path.join(root, rel)) or read(root, rel) != data:
            write(root, rel, data)
            changed.append(rel)
    if not (m.applied(read(root, adapter)) and m.applied(read(root, script), m.SCRIPT_EDITS)):
        refuse("x-release: a target does not contain the patch after applying")
    return changed
