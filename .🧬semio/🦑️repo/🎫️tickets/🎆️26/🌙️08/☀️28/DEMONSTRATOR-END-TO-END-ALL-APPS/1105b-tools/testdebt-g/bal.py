import sys,re
def check(p):
    t=open(p).read();st=[];i=0;n=len(t);line=1
    while i<n:
        c=t[i]
        if c=="\n":line+=1
        if t.startswith("//",i):
            j=t.find("\n",i);i=n if j<0 else j;continue
        if t.startswith("/*",i):
            j=t.find("*/",i);line+=t[i:j].count("\n");i=j+2;continue
        if c=='"':
            i+=1
            while t[i]!='"':
                if t[i]=="\\":i+=1
                if t[i]=="\n":line+=1
                i+=1
            i+=1;continue
        if c=="r" and re.match(r'r#*"',t[i:i+4]):
            m=re.match(r'r(#*)"',t[i:]);end='"'+m.group(1);j=t.find(end,i+len(m.group(0)));line+=t[i:j].count("\n");i=j+len(end);continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[i:])
            if m:i+=len(m.group(0));continue
        if c in "([{":st.append((c,line))
        elif c in ")]}":
            if not st or "([{".index(st[-1][0])!=")]}".index(c):print(p,"mismatch",c,"line",line,"open",st[-1] if st else None);return
            st.pop()
        i+=1
    if st:print(p,"unclosed",st[-3:])
    else:print(p,"ok")
for p in sys.argv[1:]:check(p)
