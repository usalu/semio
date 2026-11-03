import json,sys,math,re
S="C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/"
def load(a): return json.load(open(S+a+"/🔣️.json",encoding="utf8"))["asset"]
def world(asset):
    W={}
    for b in asset["rig"]["bones"]:
        p=W.get(b["parent"]) if b["parent"] else (0,0,0)
        px,py,pa=p
        a=math.radians(pa)
        x=px+b["x"]*math.cos(a)-b["y"]*math.sin(a)
        y=py+b["x"]*math.sin(a)+b["y"]*math.cos(a)
        W[b["id"]]=(x,y,pa+b["angleDeg"])
    return W
if __name__=="__main__":
    a=sys.argv[1]; d=load(a); W=world(d)
    print(a,d["dimensions"],d["presentation"])
    for b in d["rig"]["bones"]:
        print(" bone",b["id"],"parent",b["parent"],"local",b["x"],b["y"],b["angleDeg"],"len",b["lengthM"],b["joint"],"world",[round(v,4) for v in W[b["id"]]])
    print(" poses",json.dumps(d.get("poses"),ensure_ascii=False)[:1500])
    print(" clips",json.dumps(d.get("clips"),ensure_ascii=False)[:2500])
    print(" regions",[ (r["id"],r.get("model")) for r in d["thermal"]["regions"]] if "regions" in d["thermal"] else d["thermal"].keys())
    print(" materials",[m["id"] if isinstance(m,dict) else m for m in d["materials"]])
    print(" anchors",[(x["id"],x["bone"],x["x"],x["y"]) for x in d["anchors"]])
