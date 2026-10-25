import re,subprocess,sys
dry="--dry" in sys.argv
files=[f for f in subprocess.run(["rg","-l","store::(\\{[^;]*)?pack::snapshot","-g","*.rs","✏️s"],capture_output=True,text=True).stdout.split("\n") if f]
n=0
for f in files:
    s=open(f,encoding="utf-8").read(); o=s
    s=s.replace("store::pack::snapshot::","store::snapshot::")
    out=[];i=0
    for m in re.finditer(r"store::\{",s):
        pass
    res="";pos=0
    for m in re.finditer(r"store::\{",s):
        if m.start()<pos: continue
        j=m.end();d=1
        while j<len(s) and d:
            d+= (s[j]=="{")-(s[j]=="}"); j+=1
        body=s[m.end():j-1]
        nb=re.sub(r"(?<![\w:])pack::snapshot::","snapshot::",body)
        res+=s[pos:m.end()]+nb+"}"; pos=j
    res+=s[pos:]
    if res!=o:
        n+=1
        if not dry: open(f,"w",encoding="utf-8").write(res)
print("files",n,"of",len(files))
