#!/usr/bin/env python3
"""📦️ usage: stage.py <check|apply|revert|status> -> lands (or removes) the staged oracle2-g batch against the live tree.
staged/files = the new version of every file (NEW files have no staged/base twin), staged/base = the bytes the edit started from.
check: per file NEW / UNCHANGED-BASE (copy) / MERGE (a peer edited the file since staging: a three-way merge with `git merge-file` on temporary copies, clean or not) / EXISTS (a new file already exists with other bytes) / DONE (already applied).
apply: refuses everything when one file would conflict, otherwise writes all; the bytes of every touched file are first saved under C/🗑️generated/oracle2-g/backup (so a red product build can be undone with `revert`, which merges the reverse change the same way and removes the NEW files)."""
import filecmp, os, shutil, subprocess, sys, tempfile
ROOT = os.environ.get("O2_ROOT", "/Users/ueli/Documents/semio")
C = "/Users/ueli/Documents/semio" + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S = C + "/oracle2-g-tools/staged"
BACKUP = C + "/🗑️generated/oracle2-g/backup"
def files():
    for base, _, names in os.walk(S + "/files"):
        for name in names:
            if name != ".DS_Store": yield os.path.relpath(os.path.join(base, name), S + "/files")
def same(a, b): return os.path.exists(a) and os.path.exists(b) and filecmp.cmp(a, b, shallow=False)
def merge(live, base, new):
    with tempfile.TemporaryDirectory(dir=os.environ.get("O2_TMP")) as d:
        run = subprocess.run(["git", "merge-file", "-p", "-L", "live", "-L", "base", "-L", "staged", live, base, new], capture_output=True)
        return run.returncode, run.stdout
def plan(mode):
    rows = []
    for f in sorted(files()):
        live, new, base = ROOT + "/" + f, S + "/files/" + f, S + "/base/" + f
        has_base = os.path.exists(base)
        if mode == "apply":
            if not os.path.exists(live): rows.append((f, "NEW", None))
            elif same(live, new): rows.append((f, "DONE", None))
            elif not has_base: rows.append((f, "EXISTS", None))
            elif same(live, base): rows.append((f, "UNCHANGED-BASE", new))
            else:
                code, out = merge(live, base, new); rows.append((f, "MERGE-CLEAN" if code == 0 else "MERGE-CONFLICT(%d)" % code, out))
        else:
            if not os.path.exists(live): rows.append((f, "GONE", None))
            elif not has_base: rows.append((f, "REMOVE-NEW" if same(live, new) else "CHANGED-NEW", None))
            elif same(live, new): rows.append((f, "RESTORE-BASE", open(base, "rb").read()))
            elif same(live, base): rows.append((f, "DONE", None))
            else:
                code, out = merge(live, new, base); rows.append((f, "UNMERGE-CLEAN" if code == 0 else "UNMERGE-CONFLICT(%d)" % code, out))
    return rows
mode = sys.argv[1] if len(sys.argv) > 1 else "check"
rows = plan("revert" if mode == "revert" else "apply")
bad = [r for r in rows if r[1].startswith(("MERGE-CONFLICT", "UNMERGE-CONFLICT", "EXISTS", "CHANGED-NEW"))]
for f, state, _ in rows: print("%-22s %s" % (state, f.split("🔭️read-oracle/")[-1]))
if mode in ("check", "status"): sys.exit(1 if bad else 0)
if bad: print("REFUSED: no file touched"); sys.exit(1)
os.makedirs(BACKUP, exist_ok=True)
for f, state, data in rows:
    live = ROOT + "/" + f
    if state in ("DONE", "GONE"): continue
    if os.path.exists(live):
        target = BACKUP + "/" + f; os.makedirs(os.path.dirname(target), exist_ok=True)
        if not os.path.exists(target): shutil.copy2(live, target)
    if mode == "apply":
        os.makedirs(os.path.dirname(live), exist_ok=True)
        if state == "NEW": shutil.copy2(S + "/files/" + f, live)
        elif state == "UNCHANGED-BASE": shutil.copy2(S + "/files/" + f, live)
        else: open(live, "wb").write(data)
    else:
        if state == "REMOVE-NEW":
            os.remove(live)
            directory = os.path.dirname(live)
            while "🔭️read-oracle/" in directory and not os.listdir(directory):
                os.rmdir(directory); directory = os.path.dirname(directory)
        else: open(live, "wb").write(data)
print("applied" if mode == "apply" else "reverted", len([r for r in rows if r[1] not in ("DONE", "GONE")]), "files")
