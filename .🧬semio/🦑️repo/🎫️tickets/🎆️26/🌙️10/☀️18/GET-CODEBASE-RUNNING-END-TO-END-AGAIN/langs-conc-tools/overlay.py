#!/usr/bin/env python3
# 🧪 Overlay checkout for the langs-conc verification: every entry is a symlink to the live checkout until a write needs it, then the chain of parents becomes real directories (siblings stay links) and the file a private copy. Nothing is ever written through a link.
import os, shutil, subprocess, sys
REAL = "/Users/ueli/Documents/semio"
def base(overlay):
    os.makedirs(overlay, exist_ok=True)
    for name in os.listdir(REAL):
        if name in (".git", "bazel-out", "dist", "temp", "holder", "undefined"): continue
        target = os.path.join(overlay, name)
        if not os.path.lexists(target): os.symlink(os.path.join(REAL, name), target)
COPIES = ["🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/🖱️ui/🎨️styling", "🧰️framework/📦️packages/🟦️typescript/🏃️process", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", "✏️s/🔌️plugins/🔋️energy/🔮️oracles"]
SKIP = shutil.ignore_patterns(".venv", "dist", "obj", "bin", ".git", "target", ".cache", "__pycache__")
def copies(overlay):
    for rel in COPIES:
        destination = os.path.join(overlay, rel)
        materialize_parent(overlay, destination)
        if os.path.islink(destination) or not os.path.lexists(destination):
            if os.path.islink(destination): os.unlink(destination)
            shutil.copytree(os.path.join(REAL, rel), destination, symlinks=True, ignore=SKIP)
def isolated_git(overlay):
    # 🔒 Gives the overlay a git repository of its own whose index mirrors the tracked paths of the checkout; nothing here may touch the real .git, so the environment is stripped of every GIT_* variable and each command is pinned to the overlay.
    environment = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    git = lambda *args, **kw: subprocess.run(["git", *args], env=environment, capture_output=True, **kw)
    if not os.path.isdir(os.path.join(overlay, ".git")):
        if os.path.lexists(os.path.join(overlay, ".git")): os.unlink(os.path.join(overlay, ".git"))
        git("init", "-q", overlay)
    top = git("-C", overlay, "rev-parse", "--show-toplevel", "--git-dir").stdout.decode().split("\n")
    if os.path.realpath(top[0]) != os.path.realpath(overlay) or os.path.realpath(os.path.join(overlay, top[1])) != os.path.realpath(os.path.join(overlay, ".git")): raise SystemExit("REFUSED: overlay git repository is not isolated: " + repr(top))
    listing = git("-C", REAL, "ls-files", "-s", "-z").stdout.split(b"\0")
    prefix = ".🧬semio/".encode("utf8")
    kept = b"\0".join(row for row in listing if row and not row.split(b"\t", 1)[1].startswith(prefix)) + b"\0"
    done = git("-C", overlay, "update-index", "-z", "--index-info", input=kept)
    if done.returncode != 0: raise SystemExit("overlay index failed: " + done.stderr.decode()[:300])
def relink(overlay, link):
    target = os.path.realpath(link)
    if os.path.islink(link): os.unlink(link)
    os.makedirs(link, exist_ok=True)
    for name in os.listdir(target):
        source = os.path.join(target, name)
        if name.startswith("@") and os.path.isdir(source) and not os.path.islink(source):
            relink_scope(overlay, source, os.path.join(link, name))
        else:
            os.symlink(source, os.path.join(link, name))
def relink_scope(overlay, source, destination):
    os.makedirs(destination, exist_ok=True)
    for name in os.listdir(source):
        point = os.path.join(source, name)
        if os.path.islink(point):
            where = os.readlink(point)
            if not os.path.isabs(where): where = os.path.normpath(os.path.join(source, where))
            if where.startswith(REAL + "/"): where = overlay + where[len(REAL):]
            os.symlink(where, os.path.join(destination, name))
        else: os.symlink(point, os.path.join(destination, name))
def node_modules(overlay):
    link = os.path.join(overlay, "node_modules")
    if os.path.islink(link): relink(overlay, link)
def real_chain(overlay, path):
    rel = os.path.relpath(path, overlay)
    if rel.startswith(".."): raise SystemExit("outside overlay: " + path)
    return rel.split(os.sep)
def materialize_parent(overlay, path):
    cur_ov, cur_real = overlay, REAL
    for part in real_chain(overlay, path)[:-1]:
        nxt = os.path.join(cur_ov, part)
        if os.path.islink(nxt):
            target = os.path.realpath(nxt); os.unlink(nxt); os.makedirs(nxt)
            for child in os.listdir(target): os.symlink(os.path.join(target, child), os.path.join(nxt, child))
        elif not os.path.lexists(nxt): os.makedirs(nxt)
        modules = os.path.join(nxt, "node_modules")
        if os.path.islink(modules) and os.path.realpath(modules).startswith(REAL + "/") and os.path.isdir(modules): relink(overlay, modules)
        cur_ov = nxt; cur_real = os.path.join(cur_real, part)
def write(overlay, path, data):
    materialize_parent(overlay, path)
    if os.path.islink(path): os.unlink(path)
    real = os.path.realpath(os.path.dirname(path))
    if not real.startswith(os.path.realpath(overlay) + os.sep) and real != os.path.realpath(overlay): raise SystemExit("REFUSED: write would leave the overlay: " + path)
    with open(path, "wb") as out: out.write(data)
def remove(overlay, path):
    materialize_parent(overlay, path)
    if os.path.lexists(path): os.unlink(path)
if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/langs-conc/ov"
    base(target); copies(target); node_modules(target); isolated_git(target); print("overlay ready")
