"""📋️ Prints the hard finding groups of a scan report (`🔎️scan.ts --report`): groups.py <report.json> [<asset id>…]"""
import json, sys

rows = json.load(open(sys.argv[1]))["rows"]
for row in rows:
    if len(sys.argv) > 2 and row["id"] not in sys.argv[2:]:
        continue
    print(f"## {row['id']} hard {row['summary']['hard']} {row['summary']['codes']}")
    for g in row["groups"]:
        if g["tier"] != "hard":
            continue
        w = g["worst"]
        print(f"  {g['code']} {'/'.join(g['views'])} bone={g.get('bone')} anchor={g.get('anchor')} place={g.get('place')} n={g['count']} scales={','.join(g['scales'])} poses={','.join(g['poses'])} worst {w.get('deltaMm')}mm@{w['scale']}/{w['pose']}: {w['message'][:150]}")
