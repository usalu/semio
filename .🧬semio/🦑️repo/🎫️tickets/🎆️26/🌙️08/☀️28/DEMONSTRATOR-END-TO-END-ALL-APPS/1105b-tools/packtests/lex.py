import re
def lex_pairs(s):
    n=len(s);stack=[];match={};parent={}
    i=0;in_str=False;raw=None;blk=0;ln=False
    while i<n:
        c=s[i]
        if ln:
            if c=='\n':ln=False
            i+=1;continue
        if blk:
            if s.startswith('*/',i):blk-=1;i+=2;continue
            if s.startswith('/*',i):blk+=1;i+=2;continue
            i+=1;continue
        if raw is not None:
            if s.startswith('"'+raw,i):k=len(raw);raw=None;i+=1+k;continue
            i+=1;continue
        if in_str:
            if c=='\\':i+=2;continue
            if c=='"':in_str=False
            i+=1;continue
        if s.startswith('//',i):ln=True;i+=2;continue
        if s.startswith('/*',i):blk=1;i+=2;continue
        m=re.match(r'b?r(#*)"',s[i:i+12])
        if m and (i==0 or not(s[i-1].isalnum() or s[i-1]=='_')):raw=m.group(1);i+=m.end();continue
        if c=='"':in_str=True;i+=1;continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[i:i+12])
            if m:i+=m.end();continue
        if c in '([{':stack.append(i)
        elif c in ')]}':
            if stack:o=stack.pop();match[o]=i;match[i]=o
        i+=1
    return match
