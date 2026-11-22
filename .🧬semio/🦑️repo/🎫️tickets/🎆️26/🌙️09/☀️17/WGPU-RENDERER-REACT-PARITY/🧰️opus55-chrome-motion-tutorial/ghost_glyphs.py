import json, re, sys, glob, os
ROOT="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/👆️cursor"
FILES={"light":{"default":"🖱️default/☀️light.svg","pointer":"👆️pointer/☀️light.svg","grab":"✋️grab/☀️light.svg","grabbing":"✊️grabbing/☀️light.svg","crosshair":"🎯️crosshair/☀️light.svg","move":"🧭️move/✏️light-source.svg"},
"dark":{"default":"🖱️default/🌙️dark.svg","pointer":"👆️pointer/🖋️dark-source.svg","grab":"✋️grab/🌙️dark.svg","grabbing":"✊️grabbing/☀️light.svg","crosshair":"🎯️crosshair/🌙️dark.svg","move":"🧭️move/🌙️dark.svg"}}
def parse_d(d):
    t=re.findall(r'[a-zA-Z]|-?\d*\.?\d+(?:e-?\d+)?',d)
    pts=[];i=0;x=y=0.0;cmd=None
    def num():
        nonlocal i
        v=float(t[i]);i+=1;return v
    while i<len(t):
        if re.match(r'[a-zA-Z]',t[i]): cmd=t[i];i+=1
        if cmd in 'zZ': continue
        if cmd=='m': x+=num();y+=num();pts.append((x,y));cmd='l'
        elif cmd=='M': x=num();y=num();pts.append((x,y));cmd='L'
        elif cmd=='l': x+=num();y+=num();pts.append((x,y))
        elif cmd=='L': x=num();y=num();pts.append((x,y))
        elif cmd=='h': x+=num();pts.append((x,y))
        elif cmd=='H': x=num();pts.append((x,y))
        elif cmd=='v': y+=num();pts.append((x,y))
        elif cmd=='V': y=num();pts.append((x,y))
        else: raise SystemExit("cmd "+cmd)
    return pts
def parse(path):
    s=open(os.path.join(ROOT,path)).read()
    w=float(re.search(r'<svg[^>]*?\swidth="([\d.]+)"',s,re.S).group(1)); h=float(re.search(r'<svg[^>]*?\sheight="([\d.]+)"',s,re.S).group(1))
    tx,ty=re.search(r'transform="translate\(([-\d.]+),([-\d.]+)\)"',s).groups();tx=float(tx);ty=float(ty)
    polys=[]
    for m in re.finditer(r'<path\s+style="([^"]*)"\s+d="([^"]*)"',s):
        st=dict(p.split(':') for p in m.group(1).split(';') if p)
        pts=[(round(px+tx,4),round(py+ty,4)) for px,py in parse_d(m.group(2))]
        polys.append({"fill":st["fill"],"stroke":st["stroke"],"strokeWidth":float(st["stroke-width"]),"points":[list(p) for p in pts]})
    return {"width":round(w,4),"height":round(h,4),"polygons":polys}
out={"contractVersion":1,"sourceDir":"🖼️assets/👆️cursor","appearances":{a:{g:parse(f) for g,f in m.items()} for a,m in FILES.items()}}
json.dump(out,open(sys.argv[1],"w"),indent=2,ensure_ascii=False)
for a in out["appearances"]:
    for g,v in out["appearances"][a].items(): print(a,g,v["width"],v["height"],[len(p["points"]) for p in v["polygons"]],[p["fill"] for p in v["polygons"]])
