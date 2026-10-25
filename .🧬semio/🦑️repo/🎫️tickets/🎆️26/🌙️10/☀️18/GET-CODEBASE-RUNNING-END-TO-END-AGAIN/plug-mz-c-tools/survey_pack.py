import re,subprocess,collections,sys
files=subprocess.run(["rg","--files","-g","*.rs","✏️s"],capture_output=True,text=True).stdout.split("\n")
pat=re.compile(sys.argv[1]); c=collections.Counter(); ex={}
for f in files:
    if not f: continue
    try: s=open(f,encoding="utf-8").read()
    except: continue
    for m in pat.finditer(s):
        k=m.group(0)[:160]; c[k]+=1; ex.setdefault(k,f[-60:])
for k,n in c.most_common(40): print(n,k.replace("\n"," "))
