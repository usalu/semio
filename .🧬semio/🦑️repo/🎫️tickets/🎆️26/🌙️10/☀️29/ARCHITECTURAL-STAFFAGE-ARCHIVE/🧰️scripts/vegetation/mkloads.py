import json,sys
# usage: mkloads.py kind ref rig.json "bone:comX:F[:fy],..." [stemBone:from:to:w]  -> JSON lines of load objects (formatting of hand-chosen values; titles from the rig)
kind,ref,rigf,items=sys.argv[1:5]; T={b["id"]:b["title"] for b in json.load(open(rigf))["rig"]["bones"]}; out=[]
pre={"wind":("wind-","Wind drag on ","Winddruck auf "),"snow":("snow-","Snow on ","Schnee auf ")}[kind]
for it in items.split(","):
    p=it.split(":"); b,cx,f=p[0],float(p[1]),float(p[2])
    out.append({"id":pre[0]+b,"kind":kind,"title":{"en":pre[1]+T[b]["en"][0].lower()+T[b]["en"][1:],"de":pre[2]+T[b]["de"]},"bone":b,"x":cx,"y":0,"fxN":f if kind=="wind" else 0,"fyN":-f if kind=="snow" else 0,"source":"literature" if kind=="wind" else "authored-estimate","reference":ref})
if len(sys.argv)>5:
    b,a,z,w=sys.argv[5].split(":"); out.append({"id":"wind-"+b,"kind":"wind","title":{"en":"Wind drag on "+T[b]["en"][0].lower()+T[b]["en"][1:],"de":"Winddruck auf "+T[b]["de"]},"fxN":0,"fyN":0,"distributed":{"bone":b,"fromM":float(a),"toM":float(z),"wxNPerM":float(w),"wyNPerM":0},"source":"literature","reference":ref})
print(",\n".join(json.dumps(o,separators=(",",":"),ensure_ascii=False) for o in out))
