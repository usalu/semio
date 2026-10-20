"""🧱️ Reads a repository file as it is live and as it will be once the round-5 lock-sync batch is applied on top, so every anchor of this set can be proven against both."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths
LOCK_SYNC = os.path.join(paths.C, "lock-sync-tools", "staged", "plan.json")
def live(relative):
    with open(os.path.join(paths.ROOT, relative), encoding="utf-8", newline="") as handle:
        return handle.read()
def after_lock_sync(relative):
    text = live(relative)
    plan = json.load(open(LOCK_SYNC, encoding="utf-8"))
    for item in plan["files"]:
        if item["path"] != relative:
            continue
        for edit in item["edits"]:
            old, new, counts = edit["old"], edit["new"], edit["counts"]
            inserts = bool(new) and old in new
            if inserts and text.count(new) > 0 and text.count(new) >= text.count(old):
                continue
            if not inserts and text.count(old) == 0 and text.count(new) > 0:
                continue
            if text.count(old) in counts:
                text = text.replace(old, new)
            else:
                raise RuntimeError("lock-sync anchor drift in %s [%s]" % (relative, edit["id"]))
    return text
