"""Lands the staged H41 files atomically. Usage: land.py check|apply. Aborts without any write when a live file differs from the base the stage was cut from."""
import hashlib, json, os, sys

ROOT = "/Users/ueli/Documents/semio"
STAGE = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H41/co"
HERE = os.path.dirname(os.path.abspath(__file__))
manifest = json.load(open(os.path.join(HERE, "base-shas.json"), encoding="utf8"))


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


problems = []
for rel, base in manifest["files"].items():
    live = os.path.join(ROOT, rel)
    if sha(live) != base:
        problems.append(rel)
if problems:
    print("LIVE FILES CHANGED SINCE THE STAGE WAS CUT:\n" + "\n".join(problems))
    sys.exit(1)
if sys.argv[1] == "check":
    print("ok", len(manifest["files"]), "files")
    sys.exit(0)
for rel in manifest["order"]:
    live, staged = os.path.join(ROOT, rel), os.path.join(STAGE, rel)
    temporary = live + ".h41"
    with open(staged, "rb") as source, open(temporary, "wb") as target:
        target.write(source.read())
    os.chmod(temporary, os.stat(live).st_mode & 0o7777)
    os.replace(temporary, live)
    print("landed", rel[-60:])
