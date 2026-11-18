"""🔎 Changed shapes and volumes of an asset against a revision: python3 spread-diff.py <rev> <category-dir> <slug> [scale]"""
import json,subprocess,sys
rev,cat,slug=sys.argv[1:4]; only=sys.argv[4] if len(sys.argv)>4 else None
P=f"🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/{cat}/{slug}/🔣️.json"
R="/Users/ueli/Documents/semio"
h=json.loads(subprocess.check_output(["git","-C",R,"show",f"{rev}:{P}"]))["asset"]
c=json.load(open(f"{R}/{P}"))["asset"]
for sc,rep in c["representations"].items():
  if only and sc!=only: continue
  if rep.get("status")!="authored": continue
  hr=h["representations"].get(sc)
  for v,shapes in rep["geometry"].items():
    hm={s["id"]:s for s in hr["geometry"].get(v,[])}
    cm={s["id"]:s for s in shapes}
    for i,s in cm.items():
      if i not in hm: print(sc,v,"ADDED",i,s.get("bone"),s.get("kind"))
      elif hm[i]!=s:
        ks=[k for k in set(s)|set(hm[i]) if s.get(k)!=hm[i].get(k)]
        print(sc,v,"CHANGED",i,s.get("bone"),ks)
    for i in hm:
      if i not in cm: print(sc,v,"REMOVED",i,hm[i].get("bone"))
for k in ("volumes","poses","clips","skeleton"):
  if h.get(k)!=c.get(k): print("differs:",k)
