import json,sys
slug,sc,view=sys.argv[1:4]
TR=sys.argv[4] if len(sys.argv)>4 else "trace"
THR=float(sys.argv[5]) if len(sys.argv)>5 else 0
T=json.load(open("vegetation-work/"+TR+"-"+slug+".json"))[sc][view]
print(T["viewFail"])
for b,t in T.items():
    if b=="viewFail": continue
    if b=="strokes":
        for c in t: print("STROKE",c["id"],c["bone"],len(c["runs"]))
        continue
    if b=="clips":
        for c in t: print("CLIP",c["id"],c["bone"],c["extra"],len(c["rings"]))
        continue
    if max(t["miss"],t["extra"])<THR: continue
    print(b.ljust(18),"iou",t["iou"],"miss",t["miss"],"extra",t["extra"],"drawn",t["drawn"][:3],"area",t["area"])
