#!/usr/bin/env python3
"""usage: fleet-blobs.py -- for every store checks that each blob referenced by a journal event exists (the store invariant: blob before event); prints missing counts per store."""
import glob, json, os, sys
import fleet
out = {}
for g in fleet.group_ids():
    s = fleet.store(g); seen = set(); missing = []
    for seg in glob.glob(f"{s}/v1/origins/*/segments/*.jsonl"):
        for line in open(seg, errors="replace"):
            try: e = json.loads(line)
            except ValueError: continue
            sha = e.get("sha256")
            if sha and sha not in seen:
                seen.add(sha)
                if not os.path.exists(f"{s}/v1/blobs/{sha[:2]}/{sha}"): missing.append((seg.split("/origins/")[1].split("/")[0], sha[:12]))
    out[g] = {"blobs_referenced": len(seen), "missing": len(missing), "sample": missing[:3]}
print(json.dumps(out))
