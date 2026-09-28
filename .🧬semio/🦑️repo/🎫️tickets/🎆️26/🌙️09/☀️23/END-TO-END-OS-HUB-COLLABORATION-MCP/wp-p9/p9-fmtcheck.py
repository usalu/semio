#!/usr/bin/env python3
"""🎨️ Proves every Rust line P9's patch set adds is rustfmt-stable: loads the set's hunks (`patches/p9-agent-lane.py`), formats
each touched stage file with the repo `rustfmt.toml` (`skip_children`) and lists every added line rustfmt would re-wrap.
Exit 1 when any added line is unstable. Usage: p9-fmtcheck.py"""
import subprocess
import sys
import types
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
STAGE = TREE / ".🧬semio/🌐hub/s14-p9-stage/stage"
PATCH = Path(__file__).parent / "patches/p9-agent-lane.py"

hunks = []
module = types.ModuleType("p9_patch")
module.ROOT = TREE
module.replace = lambda part, path, old, new, count=1: hunks.append((str(path.relative_to(TREE)), old, new))
module.create = lambda part, path, content: hunks.append((str(path.relative_to(TREE)), "", content))
module.finish = lambda doc: None
sys.modules["p9_patch"] = module
exec(compile(PATCH.read_text(encoding="utf-8"), str(PATCH), "exec"), {"__name__": "p9"})

added: dict = {}
for rel, old, new in hunks:
    if rel.endswith(".rs"):
        old_lines = set(old.splitlines())
        added.setdefault(rel, []).extend(line for line in new.splitlines() if line.strip() and line not in old_lines)
unstable = 0
for rel, lines in added.items():
    source = (STAGE / rel).read_text(encoding="utf-8")
    run = subprocess.run(["rustfmt", "--edition", "2021", "--config-path", str(TREE / "rustfmt.toml"), "--config", "skip_children=true", "--emit", "stdout", "-q"], input=source, capture_output=True, text=True)
    if run.returncode != 0:
        print(f"rustfmt failed on {rel}: {run.stderr[:300]}")
        unstable += 1
        continue
    formatted = set(run.stdout.splitlines())
    bad = [line for line in lines if line not in formatted]
    unstable += len(bad)
    for line in bad:
        print(f"{rel[-70:]}: {line.strip()[:160]}")
print(f"p9-fmtcheck: {sum(len(lines) for lines in added.values())} added line(s) in {len(added)} file(s), {unstable} unstable")
sys.exit(1 if unstable else 0)
