import re,sys
MOD=r'(?:(?:public|private|protected|readonly|override)\s+)+'
def split(s):
    out=[];d=0;cur=""
    prev=""
    for ch in s:
        if ch in "<([{": d+=1
        elif ch in ">)]}" and not (ch==">" and prev=="="): d-=1
        prev=ch
        if ch=="," and d==0: out.append(cur);cur=""
        else: cur+=ch
    if cur.strip(): out.append(cur)
    return out
def fix(path):
    s=open(path,encoding="utf-8").read()
    pat=re.compile(r'^(?P<ind>[ \t]*)constructor\((?P<params>[^\n]*)\)\s*\{(?P<body>[^\n]*)\}[ \t]*$',re.M)
    changed=0
    def rep(m):
        nonlocal changed
        ps=split(m.group("params"))
        if not any(re.match(r'\s*'+MOD,p) for p in ps): return m.group(0)
        fields=[];names=[];newps=[]
        for p in ps:
            mm=re.match(r'(\s*)('+MOD+r')(.*)$',p,re.S)
            if not mm: newps.append(p);continue
            mods,rest=mm.group(2).split(),mm.group(3).strip()
            nm=re.match(r'(\w+)(\?)?\s*(?::\s*(.*?))?(?:\s*=(?!>)\s*(.*))?$',rest,re.S)
            name,opt,typ,default=nm.group(1),nm.group(2),nm.group(3),nm.group(4)
            if not typ: print('MANUAL untyped',name); return 0
            fm=" ".join(x for x in mods if x!="override" or True)
            fields.append(f"{m.group('ind')}{fm} {name}{'?' if opt else ''}{': '+typ if typ else ''};")
            names.append(name)
            newps.append(m.group(0) and (name+(opt or "")+(": "+typ if typ else "")+(" = "+default if default else "")))
        assigns=" ".join(f"this.{n} = {n};" for n in names)
        body=m.group("body").strip()
        if body.startswith("super("):
            end=body.find(");")
            if end<0: return m.group(0)
            body=body[:end+2]+" "+assigns+body[end+2:]
        else: body=assigns+(" "+body if body else "")
        changed+=1
        return "\n".join(fields)+f"\n{m.group('ind')}constructor({', '.join(x.strip() for x in newps)}) {{ {body} }}"
    s2=pat.sub(rep,s)
    if changed: open(path,"w",encoding="utf-8").write(s2)
    return changed
for p in sys.argv[1:]: print(p,fix(p))
