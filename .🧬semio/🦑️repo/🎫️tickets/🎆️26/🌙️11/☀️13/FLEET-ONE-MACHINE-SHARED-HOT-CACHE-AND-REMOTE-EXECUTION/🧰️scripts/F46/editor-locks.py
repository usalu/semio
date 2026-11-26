import os,re
for f in ["/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.lock","/Users/ueli/Documents/semio/🌎️hub/Cargo.lock"]:
    t=open(f,encoding='utf8').read()
    parts=re.split(r'(?=\[\[package\]\]\n)',t)
    out=[];n=0
    for b in parts:
        m=re.match(r'\[\[package\]\]\nname = "(semio-s-artifact-stdio-dwg-editor(?:-ac1018)?)"\n',b)
        if m:
            d=re.search(r'dependencies = \[\n(.*?)\]\n',b,re.S)
            items=re.findall(r'"([^"]+)"',d.group(1))
            for x in ("semio-s-artifact-stdio-dwg-read","semio-s-artifact-stdio-dwg-write"):
                if x not in items: items.append(x)
            items=sorted(set(items))
            b=b[:d.start()]+"dependencies = [\n"+"".join(f' "{i}",\n' for i in items)+"]\n"+b[d.end():]
            n+=1
        out.append(b)
    s="".join(out)
    tmp=f+".f46tmp";open(tmp,'w',encoding='utf8').write(s);os.replace(tmp,f);print(f[-30:],n)
