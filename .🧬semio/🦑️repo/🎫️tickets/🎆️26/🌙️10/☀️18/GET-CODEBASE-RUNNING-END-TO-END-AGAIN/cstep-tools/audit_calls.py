"""cstep: print every `.close_step(`/`InteractiveJob::close_step(` call with its enclosing fn name, for manual review. usage: audit_calls.py listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace,fn_items
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s)
    fns=list(fn_items(s,m))
    short=re.sub(r'✏️s/🔌️plugins/|/🗿️artifacts|/🏅️standards/🔖️[^/]*/🪆️subsets/[^/]*','',p)
    for r in re.finditer(r'(?:\.|::)close_step\s*\(',m):
        po=r.end()-1;pc=match_brace(m,po)
        ln=s.count('\n',0,r.start())+1
        enc='-'
        for name,st,bo,be,sig in fns:
            if bo<r.start()<be: enc=name
        line=s.splitlines()[ln-1].strip()
        print(f"{short[-70:]}:{ln}:{enc}: {line[:150]}")
