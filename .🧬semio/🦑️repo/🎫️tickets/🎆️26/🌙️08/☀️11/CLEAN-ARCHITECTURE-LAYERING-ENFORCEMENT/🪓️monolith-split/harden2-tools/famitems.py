import sys,os,re,json,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import scan as S
def main(root,pfile,famre,out,extra=None):
    mods,order,r=S.load_tree(root)
    P=next(m for m in order if os.path.realpath(m.path)==os.path.realpath(pfile))
    fam=re.compile(famre)
    inP=[m for m in order if P in S.ancestors(m) or (extra and re.search(extra,m.path))]
    idents={};decls={}
    for m in order:
        ids=collections.defaultdict(list)
        for i,t in enumerate(m.toks):
            if t[0]=='id':
                prev=m.toks[i-1][1] if i else ''
                if prev=='.': continue
                ids[t[1]].append(t[2]+1)
        idents[m.key]=ids
        dd=set()
        for i,t in enumerate(m.toks):
            if t[0]=='id' and t[1] in ('fn','struct','enum','let','mut','const','static','type','trait','mod','union') and i+1<len(m.toks) and m.toks[i+1][0]=='id': dd.add(m.toks[i+1][1])
            if t[0]=='id' and i+1<len(m.toks) and m.toks[i+1][1]==':' and not (i+2<len(m.toks) and m.toks[i+2][1]==':'): dd.add(t[1])
        decls[m.key]=dd
    res=[]
    for m in order:
        if not fam.search(m.path): continue
        for d in m.defs:
            if d['kind'] not in ('struct','enum','fn','const','static','type','trait','union','macro','mod'): continue
            sm=S.scope_module(m,d['vis'])
            if sm is None: continue
            nm=d['name']
            for u in inP:
                if u is m or S.in_scope(sm,u): continue
                if nm in idents[u.key] and nm not in decls[u.key]:
                    res.append(dict(kind=d['kind'],name=nm,vis=d['vis'],def_file=m.path,def_line=d['line'],use_file=u.path,lines=idents[u.key][nm][:3]))
    json.dump(res,open(out,'w'),ensure_ascii=False)
    c=collections.Counter(x['vis'] for x in res)
    print("top-level item candidates",len(res),dict(c))
    for x in res[:40]: print(" ",x['kind'],x['name'],repr(x['vis']),x['def_file'].split('/')[-2],x['def_line'],'->',x['use_file'].split('semio/')[-1][-60:],x['lines'])
if __name__=="__main__": main(*sys.argv[1:6])
