import json,glob,os,sys
ids="""cooling-heating-comparison-cooler cooling-heating-comparison-laptop cooling-heating-comparison-laptop-summer cooling-heating-comparison-toaster-summer cooling-summer-strategy-heater heating-internal-gains-pendant-lamp-warm-up heating-internal-gains-server-rack physics-fundamentals-kettle physics-fundamentals-pot physics-fundamentals-shower physics-fundamentals-vacuum shared-boiler-heating-cycle shared-tank-charge-up shared-toaster-winter shared-watt-heater shared-watt-laptop shared-watt-toaster ventilation-mechanical-pendulum-fan cooling-summer-strategy-wheel heating-calculation-radiator heating-calculation-radiator-mirrored shared-radiator-thermostat-front""".split()
root=".🧬semio/🎓️teaching/⚡️cache/staffage"
man=json.load(open("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json",encoding="utf8"))["casts"]
for i in ids:
    c=sorted(glob.glob(f"{root}/*/{i}/🔣️.json"),key=os.path.getmtime)
    if not c: print(i,"MISSING");continue
    d=json.load(open(c[-1],encoding="utf8"));m=man[i]
    print(i,"fps",d["fps"],"n",len(d["frames"]),"dur",d["durationS"],"loop",d["loop"],"| tl",[ (t["t"],t["environment"]) for t in m.get("timeline",[])][:6],"clip",m.get("clip"),"fpsM",m.get("fps"))
