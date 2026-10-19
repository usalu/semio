"""cstep: lift grant-closure close bodies that close nested InteractiveJob children onto close_original_nested_step.
usage: nested.py [--write] listfile 'recvRegex' ['argRegex']
 rewrites inside `close_original_granted_step(cx,|G|{...})` blocks:
  - helper name + closure params `|cx,G|`
  - `let step = RECV.close_step(ARG);`  and `let step = [path::]InteractiveJob::close_step(RECV, ARG);`
      -> `let step = cx.with_retained_loan(ARG,|same|[path::]InteractiveJob::close_step(RECV,same))?;`
  - `return Err(X)` -> `return Err(semio_framework_job::OriginalTurnFailure::debit_original(X,cx))`"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
write='--write' in sys.argv;force='--all' in sys.argv
a=[x for x in sys.argv[1:] if not x.startswith('--')]
lst,recv=a[0],a[1];argre=a[2] if len(a)>2 else r'\w+'
head=re.compile(r'close_original_granted_step\(cx,\|(\w+)\|\{')
for p in open(lst).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if 'close_original_granted_step(' not in s: continue
    m=mask(s);out=s;n=0
    for c in reversed(list(head.finditer(m))):
        b=c.end()-1;e=match_brace(m,b);blk=s[b:e+1];bm=m[b:e+1]
        pat1=re.compile(r'(?<![\w.])(?P<r>'+recv+r')\.close_step\((?P<a>'+argre+r')\)')
        pat2=re.compile(r'let step = (?P<p>(?:[\w]+::)*)InteractiveJob::close_step\((?P<r>[^,()]*(?:\(\))?),\s*(?P<a>'+argre+r')\);')
        if not force and not (pat1.search(bm) or pat2.search(bm)): continue
        nb=blk
        nb=pat1.sub(lambda r:f"cx.with_retained_loan({r.group('a')}, |same| {r.group('r')}.close_step(same))?",nb)
        nb=pat2.sub(lambda r:f"let step = cx.with_retained_loan({r.group('a')}, |same| {r.group('p')}InteractiveJob::close_step({r.group('r')}, same))?;",nb)
        nm=mask(nb)
        # return Err(X) -> debit
        res=[];pos=0
        for r in re.finditer(r'\breturn Err\(',nm):
            po=r.end()-1;pc=match_brace(nm,po)
            inner=nb[po+1:pc]
            if inner.startswith('semio_framework_job::OriginalTurnFailure'): continue
            res.append((po+1,pc,f"semio_framework_job::OriginalTurnFailure::debit_original({inner},cx)"))
        for x,y,t in reversed(res): nb=nb[:x]+t+nb[y:]
        head_new=f"close_original_nested_step(cx,|cx,{c.group(1)}|{{"
        out=out[:c.start()]+head_new+nb[1:]+out[e+1:]
        n+=1
    if n:
        print(p[-80:],n)
        if write: open(p,'w',encoding='utf-8').write(out)
