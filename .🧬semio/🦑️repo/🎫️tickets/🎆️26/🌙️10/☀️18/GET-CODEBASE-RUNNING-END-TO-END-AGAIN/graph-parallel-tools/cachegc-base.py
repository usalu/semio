#!/usr/bin/env python3
"""🧰️ Brings a scratch tree to the state the live tree has after cache-gc staged3 and staged4 are applied (only the graph storage file and the member index files that matter for the graph farm), so that round 1 and round 2 of the graph farm set are generated, applied and tested on the order the coordinator fixed: cache-gc apply3, apply4, graph-parallel round 1, round 2.
usage: cachegc-base.py ROOT [--files-only]   (rewrites the storage file of ROOT in place and copies the new files of both sets; idempotent)"""
import os, pathlib, shutil, sys

HERE = pathlib.Path(__file__).resolve().parent
GC = next(path for path in [HERE.parent / "cache-gc-tools"] if path.exists())
os.environ["CACHE_GC_ROOT"] = sys.argv[1]
sys.path.insert(0, str(GC))
import patches, paths  # noqa: E402

root = pathlib.Path(sys.argv[1])
storage = root / paths.TARGET["graph-storage"]
text = storage.read_text(encoding="utf8")
after4 = text
if "--files-only" not in sys.argv:
    patches.STAGED = os.path.join(str(GC), "staged3")
    after3 = patches.EDITORS3["graph-storage"](text)
    patches.STAGED = os.path.join(str(GC), "staged4")
    after4 = patches.EDITORS4["graph-storage"](after3)
    storage.write_text(after4, encoding="utf8")
for stage, table in (("staged3", paths.NEW3), ("staged4", paths.NEW4)):
    for name, relative in table.items():
        source = GC / stage / name
        if source.is_file():
            target = root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)
print("storage:", "changed" if after4 != text else "already", "member hook:", "recordBuildGraphOptionalMemberV1(scope" in after4)
