import os,re,sys,subprocess
ROOT="/Users/ueli/Documents/semio"
test,scan=sys.argv[1],sys.argv[2]
exts=sys.argv[3].split(",") if len(sys.argv)>3 else [".rs"]
t=open(os.path.join(ROOT,test),encoding="utf-8").read()
pat=re.compile(r'(?:includes|toContain|indexOf|toMatch)\(\s*("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'|`(?:[^`\\]|\\.)*`)')
seen=[]
for m in pat.finditer(t):
    raw=m.group(1)[1:-1]
    s=raw.encode().decode("unicode_escape") if "\\" in raw and "\\u" not in raw and False else raw
    s=s.replace('\\"','"').replace("\\'","'").replace("\\\\","\\")
    if len(s)<4 or s in seen: continue
    seen.append(s)
files=[]
for dp,dn,fn in os.walk(os.path.join(ROOT,scan)):
    for f in fn:
        if any(f.endswith(e) for e in exts): files.append(os.path.join(dp,f))
texts={f:open(f,encoding="utf-8",errors="replace").read() for f in files}
for s in seen:
    hits=[f for f,x in texts.items() if s in x]
    short=[h.replace(ROOT+"/","").replace(scan+"/","") for h in hits]
    prod=[h for h in short if "/🧪️tests/" not in h and "🧪️testing" not in h]
    print(("OK  " if prod else ("TEST" if short else "MISS")), repr(s[:90]), "->", (prod or short)[:3])
