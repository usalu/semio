#!/usr/bin/env python3
"""📦️ usage: stage.py <check|apply|revert|status> -> lands (or removes) the staged conv-g batch (docstring emojis, missing docstrings, EN/DE refusals, policy constants of the 1025f/1026g production modules) against the live tree.
staged/files = the new version of every file (NEW files have no staged/base twin), staged/base = the bytes the edit started from. The nine read-oracle files started from the staged files of oracle2-g, so they need oracle2-g applied first (state PREREQ-ORACLE2 otherwise).
check: per file NEW / UNCHANGED-BASE (copy) / MERGE (a peer edited the file since staging: a three-way merge with `git merge-file`, clean or not) / EXISTS / DONE / PREREQ-ORACLE2.
apply: refuses everything when one file would conflict or a prerequisite is missing, otherwise writes all; the bytes of every touched file are first saved under C/🗑️generated/conv-g/backup so `revert` can undo (it merges the reverse change the same way and removes the NEW files)."""
import filecmp, os, shutil, subprocess, sys, tempfile
ROOT = os.environ.get("CG_ROOT", "/Users/ueli/Documents/semio")
C = "/Users/ueli/Documents/semio" + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S = C + "/conv-g-tools/staged"
O2 = C + "/oracle2-g-tools/staged"
BACKUP = os.environ.get("CG_BACKUP", C + "/🗑️generated/conv-g/backup")
def files():
    for base, _, names in os.walk(S + "/files"):
        for name in names:
            if name != ".DS_Store": yield os.path.relpath(os.path.join(base, name), S + "/files")
def same(a, b): return os.path.exists(a) and os.path.exists(b) and filecmp.cmp(a, b, shallow=False)
def merge(live, base, new):
    run = subprocess.run(["git", "merge-file", "-p", "-L", "live", "-L", "base", "-L", "staged", live, base, new], capture_output=True)
    return run.returncode, run.stdout
def prerequisite(f):
    return same(S + "/base/" + f, O2 + "/files/" + f) and os.path.exists(O2 + "/base/" + f)
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
            elif prerequisite(f) and same(live, O2 + "/base/" + f): rows.append((f, "PREREQ-ORACLE2", None))
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
bad = [r for r in rows if r[1].startswith(("MERGE-CONFLICT", "UNMERGE-CONFLICT", "EXISTS", "CHANGED-NEW", "PREREQ"))]
for f, state, _ in rows: print("%-22s %s" % (state, f.replace("🧰️framework/", "FW/")[-110:]))
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
        if state in ("NEW", "UNCHANGED-BASE"): shutil.copy2(S + "/files/" + f, live)
        else: open(live, "wb").write(data)
    else:
        if state == "REMOVE-NEW":
            os.remove(live)
            directory = os.path.dirname(live)
            while directory.endswith("/🧬️schema") and not os.listdir(directory):
                os.rmdir(directory); directory = os.path.dirname(directory)
        else: open(live, "wb").write(data)
print("applied" if mode == "apply" else "reverted", len([r for r in rows if r[1] not in ("DONE", "GONE")]), "files")
