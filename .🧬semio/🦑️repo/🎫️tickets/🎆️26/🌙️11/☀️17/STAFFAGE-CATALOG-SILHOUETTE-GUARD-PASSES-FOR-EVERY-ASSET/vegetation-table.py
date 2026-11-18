import sys,json,collections
for f in sys.argv[1:]:
  for l in open(f):
    r=json.loads(l)
    by=collections.defaultdict(list)
    for x in r.get("fails",[]):
        by[x.get("scale")].append(x)
    print(r["asset"],len(r.get("fails",[])))
    for sc in sorted(by,key=lambda s:-(s or 0)):
        print("  ",sc," ".join(f"{x['kind'][:4]}:{x.get('view','ax'+str(x.get('axis')))[:4]}={x.get('iou',x.get('h',x.get('got')))}" for x in by[sc]))
