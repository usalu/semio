"""🔁 SCCs over a union of file slices. Usage: scc2.py path:lo:hi [path:lo:hi ...]"""
import sys,re
from outline import outline
from rs import strip,idents
from scc import impl_type
def analyse(slices):
    name_re=re.compile(r'(?:fn|struct|enum|trait|type|static|const|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)')
    nodes={}
    for path,lo,hi in slices:
        src=open(path,encoding='utf-8').read(); lines=src.split('\n'); st=strip(src).split('\n')
        for kind,nm,s,e,h,vis in outline(path):
            if not (lo<=h<=hi) or kind=='use': continue
            if kind=='impl':
                t=impl_type(lines[h-1]); key=t or '<impl>'
            else:
                m=re.match(r'\s*([A-Za-z_][A-Za-z0-9_]*)',nm or ''); key=m.group(1) if m else '<anon%s:%d>'%(path,h)
            d=nodes.setdefault(key,{'n':0,'text':'','ranges':[]})
            d['n']+=e-s+1; d['text']+=re.sub(r'(?<!\.)\.\s*[A-Za-z_][A-Za-z0-9_]*','.',  '\n'.join(st[s-1:e]))+'\n'; d['ranges'].append((path,s,e))
    names=set(nodes)
    g={k:set() for k in nodes}
    for k,d in nodes.items():
        for i in idents(d['text']):
            if i in names and i!=k: g[k].add(i)
    return nodes,g
def sccs(g):
    sys.setrecursionlimit(100000)
    idx={};low={};stack=[];on=set();res=[];c=[0]
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
    for v in g:
        if v not in idx: sc(v)
    return res
if __name__=='__main__':
    sl=[]
    for a in sys.argv[1:]:
        p,lo,hi=a.rsplit(':',2); sl.append((p,int(lo),int(hi)))
    nodes,g=analyse(sl)
    res=sccs(g)
    big=sorted(res,key=lambda c:-sum(nodes[x]['n'] for x in c))
    print('nodes',len(nodes),'sccs',len(res),'lines',sum(d['n'] for d in nodes.values()))
    for c in big[:5]:
        n=sum(nodes[x]['n'] for x in c)
        if len(c)>1: print('SCC size',len(c),'lines',n, sorted(c, key=lambda x:-nodes[x]['n'])[:30])
