import json,sys,numpy as np
f=json.load(open(sys.argv[1]))
v=lambda a:np.array(a,float)
for r in f["parameter"]:
    u,d=v(r["rayDirection"]),v(r["axis"])
    if not np.linalg.norm(u) or not np.linalg.norm(d): assert r["s"] is None,r["name"];continue
    u/=np.linalg.norm(u);d/=np.linalg.norm(d)
    M=np.array([d,-u]).T
    if np.linalg.matrix_rank(M,tol=1e-9)<2: assert r["s"] is None,r["name"];continue
    s=np.linalg.lstsq(M,-(v(r["axisOrigin"])-v(r["rayOrigin"])),rcond=None)[0][0]
    assert abs(s-r["s"])<1e-9,(r["name"],s)
for r in f["pick"]:
    near=[(np.hypot(h["x"]-r["x"],h["y"]-r["y"]),i) for i,h in enumerate(r["handles"])]
    near=[n for n in near if n[0]<=r["radiusPx"]]
    got=r["handles"][min(near)[1]]["id"] if near else None
    assert got==r["expected"],r["name"]
for r in f["targets"]:
    m=np.array(r["viewProj"],float).reshape(4,4).T
    out=[]
    for it in r["layer"]["items"]:
        if "axis" not in it: continue
        c=m@np.append(v(it["position"]),1.0)
        if not c[3]>1e-12: continue
        n=c[:3]/c[3]
        if not 0<=n[2]<=1: continue
        out.append({"id":it["id"],"x":(n[0]+1)/2*r["width"],"y":(1-n[1])/2*r["height"],"origin":it["position"],"axis":it["axis"]})
    assert len(out)==len(r["expected"]),r["name"]
    for a,b in zip(out,r["expected"]):
        assert a["id"]==b["id"] and abs(a["x"]-b["x"])<1e-9 and abs(a["y"]-b["y"])<1e-9 and a["origin"]==b["origin"] and a["axis"]==b["axis"],r["name"]
for r in f["sequences"]:
    st=None
    for ev,exp in zip(r["events"],r["dispatches"]):
        got=None
        if ev["kind"]=="press": st=dict(h=ev["handle"],o=ev["s"],s=ev["s"],moved=False)
        elif st is not None:
            if ev["kind"]=="move":
                if ev["s"]!=st["s"]:
                    st["s"]=ev["s"];st["moved"]=True;got=("delta",ev["s"],ev["modifiers"],ev["at"])
            else:
                if st["moved"]: got=("commit" if ev["kind"]=="release" else "abort",ev["s"] if ev["kind"]=="release" else st["s"],ev["modifiers"],ev["at"])
                st0=st;st=None
        want=None if exp is None else (exp["trigger"],exp["input"]["s"],exp["input"]["modifiers"],exp["input"]["at"])
        assert got==want,(r["name"],got,want)
        if exp is not None: assert exp["input"]["origin"] in [st["o"] if st else st0["o"]],r["name"]
for r in f["payload"]:
    a=r["authored"] or {}
    merged={k:x for k,x in a.items() if k not in r["input"]};merged.update(r["input"])
    assert merged==r["expected"],r["name"]
print("oracle ok")
