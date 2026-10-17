import re,sys,os,json,time
# usage: genericize.py <tag> <bound> <app-ident> <file>... : trait impls `impl ...<EditorApp<APP>> for X {` become `impl<A: BOUND> ...<EditorApp<A>> for X {` and body occurrences follow; prints leftovers
tag,bound,app=sys.argv[1:4]; files=sys.argv[4:]; undo={}
for p in files:
    s=open(p,encoding='utf8').read(); undo[p]=s
    out=[];i=0
    pat=re.compile(r'^impl ([^\n{]*?)EditorApp<'+app+r'>([^\n{]*?) for ([^\n{]*)\{',re.M)
    while True:
        m=pat.search(s,i)
        if not m: out.append(s[i:]);break
        out.append(s[i:m.start()])
        j=m.end();d=1
        while d:
            c=s[j]
            if c=='{':d+=1
            elif c=='}':d-=1
            j+=1
        body=s[m.end():j]
        head=f'impl<A: {bound}> {m.group(1)}EditorApp<A>{m.group(2)} for {m.group(3)}{{'
        out.append(head+body.replace(f'EditorApp<{app}>','EditorApp<A>'))
        i=j
    t=''.join(out)
    open(p,'w',encoding='utf8').write(t)
    left=[(k+1,l.strip()[:150]) for k,l in enumerate(t.split('\n')) if app in l and not l.strip().startswith('//')]
    print('##',p[-44:],len(left))
    for k,l in left: print('   ',k,l)
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-{tag}-{int(time.time())}.json','w'))
