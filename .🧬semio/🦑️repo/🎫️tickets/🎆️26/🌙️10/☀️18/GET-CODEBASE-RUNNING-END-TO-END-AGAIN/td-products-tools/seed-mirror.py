#!/usr/bin/env python3
"""🌱 Seeds a mirror (C/mirror-tools/mirror.py) with the gitignored inputs a clean checkout generates and a mirror therefore lacks: generated sources (`🤖️generated`, `🔤️tokens`, `pkg`, `🕸️bindings`, `Cargo.lock`), the wasm package folders and `.github`.
usage: python3 -I seed-mirror.py <mirror-root> [--dry]   (ordinary copies, APFS clones where the volume allows; never symlinks, never touches the live tree)"""
import os, shutil, subprocess, sys
SKIP_PARTS = {"node_modules", ".cache", "dist", ".venv", "__pycache__", "bin", "obj", ".🧾️store", ".DS_Store", ".git"}
here = os.path.dirname(os.path.abspath(__file__))
live = os.path.abspath(os.path.join(here, *([".."] * 8)))
mirror = os.path.abspath(sys.argv[1]); dry = "--dry" in sys.argv
listing = subprocess.run(["git", "ls-files", "-o", "-i", "--exclude-standard", "--directory", "-z", "--", "🧰️framework", "🎓️teaching"], cwd=live, check=True, capture_output=True).stdout.decode().split("\0")
picked = []
for entry in filter(None, listing):
    parts = entry.rstrip("/").split("/")
    if any(part in SKIP_PARTS for part in parts) or (len(parts) == 2 and parts[1].startswith(".")): continue
    picked.append(entry.rstrip("/"))
picked.append(".github")
copied = 0
for rel in picked:
    source, target = os.path.join(live, rel), os.path.join(mirror, rel)
    if not os.path.exists(source) or (os.path.exists(target) and rel == ".github"): continue
    copied += 1
    if dry: print(rel); continue
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if os.path.isdir(source):
        shutil.rmtree(target, ignore_errors=True)
        if subprocess.run(["cp", "-Rc", source, target], capture_output=True).returncode != 0: shutil.copytree(source, target, symlinks=False, ignore=shutil.ignore_patterns("node_modules"))
    else: shutil.copyfile(source, target); shutil.copymode(source, target)
    for folder, names, _ in os.walk(target):
        for name in list(names):
            link = os.path.join(folder, name)
            if os.path.islink(link): os.unlink(link); names.remove(name)
print(f"seeded {copied} entries into {mirror}", file=sys.stderr)
