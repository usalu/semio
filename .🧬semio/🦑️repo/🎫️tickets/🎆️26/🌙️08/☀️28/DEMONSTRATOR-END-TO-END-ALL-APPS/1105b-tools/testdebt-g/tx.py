import re,sys
E="crate::effect"
def balanced(t,start):
    d=0
    for i in range(start,len(t)):
        c=t[i]
        if c=="(":d+=1
        elif c==")":
            d-=1
            if d==0:return i
    raise Exception("unbalanced")
def split_args(a):
    out=[];d=0;cur="";params=False
    for ch in a:
        if ch in "([{":d+=1
        elif ch in ")]}":d-=1
        if ch=="|" and d==0:
            if params:params=False
            elif cur.strip()=="":params=True
        if ch=="," and d==0 and not params:out.append(cur);cur=""
        else:cur+=ch
    out.append(cur);return out
def calls(t,pat,fn):
    out="";pos=0;n=0
    for m in re.finditer(pat,t):
        if m.start()<pos:continue
        op=m.end()-1;assert t[op]=="(",(pat,t[op-5:op+5])
        cl=balanced(t,op);r=fn(m,t[op+1:cl])
        if r is None:continue
        out+=t[pos:m.start()]+r;pos=cl+1;n+=1
    return out+t[pos:],n
def admit(m,a):
    if a.rstrip().endswith("&mut None") or "&mut actual" in a.split(",")[-1]:return None
    return "SealedShared::admit(%s,&mut None)"%a
def close_step(m,a):
    if a.strip()=="effect":return None
    return "%s::settle(%s,|effect|%s.close_step(effect))"%(E,a,m.group(1))
def step(m,a):
    if a.strip()=="effect":return None
    return "%s::settle(%s,|effect|%s.step(effect)).result"%(E,a,m.group(1))
def advance(recv):
    def f(m,a):
        parts=split_args(a)
        if parts[-1].strip().startswith("&mut") or parts[-1].strip()=="effect":return None
        return "%s::run(%s,|effect|%s.advance(%s,effect))"%(E,parts[-1],m.group(1),",".join(parts[:-1]))
    return f
def apply(path,rules):
    t=open(path).read()
    for pat,fn in rules:
        t,n=calls(t,pat,fn);print(path.split("/")[-3:-1],pat,n)
    open(path,"w").write(t)
R=[(r"SealedShared::admit\(",admit),(r"(\w+)\.close_step\(",close_step),(r"\b(owner|retired)\.step\(",step),(r"\b(x|plan|cursor|read)\.advance\(",advance(None))]
if __name__=="__main__":
    for p in sys.argv[1:]:apply(p,R)
