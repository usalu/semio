#!/usr/bin/env python3
"""usage: scoped-oracle.py <sandbox-dir> <artifact-dir> <crate-snake> <lib-root-relative-to-artifact> <restore-spec>...
restore-spec: <path-relative-to-repo>=<git-ref> : in sandbox A that file is restored from the git ref; sandbox B keeps the working tree.
Runs the analysis-only resolution oracle (RESOLUTION-ORACLE.md) lib and --test on A and B and compares the error multisets."""
import os, re, shutil, subprocess, sys, collections
REPO = "/Users/ueli/Documents/semio"
SBX, ART, CRATE, ROOT = sys.argv[1:5]
SPECS = [s.split("=", 1) for s in sys.argv[5:]]
TOOLCHAIN = "+nightly-2026-07-20"
SBX = os.path.realpath(SBX)
assert os.path.realpath(SBX).startswith("/private/tmp/claude-501/"), "sandbox must be outside the repository"
IGNORE = shutil.ignore_patterns("target", "node_modules", ".git", "🗑️generated")
rx_ext = re.compile(r"^\s*extern\s+crate\s+\w+\s+as\s+\w+;[ \t]*$", re.M)
rx_glob = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)

def neutralize(tree):
    mods = set()
    files = []
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith(".rs"):
                files.append(os.path.join(dp, f))
    texts = {p: open(p, encoding="utf-8", errors="ignore").read() for p in files}
    for t in texts.values():
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", t, re.M):
            mods.add(m.group(1))
    keep = {"crate", "super", "self"} | mods
    for p, t in texts.items():
        t2 = rx_ext.sub("", t)
        t2 = rx_glob.sub(lambda m: m.group(0) if m.group(3) in keep else m.group(1) + "const _: () = ();", t2)
        if t2 != t:
            assert os.path.realpath(p).startswith(SBX)
            open(p, "w", encoding="utf-8").write(t2)

def build(name, restore):
    dst = os.path.join(SBX, name)
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(os.path.join(REPO, ART), os.path.join(dst, ART), symlinks=True, ignore=IGNORE)
    if restore:
        for path, ref in SPECS:
            text = subprocess.run(["git", "-c", "core.quotepath=false", "show", f"{ref}:{path}"], cwd=REPO, capture_output=True).stdout
            target = os.path.join(dst, path)
            assert os.path.realpath(os.path.dirname(target)).startswith(SBX)
            open(target, "wb").write(text)
    neutralize(dst)
    return dst

def run(dst, flags):
    root = os.path.join(dst, ART, ROOT)
    for attempt in range(6):
        r = subprocess.run(["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", CRATE, "--emit=metadata", "--error-format=short", "-o", os.path.join(dst, ".oracle.rmeta"), *flags, root], capture_output=True, text=True)
        miss = re.findall(r"couldn't find file `([^`]+)`", r.stderr)
        pulled = 0
        for m in miss:
            rel = os.path.relpath(os.path.normpath(m), dst)
            src = os.path.join(REPO, rel)
            if os.path.isfile(src) and not os.path.exists(m):
                os.makedirs(os.path.dirname(m), exist_ok=True)
                shutil.copyfile(src, m)
                pulled += 1
        if not pulled:
            break
    errs = []
    for line in r.stderr.splitlines():
        if ": error" in line or ": warning" in line:
            kind = "E" if ": error" in line else "W"
            msg = line.split(": ", 1)[1] if ": " in line else line
            errs.append(kind + " " + msg.replace(dst, "<sbx>"))
    return collections.Counter(errs)

os.makedirs(SBX, exist_ok=True)
a = build("A-original", True)
b = build("B-working", False)
ok = True
for name, flags in [("lib", []), ("test", ["--test"])]:
    ca, cb = run(a, flags), run(b, flags)
    only_b = cb - ca
    only_a = ca - cb
    print(f"{name}: original {sum(ca.values())} errors/warnings, working {sum(cb.values())}, only-working {sum(only_b.values())}, only-original {sum(only_a.values())}")
    for k, v in list(only_b.items())[:10]:
        print("   only-working x%d %s" % (v, k[:300]))
    for k, v in list(only_a.items())[:10]:
        print("   only-original x%d %s" % (v, k[:300]))
    ok = ok and not only_b
sys.exit(0 if ok else 1)
