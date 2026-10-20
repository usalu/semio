import re,sys

def mask(s):
    out=list(s);i=0;n=len(s)
    while i<n:
        c=s[i]
        if s.startswith("//",i):
            j=s.find("\n",i);j=n if j<0 else j
            for k in range(i,j):out[k]=" "
            i=j
        elif s.startswith("/*",i):
            j=s.find("*/",i+2);j=n if j<0 else j+2
            for k in range(i,j):
                if s[k]!="\n":out[k]=" "
            i=j
        elif c=='"' or (c=='r' and re.match(r'r#*"',s[i:i+10]) and (i==0 or not (s[i-1].isalnum() or s[i-1]=='_'))):
            if c=='r':
                m=re.match(r'r(#*)"',s[i:]);h=m.group(1);j=s.find('"'+h,i+len(m.group(0)));j=n if j<0 else j+1+len(h)
            else:
                j=i+1
                while j<n and s[j]!='"':
                    j+=2 if s[j]=='\\' else 1
                j+=1
            for k in range(i,min(j,n)):
                if s[k]!="\n":out[k]=" "
            i=j
        elif c=="'":
            m=re.match(r"'(\\.|[^\\'])'",s[i:i+6])
            if m:
                for k in range(i,i+len(m.group(0))):out[k]=" "
                i+=len(m.group(0))
            else:i+=1
        else:i+=1
    return "".join(out)

def remove(path,header_re,names,occurrence=0):
    s=open(path).read();m=mask(s)
    heads=[x for x in re.finditer(header_re,m)]
    if len(heads)<=occurrence:raise SystemExit(f"no header {header_re} in {path}")
    h=heads[occurrence];b=m.find("{",h.end()-1 if m[h.end()-1]=="{" else h.end())
    if m[h.end()-1]=="{":b=h.end()-1
    depth=0;i=b;end=None
    while i<len(m):
        if m[i]=="{":depth+=1
        elif m[i]=="}":
            depth-=1
            if depth==0:end=i;break
        i+=1
    body_start=b+1
    items=[];i=body_start;depth=0;start=None
    pos=body_start
    cut=[]
    # iterate top-level items
    j=body_start
    item_start=body_start
    d=0
    while j<end:
        ch=m[j]
        if ch in "{([":d+=1
        elif ch in "})]":d-=1
        if d==0 and (ch==";" or (ch=="}" )):
            seg=(item_start,j+1)
            items.append(seg);item_start=j+1
        j+=1
    removed=[]
    for a,z in items:
        txt=m[a:z]
        mm=re.search(r'\bfn\s+([A-Za-z_]\w*)',txt)
        if not mm:continue
        if mm.group(1) in names:
            raw=s[a:z]
            cut.append((a,z));removed.append(mm.group(1))
    missing=set(names)-set(removed)
    out=s
    for a,z in sorted(cut,reverse=True):
        # extend to include preceding blank/doc/attr lines belonging to the item: items begin after previous item; strip leading docs are inside seg already
        out=out[:a]+out[z:]
    open(path,"w").write(out)
    return removed,missing

if __name__=="__main__":
    path,header,*names=sys.argv[1:]
    r,mi=remove(path,header,set(names))
    print("removed",len(r),sorted(r));print("missing",sorted(mi))
