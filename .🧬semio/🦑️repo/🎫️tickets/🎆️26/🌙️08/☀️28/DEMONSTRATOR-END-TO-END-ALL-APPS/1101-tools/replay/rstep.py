import re,sys,os
# usage: rstep.py <log> : fix RetirementStep::{Progress(x),Failure(x),Complete} arity in files named by 'enum variant takes 2 arguments' or 'RetirementStep::Complete' mismatches
log=sys.argv[1]
s=open(log,errors='replace').read()
files=set()
for m in re.finditer(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> ([^\n]*?):(\d+):(\d+)\n((?:[^\n]*\n){1,6})',s):
    if 'takes 2 arguments' in m.group(1) or 'RetirementStep::Complete' in m.group(5):
        f=os.path.normpath(m.group(2))
        if os.path.isfile(f):files.add(f)
def fix(t):
    out=[];i=0;n=0
    while True:
        m=re.compile(r'RetirementStep::(Progress|Failure|Complete)').search(t,i)
        if not m:out.append(t[i:]);break
        out.append(t[i:m.end()]);i=m.end();kind=m.group(1)
        if t[i:i+1]!='(':
            if kind=='Complete':out.append('(0)');n+=1
            continue
        d=0;j=i
        while j<len(t):
            c=t[j]
            if c=='(':d+=1
            elif c==')':
                d-=1
                if d==0:break
            j+=1
        inner=t[i+1:j]
        # already 2-arity? count top-level commas
        dd=0;commas=0
        for c in inner:
            if c in '([{':dd+=1
            elif c in ')]}':dd-=1
            elif c==',' and dd==0:commas+=1
        if kind=='Complete':
            out.append(t[i:j+1]);i=j+1;continue
        if (kind=='Progress' and commas>=1 and re.search(r',\s*(\d+|[a-z_.]+)$',inner.strip())) :
            out.append(t[i:j+1]);i=j+1;continue
        if kind=='Failure' and commas>=1 and re.search(r',\s*(\d+|[a-z_.]+)$',inner.strip()) and not inner.strip().endswith('")'):
            out.append(t[i:j+1]);i=j+1;continue
        extra=',1' if kind=='Progress' else ',0'
        out.append('('+inner+extra+')');i=j+1;n+=1
    return ''.join(out),n
tot=0
for f in files:
    t=open(f).read();u,n=fix(t)
    if n:open(f,'w').write(u);tot+=n;print(f[-50:],n)
print('rstep',tot)
