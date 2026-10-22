import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/cache-gc/m4"
HERE = os.path.dirname(os.path.abspath(__file__))
chain = [{**patches.EDITORS3, "catalog": patches.EDITORS2["catalog"]}, patches.EDITORS4]
patched = {}
for editors in chain:
    for key, editor in editors.items():
        target = TARGET4.get(key) or TARGET[key]
        if target not in patched:
            with open(J(ROOT, target), encoding="utf-8") as handle:
                patched[target] = handle.read()
        patched[target] = editor(patched[target])
with open(J(ROOT, TARGET["help-snapshots"]), encoding="utf-8") as handle:
    patched.setdefault(TARGET["help-snapshots"], handle.read())
for staged, files in (("staged3", NEW3), ("staged4", NEW4)):
    for name, target in files.items():
        with open(J(HERE, staged, name), encoding="utf-8") as handle:
            patched[target] = handle.read()
shutil.rmtree(OVERLAY, ignore_errors=True)
os.makedirs(OVERLAY)


def mirror(rel_dir, children):
    source, destination = J(ROOT, rel_dir), J(OVERLAY, rel_dir)
    os.makedirs(destination, exist_ok=True)
    wanted = {}
    for child in children:
        head, _, rest = child.partition(os.sep)
        wanted.setdefault(head, []).append(rest)
    if os.path.isdir(source):
        for entry in os.listdir(source):
            if entry not in wanted and not os.path.lexists(J(destination, entry)):
                os.symlink(J(source, entry), J(destination, entry))
    for head, rests in wanted.items():
        if rests == [""]:
            with open(J(destination, head), "w", encoding="utf-8") as handle:
                handle.write(patched[os.path.join(rel_dir, head)])
        else:
            mirror(J(rel_dir, head), [rest for rest in rests if rest])


mirror("", list(patched))
for rel in (J(CLOSURES, TESTS), J(B, TESTS), J(B, E(0x1F4E5) + "inputs", STORAGE, TESTS)):
    link = J(OVERLAY, rel)
    if os.path.islink(link) or not os.path.exists(link):
        if os.path.islink(link):
            os.unlink(link)
        shutil.copytree(J(ROOT, rel), link, symlinks=True)
for entry in ("node_modules", "package.json", "tsconfig.json", "bunfig.toml", "bun.lock"):
    if os.path.exists(J(ROOT, entry)) and not os.path.lexists(J(OVERLAY, entry)):
        os.symlink(J(ROOT, entry), J(OVERLAY, entry))
print("mirror", OVERLAY, len(patched), "file(s)")
