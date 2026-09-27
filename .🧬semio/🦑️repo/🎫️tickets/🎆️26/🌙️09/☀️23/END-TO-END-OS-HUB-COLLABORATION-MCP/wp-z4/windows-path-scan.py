#!/usr/bin/env python3
"""🪟️ Z4: every path a clone checks out (tracked + untracked-not-ignored, tickets excluded) against NTFS/Win32 naming
rules: forbidden characters, reserved device names, trailing dot/space, non-NFC names, case-folding collisions,
symlinks, and the MAX_PATH budget below a 20-unit clone root. usage: windows-path-scan.py <repo>"""
import re, subprocess, sys, unicodedata
from collections import defaultdict

repo = sys.argv[1]
raw = subprocess.run(["git", "-C", repo, "ls-files", "-co", "--exclude-standard", "-s", "-z"], capture_output=True, check=True).stdout.decode("utf-8")
entries = []
for item in raw.split("\0"):
    if not item: continue
    if "\t" in item and re.match(r"^\d{6} ", item): meta, path = item.split("\t", 1); mode = meta.split(" ")[0]
    else: mode, path = "untracked", item
    if path.startswith(".🧬semio/🦑️repo/🎫️tickets/") or path.startswith(".tmp"): continue
    entries.append((mode, path))
RESERVED = re.compile(r"^(CON|PRN|AUX|NUL|COM[0-9¹²³]|LPT[0-9¹²³])(\..*)?$", re.I)
BAD = re.compile(r'[<>:"|?*\x00-\x1f\\]')
problems = defaultdict(list)
folded = defaultdict(set)
dirs = set()
for mode, path in entries:
    parts = path.split("/")
    for i in range(1, len(parts)): dirs.add("/".join(parts[:i]))
    if mode == "120000": problems["symlink"].append(path)
    if mode == "160000": problems["submodule"].append(path)
    for part in parts:
        if BAD.search(part): problems["forbidden-char"].append(path); break
        if RESERVED.match(part): problems["reserved-name"].append(path); break
        if part.endswith(".") or part.endswith(" "): problems["trailing-dot-or-space"].append(path); break
        if unicodedata.normalize("NFC", part) != part: problems["non-nfc"].append(path); break
    folded[path.casefold()].add(path)
    units = len((r"C:\Users\Jane\src\\" + path.replace("/", "\\")).encode("utf-16-le")) // 2
    if units >= 260: problems["max-path-over-260-below-20-unit-root"].append(f"{units} {path}")
for d in dirs: folded[d.casefold()].add(d)
for key, variants in folded.items():
    if len(variants) > 1: problems["case-collision"].append(" | ".join(sorted(variants)))
print(f"paths={len(entries)} dirs={len(dirs)}")
for kind in ["forbidden-char", "reserved-name", "trailing-dot-or-space", "non-nfc", "case-collision", "symlink", "submodule", "max-path-over-260-below-20-unit-root"]:
    rows = problems.get(kind, [])
    print(f"\n## {kind}: {len(rows)}")
    for row in sorted(rows)[:25]: print(f"  {row}")
