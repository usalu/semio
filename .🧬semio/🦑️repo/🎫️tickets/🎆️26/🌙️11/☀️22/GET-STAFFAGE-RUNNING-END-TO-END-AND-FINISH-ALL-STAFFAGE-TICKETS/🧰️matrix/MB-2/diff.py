"""🆚️ The shape ids one band has and another lacks, per view, and the shapes of the same id whose JSON differs: `python3 diff.py <asset.json> <scale-a> <scale-b>`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
A, B = (a["representations"][k]["geometry"] for k in sys.argv[2:4])
for view in ("elevation", "front", "plan"):
    ia, ib = {s["id"]: s for s in A[view]}, {s["id"]: s for s in B[view]}
    print(view, "only", sys.argv[2], [k for k in ia if k not in ib])
    print(view, "only", sys.argv[3], [k for k in ib if k not in ia])
    print(view, "differ", [k for k in ia if k in ib and json.dumps(ia[k], sort_keys=True) != json.dumps(ib[k], sort_keys=True)])
