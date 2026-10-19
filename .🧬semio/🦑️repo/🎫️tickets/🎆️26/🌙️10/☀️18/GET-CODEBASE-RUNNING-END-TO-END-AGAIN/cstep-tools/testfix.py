"""cstep: port direct `close_step(<grant>)` calls in test code onto `close_turn_fixture(grant, |cx| ..close_step(cx))`.
usage: testfix.py [--write] [--diff] hits.txt   (hits: `path:line: [kind] ...` lines of typed_calls.py)"""
import re,sys,os,collections,difflib
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
write='--write' in sys.argv;show='--diff' in sys.argv
hits=collections.defaultdict(set)
for l in open([a for a in sys.argv[1:] if not a.startswith('--')][0]).read().split('\n'):
    m=re.match(r'(.+?):(\d+): \[(\w+)\]',l)
    if m: hits[m.group(1)].add(int(m.group(2)))
chain=re.compile(r'[\w.:&*\[\]()]')
def recv_start(m,k):
    j=k;depth=0
    while j>0:
        c=m[j-1]
        if c in ')]': depth+=1
        elif c in '([':
            if depth==0: break
            depth-=1
        elif depth==0 and not (c.isalnum() or c in '_.:&*'):
            break
        j-=1
    return j
for p,lines in hits.items():
    s=open(p,encoding='utf-8').read();m=mask(s);edits=[]
    for r in re.finditer(r'(?:\.|::)close_step\s*\(',m):
        ln=s.count('\n',0,r.start())+1
        if ln not in lines: continue
        po=r.end()-1;pc=match_brace(m,po)
        line=s.splitlines()[ln-1]
        if 'EnergyJobAuthority::' in line or 'with_retained_loan' in line or 'caller.turn' in line or 'close_turn_fixture' in line: continue
        arg=s[po+1:pc]
        if not arg.strip() or 'rejected.close_step' in line: continue
        if r.group(0).startswith('::'):
            # UFCS: `Path::close_step(RECV, ARG)` -> split on last top-level comma
            depth=0;cut=-1
            for i,ch in enumerate(m[po+1:pc]):
                if ch in '([{<': depth+=1
                elif ch in ')]}>': depth-=1
                elif ch==',' and depth==0: cut=i
            if cut<0: continue
            recv_expr=s[po+1:po+1+cut].strip();garg=s[po+1+cut+1:pc].strip()
            ps=recv_start(m,r.start())
            while ps>0 and (m[ps-1].isalnum() or m[ps-1] in '_:'): ps-=1
            path=s[ps:r.start()+2]
            new=f"semio_framework_job::close_with_self_funded_context({garg}, |cx| {s[ps:r.start()]}::close_step({recv_expr}, cx)).expect(\"original fixture close turn\")"
            edits.append((ps,pc+1,new))
        else:
            ps=recv_start(m,r.start())
            recv=s[ps:r.start()]
            new=f"semio_framework_job::close_with_self_funded_context({arg.strip()}, |cx| {recv}.close_step(cx)).expect(\"original fixture close turn\")"
            edits.append((ps,pc+1,new))
    if not edits: continue
    # drop nested overlaps
    edits.sort(key=lambda t:(t[0],-t[1]))
    keep=[];end=-1
    for e in edits:
        if e[0]>=end: keep.append(e);end=e[1]
    ns=s
    for a,b,t in sorted(keep,reverse=True): ns=ns[:a]+t+ns[b:]
    print('CHG',p[-90:],len(keep))
    if show: sys.stdout.writelines(difflib.unified_diff(s.splitlines(True),ns.splitlines(True),'a','b',n=0))
    if write: open(p,'w',encoding='utf-8').write(ns)
