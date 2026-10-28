import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
d = m.orientation_study()["design"]
for wwr in (.1,.2,.3,.45,.6,.8):
    for ov in (0,.6,1.2):
        for t in (.1,.16,.3):
            es = {th: m.evaluate({**d,"wwr":wwr,"overhang":ov,"insulation":t,"orientation":float(th)})["electricity"] for th in range(90,451,2)}
            lo = min(es,key=es.get); fe = min((k for k in es if 270<=k<=450), key=es.get)
            loc = [k for k in range(92,449,2) if es[k]<es[k-2] and es[k]<=es[k+2]]
            print(wwr, ov, t, "global", lo, round(es[lo]), "feasible", fe, round(es[fe]), "locals", loc)
