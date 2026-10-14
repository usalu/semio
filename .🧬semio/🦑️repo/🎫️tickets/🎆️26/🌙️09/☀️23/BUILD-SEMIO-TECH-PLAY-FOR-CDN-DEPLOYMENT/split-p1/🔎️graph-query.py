#!/usr/bin/env python3
"""🔎️ Queries the dumped file graph: `edges <layerA> <layerB> [lib|test]` lists the file pairs behind one layer edge."""
import json, sys
G = json.load(open(sys.argv[1]))["files"]
cmd = sys.argv[2]
short = lambda p: p.replace("✏️s/🔌️plugins/🔋️energy/", "").replace("🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/", "ANY/")
if cmd == "edges":
    a, b = sys.argv[3], sys.argv[4]; want = sys.argv[5] if len(sys.argv) > 5 else None
    for f, row in G.items():
        if not row["layer"].startswith(a): continue
        for t, testonly in row["refs"].items():
            if G.get(t, {}).get("layer", "?").startswith(b) and G[t]["layer"] != row["layer"]:
                kind = "test" if row["test"] or testonly else "lib"
                if want in (None, kind): print(kind, short(f), "->", short(t))
