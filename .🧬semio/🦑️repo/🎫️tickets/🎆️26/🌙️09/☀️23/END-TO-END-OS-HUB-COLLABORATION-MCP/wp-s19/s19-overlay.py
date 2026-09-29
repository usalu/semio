#!/usr/bin/env python3
"""🪞️ S19 overlay: APFS-clones every tracked + untracked-unignored file outside `.🧬semio/` and every gitignored `🤖️generated`
directory into the overlay root, then re-syncs changed files on later runs (size/mtime). `node_modules` is a symlink.
Every sync also prunes overlay files the tree no longer has (outside the overlay's own `.s19-*`, `node_modules`, `.🧬semio`)
and mirrors `node_modules` (top level → the tree's entries, `@semio-tech/*` → the tree's relative links, so workspace packages
resolve INSIDE the overlay). `--reset-staged` puts every payload path back to the tree's content (a create the tree lacks is
removed) and rewrites its `.old` base, so the codemods re-run on the current tree.
usage: s19-overlay.py <overlay-root> [--reset-staged] [--no-protect]"""
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
protected = {entry["path"] for entry in json.load(open(manifest, encoding="utf-8"))} if os.path.exists(manifest) and "--no-protect" not in sys.argv else set()
protect_list = os.path.join(here, "s19-overlay-protect.txt")
if os.path.exists(protect_list) and "--no-protect" not in sys.argv:
    protected |= {line.strip() for line in open(protect_list, encoding="utf-8") if line.strip() and not line.startswith("#")}
if "--reset-staged" in sys.argv and os.path.exists(manifest):
    for entry in json.load(open(manifest, encoding="utf-8")):
        source, destination = os.path.join(repo, entry["path"]), os.path.join(target, entry["path"])
        old = os.path.join(here, "payload", f"{entry['id']}.old")
        for stale in (destination, old, os.path.join(here, "payload", f"{entry['id']}.new")):
            if os.path.lexists(stale):
                os.unlink(stale)
        if os.path.exists(source):
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            clonefile(source.encode(), destination.encode(), 0)
            with open(source, encoding="utf-8") as handle, open(old, "w", encoding="utf-8") as out:
                out.write(handle.read())
        entry_mode = "modify" if os.path.exists(source) else "create"
        print(f"reset {entry['id']} {entry_mode} {entry['path']}")
placed = sum(place(r) for r in rels if r not in protected)
keep = set(rels) | protected
OWN = (".s19-build", ".s19-target", ".s19-emitter", "node_modules", ".🧬semio", "target")
pruned = 0
for base, dirs, files in os.walk(target):
    rel_base = os.path.relpath(base, target)
    if rel_base == ".":
        dirs[:] = [d for d in dirs if d not in OWN]
    for name in files:
        rel = os.path.normpath(os.path.join(rel_base, name))
        if rel not in keep and rel not in OWN:
            os.unlink(os.path.join(base, name))
            pruned += 1
modules, overlay_modules = os.path.join(repo, "node_modules"), os.path.join(target, "node_modules")
if os.path.islink(overlay_modules):
    os.unlink(overlay_modules)
os.makedirs(os.path.join(overlay_modules, "@semio-tech"), exist_ok=True)
for name in os.listdir(modules):
    link = os.path.join(overlay_modules, name)
    if name == "@semio-tech" or os.path.lexists(link):
        continue
    os.symlink(os.path.join(modules, name), link)
for name in os.listdir(os.path.join(modules, "@semio-tech")):
    source, link = os.path.join(modules, "@semio-tech", name), os.path.join(overlay_modules, "@semio-tech", name)
    wanted = os.readlink(source) if os.path.islink(source) else source
    if os.path.lexists(link):
        if os.path.islink(link) and os.readlink(link) == wanted:
            continue
        os.unlink(link)
    os.symlink(wanted, link)
print(f"s19-overlay: {len(rels)} files considered, {placed} placed, {pruned} pruned, {len(ignored_dirs)} generated entries → {target}")
