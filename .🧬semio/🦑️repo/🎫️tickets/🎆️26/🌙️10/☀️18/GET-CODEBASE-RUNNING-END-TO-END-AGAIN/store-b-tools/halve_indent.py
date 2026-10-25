import sys,re
def multiline_string_lines(t):
    i=0;n=len(t);line=1;bad=[];
    while i<n:
        c=t[i]
        if c=='\n':line+=1;i+=1;continue
        if t.startswith('//',i):
            j=t.find('\n',i);i=n if j<0 else j;continue
        if t.startswith('/*',i):
            depth=1;i+=2
            while i<n and depth:
                if t.startswith('/*',i):depth+=1;i+=2
                elif t.startswith('*/',i):depth-=1;i+=2
                else:
                    if t[i]=='\n':line+=1
                    i+=1
            continue
        m=re.compile(r'b?r(#*)"').match(t,i)
        if m:
            end='"'+m.group(1);j=t.find(end,m.end())
            if j<0:return None
            seg=t[m.end():j]
            if '\n' in seg:bad.append(line)
            line+=seg.count('\n');i=j+len(end);continue
        if c=='"' or t.startswith('b"',i):
            i+= 2 if c=='b' else 1
            while i<n and t[i]!='"':
                if t[i]=='\\':
                    if t[i+1]=='\n':bad.append(line)
                    line+=(t[i+1]=='\n');i+=2;continue
                if t[i]=='\n':bad.append(line);line+=1
                i+=1
            i+=1;continue
        if c=="'":
            m=re.compile(r"'(\\.[^']*|[^\\'])'").match(t,i)
            i= m.end() if m else i+1;continue
        i+=1
    return bad
for p in sys.argv[1:]:
    t=open(p).read()
    bad=multiline_string_lines(t)
    if bad is None or bad:print('SKIP',p,bad[:5] if bad else 'lexfail');continue
    out=[]
    saved=0
    for l in t.split('\n'):
        k=len(l)-len(l.lstrip(' '))
        if k and l.strip() and not l.lstrip().startswith(('//','*')) :
            nk=(k+1)//2 if k%2 else k//2
            saved+=k-nk;l=' '*nk+l[k:]
        elif k and l.strip():
            nk=k//2 if k%2==0 else (k+1)//2;saved+=k-nk;l=' '*nk+l[k:]
        out.append(l)
    open(p,'w').write('\n'.join(out));print('OK',p,saved)
