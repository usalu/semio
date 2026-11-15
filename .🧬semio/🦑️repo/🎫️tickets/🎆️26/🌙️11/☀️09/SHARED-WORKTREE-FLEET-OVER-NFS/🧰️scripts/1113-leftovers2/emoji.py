import os,re,sys
root=sys.argv[1]; used={}
for d,_,fs in os.walk(root):
    for f in fs:
        if f.endswith(".ts"):
            for m in re.finditer(r"/\*\* (\S+?)(?:️)? ",open(os.path.join(d,f),encoding="utf8").read()):
                used[m.group(1)]=used.get(m.group(1),0)+1
for c in sys.argv[2:]: print(c,used.get(c,0))
