#!/usr/bin/env python3
"""Applies the staged bzl-tests-rest set to the live tree, all or nothing. Default is a dry run. `--write` applies: every edited file must still equal the text its staged copy was built from (staged/bases.json), otherwise nothing is written and `build-staged.py` must be run again; new files are written whole. Every replaced file is backed up first, any write error restores all of them. `--undo` restores the backup of the last write. `--check` exits 1 when the live tree does not carry the set yet."""
import hashlib, json, os, shutil, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import layout

STAGED = os.path.join(layout.HERE, "staged")
BACKUPS = os.path.join(os.path.dirname(layout.HERE), "🗑️generated", "bzl-tests-rest", "backup")

def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None

def plan():
    bases = json.load(open(os.path.join(STAGED, "bases.json")))
    rows, drift = [], []
    for flat in layout.NEW:
        src = os.path.join(STAGED, flat)
        if not os.path.exists(src):
            drift.append("%s: staged file missing" % flat)
            continue
        live = layout.dest(flat)
        rows.append(("new" if sha(live) is None else ("same" if sha(live) == sha(src) else "replace"), flat, src, live))
    for flat in layout.EDIT:
        src = os.path.join(STAGED, flat)
        live = layout.dest(flat)
        if flat not in bases or not os.path.exists(src):
            drift.append("%s: not built (run build-staged.py)" % flat)
            continue
        if sha(live) == sha(src):
            rows.append(("same", flat, src, live))
        elif sha(live) != bases[flat]:
            drift.append("%s: live text moved since it was staged (run build-staged.py)" % flat)
        else:
            rows.append(("edit", flat, src, live))
    return rows, drift

def undo():
    last = sorted(os.listdir(BACKUPS))[-1] if os.path.isdir(BACKUPS) and os.listdir(BACKUPS) else None
    if not last:
        raise SystemExit("no backup to restore")
    directory = os.path.join(BACKUPS, last)
    manifest = json.load(open(os.path.join(directory, "manifest.json")))
    for row in manifest["rows"]:
        live = layout.dest(row["flat"])
        if row["before"] is None:
            if os.path.exists(live) and sha(live) == row["after"]:
                os.remove(live)
                parent = os.path.dirname(live)
                while parent != layout.ROOT and not os.listdir(parent):
                    os.rmdir(parent)
                    parent = os.path.dirname(parent)
        else:
            if sha(live) != row["after"]:
                raise SystemExit("%s moved since the apply; nothing restored" % row["flat"])
    for row in manifest["rows"]:
        live = layout.dest(row["flat"])
        if row["before"] is not None:
            shutil.copyfile(os.path.join(directory, row["flat"]), live)
    print("restored", len(manifest["rows"]), "files from", last)

def main():
    if "--undo" in sys.argv:
        undo()
        return
    rows, drift = plan()
    for kind, flat, _src, live in rows:
        print("%-8s %-22s %s" % (kind, flat, os.path.relpath(live, layout.ROOT)[-96:]))
    if drift:
        print("\nREFUSED:\n  " + "\n  ".join(drift))
        raise SystemExit(1)
    pending = [row for row in rows if row[0] != "same"]
    if "--check" in sys.argv:
        raise SystemExit(1 if pending else 0)
    print("\n%d files to write, %d already in place" % (len(pending), len(rows) - len(pending)))
    if "--write" not in sys.argv or not pending:
        return
    directory = os.path.join(BACKUPS, time.strftime("%Y%m%d-%H%M%S"))
    os.makedirs(directory)
    manifest = []
    for kind, flat, src, live in pending:
        before = sha(live)
        if before is not None:
            shutil.copyfile(live, os.path.join(directory, flat))
        manifest.append({"flat": flat, "before": before, "after": sha(src)})
    json.dump({"rows": manifest}, open(os.path.join(directory, "manifest.json"), "w"), indent=1)
    written = []
    try:
        for kind, flat, src, live in pending:
            os.makedirs(os.path.dirname(live), exist_ok=True)
            temporary = live + ".bzl-tests-rest.tmp"
            shutil.copyfile(src, temporary)
            os.replace(temporary, live)
            written.append((flat, live))
    except Exception:
        for row in manifest:
            live = layout.dest(row["flat"])
            if row["before"] is None:
                if os.path.exists(live):
                    os.remove(live)
            else:
                shutil.copyfile(os.path.join(directory, row["flat"]), live)
        raise
    print("written %d files; backup %s" % (len(written), directory))

if __name__ == "__main__":
    main()
