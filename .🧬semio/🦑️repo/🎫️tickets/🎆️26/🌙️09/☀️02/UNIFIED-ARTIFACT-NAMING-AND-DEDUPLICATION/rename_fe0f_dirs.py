#!/usr/bin/env python3
"""🔧 Rename every directory named exactly ◻2d / 📄txt / 📰xml (missing U+FE0F)
to the SSOT-correct ◻️2d / 📄️txt / 📰️xml, repo-wide. Deepest paths first so
renaming an ancestor never invalidates an already-captured descendant path."""
import os, json

REPO = "/Users/ueli/Documents/semio"
os.chdir(REPO)
EXCLUDE = {"node_modules", "target", "dist", ".git"}
RENAME_MAP = {
    "◻2d": "◻️2d",
    "📄txt": "📄️txt",
    "📰xml": "📰️xml",
}

dir_hits = []
for root, dirs, files in os.walk("."):
    dirs[:] = [d for d in dirs if d not in EXCLUDE]
    for d in list(dirs):
        if d in RENAME_MAP:
            dir_hits.append(os.path.join(root, d))

dir_hits.sort(key=lambda p: p.count(os.sep), reverse=True)

log = []
for old in dir_hits:
    parent, base = os.path.split(old)
    new = os.path.join(parent, RENAME_MAP[base])
    if not os.path.exists(old):
        log.append({"old": old, "new": new, "status": "MISSING(already renamed as ancestor?)"})
        continue
    if os.path.exists(new):
        log.append({"old": old, "new": new, "status": "TARGET_EXISTS_SKIPPED"})
        continue
    os.rename(old, new)
    log.append({"old": old, "new": new, "status": "RENAMED"})

print(f"Total candidates: {len(dir_hits)}")
ok = sum(1 for e in log if e["status"] == "RENAMED")
print(f"Renamed: {ok}")
for e in log:
    if e["status"] != "RENAMED":
        print("NOTE:", e)

with open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/UNIFIED-ARTIFACT-NAMING-AND-DEDUPLICATION/🗑️generated/dir_rename_log.json", "w") as f:
    json.dump(log, f, ensure_ascii=False, indent=2)
