import json,sys
out=set()
for l in open(sys.argv[1]):
    r=json.loads(l)
    for x in r.get("fails",[]):
        if x.get("kind") in ("iou","hausdorff"): out.add("%s/%s"%(x["scale"],x["view"]))
print(" ".join(sorted(out)))
