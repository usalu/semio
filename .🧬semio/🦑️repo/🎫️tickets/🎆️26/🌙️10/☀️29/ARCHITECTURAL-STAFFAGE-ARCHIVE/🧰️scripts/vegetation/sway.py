import json, math, sys
# usage: sway.py <spec.json> sway|gust ; spec: {"windy":{bone:deg}, "storm":{bone:deg}, "amp":{bone:[A,phase]}, "period":4, "step":0.5, "gustFrac":[[t,f],...]}
spec=json.load(open(sys.argv[1])); mode=sys.argv[2]
r=lambda v: round(v,2)
if mode=="sway":
    P=spec["period"]; step=spec["step"]; n=int(round(P/step))
    for i in range(n+1):
        t=i*step; b={k:{"angleDeg":r(spec["windy"][k]+A*math.sin(2*math.pi*(t/P-ph)))} for k,(A,ph) in spec["amp"].items()}
        print(json.dumps({"t":t,"pose":spec["pose"],"ease":"smooth","bones":b},separators=(",",":")))
else:
    for t,f in spec["gustFrac"]:
        b={k:{"angleDeg":r(v*f)} for k,v in spec["storm"].items()}
        print(json.dumps({"t":t,"pose":spec["pose"],"ease":"smooth","bones":b},separators=(",",":")))
