import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/cache-gc/m2"
patched = {}
with open(J(ROOT, TARGET["catalog"]), encoding="utf-8") as handle:
    patched[TARGET["catalog"]] = patches.EDITORS2["catalog"](handle.read())
with open(J(ROOT, TARGET["help-snapshots"]), encoding="utf-8") as handle:
    patched[TARGET["help-snapshots"]] = handle.read()
for name, target in NEW2.items():
    with open(J(HERE2 := os.path.dirname(os.path.abspath(__file__)), "staged2", name), encoding="utf-8") as handle:
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
for entry in ("node_modules", "package.json", "tsconfig.json", "bunfig.toml", "bun.lock"):
    if os.path.exists(J(ROOT, entry)) and not os.path.lexists(J(OVERLAY, entry)):
        os.symlink(J(ROOT, entry), J(OVERLAY, entry))
print("mirror", OVERLAY, len(patched), "file(s)")
