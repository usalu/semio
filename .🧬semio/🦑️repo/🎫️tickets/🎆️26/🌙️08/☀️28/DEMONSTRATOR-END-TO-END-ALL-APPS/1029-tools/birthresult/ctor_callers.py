import re,sys,collections
G=sys.argv[1]; mode=sys.argv[2]
SKIP=("🏗️builder/🦀️.rs","🛬️lifted/🌳️tree/🦀️.rs","🧩️component/🦀️.rs","🌉️wasm/","🚪️ingress/")
CALL=re.compile(r'(?:VcsArtifactApp|EditorSurfaceApp)(?:::<[^()]*?>)?::(?:with_registry_on_bus|with_registry|new)\(')
def close(t,i):
    d=0;j=i
    while True:
        c=t[j]
        if c=='"':
            j+=1
            while t[j]!='"':
                if t[j]=='\\': j+=1
                j+=1
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[j:])
            if m: j+=len(m.group(0))-1
        elif c=='/' and t[j+1]=='/':
            j=t.index('\n',j);continue
        elif c=='(':d+=1
        elif c==')':
            d-=1
            if d==0:return j
        j+=1
files=sorted({l.split(':',1)[0] for l in open(G,encoding='utf-8') if not any(s in l.split(':',1)[0] for s in SKIP)})
kinds=collections.Counter();edits=0
MSG='.expect("the reference snapshot births")'
for f in files:
    t=open(f,encoding='utf-8').read()
    out=[]
    for m in CALL.finditer(t):
        o=m.end()-1
        pre=t[max(0,m.start()-40):m.start()]
        if 'fn ' in t[t.rfind('\n',0,m.start())+1:m.start()] : continue
        c=close(t,o)
        tail=t[c+1:c+40].replace('\n','\\n')
        k=re.sub(r'[A-Za-z0-9_:<>]+','x',tail)[:14]
        kinds[(tail.startswith('.await'),k)]+=1
        if mode=='show': print(f[-50:],t.count('\n',0,m.start())+1,repr(pre[-25:]),repr(tail[:30]))
        rest=t[c+1:]
        if rest.startswith('.await'):ins=c+1+len('.await')
        elif rest.startswith(').await'):ins=c+1+len(').await')
        else:
            print('UNHANDLED',f,t.count('\n',0,m.start())+1,repr(rest[:30]));continue
        out.append(ins)
    if mode=='apply' and out:
        for ins in sorted(set(out),reverse=True):t=t[:ins]+MSG+t[ins:]
        open(f,'w',encoding='utf-8').write(t);edits+=len(set(out))
print(kinds,edits)
