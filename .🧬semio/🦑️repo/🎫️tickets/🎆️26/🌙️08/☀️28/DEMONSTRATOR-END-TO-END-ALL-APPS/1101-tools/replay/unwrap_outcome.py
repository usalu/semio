import re,sys
for p in sys.argv[1:]:
    t=open(p).read();n=0;i=0;out=[]
    while True:
        m=re.compile(r'(?<![\w:])built\(').search(t,i)
        if not m:out.append(t[i:]);break
        out.append(t[i:m.start()]);j=m.end();d=1;k=j
        while k<len(t) and d>0:
            c=t[k]
            if c=='(':d+=1
            elif c==')':d-=1
            k+=1
        call=t[m.start():k]
        if 'RetainedWorkOutcome' in call and 'fn built' not in t[max(0,m.start()-4):m.start()+8]:
            # find closure head ',||' at depth 1
            dd=0;pos=None
            for q,c in enumerate(call[len('built('):],start=len('built(')):
                if c in '([{':dd+=1
                elif c in ')]}':dd-=1
                elif c=='|' and dd==0 and call[q:q+2]=='||' and call[q-1]==',':pos=q;break
            if pos is not None:
                call='semio_framework_job::admit_original_construction('+call[len('built('):pos]+'|_|'+call[pos+2:];n+=1
        out.append(call);i=k
    if n:open(p,'w').write(''.join(out))
    print(p[-45:],n)
