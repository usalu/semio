"""📋️ The hard findings of a scan report (`T/🔎️scan.ts --report`) per asset: `python3 scan-table.py <report.json> [--groups]` prints per asset the hard and reported counts and the codes, and with `--groups` every hard group that is not a per-bone synchrony finding (whole-figure extents, anchors, dimensions, copies) with its worst case."""
import json, sys
for row in json.load(open(sys.argv[1]))["rows"]:
    s = row["summary"]
    print(f'{row["id"]}: hard {s["hard"]} reported {s["reported"]} codes {s["codes"]} anchors {s["anchorPlaces"]}')
    if "--groups" in sys.argv:
        for g in row["groups"]:
            if g["tier"] == "hard" and not g["code"].startswith("sync-bone"):
                print(f'   {g["code"]} {g.get("bone") or g.get("anchor") or ""} {"/".join(g["views"])} scales {",".join(g["scales"])} poses {len(g["poses"])} n {g["count"]} worst {g["worst"]["deltaMm"]:.0f} mm @ {g["worst"]["pose"]} 1:{g["worst"]["scale"]}: {g["worst"]["message"][:170]}')
