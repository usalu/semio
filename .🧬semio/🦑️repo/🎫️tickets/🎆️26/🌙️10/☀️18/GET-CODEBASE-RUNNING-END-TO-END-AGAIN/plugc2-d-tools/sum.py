import re,sys,collections
log=sys.argv[1]; width=int(sys.argv[2]) if len(sys.argv)>2 else 200
buf=[]; 
def norm(b):
    m=re.match(r'^(.*?):(\d+):(\d+): (error(?:\[E\d+\])?): (.*)$',b)
    if not m: return None
    p,l,c,code,msg=m.groups()
    p=re.sub(r'^.*?(🏅️standards|🔨️modules|🧰️framework|✏️s/|🧪️tests|🌎️hub)', r'\1', p)
    p=re.sub(r'(/\./)+','/',p)
    return p,l,code,msg
for l in open(log,errors='replace'):
    l=l.rstrip('\n')
    if re.search(r': error(\[|:)',l): buf.append(l)
    m=re.match(r'error: could not compile `([^`]+)` \(([^)]*)\)',l)
    if m:
        print("=====",m.group(1),m.group(2),len(buf))
        grp=collections.OrderedDict()
        for b in buf:
            n=norm(b)
            if not n: continue
            p,ln,code,msg=n
            key=(code,re.sub(r'`[^`]*`','`_`',msg)[:90])
            e=grp.setdefault(key,[0,p,ln,msg]); e[0]+=1
        for (code,k),(cnt,p,ln,msg) in grp.items():
            print(f"  x{cnt} {code} {msg[:width]}\n       e.g. {p[-150:]}:{ln}")
        buf=[]
