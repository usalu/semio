#!/usr/bin/env python3
"""🔁️ Rebases the U6 T4 overlay onto the current live tree: saves every file the overlay changed (and its base pre-image), refreshes
base and overlay from live (`wp-lb2/lb2-overlay.py`), then writes `git merge-file -p <live> <old base> <mine>` into the overlay — a
3-way merge that keeps both the live edits (T5 codemods, trains) and the set. Conflicted files are left with markers and listed.
Usage: rebase.py <changed-list from `make_set.py /dev/null --list`>"""
import shutil
import subprocess
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
HUB = LIVE / ".🧬semio/🌐hub"
BASE, OVERLAY = HUB / "s14-u6-base", HUB / "s14-u6-overlay"
MINE, OLD = HUB / "s14-u6-rebase-mine", HUB / "s14-u6-rebase-base"
CLONE = LIVE / ".tmp-ticket/wp-lb2/lb2-overlay.py"


def main() -> int:
    rows = [line.split(" ", 1) for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
    for kind, rel in rows:
        (MINE / rel).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(OVERLAY / rel, MINE / rel)
        if kind == "EDIT":
            (OLD / rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(BASE / rel, OLD / rel)
    for root in (BASE, OVERLAY):
        subprocess.run([sys.executable, str(CLONE), str(root)], check=True)
    conflicts, gone = [], []
    for kind, rel in rows:
        if kind == "NEW":
            shutil.copyfile(MINE / rel, OVERLAY / rel)
            continue
        if not (LIVE / rel).exists():
            gone.append(rel)
            continue
        merged = subprocess.run(["git", "merge-file", "-p", str(LIVE / rel), str(OLD / rel), str(MINE / rel)], capture_output=True)
        (OVERLAY / rel).write_bytes(merged.stdout)
        if merged.returncode != 0:
            conflicts.append((rel, merged.returncode))
    print(f"rebased {len(rows)} files; conflicts={len(conflicts)} gone={len(gone)}")
    for rel, count in conflicts:
        print("CONFLICT", count, rel)
    for rel in gone:
        print("GONE", rel)
    return 0


if __name__ == "__main__":
    sys.exit(main())
