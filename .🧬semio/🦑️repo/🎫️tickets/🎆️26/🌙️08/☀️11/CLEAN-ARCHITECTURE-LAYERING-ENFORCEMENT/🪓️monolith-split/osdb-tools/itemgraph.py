"""🕸️ Item-level dependency groups inside one file. Usage: itemgraph.py <file> <ranges.txt>
ranges.txt: lines 'group start end' (1-based inclusive) describing groups over top-level items."""
import sys,re,collections
from outline import outline
from rs import strip,idents
def run(path,ranges):
    src=open(path,encoding='utf-8').read(); lines=src.split('\n')
    st=strip(src).split('\n')
    items=outline(path)
    def grp(s):
        for g,a,b in ranges:
            if a<=s<=b: return g
        return None
    defs={}  # name -> list of (group, kind)
    name_re=re.compile(r'(?:fn|struct|enum|trait|type|static|const|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)')
    item_info=[]
    for kind,name,s,e,h,vis in items:
        g=grp(h)
        if kind in('use',): continue
        m=name_re.search(lines[h-1])
        nm=m.group(1) if m else None
        if kind=='impl':
            nm=None
        item_info.append((kind,nm,s,e,h,g))
        if nm: defs.setdefault(nm,set()).add(g)
    edges=collections.defaultdict(lambda: collections.defaultdict(set))
    for kind,nm,s,e,h,g in item_info:
        txt='\n'.join(st[s-1:e])
        for i in idents(txt):
            if i in defs:
                for dg in defs[i]:
                    if dg!=g and len(defs[i])==1: edges[g][dg].add(i)
    for g in edges:
        for dg,names in edges[g].items():
            print(f'{g} -> {dg}: {len(names)} {sorted(names)[:12]}')
if __name__=='__main__':
    ranges=[]
    for l in open(sys.argv[2]):
        p=l.split()
        if len(p)==3: ranges.append((p[0],int(p[1]),int(p[2])))
    run(sys.argv[1],ranges)
