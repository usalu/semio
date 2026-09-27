#!/usr/bin/env python3
"""🪞️ S19 overlay: APFS-clones every tracked + untracked-unignored file outside `.🧬semio/` and every gitignored `🤖️generated`
directory into the overlay root, then re-syncs changed files on later runs (size/mtime). `node_modules` is a symlink.
usage: s19-overlay.py <overlay-root> [--sync]"""
import ctypes, json, os, subprocess, sys

repo = "/Users/ueli/Documents/semio"
target = sys.argv[1]
libc = ctypes.CDLL("libSystem.B.dylib", use_errno=True)
clonefile = libc.clonefile
clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]

def git(*args):
    return subprocess.run(["git", *args], cwd=repo, capture_output=True, check=True).stdout.decode().split("\0")

def place(rel):
    source = os.path.join(repo, rel)
    if not os.path.lexists(source) or os.path.isdir(source) and not os.path.islink(source):
        return 0
    destination = os.path.join(target, rel)
    if os.path.lexists(destination):
        s, d = os.lstat(source), os.lstat(destination)
        if s.st_size == d.st_size and int(s.st_mtime) == int(d.st_mtime):
            return 0
        os.unlink(destination)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if os.path.islink(source):
        os.symlink(os.readlink(source), destination)
    elif clonefile(source.encode(), destination.encode(), 0) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {rel}")
    return 1

rels = [r for r in git("ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", ".", ":!:.🧬semio/**") if r]
GENERATED_SOURCES = ("🤖️generated", "🦀️.rs", "🔤️tokens", "🕸️bindings")
ignored = [r for r in git("ls-files", "-z", "--others", "--ignored", "--exclude-standard", "--directory", "--", ".", ":!:.🧬semio/**", ":!:node_modules/**", ":!:.tmp-ticket/**") if r]
ignored_dirs = [r.rstrip("/") for r in ignored if r.rstrip("/").split("/")[-1].startswith(GENERATED_SOURCES) and "/node_modules/" not in r and "/target/" not in r]
for d in ignored_dirs:
    full = os.path.join(repo, d)
    if os.path.isfile(full):
        rels.append(d)
        continue
    for base, _, files in os.walk(full):
        rels.extend(os.path.relpath(os.path.join(base, f), repo) for f in files)
here = os.path.dirname(os.path.abspath(__file__))
manifest = os.path.join(here, "payload", "manifest.json")
protected = {entry["path"] for entry in json.load(open(manifest, encoding="utf-8"))} if os.path.exists(manifest) else set()
protect_list = os.path.join(here, "s19-overlay-protect.txt")
if os.path.exists(protect_list):
    protected |= {line.strip() for line in open(protect_list, encoding="utf-8") if line.strip() and not line.startswith("#")}
placed = sum(place(r) for r in rels if r not in protected)
link = os.path.join(target, "node_modules")
if not os.path.lexists(link):
    os.symlink(os.path.join(repo, "node_modules"), link)
print(f"s19-overlay: {len(rels)} files considered, {placed} placed, {len(ignored_dirs)} generated entries → {target}")
