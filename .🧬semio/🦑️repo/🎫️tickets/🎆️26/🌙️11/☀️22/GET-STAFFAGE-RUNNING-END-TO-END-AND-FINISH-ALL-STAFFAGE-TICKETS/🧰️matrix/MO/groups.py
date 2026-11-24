"""📋️ Prints the hard finding groups of a scan report (`🔎️scan.ts --report`): groups.py <report.json> [<asset id> …]; ALL=1 adds the reported tier."""
import json, os, sys
rows = json.load(open(sys.argv[1]))["rows"]
want = set(sys.argv[2:])
for row in rows:
    if want and row["id"] not in want:
        continue
    print(f"## {row['id']} hard {row['summary']['hard']} reported {row['summary']['reported']} codes {row['summary']['codes']}")
    for g in row["groups"]:
        if g["tier"] != "hard" and not os.environ.get("ALL"):
            continue
        w = g["worst"]
        print(f"  {g['code']} {'/'.join(g['views'])} bone={g.get('bone','')} anchor={g.get('anchor','')} {g.get('place','')} n={g['count']} scales={','.join(g['scales'])} poses={','.join(g['poses'])}\n      worst {w['pose']} 1:{w['scale']} {w.get('deltaMm')} mm: {w['message']}")
