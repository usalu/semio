#!/usr/bin/env python3
# Applies the staged parity set to the live repo after the write pause. Default is a dry run; --write applies all-or-nothing (backups under the ticket's generated folder); --undo restores them.
# Refuses on drift of any modified file (live hash != recorded base; run build-staged.py again), on a differing existing new file, and while a dependency sync/lock runs.
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
BACKUP = os.path.dirname(HERE) + "/🗑️generated/parity-apply"
layout = json.load(open(HERE + "/layout.json", encoding="utf-8"))
bases = json.load(open(HERE + "/staged/bases.json"))
sha = lambda b: hashlib.sha256(b).hexdigest()
write = "--write" in sys.argv
if "--undo" in sys.argv:
    journal = json.load(open(BACKUP + "/journal.json"))
    for rel, state in journal.items():
        live = os.path.join(ROOT, rel)
        if state == "new": os.path.exists(live) and os.remove(live)
        else: shutil.copy2(os.path.join(BACKUP, "files", rel), live)
        print("restored", rel)
    sys.exit(0)
refused = []
if write and subprocess.run(["pgrep", "-f", "dependencies (sync|lock)"], capture_output=True).returncode == 0: refused.append("a dependency sync/lock is running")
plan = []
for flat, rel in layout["modified"].items():
    live = os.path.join(ROOT, rel); current = sha(open(live, "rb").read())
    if current != bases[flat]: refused.append("drift: %s (live %s != base %s); run build-staged.py again" % (rel, current[:12], bases[flat][:12]))
    plan.append((rel, open(HERE + "/staged/mod/" + flat, "rb").read(), "modified"))
for flat, rel in layout["new"].items():
    live = os.path.join(ROOT, rel); data = open(HERE + "/staged/" + flat, "rb").read()
    if os.path.exists(live) and open(live, "rb").read() != data: refused.append("new file exists with other content: " + rel)
    plan.append((rel, data, "new"))
for rel, _, _ in plan:
    if len(os.path.join(ROOT, rel)) > 255: refused.append("path longer than 255 characters: " + rel)
if refused:
    print("\n".join("REFUSED " + r for r in refused)); sys.exit(1)
if not write:
    for rel, _, state in plan: print("would write (%s) %s" % (state, rel))
    print("dry run: %d files; use --write to apply" % len(plan)); sys.exit(0)
os.makedirs(BACKUP + "/files", exist_ok=True)
journal = {}
try:
    for rel, data, state in plan:
        live = os.path.join(ROOT, rel)
        if state == "modified":
            os.makedirs(os.path.dirname(BACKUP + "/files/" + rel), exist_ok=True); shutil.copy2(live, BACKUP + "/files/" + rel); journal[rel] = "modified"
        else: journal[rel] = "new" if not os.path.exists(live) else "same"
        json.dump(journal, open(BACKUP + "/journal.json", "w"), indent=1)
        os.makedirs(os.path.dirname(live), exist_ok=True)
        tmp = live + ".parity.tmp"; open(tmp, "wb").write(data); os.replace(tmp, live)
        print("wrote", rel)
except Exception:
    print("FAILED; undoing"); os.system("%s %s --undo" % (sys.executable, os.path.abspath(__file__))); raise
