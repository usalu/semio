import re,sys,json
sys.path.insert(0,".")
from mk import R
VE="semio_framework_value::ValueError::invariant"
def enclosing_sig(L,n,col):
    # find nearest preceding `fn name(...)->ret{` start on same/previous lines at brace depth 0 relative; approximate
    for k in range(n-1,max(-1,n-60),-1):
        ln=L[k] if k!=n-1 else L[k][:col]
        ms=list(re.finditer(r'\bfn \w+',ln))
        if ms:
            tail=ln[ms[-1].start():]
            r=re.search(r'->\s*(Result<[^{]*)',tail)
            if not r and k+1<len(L) and k==n-1:return None
            return r.group(1) if r else None
    return None
def in_closure(line,pos):
    stack=[];i=0;seg_start=0
    while i<pos:
        c=line[i]
        if c=='{':
            j=i-1
            while j>=0 and line[j]==' ':j-=1
            stack.append(j>=0 and line[j]=='|')
            seg_start=i+1
        elif c=='}':
            if stack:stack.pop()
            seg_start=i+1
        elif c==';':seg_start=i+1
        i+=1
    if any(stack):return True
    return '|' in line[seg_start:pos]
def candidates(f,n):
    L=open(R+"/"+f,encoding="utf8").read().split("\n");line=L[n-1];out=[]
    for m in re.finditer(r'((?:self\.)?[A-Za-z_][\w\.]*)\.(as_ref|as_mut|take)\(\)\.(unwrap\(\)|expect\("([^"]*)"\))',line):
        pos=m.start()
        if in_closure(line,pos):continue
        # require simple field path and fn-level Result return
        sig=enclosing_sig(L,n,pos)
        if not sig:continue
        if not re.match(r'Result<[^,]*,\s*(?:semio_framework_job::)?(OriginalTurnFailure|ValueError|semio_framework_value::ValueError)>',sig.replace("\n"," ")):continue
        field=m.group(1).split('.')[-1]
        msg=(m.group(4) or f"{field}").replace('"',"'")
        if not msg.endswith(("absent","missing")):msg=msg+" is absent"
        old=m.group(0)
        new=f'{m.group(1)}.{m.group(2)}().ok_or_else(||{VE}("{msg}"))?'
        ctx=line[max(0,pos-20):m.end()+12]
        out.append((ctx,ctx.replace(old,new)))
    return out
