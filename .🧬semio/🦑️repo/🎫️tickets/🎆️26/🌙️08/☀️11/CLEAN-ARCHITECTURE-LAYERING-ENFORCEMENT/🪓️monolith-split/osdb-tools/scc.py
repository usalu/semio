"""🔁 Item-level SCCs of one file: impls merge into their self type; edges by identifier use. Usage: scc.py <file> [lo hi]"""
import sys,re,collections
from outline import outline
from rs import strip,idents
def impl_type(head):
    h=head
    h=re.sub(r'^impl\s*(<[^{]*?>)?\s*','',h) if False else h
    m=re.match(r'impl\s*(?:<[^>]*(?:<[^>]*>[^>]*)*>)?\s*(.*)',h)
    rest=m.group(1) if m else h
    if ' for ' in rest: rest=rest.split(' for ',1)[1]
    m2=re.match(r'[&\s]*(?:dyn\s+)?([A-Za-z_][A-Za-z0-9_:]*)',rest)
    return m2.group(1).split('::')[-1] if m2 else None
def analyse(path,lo=1,hi=10**9):
    src=open(path,encoding='utf-8').read(); lines=src.split('\n'); st=strip(src).split('\n')
    items=[it for it in outline(path) if lo<=it[4]<=hi and it[0]!='use']
    name_re=re.compile(r'(?:fn|struct|enum|trait|type|static|const|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)')
    nodes={}  # name -> dict(lines,text)
    order=[]
    for kind,nm,s,e,h,vis in items:
        if kind=='impl':
            t=impl_type(lines[h-1]); key=t or '<impl>'
        else:
            m=name_re.search(lines[h-1]); key=m.group(1) if m else '<anon%d>'%h
        d=nodes.setdefault(key,{'n':0,'text':'','ranges':[]})
        d['n']+=e-s+1; d['text']+='\n'.join(st[s-1:e])+'\n'; d['ranges'].append((s,e))
    names=set(nodes)
    g={k:set() for k in nodes}
    for k,d in nodes.items():
        for i in idents(d['text']):
            if i in names and i!=k: g[k].add(i)
    # tarjan
    idx={};low={};stack=[];on=set();res=[];c=[0]
    sys.setrecursionlimit(100000)
    def sc(v):
        idx[v]=low[v]=c[0];c[0]+=1;stack.append(v);on.add(v)
        for w in g[v]:
            if w not in idx: sc(w); low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],idx[w])
        if low[v]==idx[v]:
            comp=[]
            while True:
                w=stack.pop();on.discard(w);comp.append(w)
                if w==v:break
            res.append(comp)
    for v in nodes:
        if v not in idx: sc(v)
    return nodes,g,res
if __name__=='__main__':
    lo=int(sys.argv[2]) if len(sys.argv)>2 else 1; hi=int(sys.argv[3]) if len(sys.argv)>3 else 10**9
    nodes,g,res=analyse(sys.argv[1],lo,hi)
    big=sorted(res,key=lambda c:-sum(nodes[x]['n'] for x in c))
    print('nodes',len(nodes),'sccs',len(res))
    for c in big[:6]:
        n=sum(nodes[x]['n'] for x in c)
        if len(c)>1: print('SCC size',len(c),'lines',n, sorted(c, key=lambda x:-nodes[x]['n'])[:25])
