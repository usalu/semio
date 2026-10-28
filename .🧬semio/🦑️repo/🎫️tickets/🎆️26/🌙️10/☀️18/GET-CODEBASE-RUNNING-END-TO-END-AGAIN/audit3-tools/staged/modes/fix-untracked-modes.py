#!/usr/bin/env python3
"""🔓 Repairs the NEW (untracked, not ignored) source files a staging tool left at 0600/0700: `xplat5-tools/fix-modes.py` repairs tracked files from the index mode, but a file a batch added has no index entry yet, so it keeps the `mkstemp` mode until the next auto-commit records it (git keeps only the exec bit) and every later chmod flips its member mode in the membership frontier once more.
Rule: a regular file without group/other read gets them (`| 0o044`), an owner-executable one also gets group/other execute (`| 0o011`); `.🧬semio/` (tickets) and secret-looking names are skipped.
usage: python3 -I fix-untracked-modes.py --root <root> [--apply]   (dry run lists the files; exit 0)"""
import os, re, stat, subprocess, sys
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else sys.exit("--root required")
apply = "--apply" in sys.argv
SECRET = re.compile(r"(^|/)(\.env[^/]*|[^/]*(secret|token|credential|password|\.pem|\.key)[^/]*)$", re.I)
listing = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "-o", "--exclude-standard", "-z"], cwd=root, capture_output=True, check=True).stdout
changed = 0
for raw in listing.split(b"\0"):
    if not raw: continue
    rel = raw.decode("utf8", "surrogateescape")
    if rel.startswith(".🧬semio/") or SECRET.search(rel): continue
    path = os.path.join(root, rel)
    try: info = os.lstat(path)
    except FileNotFoundError: continue
    if not stat.S_ISREG(info.st_mode): continue
    mode = stat.S_IMODE(info.st_mode)
    if mode & 0o044 == 0o044: continue
    wanted = mode | 0o044 | (0o011 if mode & 0o100 else 0)
    changed += 1
    print(("chmod " if apply else "would chmod ") + oct(mode) + " -> " + oct(wanted) + " " + rel)
    if apply: os.chmod(path, wanted)
print(("repaired " if apply else "would repair ") + str(changed) + " untracked file(s)")
