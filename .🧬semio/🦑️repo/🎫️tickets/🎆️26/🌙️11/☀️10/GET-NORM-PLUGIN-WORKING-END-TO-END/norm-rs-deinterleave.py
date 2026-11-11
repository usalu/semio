import re,sys
def fix(path):
    L=open(path).read().split("\n"); out=[]; i=0; n=0
    hdr=re.compile(r'^(\s*)(\("[^"]+", \(|"[^"]+" => \()$')
    while i<len(L):
        m=hdr.match(L[i])
        if m and i+1<len(L) and hdr.match(L[i+1]):
            hs=[]
            while hdr.match(L[i]): hs.append(L[i]); i+=1
            body=[]
            while not re.match(r'^\s*(\)\),|\),)$',L[i]): body.append(L[i]); i+=1
            close=L[i]; i+=1
            for h in hs:
                name=re.search(r'"([^"]+)"',h).group(1)
                out.append(h); out+= [b for b in body if "/"+name+"/" in b or "🎯"+name+"/" in b or name+"/" in b]; out.append(close); n+=1
            continue
        out.append(L[i]); i+=1
    open(path,"w").write("\n".join(out)); print(path[-60:],n)
for p in sys.argv[1:]: fix(p)
