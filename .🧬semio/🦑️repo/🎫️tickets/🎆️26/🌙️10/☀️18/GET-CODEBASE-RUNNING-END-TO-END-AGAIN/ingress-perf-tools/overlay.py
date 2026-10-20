#!/usr/bin/env python3
"""🧱 Builds a COPY overlay of the module closure of the given entry files under an output directory (never symlinks)."""
import json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isfile(os.path.join(ROOT, "bunfig.toml")):
    ROOT = os.path.dirname(ROOT)

def closure(entries):
    files = set()
    for entry in entries:
        with tempfile.TemporaryDirectory(dir=os.path.join(HERE,"..","🗑️generated")) as tmp:
            meta = os.path.join(tmp, "m.json")
            subprocess.run(["bun", "build", "--target=bun", "--outdir="+tmp, "--metafile="+meta, "./"+entry], check=True, stdout=subprocess.DEVNULL, cwd=ROOT)
            files.update(json.load(open(meta))["inputs"].keys())
    return sorted(files)

def main():
    out, entries = sys.argv[1], sys.argv[2:]
    os.makedirs(out, exist_ok=True)
    count = 0
    for rel in closure(entries):
        src = os.path.join(ROOT, rel)
        if not os.path.isfile(src):
            continue
        dst = os.path.join(out, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)
        count += 1
    for rel in ("package.json", "bunfig.toml", "tsconfig.json"):
        if os.path.isfile(os.path.join(ROOT, rel)):
            shutil.copyfile(os.path.join(ROOT, rel), os.path.join(out, rel))
    print(f"copied {count} files to {out}")

main()
