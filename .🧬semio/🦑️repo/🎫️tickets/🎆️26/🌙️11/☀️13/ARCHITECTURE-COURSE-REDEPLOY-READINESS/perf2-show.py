import json,sys
for f in sys.argv[1:]:
    print("##",f.split("/")[-1])
    for l in open(f):
        d=json.loads(l); n=d.get("nodes",{}); p=d["pets"]
        print(f'{d["label"]:26} long {d["long"]} | pets frames {p["frames"]} p90 {p["p90"]} max {p["max"]} >16: {p["over16"]} | nodes max {n.get("max")} top {n.get("top")}')
