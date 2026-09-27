#!/usr/bin/env python3
"""🪞️ WG11: runs one prepared patch script against WG11's overlay instead of the tree — every absolute repo path in it (and in the
sibling scripts it runs via `with_name("…py")`) rewritten to the overlay — first as a dry run, then with `--apply`.
Usage: python3 wg11-overlay-apply.py <overlayRoot> <patch.py>"""
import re
import subprocess
import sys
from pathlib import Path

REPO = "/Users/ueli/Documents/semio"
overlay, script = sys.argv[1], Path(sys.argv[2]).resolve()
staging = Path(overlay) / ".wg11-patches" / script.parent.name
staging.mkdir(parents=True, exist_ok=True)


def stage(path: Path) -> Path:
    source = path.read_text(encoding="utf-8")
    rewritten = re.sub(r'"' + re.escape(REPO) + r'(?=["/])', lambda _: f'"{overlay}', source)
    if rewritten == source:
        sys.exit(f"{path.name}: no absolute repo path to rewrite")
    target = staging / path.name
    target.write_text(rewritten, encoding="utf-8")
    for sibling in re.findall(r'with_name\("([^"]+\.py)"\)', source):
        stage(path.with_name(sibling))
    return target


staged = stage(script)
for args in ([], ["--apply"]):
    result = subprocess.run([sys.executable, str(staged), *args], capture_output=True, text=True)
    tail = (result.stdout + result.stderr).strip().splitlines()[-1:] or [""]
    print(f"[wg11-ov] {script.name} {'apply' if args else 'dry-run'} rc={result.returncode} {tail[0]}")
    if result.returncode != 0:
        sys.exit(1)
