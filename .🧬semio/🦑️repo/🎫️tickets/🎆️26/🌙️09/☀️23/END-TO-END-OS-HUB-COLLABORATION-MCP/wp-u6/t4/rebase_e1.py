#!/usr/bin/env python3
"""🔀️ Rebases the E1 paged-DOCX set onto the live tree: snapshots the set's live files into `<base>` (the new pre-E1 base),
three-way merges each edited file (live ← pre-E1 `s14-u6-base2` → proven E1 overlay) into `<target>`, and resolves a conflict
only where it is listed in `OVERLAY_WINS` (E1 deleted the whole region the live tree edited). New files are taken verbatim
from the overlay. Prints the conflicts it met. Usage: rebase_e1.py <base> <target>"""
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from make_e1_set import CREATED, EDITED, REMOVED  # noqa: E402

ROOT = Path("/Users/ueli/Documents/semio")
HUB = ROOT / ".🧬semio/🌐hub"
PRE, OVERLAY = HUB / "s14-u6-base2", HUB / "s14-u6-overlay"
OVERLAY_WINS = {"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs"}


def overlay_side(merged: str) -> str:
    out, state = [], "keep"
    for line in merged.splitlines(keepends=True):
        if line.startswith("<<<<<<< "):
            state = "live"
        elif line.startswith("||||||| ") and state == "live":
            state = "pre"
        elif line.startswith("=======") and state == "pre":
            state = "overlay"
        elif line.startswith(">>>>>>> ") and state == "overlay":
            state = "keep"
        elif state in ("keep", "overlay"):
            out.append(line)
    return "".join(out)


def main() -> int:
    base, target = Path(sys.argv[1]), Path(sys.argv[2])
    problems = 0
    for rel in [*EDITED, *REMOVED]:
        (base / rel).parent.mkdir(parents=True, exist_ok=True)
        (base / rel).write_bytes((ROOT / rel).read_bytes())
    for rel in EDITED:
        merged = subprocess.run(["diff3", "-m", str(ROOT / rel), str(PRE / rel), str(OVERLAY / rel)], capture_output=True, text=True)
        conflicts = merged.stdout.count("\n<<<<<<< ") + merged.stdout.startswith("<<<<<<< ")
        if merged.returncode not in (0, 1):
            print("DIFF3-ERROR", rel, merged.stderr.strip())
            problems += 1
            continue
        text = merged.stdout
        if conflicts:
            print(f"CONFLICTS x{conflicts} {rel} -> {'overlay side' if rel in OVERLAY_WINS else 'UNRESOLVED'}")
            if rel not in OVERLAY_WINS:
                problems += 1
                continue
            text = overlay_side(text)
        (target / rel).parent.mkdir(parents=True, exist_ok=True)
        (target / rel).write_text(text)
    for rel in CREATED:
        (target / rel).parent.mkdir(parents=True, exist_ok=True)
        (target / rel).write_bytes((OVERLAY / rel).read_bytes())
    print(f"rebased {len(EDITED)} edited, {len(CREATED)} created, {len(REMOVED)} removed; problems={problems}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
