import re,sys
sys.path.insert(0,'.')
from sib import src,defs
# bare uses: not preceded by '.', '::', or ident char; not a field/let binding crudely
def bare(t,n):
    out=[]
    for i,l in enumerate(t.split('\n')):
        for m in re.finditer(r'(?<![A-Za-z0-9_.:])'+n+r'(?![A-Za-z0-9_])',l):
            out.append((i+1,l.strip()[:120]))
    return out
order=["event_stream","command_framework","tree_source","mcp_verb","verbs","command_tree","run_proj"]
tgt=sys.argv[1]
for o in order:
    if o==tgt: continue
    for n,vis in sorted(defs[o].items()):
        b=bare(src[tgt],n)
        # filter out definitions of local
        b=[x for x in b if not re.search(r'\b(fn|let|struct|enum|const|mut)\s+'+n+r'\b',x[1]) and not re.search(r'\b'+n+r'\s*:',x[1]) and not re.search(r'\|'+n+r'\|',x[1])]
        if b: print(f"{tgt}<-{o}: {n} [{vis}] x{len(b)} e.g. L{b[0][0]}: {b[0][1]}")
