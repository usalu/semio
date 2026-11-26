"""🫃️ Leaves the trunk as drawn: ltrunk.py <id>… writes `loft/<id>.json` as the full proposal `loft/<id>.all.json` (made from the current list when it does not exist) without the ops of the trunk bones (TRUNK, default "pelvis lumbar thorax trunk chest"). A trunk carries several shapes per view (reported tier of the gate) and its traced outlines agree better with the volume than a loft of superellipses does (front of `red-fox` 1:50: 1.9 mm against 20.6 mm), so it becomes a loft only where the gate asks for it."""
import json, os, pathlib, shutil, sys
W = pathlib.Path(__file__).resolve().parent
TRUNK = set(os.environ.get("TRUNK", "pelvis lumbar thorax trunk chest").split())
for a in sys.argv[1:]:
    path, full = W / "loft" / f"{a}.json", W / "loft" / f"{a}.all.json"
    if not full.exists():
        shutil.copyfile(path, full)
    ops = json.load(open(full))
    kept = [op for op in ops if op.get("bone") not in TRUNK]
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in kept) + "\n]\n")
    print(a, len(ops) - len(kept), "trunk ops left out,", len(kept), "kept")
