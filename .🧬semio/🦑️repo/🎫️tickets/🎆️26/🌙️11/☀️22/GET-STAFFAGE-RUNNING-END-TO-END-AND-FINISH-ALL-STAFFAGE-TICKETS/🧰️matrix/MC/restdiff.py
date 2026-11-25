"""🪞️ What a repair moved in the rest drawing: `python3 restdiff.py <boxes dir before> <boxes dir after> <catalog document or backup> [mm] [pose]` compares, per authored scale and view, the evaluated boxes of every shape in the default pose (or the given pose) from two runs of `boxes.ts` and prints every shape whose box moved by more than `mm` (default 1), every shape added or gone, and the view extents of both; nothing printed for a view means its rest drawing kept every box."""
import json, sys
before_dir, after_dir, doc = sys.argv[1:4]
limit = float(sys.argv[4]) if len(sys.argv) > 4 else 1
a = json.load(open(doc))["asset"]
pose = sys.argv[5] if len(sys.argv) > 5 else a["presentation"]["defaultPose"]
before, after = json.load(open(f"{before_dir}/{a['id']}.json")), json.load(open(f"{after_dir}/{a['id']}.json"))
moved = 0
for scale in before:
    for view in ("elevation", "front", "plan"):
        b = {r[0]: r for r in before[scale][pose][view]}
        n = {r[0]: r for r in after[scale][pose][view]}
        ext = lambda rows: [min(r[2] for r in rows.values()), max(r[3] for r in rows.values()), min(r[4] for r in rows.values()), max(r[5] for r in rows.values())] if rows else []
        lines = []
        for sid in sorted(set(b) | set(n)):
            if sid not in n:
                lines.append(f"      gone  {sid} {b[sid][2:]}")
            elif sid not in b:
                lines.append(f"      added {sid} [{n[sid][1]}] {n[sid][2:]}")
            else:
                d = max(abs(x - y) for x, y in zip(b[sid][2:], n[sid][2:]))
                if d > limit:
                    lines.append(f"      moved {sid} {d} mm: {b[sid][2:]} -> {n[sid][2:]}")
                if b[sid][1] != n[sid][1]:
                    lines.append(f"      bone  {sid} {b[sid][1]} -> {n[sid][1]}")
        if lines or ext(b) != ext(n):
            print(f"   1:{scale} {view} {pose}: extents {ext(b)} -> {ext(n)}")
            print("\n".join(lines[:40]))
            moved += len(lines)
print(f"{a['id']} {pose}: {moved} differences over {limit} mm")
