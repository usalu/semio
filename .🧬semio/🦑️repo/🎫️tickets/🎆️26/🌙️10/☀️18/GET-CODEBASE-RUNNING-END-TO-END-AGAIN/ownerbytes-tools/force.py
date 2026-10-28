"""🔁 Copies every staged row of `ownerbytes` over the same path in a mirror root, whatever it holds (mirror iteration only; never run against the live root). usage: python3 -I force.py <mirror-root>"""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "ownerbytes")
ROOT = os.path.abspath(sys.argv[1])
if not ROOT.startswith("/private/tmp/m/") and "/sm/" not in ROOT:
    sys.exit("mirror roots only")
for rel, row in json.load(open(os.path.join(STAGED, "manifest.json"), encoding="utf8")).items():
    target = os.path.join(ROOT, rel)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copyfile(os.path.join(STAGED, "files", row["file"]), target)
print("forced")
