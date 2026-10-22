#!/usr/bin/env python3
"""Frozen, read-only-by-convention copy of the tracked+untracked-non-ignored workspace files (without tickets) so an experiment sees one stable tree while peers keep editing the live one.
usage: snapshot.py [DEST]   (default /private/tmp/graph-parallel/ws)   writes DEST/../files.json (the file list)
Stdlib only; symlinks stay symlinks; node_modules is linked, not copied."""
import json, os, pathlib, shutil, subprocess, sys

dest = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/graph-parallel/ws").resolve()
root = pathlib.Path(__file__).resolve()
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
out = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "-c", "-o", "--exclude-standard"], cwd=root, capture_output=True).stdout.split(b"\0")
files = sorted(f.decode() for f in out if f and not f.decode().startswith(".\U0001F9EC" + "semio/"))
files = [f for f in files if not f.startswith(".git/") and (root / f).is_file() or (root / f).is_symlink()]
copied = 0
for f in files:
    src, dst = root / f, dest / f
    dst.parent.mkdir(parents=True, exist_ok=True)
    if src.is_symlink():
        if not dst.is_symlink():
            os.symlink(os.readlink(src), dst)
    elif not dst.exists() or dst.stat().st_size != src.stat().st_size:
        shutil.copyfile(src, dst)
        shutil.copystat(src, dst)
        copied += 1
link = dest / "node_modules"
if not link.exists() and (root / "node_modules").exists():
    os.symlink(root / "node_modules", link)
(dest.parent / "files.json").write_text(json.dumps(files), encoding="utf8")
print("files", len(files), "copied", copied, "->", dest)
