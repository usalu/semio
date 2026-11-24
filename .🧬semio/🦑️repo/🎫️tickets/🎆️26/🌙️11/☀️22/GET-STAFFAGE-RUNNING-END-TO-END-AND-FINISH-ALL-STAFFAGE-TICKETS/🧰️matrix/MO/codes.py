"""🧮️ One line per asset of a scan report (`🔎️scan.ts --report`): the hard findings by code with the worst delta, and the count of the reported tier: `python3 codes.py <report.json>`."""
import json, sys
for row in json.load(open(sys.argv[1]))["rows"]:
    by = {}
    for g in row["groups"]:
        if g["tier"] != "hard":
            continue
        n, worst = by.get(g["code"], (0, 0))
        by[g["code"]] = (n + g["count"], max(worst, g["worst"].get("deltaMm") or 0))
    text = ", ".join(f"{code} ×{n} ({worst:.0f} mm)" for code, (n, worst) in sorted(by.items())) or "none"
    print(f"{row['id']}\t{row['summary']['hard']}\t{text}\t{row['summary']['reported']}")
