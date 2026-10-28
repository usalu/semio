#!/usr/bin/env python3
"""🏗️ Rebuilds `staged/split` from the CURRENT live files: copies every batch file of the live root into a scratch root, runs `edit_split.py` there, adds the hand-written new files of `new_split/`, then writes base/, files/ and manifest.json. usage: python3 -I build_split.py [live-root]"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C)))))))
STAGE = os.path.join(HERE, "staged", "split")
SCRATCH = os.path.join(C, "🗑️generated", "dashhot", "scratch-split")
NEW = os.path.join(HERE, "new_split")
def sha(data): return hashlib.sha256(data).hexdigest()
shutil.rmtree(SCRATCH, ignore_errors=True)
listing = subprocess.run([sys.executable, "-I", os.path.join(HERE, "list_split.py")], capture_output=True, text=True, check=True).stdout.split("\n")
files = [row for row in listing if row]
for rel in files:
    os.makedirs(os.path.dirname(os.path.join(SCRATCH, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(LIVE, rel), os.path.join(SCRATCH, rel))
subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit_split.py"), SCRATCH], check=True)
new = {}
for base, _dirs, names in os.walk(NEW):
    for name in names:
        rel = os.path.relpath(os.path.join(base, name), NEW)
        new[rel] = open(os.path.join(base, name), "rb").read()
shutil.rmtree(STAGE, ignore_errors=True)
os.makedirs(os.path.join(STAGE, "base")); os.makedirs(os.path.join(STAGE, "files"))
manifest = {}
for index, rel in enumerate(sorted(set(files) | set(new))):
    ext = os.path.splitext(rel)[1] or ".txt"
    key = f"{index:02d}{ext}"
    live_path = os.path.join(LIVE, rel)
    base = open(live_path, "rb").read() if os.path.exists(live_path) else None
    data = new[rel] if rel in new else open(os.path.join(SCRATCH, rel), "rb").read()
    if base is not None: open(os.path.join(STAGE, "base", key), "wb").write(base)
    open(os.path.join(STAGE, "files", key), "wb").write(data)
    manifest[rel] = {"file": key, "base": sha(base) if base is not None else None, "new": sha(data)}
json.dump(manifest, open(os.path.join(STAGE, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
for name, source in (("stage.py", "stage_split.py"), ("verify.sh", "verify_split.sh")):
    shutil.copyfile(os.path.join(HERE, source), os.path.join(STAGE, name))
shutil.rmtree(SCRATCH, ignore_errors=True)
print(f"staged {len(manifest)} files in {STAGE}")
