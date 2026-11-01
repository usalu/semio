import re,sys,os
log=sys.argv[1]
s=open(log,errors='replace').read()
edits={}
for m in re.finditer(r'error\[E0053\]: method `(\w+)` has an incompatible type for trait\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    f=os.path.normpath(m.group(2))
    if os.path.isfile(f) and m.group(1) in ('close_step','retire','retire_owned','close_step_controlled','close_original_step'):
        edits.setdefault(f,set()).add(int(m.group(3)))
tot=0
for f,lines in edits.items():
    t=open(f).read();L=t.split('\n');offs=[0]
    for l in L:offs.append(offs[-1]+len(l)+1)
    for ln in sorted(lines,reverse=True):
        a=offs[ln-1]
        mm=re.compile(r'->\s*(Result<)').search(t,a,a+offs[ln]-a+400)
        if not mm:print('nomatch',f[-40:],ln);continue
        i=mm.start(1);d=0;j=i
        while j<len(t):
            c=t[j]
            if c=='<':d+=1
            elif c=='>' and t[j-1]!='-':
                d-=1
                if d==0:break
            j+=1
        ty=t[i:j+1]
        k=t.index('{',j)
        # body match
        d=0;e=k
        while e<len(t):
            c=t[e]
            if c=='{':d+=1
            elif c=='}':
                d-=1
                if d==0:break
            e+=1
        body=t[k+1:e]
        inner=ty[len('Result<'):-1]
        out="semio_framework_value::RetainedWorkOutcome<"+inner+">"
        new="{ let result = (|| -> "+ty+" {"+body+"})(); let work = u64::from(result.is_ok()); semio_framework_value::RetainedWorkOutcome { result, work } }"
        t=t[:i]+out+t[j+1:k]+new+t[e+1:]
        tot+=1
    open(f,'w').write(t)
print('sigwrap',tot)
