import sys
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/tour-guide/🔣️.json"
pts=sys.argv[1:] and [tuple(map(float,a.split(","))) for a in sys.argv[1:]]
lines=open(P,encoding="utf8").read().split("\n")
i=next(k for k,l in enumerate(lines) if l=='                  null,' and lines[k+1]=='                  null' and lines[k+4].strip()=='"closed": true,' and '"id": "neck"' in lines[k+5])
ind=" "*16
out=[]
for n,(x,y) in enumerate(pts):
    out+= [ind+"[",ind+f"  {x:g},",ind+f"  {y:g}",ind+"]"+("," if n<len(pts)-1 else "")]
lines[i-1:i+3]=out
open(P,"w",encoding="utf8").write("\n".join(lines))
