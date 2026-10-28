"""🔴 Red-on-base law helper: reverts a staged batch in the mirror, then re-applies only the files whose path contains one of the given markers (the new tests)."""
import json, os, shutil, subprocess, sys
batch, mirror = sys.argv[1], sys.argv[2]; markers = sys.argv[3:]
d = os.path.join(os.path.dirname(os.path.abspath(__file__)), "staged", batch)
subprocess.run([sys.executable, "-I", os.path.join(d, "stage.py"), "revert", "--root", mirror], check=True, stdout=subprocess.DEVNULL)
for rel, row in json.load(open(os.path.join(d, "manifest.json"), encoding="utf8")).items():
    if any(m in rel for m in markers): shutil.copyfile(os.path.join(d, "files", row["file"]), os.path.join(mirror, rel)); print("test-only:", rel[-70:])
