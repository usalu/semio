import json,sys
# usage: bone.py "id|parent|titleEn|titleDe|x|angle|len|min|max|kg|comX|dist|ref[|dia]" ... -> bone JSON lines (formatting of hand-chosen rig values; dia (m) adds a solid circular wood section)
import math
mat=sys.argv[1]; out=[]
for a in sys.argv[2:]:
    p=a.split("|"); i,par,te,td,x,ang,L,mn,mx,kg,cx,dist,ref=p[:13]
    b={"id":i,"parent":None if par=="-" else par,"title":{"en":te,"de":td},"x":float(x),"y":0,"angleDeg":float(ang),"lengthM":float(L),"joint":({"type":"fixed"} if mn=="fixed" else {"type":"revolute","minDeg":float(mn),"maxDeg":float(mx)}),"mass":{"kg":float(kg),"comX":float(cx),"comY":0,"distribution":dist,"source":"authored-estimate","reference":ref}}
    if len(p)>13 and p[13]:
        d=float(p[13]); b["section"]={"areaM2":round(math.pi*d*d/4,6),"secondMomentM4":float("%.4g"%(math.pi*d**4/64)),"extremeFibreM":d/2,"material":mat,"title":{"en":f"Solid section, d = {d*100:.0f} cm","de":f"Vollquerschnitt, d = {d*100:.0f} cm"}}
    out.append(b)
print(",\n".join(json.dumps(b,separators=(",",":"),ensure_ascii=False) for b in out))
