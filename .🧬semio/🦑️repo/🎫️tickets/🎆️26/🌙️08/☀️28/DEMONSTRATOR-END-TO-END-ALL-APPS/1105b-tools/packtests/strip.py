import re,sys
def strip(path):
    s=open(path,encoding='utf8').read()
    n=len(s);in_str=False;raw=None;in_block=0;in_line=False
    keep=[False];i=0
    while i<n:
        c=s[i]
        if in_line:
            if c=='\n': in_line=False; keep.append(False)
            i+=1;continue
        if in_block:
            if s.startswith('*/',i): in_block-=1;i+=2;continue
            if s.startswith('/*',i): in_block+=1;i+=2;continue
            if c=='\n': keep.append(True)
            i+=1;continue
        if raw is not None:
            if s.startswith('"'+raw,i): k=len(raw);raw=None;i+=1+k;continue
            if c=='\n': keep.append(True)
            i+=1;continue
        if in_str:
            if c=='\\':
                if i+1<n and s[i+1]=='\n': keep.append(True)
                i+=2;continue
            if c=='"': in_str=False
            elif c=='\n': keep.append(True)
            i+=1;continue
        if s.startswith('//',i): in_line=True;i+=2;continue
        if s.startswith('/*',i): in_block=1;i+=2;continue
        m=re.match(r'b?r(#*)"',s[i:i+12])
        if m and (i==0 or not (s[i-1].isalnum() or s[i-1]=='_')): raw=m.group(1);i+=m.end();continue
        if c=='"': in_str=True;i+=1;continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[i:i+12])
            if m: i+=m.end();continue
        if c=='\n': keep.append(False)
        i+=1
    lines=s.split('\n')
    assert len(keep)==len(lines),(len(keep),len(lines))
    t='\n'.join(l if k else l.lstrip(' ') for l,k in zip(lines,keep))
    open(path,'w',encoding='utf8').write(t)
    print(path[-45:],len(s),len(t),sum(keep))
for p in sys.argv[1:]:strip(p)
