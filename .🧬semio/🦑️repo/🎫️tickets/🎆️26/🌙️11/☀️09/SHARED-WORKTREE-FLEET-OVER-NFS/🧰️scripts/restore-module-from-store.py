"""Materialise archive-skipped files of the shared-worktree module from the store's blobs (latest journal entry per path)."""
import glob, hashlib, json, os, shutil, sys
STORE, WORKTREE, PREFIX = sys.argv[1], sys.argv[2], sys.argv[3]
latest = {}
for segment in sorted(glob.glob(os.path.join(STORE, "v1/origins/*/segments/*.jsonl"))):
    if os.path.basename(segment).startswith("._"): continue
    with open(segment, encoding="utf-8") as lines:
        for line in lines:
            if PREFIX not in line: continue
            event = json.loads(line)
            if event["path"].startswith(PREFIX) and (event["path"] not in latest or event["hlc"] > latest[event["path"]]["hlc"]): latest[event["path"]] = event
for path, event in sorted(latest.items()):
    target = os.path.join(WORKTREE, path)
    if event["operation"] != "put": print("latest is", event["operation"], path); continue
    if os.path.exists(target):
        same = hashlib.sha256(open(target, "rb").read()).hexdigest() == event["sha256"]
        print("present" if same else "updating", path)
        if same: continue
    blob = os.path.join(STORE, "v1/blobs", event["sha256"][:2], event["sha256"])
    data = open(blob, "rb").read()
    assert hashlib.sha256(data).hexdigest() == event["sha256"], path
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "wb") as out: out.write(data)
    if event.get("executable"): os.chmod(target, 0o755)
    print("restored", event["hlc"], path)
