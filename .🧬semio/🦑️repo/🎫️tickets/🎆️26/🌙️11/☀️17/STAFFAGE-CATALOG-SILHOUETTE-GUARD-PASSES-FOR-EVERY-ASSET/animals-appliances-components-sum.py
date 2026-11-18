import json,sys
for l in open(sys.argv[1]):
    j=json.loads(l)
    if "error" in j: print(j["asset"],"ERROR",j["error"]); continue
    f=[]
    for x in j["fails"]:
        k=x["kind"]
        if k=="bbox": f.append(f'{x["scale"]}:b{x["axis"]}({x["got"]}/{x["want"]})')
        elif k=="iou": f.append(f'{x["scale"]}:i{x["view"][0]}({x["iou"]})')
        else: f.append(f'{x["scale"]}:h{x["view"][0]}({x["h"]})')
    print(j["asset"],len(f)," ".join(f))
