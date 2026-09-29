#!/usr/bin/env python3
"""📤️ CD1 T6 set A — cad `brep:out` exports every pane shape through the semio brep's ONE AP214 writer: the kernel's `export_step` projects solids, shells, faces, wires and edges to a snapshot and writes it with `semio_brep_part21` (moved ungated into the kernel's `🟫️step`, stdio `SemioBrepToStep` wraps it; the kernel's own solid-only writer, its `({items},)` Part-21 error and MILLI.METRE units are gone): product structure, metre/radian context, one representation per class (ABSR solids incl. conformant BREP_WITH_VOIDS/ORIENTED_CLOSED_SHELL, MSSR open shells for free faces, EBWSR connected edge sets for free wires); `SemioBrepFromStep` reads BREP_WITH_VOIDS/ORIENTED_CLOSED_SHELL/ORIENTED_FACE; cad drops the repair shim and the STEP→semio→STEP double trip, export errors carry their reason (`cad.export.failed`, no silent spatial fallback); laws: step-exchange vectors (golden bytes + census), refusals, OpenCascade void reader, modelspace brep:out, cad export laws, OCCT exchange law (TS).
Usage: cd1-step-exchange.py [--dry-run (default) | --write | --revert] [--root <tree>] — backups `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-t6/cd1-step-exchange`."""
import json, os, shutil, sys

PAYLOAD = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "cd1-step-exchange.payload.json"), encoding="utf-8"))
EDITS = [(rel, [tuple(pair) for pair in pairs]) for rel, pairs in PAYLOAD["edits"]]
CREATES = [tuple(row) for row in PAYLOAD["creates"]]
BACKUPS = PAYLOAD["backups"]
ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
MODE = "--write" if "--write" in sys.argv else "--revert" if "--revert" in sys.argv else "--dry-run"


def read(rel):
    return open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read()


def plan():
    out, faults = {}, []
    for rel, pairs in EDITS:
        text = read(rel)
        for before, after in pairs:
            if text.count(before) != 1:
                faults.append(f"{rel}: anchor matches {text.count(before)}× — {before[:120]!r}")
                break
            text = text.replace(before, after)
        out[rel] = text
    for rel, text in CREATES:
        if os.path.exists(os.path.join(ROOT, rel)):
            faults.append(f"{rel}: exists already")
        out[rel] = text
    return out, faults


if MODE == "--revert":
    for rel, _ in EDITS + CREATES:
        saved = os.path.join(BACKUPS, rel)
        target = os.path.join(ROOT, rel)
        if os.path.exists(saved):
            shutil.copy2(saved, target)
        elif os.path.exists(target) and rel in dict(CREATES):
            os.remove(target)
    print("reverted")
    sys.exit(0)
applied = all(dict(CREATES).get(rel) == read(rel) for rel, _ in CREATES if os.path.exists(os.path.join(ROOT, rel))) and CREATES and all(os.path.exists(os.path.join(ROOT, rel)) for rel, _ in CREATES) and all(all(read(rel).count(after) >= 1 for _, after in pairs) for rel, pairs in EDITS)
if applied:
    print("already applied")
    sys.exit(0)
out, faults = plan()
print(f"{len(EDITS)} edited + {len(CREATES)} created, faults {len(faults)}")
for fault in faults:
    print("FAULT", fault)
if faults:
    sys.exit(1)
if MODE == "--write":
    for rel, text in out.items():
        target = os.path.join(ROOT, rel)
        if os.path.exists(target):
            os.makedirs(os.path.dirname(os.path.join(BACKUPS, rel)), exist_ok=True)
            shutil.copy2(target, os.path.join(BACKUPS, rel))
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8", newline="").write(text)
    print("written")
