#!/usr/bin/env python3
"""🛫️ T14 overlay preflight before queueing a hold: (1) every set's dry run on the overlay must say it is applied ("nothing to do"),
(2) no `use`/`mod`/`import`/`macro_rules!`/`fn`/`struct`/`const` line and no spliced `label:` appears more often in an overlay
file than in its live twin, (3) writes the changed-file listing `generated/s14b-overlay-changed-<n>.txt` (overlay files newer
than the sync stamp) that `land-w3.py` compares against. usage: overlay-preflight.py <n>"""
import os
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
O = ROOT / ".🧬semio/🌐hub/s14-t14-overlay"
T = ROOT / ".tmp-ticket/wp-t14"
number = sys.argv[1]
env = dict(os.environ, P8_ROOT=str(O))
SETS = [
    ("f9", ["python3", T / "f9/content-id.py", "--root", O]),
    ("record", ["python3", T / "f9/record-instrument.py", "--root", O]),
    ("g12", ["python3", ROOT / ".tmp-ticket/wp-g12/g12-authoring-seed.py", "--dry-run", "--root", O]),
    ("follow", ["python3", T / "p8class/follow-children.py", "--root", O]),
    ("orphan", ["python3", T / "verify-applied-p8.py", ROOT / ".tmp-ticket/wp-p8/patches/p8-orphan.py", O]),
    ("h9l", ["python3", T / "h9l/kind-label-patch.py", "--root", O]),
    ("5b", ["python3", T / "5b/dsl-value.py", "--root", O]),
    ("5b2", ["python3", T / "5b/dsl-value-b2.py", "--root", O]),
    ("item6", ["python3", T / "item6/fallback-wrappers.py", "--root", O]),
    ("plugin-tests", ["python3", T / "plugin-tests/plugin-lib-tests.py", "--root", O]),
    ("sdk-reds", ["python3", T / "sdk-reds/sdk-lib-reds.py", "--root", O]),
]
for name, command in SETS:
    result = subprocess.run([str(part) for part in command], cwd=ROOT, env=env, capture_output=True, text=True)
    print(f"## {name} rc={result.returncode}")
    print("\n".join((result.stdout + result.stderr).strip().splitlines()[-2:]))
prune = [arg for name in ("node_modules", "dist", "target", "🗑️generated", "generated", ".venv", ".t14-build", ".t14-target", ".t14-trash") for arg in ("-o", "-name", name)][1:]
found = subprocess.run(["find", str(O), "(", *prune, ")", "-prune", "-o", "-newer", str(O / ".t14-stamp"), "-type", "f", "-print0"], capture_output=True).stdout.decode("utf-8", "replace").split("\0")
changed = sorted(os.path.relpath(path, O) for path in found if path)
(T / f"generated/s14b-overlay-changed-{number}.txt").write_text("\n".join(changed) + "\n", encoding="utf-8")
KIND = re.compile(r"^\s*(pub(\([^)]*\))? )?(use |mod |import |export \{|macro_rules!|(async )?fn |struct |enum |const |static )")
SPLICE = re.compile(r"LocalizedLabel::native\([^)]*\),\s*\w*_?plugin::LocalizedLabel|label: [^\n]*label:")
problems = 0
for rel in changed:
    if not rel.endswith((".rs", ".ts", ".tsx")):
        continue
    new = (O / rel).read_text(encoding="utf-8", errors="replace").splitlines()
    old = (ROOT / rel).read_text(encoding="utf-8", errors="replace").splitlines() if (ROOT / rel).is_file() else []
    before, after = Counter(line.strip() for line in old if KIND.match(line)), Counter(line.strip() for line in new if KIND.match(line))
    for line, count in after.items():
        if count > 1 and count > before.get(line, 0) and not line.startswith(("fn main", "use super::*")):
            problems += 1
            print(f"INCREASED-DUPLICATE {rel}: {before.get(line, 0)} -> {count}x {line[:140]}")
    live = set(old)
    for index, line in enumerate(new, 1):
        if SPLICE.search(line) and line not in live:
            problems += 1
            print(f"SPLICED {rel}:{index}: {line.strip()[:160]}")
print(f"changed files: {len(changed)} (listing s14b-overlay-changed-{number}.txt); duplicate/splice findings: {problems}")
