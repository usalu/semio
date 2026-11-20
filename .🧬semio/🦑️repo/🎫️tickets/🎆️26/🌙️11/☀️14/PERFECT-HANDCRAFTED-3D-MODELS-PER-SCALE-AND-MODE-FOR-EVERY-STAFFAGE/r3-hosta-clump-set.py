"""🎚️ Round 3, vegetation/hosta-clump: sets stated values of leaves in the table of r3-hosta-clump-plant.ts while tuning by eye.

Usage: python3 r3-hosta-clump-set.py <k>:<key>=<value>[,<key>=<value>...] ...   (a value starting with + or - after '~' is added: 3:Lp~+0.01)
"""
import re
import sys

path = "r3-hosta-clump-plant.ts"
text = open(path, encoding="utf-8").read()
for spec in sys.argv[1:]:
    k, pairs = spec.split(":")
    for pair in pairs.split(","):
        relative = "~" in pair
        key, value = pair.split("~" if relative else "=")
        def fix(match):
            row = match.group(0)
            old = float(re.search(key + r": (-?[\d.]+)", row).group(1))
            new = round(old + float(value) if relative else float(value), 3)
            return re.sub(key + r": -?[\d.]+", "%s: %s" % (key, int(new) if new == int(new) else new), row, count=1)
        text = re.sub(r"  \{ k: %s, tier.*\}," % k, fix, text)
open(path, "w", encoding="utf-8").write(text)
