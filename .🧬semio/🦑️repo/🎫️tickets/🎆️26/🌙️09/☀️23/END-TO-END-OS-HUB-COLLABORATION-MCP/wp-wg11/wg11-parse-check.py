#!/usr/bin/env python3
"""🧪️ WG11: parse-checks a prepared patch's PATCHED Rust files without touching the tree — each patched file (and each new .rs
file) is written to a scratch directory and run through nightly rustfmt (`skip_children`, so `#[path]` modules are not followed);
a rustfmt `error` is a parse error of the patched text. Usage: wg11-parse-check.py <patch-script.py> <scratch-dir>"""
import importlib.util
import subprocess
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("patch", sys.argv[1])
patch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(patch)
scratch = Path(sys.argv[2])
scratch.mkdir(parents=True, exist_ok=True)
files = []
for index, (path, edits) in enumerate(patch.EDITS.items()):
    files.append((f"{index:02d}-{path.parent.name.encode('ascii', 'ignore').decode() or 'x'}.rs", patch.replaced(path, path.read_text(encoding="utf-8"), edits)))
for index, (path, content) in enumerate(getattr(patch, "NEW_FILES", {}).items()):
    if path.suffix == ".rs":
        files.append((f"new-{index:02d}.rs", content))
failed = 0
for name, content in files:
    target = scratch / name
    target.write_text(content, encoding="utf-8")
    result = subprocess.run(["rustfmt", "--edition", "2021", "--config", "skip_children=true,max_width=250", "--check", str(target)], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True)
    errors = [line for line in (result.stderr + result.stdout).splitlines() if line.startswith("error")]
    failed += bool(errors)
    print(f"{name}: {'PARSE ERROR ' + errors[0] if errors else 'parses'} (rustfmt rc={result.returncode})")
    for line in errors[1:4]:
        print("   ", line)
print(f"\n{len(files)} files, {failed} with parse errors")
sys.exit(1 if failed else 0)
