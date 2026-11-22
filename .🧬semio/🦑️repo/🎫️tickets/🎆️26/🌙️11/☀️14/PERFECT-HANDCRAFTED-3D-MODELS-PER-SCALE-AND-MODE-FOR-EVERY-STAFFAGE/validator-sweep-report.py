"""🧾 Summarises a sweep of `validator-sweep.sh`: assets by state and, per asset, the issue codes with their counts; `--rules a,b` lists the assets whose only issues carry those codes and those that carry them among others. Usage: python3 validator-sweep-report.py <sweep.jsonl> [--rules volumes-feature,volumes-slender]"""
import collections
import json
import re
import sys

rows = [json.loads(line) for line in open(sys.argv[1]) if line.strip()]
rules = sys.argv[sys.argv.index("--rules") + 1].split(",") if "--rules" in sys.argv else []
code = lambda issue: issue.split(" ")[1] if issue.split(" ")[0] in ("ASSET", "PRINT") else issue.split(" ")[0]
valid, crashed, only, mixed, other = [], [], [], [], []
for row in rows:
    if row.get("crashed"):
        crashed.append(row["asset"])
        continue
    counts = collections.Counter(code(i) for i in row["issues"])
    if not counts:
        valid.append(row["asset"])
        continue
    text = f'{row["asset"]}: ' + ", ".join(f"{k} {v}" for k, v in sorted(counts.items()))
    hit = [k for k in counts if k in rules]
    sheets = sum(1 for i in row["issues"] if "declare `sheet: true`" in i)
    if sheets:
        text += f" (of volumes-feature: {sheets} undeclared sheets)"
    (only if hit and len(hit) == len(counts) else mixed if hit else other).append(text)
print(f"assets {len(rows)}, valid {len(valid)}, invalid {len(rows) - len(valid) - len(crashed)}, crashed {len(crashed)}")
for title, group in (("only the named rules", only), ("the named rules among others", mixed), ("other issues only", other), ("crashed", crashed)):
    print(f"\n## {title} ({len(group)})")
    for text in sorted(group):
        print("- " + text)
