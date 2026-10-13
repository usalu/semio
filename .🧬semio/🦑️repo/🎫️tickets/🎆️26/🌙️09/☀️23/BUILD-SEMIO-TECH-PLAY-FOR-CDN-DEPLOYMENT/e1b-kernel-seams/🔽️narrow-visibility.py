#!/usr/bin/env python3
"""🔽️ Makes every `pub(crate)` item, method and field of the given files private again (wave E1b), so that
`🔁️fix-loop.py --widen` restores exactly the visibilities rustc proves necessary. `pub(crate) use` and
`pub(crate) mod` lines are left alone. Usage: narrow-visibility.py FILE [FILE ...]
"""
import re, sys

total = 0
for f in sys.argv[1:]:
    src = open(f, encoding="utf-8").read()
    new, n = re.subn(r"\bpub\(crate\) (?!use\b|mod\b)", "", src)
    open(f, "w", encoding="utf-8").write(new)
    total += n
    print(f"[narrow] {n:4} {f.split('🏪️store/')[-1]}")
print(f"[narrow] {total} widenings removed")
