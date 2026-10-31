import sys,re
def strip(path):
    s=open(path,encoding='utf-8').read();out=[];i=0;n=0
    pat=re.compile(r'(?:println|eprintln)!\(\s*"\[DEBUG\]')
    while True:
        m=pat.search(s,i)
        if not m: out.append(s[i:]);break
        out.append(s[i:m.start()]);j=s.index('(',m.start());d=0;k=j;instr=False
        while True:
            c=s[k]
            if instr:
                if c=='\\':k+=1
                elif c=='"':instr=False
            elif c=='"':instr=True
            elif c=='(':d+=1
            elif c==')':
                d-=1
                if d==0:break
            k+=1
        k+=1
        if s[k]==';':k+=1
        i=k;n+=1
    open(path,'w',encoding='utf-8').write(''.join(out));print(path[-60:],n)
for p in sys.argv[1:]:strip(p)
