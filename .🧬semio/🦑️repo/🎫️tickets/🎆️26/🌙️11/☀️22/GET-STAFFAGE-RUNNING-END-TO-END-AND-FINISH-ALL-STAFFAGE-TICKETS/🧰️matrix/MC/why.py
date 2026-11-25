"""🔦️ Explains an extent finding from the boxes of `boxes.ts`: `python3 why.py <boxes dir> <id> <scale> <pose> <a view> <b view> <axis: height|forward|lateral> [n]` prints the extreme shapes of both views on that axis (the n lowest and highest ends), so the part one view lacks or misplaces shows."""
import json, sys
d, aid, scale, pose, va, vb, axis = sys.argv[1:8]
n = int(sys.argv[8]) if len(sys.argv) > 8 else 4
state = json.load(open(f"{d}/{aid}.json"))[scale][pose]
col = {"height": {"elevation": (4, 5), "front": (4, 5)}, "forward": {"elevation": (2, 3), "plan": (2, 3)}, "lateral": {"front": (2, 3), "plan": (4, 5)}}[axis]
for v in (va, vb):
    lo, hi = col[v]
    rows = state[v]
    print(f"{v}: {min(r[lo] for r in rows)} .. {max(r[hi] for r in rows)} mm")
    print("   low :", [(r[0], r[1], r[lo]) for r in sorted(rows, key=lambda r: r[lo])[:n]])
    print("   high:", [(r[0], r[1], r[hi]) for r in sorted(rows, key=lambda r: -r[hi])[:n]])
