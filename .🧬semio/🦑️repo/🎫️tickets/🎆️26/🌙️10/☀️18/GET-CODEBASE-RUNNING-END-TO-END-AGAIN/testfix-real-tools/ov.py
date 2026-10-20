#!/usr/bin/env python3
"""Builds the overlay /private/tmp/testfix-real/r as a COPY of the listed live subtrees (tracked + untracked, non-ignored files) and records the sha256 of every copied file in ../ov-index.json. usage: ov.py [--refresh]"""
import hashlib, json, os, shutil, subprocess, sys
R = "/Users/ueli/Documents/semio"
O = "/private/tmp/testfix-real/r"
T = os.path.dirname(os.path.abspath(__file__))
PREFIX = ["🧰️framework", "🧪️tests", "✏️s/🧪️tests", "✏️s/🔨️modules/🏗️fem", "🎓️teaching/🛂️proctor", "✏️s/🔌️plugins/🗄️stdio"]
ROOTFILES = ["package.json", "tsconfig.json", "bunfig.toml", "Cargo.toml", "rust-toolchain.toml", ".gitignore", "✏️s/Cargo.toml", "✏️s/Cargo.lock", "✏️s/package.json"]
def sha(b): return hashlib.sha256(b).hexdigest()
def main():
    if "--refresh" in sys.argv: shutil.rmtree(O, ignore_errors=True)
    os.makedirs(O, exist_ok=True)
    files = list(ROOTFILES)
    for p in PREFIX:
        out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", p], cwd=R, capture_output=True).stdout.decode().split("\0")
        files += [f for f in out if f]
    out = subprocess.run(["git", "ls-files", "-z", "*Cargo.lock"], cwd=R, capture_output=True).stdout.decode().split("\0")
    files += [f for f in out if f]
    idx = {}
    for f in sorted(set(files)):
        s = R + "/" + f
        if not os.path.isfile(s) or os.path.islink(s): continue
        d = O + "/" + f
        os.makedirs(os.path.dirname(d), exist_ok=True)
        data = open(s, "rb").read()
        open(d, "wb").write(data)
        idx[f] = sha(data)
    json.dump(idx, open(T + "/ov-index.json", "w"), ensure_ascii=False, indent=0)
    print(len(idx), "files copied")
main()
