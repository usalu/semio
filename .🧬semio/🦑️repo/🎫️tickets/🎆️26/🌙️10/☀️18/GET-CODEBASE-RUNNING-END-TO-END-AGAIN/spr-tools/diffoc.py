#!/usr/bin/env python3
"""🔬 SPR patched differential (descendant of c5b oracle5.py / RESOLUTION-ORACLE.md): one crate, HEAD sandbox against working-tree sandbox.
`rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from the crate lib root; `extern crate` lines and external globs neutralised identically on both sides; error multisets compared by (code, message, file) without line numbers.
usage: diffoc.py <name> <crate_snake> <root.rs> <copydir>... [--test] [--show-base]   (paths relative to the repo; writes only inside the sandbox)"""
import collections, os, re, shutil, subprocess, sys
REPO = "/Users/ueli/Documents/semio"
SBX = "/Users/ueli/.cache/spr/sbx"
TOOLCHAIN = "+nightly-2026-07-20"
SKIP = {"target", "node_modules", ".git", "🗑️generated", "dist", "pkg"}
LOCAL = {"crate", "self", "super", "std", "core", "alloc"}

def under(path):
    assert os.path.realpath(path).startswith(os.path.realpath(SBX) + os.sep), path

def head_files(copydir):
    out = subprocess.run(["git", "ls-tree", "-r", "-z", "--name-only", "HEAD", "--", copydir], cwd=REPO, capture_output=True).stdout.decode("utf-8")
    return [p for p in out.split("\0") if p]

def head_blob(path):
    return subprocess.run(["git", "show", "HEAD:" + path], cwd=REPO, capture_output=True).stdout

def tree_files(copydir):
    base = os.path.join(REPO, copydir)
    found = []
    for directory, names, files in os.walk(base):
        names[:] = [n for n in names if n not in SKIP]
        for file in files:
            found.append(os.path.relpath(os.path.join(directory, file), REPO))
    return found

def patch(text):
    text = re.sub(r"(?m)^\s*(pub(\([^)]*\))?\s+)?extern crate [^;]+;\s*$", "", text)
    return re.sub(r"(?m)^(\s*)(pub(\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;", lambda m: m.group(0) if m.group(4) in LOCAL else m.group(1) + "const _: () = ();", text)

def closure(read, rootrel):
    seen, queue = set(), [rootrel]
    while queue:
        rel = queue.pop()
        if rel in seen:
            continue
        data = read(rel)
        if data is None:
            continue
        seen.add(rel)
        if not rel.endswith(".rs"):
            continue
        text = data.decode("utf-8", "replace")
        for match in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)', text):
            target = match.group(1) or match.group(2)
            queue.append(os.path.normpath(os.path.join(os.path.dirname(rel), target)))
    return sorted(seen)

def build(name, side, copydirs, rootrel):
    root = os.path.join(SBX, name, side)
    under(root)
    shutil.rmtree(root, ignore_errors=True)
    if copydirs:
        listing = [head_files(d) if side == "base" else tree_files(d) for d in copydirs]
    else:
        reader = (lambda rel: head_blob(rel) or None) if side == "base" else (lambda rel: open(os.path.join(REPO, rel), "rb").read() if os.path.isfile(os.path.join(REPO, rel)) else None)
        listing = [closure(reader, rootrel)]
    for files in listing:
        for rel in files:
            destination = os.path.join(root, rel)
            under(os.path.dirname(destination))
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            data = head_blob(rel) if side == "base" else open(os.path.join(REPO, rel), "rb").read()
            if rel.endswith(".rs"):
                data = patch(data.decode("utf-8")).encode("utf-8")
            open(destination, "wb").write(data)
    return root

def run(root, crate, rootfile, flags):
    command = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "-A", "warnings", "--error-format=short"] + flags + [os.path.join(root, rootfile)]
    result = subprocess.run(command, capture_output=True, text=True, cwd=root)
    return result.stderr

def parse(stderr, root):
    rows = collections.Counter()
    for line in stderr.splitlines():
        match = re.match(r"^(.*?):(\d+):(\d+): (error(?:\[E\d+\])?: .*)$", line)
        if match:
            path = os.path.relpath(match.group(1), root) if match.group(1).startswith("/") else match.group(1)
            rows[(path, match.group(4))] += 1
    return rows

def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    flags = ["--test"] if "--test" in sys.argv else []
    name, crate, rootfile, copydirs = args[0], args[1], args[2], args[3:]
    results = {}
    for side in ("base", "cur"):
        root = build(name, side, copydirs, rootfile)
        for seed in [a[7:] for a in sys.argv if a.startswith("--seed=")]:
            if side == "cur":
                relative, old, new = seed.split("|")
                target = os.path.join(root, relative)
                text = open(target, encoding="utf-8").read()
                assert old in text, ("seed absent", old)
                open(target, "w", encoding="utf-8").write(text.replace(old, new, 1))
        stderr = run(root, crate, rootfile, flags)
        under(os.path.join(SBX, name))
        open(os.path.join(SBX, name, side + ".err"), "w", encoding="utf-8").write(stderr)
        results[side] = parse(stderr, root)
    base, cur = results["base"], results["cur"]
    print(f"{name}: base errors {sum(base.values())}, cur errors {sum(cur.values())}")
    new = cur - base
    gone = base - cur
    print(f"-- new in cur ({sum(new.values())})")
    for (path, message), count in sorted(new.items()):
        print(f"  +{count} {path}: {message}")
    if "--show-gone" in sys.argv:
        print(f"-- gone from cur ({sum(gone.values())})")
        for (path, message), count in sorted(gone.items()):
            print(f"  -{count} {path}: {message}")

main()
