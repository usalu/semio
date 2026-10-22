import re,subprocess,sys
names=set(sys.argv[1].split(","))
files=subprocess.run(["rg","-l","-g","*.rs","-g","!target","semio_framework_value\\b","."],capture_output=True,text=True).stdout.split("\n")
pat=re.compile(r"semio_framework_value\s*::\s*")
def top_items(s,i):
    out=[];
    if s[i]=="{":
        d=0;cur="";j=i
        while j<len(s):
            c=s[j]
            if c=="{":
                d+=1
                if d>1: cur+=c
            elif c=="}":
                d-=1
                if d==0:
                    out.append(cur);break
                cur+=c
            elif c=="," and d==1: out.append(cur);cur=""
            else: cur+=c
            j+=1
    else:
        m=re.match(r"[A-Za-z_0-9]+",s[i:]);out.append(m.group(0) if m else "")
    return out
for f in files:
    if not f: continue
    try:s=open(f,encoding="utf8").read()
    except: continue
    for m in pat.finditer(s):
        for it in top_items(s,m.end()):
            head=re.match(r"\s*([A-Za-z_0-9]+)",it)
            if head and head.group(1) in names:
                ln=s.count("\n",0,m.start())+1
                print(f"{f}:{ln}:{head.group(1)}")
