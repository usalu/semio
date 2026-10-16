import sys,os,re,json,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import scan as S
def subtree(m,P):
    return P in S.ancestors(m)
def main(root,pfile,famre,out,extra=None):
    mods,order,r=S.load_tree(root)
    P=next(m for m in order if os.path.realpath(m.path)==os.path.realpath(pfile))
    fam=re.compile(famre)
    facts={m.key:S.uses_in(m) for m in order}
    inP=[m for m in order if subtree(m,P) or (extra and re.search(extra,m.path))]
    res=[]
    allfields=collections.Counter(); allm=collections.Counter()
    for m in order:
        for d in m.defs:
            if d['kind']=='field': allfields[d['name']]+=1
            if d['kind'].startswith('assoc-'): allm[d['name']]+=1
    for m in order:
        if not fam.search(m.path): continue
        for d in m.defs:
            k=d['kind']
            if k not in ('field','assoc-fn','assoc-const','assoc-type','trait'): continue
            sm=S.scope_module(m,d['vis'])
            if sm is None: continue
            for u in inP:
                if u is m or S.in_scope(sm,u): continue
                f=facts[u.key]; hits=[]; nm=d['name']; ow=d.get('owner')
                if k=='field':
                    for ln in f['dotfield'].get(nm,[]): hits.append(('.field',ln))
                    for ln in f['lit'].get((ow,nm),[]): hits.append(('lit',ln))
                elif k.startswith('assoc-'):
                    for ln in f['dotcall'].get(nm,[]): hits.append(('.call',ln))
                    for ln in f['assoc'].get((ow,nm),[]): hits.append(('Type::',ln))
                    for ln in f['assoc'].get(('Self',nm),[]): hits.append(('Self::',ln))
                elif k=='trait':
                    for mn in d['methods']:
                        for ln in f['dotcall'].get(mn,[]): hits.append(('trait-method:'+mn,ln))
                if hits:
                    res.append(dict(kind=k,owner=ow,name=nm,vis=d['vis'],def_file=m.path,def_line=d['line'],use_file=u.path,hits=hits,owner_in_use=(ow in f['ident']) if ow else None))
    json.dump(res,open(out,'w'),ensure_ascii=False,indent=0)
    print("modules",len(order),"in P-subtree",len(inP),"defs checked: candidates",len(res))
    c=collections.Counter(x['vis'] for x in res); print("by vis",dict(c))
    for x in res:
        print(x['kind'],x['owner'],x['name'],repr(x['vis']),x['def_file'].split('/')[-2],x['def_line'],'<-',x['use_file'].split('semio/')[-1][-70:],x['hits'][:2],'owner_in_use',x['owner_in_use'])
if __name__=="__main__": main(*sys.argv[1:6])
