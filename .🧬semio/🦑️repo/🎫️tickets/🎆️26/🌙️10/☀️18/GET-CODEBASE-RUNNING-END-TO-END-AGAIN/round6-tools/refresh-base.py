#!/usr/bin/env python3
"""Brings the pristine overlay /private/tmp/i6/base up to the live checkout (peers keep editing): files whose size or mtime differ are cloned again, files that left the checkout are removed, the private git index is refreshed, tool scripts are re-shimmed.
Read-only on the checkout. usage: refresh-base.py [--dry]   prints how many paths changed and which build-library ones."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import overlay as O

BASE = "/private/tmp/i6/base"
O.OV = BASE
O.SHIM = "/private/tmp/i6/ov"
LIVE = O.LIVE
rows = set(O.files())
changed, removed = [], []
for rel in rows:
    src, dst = os.path.join(LIVE, rel), os.path.join(BASE, rel)
    try:
        a = os.lstat(src)
    except OSError:
        continue
    try:
        b = os.lstat(dst)
        same = a.st_size == b.st_size and a.st_mtime_ns == b.st_mtime_ns
    except OSError:
        same = False
    if not same:
        changed.append(rel)
for base, dirs, files in os.walk(BASE):
    dirs[:] = [d for d in dirs if d not in (".git", "node_modules", ".nx")]
    for f in files:
        rel = os.path.relpath(os.path.join(base, f), BASE)
        if rel not in rows and not rel.startswith(O.CREL + "/"):
            removed.append(rel)
        elif rel.startswith(O.CREL + "/") and not any(x in rel for x in ("/r6-", "/round6-tools/", "/\U0001F5D1\ufe0fgenerated/", "/backup", "/__pycache__/")) and not os.path.lexists(os.path.join(LIVE, rel)):
            removed.append(rel)
lib = [r for r in changed if "/📚️library/🏗️build/" in r or "/🏃️process/" in r]
print("changed %d (build library / process %d), removed %d" % (len(changed), len(lib), len(removed)))
for r in lib[:40]:
    print("  ", r[-110:])
if "--dry" in sys.argv:
    sys.exit(0)
for rel in removed:
    os.remove(os.path.join(BASE, rel))
for rel in changed:
    dst = os.path.join(BASE, rel)
    if os.path.lexists(dst):
        os.chmod(dst, 0o644) if os.path.isfile(dst) and not os.path.islink(dst) else None
        os.remove(dst)
O.copy_all(changed)
extra = []
for base_dir, dirs, fs in os.walk(O.C):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "__pycache__", "\U0001F5D1\ufe0fgenerated", "round6-tools") and not d.startswith("r6-") and not d.startswith("backup")]
    for f in fs:
        src = os.path.join(base_dir, f)
        rel = os.path.relpath(src, LIVE)
        try:
            a, b = os.lstat(src), os.lstat(os.path.join(BASE, rel))
            if a.st_size == b.st_size and a.st_mtime_ns == b.st_mtime_ns:
                continue
        except OSError:
            pass
        extra.append(rel)
for rel in extra:
    dst = os.path.join(BASE, rel)
    if os.path.lexists(dst):
        os.remove(dst)
print("ticket files (ignored by git) refreshed:", O.copy_all(extra))
O.fill_ticket()
subprocess.run(["git", "add", "-A"], cwd=BASE, capture_output=True)
print("shimmed", O.shim())
