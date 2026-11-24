"""🧾️ The (scale, bone) pairs a gate log reports in its hard per-bone tier and in the twin tier of `gate.ts --twins`, as the `--pairs` argument of `suggest.ts`: python3 reported.py <gate.log> [min-mm]"""
import re, sys

floor = float(sys.argv[2]) if len(sys.argv) > 2 else 0
scale, take, pairs = None, False, []
for line in open(sys.argv[1], encoding="utf8"):
    head = re.match(r"\S+ 1:(\d+) (HARD|twin|reported) (\S+)", line)
    if head:
        scale, take = head.group(1), head.group(3).startswith(("sync-bone-", "twin-"))
        continue
    item = re.match(r"    (\S+?)(/\S+)? ×\d+ (\d+) mm", line)
    if item and take and float(item.group(3)) >= floor and f"{scale}:{item.group(1)}" not in pairs:
        pairs.append(f"{scale}:{item.group(1)}")
print(",".join(pairs))
