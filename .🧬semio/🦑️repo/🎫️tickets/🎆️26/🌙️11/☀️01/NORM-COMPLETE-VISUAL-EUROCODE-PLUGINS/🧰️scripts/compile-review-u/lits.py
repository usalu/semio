import re,sys
SPECS={
 "Marker":{"id","at","shape","tone","label","utilisation","bound_check","binding"},
 "Diagram2d":{"id","title","description","axes","series","markers","annotations","focus","legend","table_twin"},
 "DiagramNote":{"id","at","text","tone"},
 "DataTable":{"columns"},
 "Fill":{"tone","opacity"},
}
def block(s,i):
    depth=0;n=len(s);j=i
    while j<n:
        c=s[j]
        if c=='"':
            j+=1
            while j<n and s[j]!='"': j+=2 if s[j]=='\\' else 1
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[j:j+12])
            if m: j+=len(m.group(0))-1
        elif c in '([{': depth+=1
        elif c in ')]}':
            depth-=1
            if depth==0: return j
        j+=1
    return n
def top_fields(body):
    out=[];depth=0;cur='';n=len(body);j=0;fields=[];start=0
    segs=[];
    while j<n:
        c=body[j]
        if c=='"':
            j+=1
            while j<n and body[j]!='"': j+=2 if body[j]=='\\' else 1
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",body[j:j+12])
            if m: j+=len(m.group(0))-1
        elif c in '([{': depth+=1
        elif c in ')]}': depth-=1
        elif c==',' and depth==0:
            segs.append(body[start:j]); start=j+1
        j+=1
    segs.append(body[start:])
    names=[];rest=False
    for sg in segs:
        sg=sg.strip()
        if not sg: continue
        if sg.startswith('..'): rest=True; continue
        m=re.match(r'([a-z_][a-z0-9_]*)\b',sg)
        names.append(m.group(1) if m else sg[:20])
    return names,rest
for p in open(sys.argv[1],encoding='utf-8').read().split('\n'):
    p=p.strip()
    if not p: continue
    try: s=open(p,encoding='utf-8').read()
    except Exception: continue
    for name,spec in SPECS.items():
        for m in re.finditer(r'(?<![\w:])'+name+r'\s*\{',s):
            pre=s[max(0,m.start()-12):m.start()]
            if re.search(r'(struct|enum|impl|for)\s*$',pre) or re.search(r'(->|<|:)\s*$',pre.rstrip()+' ' if False else pre): continue
            i=m.end()-1; e=block(s,i); body=s[i+1:e]
            names,rest=top_fields(body)
            if rest: continue
            miss=spec-set(names); extra=set(names)-spec
            if miss or extra:
                print(f"{p.split('norm/')[-1].replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}:{s.count(chr(10),0,m.start())+1}: {name} missing={sorted(miss)} extra={sorted(extra)}")
