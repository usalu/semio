"""🎚️ Feature-aware dependency resolver. For every crate R built alone with its default features, computes the set of crates in its normal+build closure (optional deps only when a feature activates them, `dep:`/`n/f`/`n?/f` honoured, target-gated deps kept as a union). reach_f(X) = number of roots whose closure contains X. Inputs: manifests.json (manifests.ts) + crates2.json (path resolution)."""
import json,os,sys,collections,time
T=os.path.dirname(os.path.abspath(__file__))
GEN=os.path.normpath(T+"/../../🗑️generated/gran-value");OUT=os.path.normpath(T+"/../../🗑️generated/gran-hubs")
class Resolver:
    def __init__(s,M=None,C=None):
        s.M=M or json.load(open(OUT+'/manifests.json'))
        s.C=C or json.load(open(GEN+'/crates2.json'))['crates']
        s.tgt={}
        for d,c in s.C.items():
            for k in('normal','build'):
                for key,t in c['deps'][k]:
                    if t in s.C: s.tgt[(d,k,key)]=t
        s.cache={}
    def deps_of(s,d):
        return [x for x in s.M[d]['deps'] if x['kind'] in('normal','build') and (d,x['kind'],x['key']) in s.tgt]
    def expand(s,d,feats):
        key=(d,feats)
        r=s.cache.get(key)
        if r is not None: return r
        m=s.M[d];ft=m['features']
        deps=s.deps_of(d)
        optnames={x['key'] for x in deps if x['optional']}
        # implicit feature n exists unless some entry uses dep:n
        used_dep=set()
        for lst in ft.values():
            for e in lst:
                if e.startswith('dep:'): used_dep.add(e[4:])
        enabled=set();act=set();req=collections.defaultdict(set)
        st=list(feats)
        while st:
            f=st.pop()
            if f in enabled: continue
            enabled.add(f)
            if f in ft:
                for e in ft[f]:
                    if e.startswith('dep:'): act.add(e[4:])
                    elif '/' in e:
                        n,sub=e.split('/',1);weak=n.endswith('?');n=n.rstrip('?')
                        if not weak and n in optnames: act.add(n)
                        req[n].add(sub)
                        if not weak and n in optnames: pass
                    else: st.append(e)
            elif f in optnames and f not in used_dep: act.add(f)
        res=[]
        for x in deps:
            if x['optional'] and x['key'] not in act: continue
            t=s.tgt[(d,x['kind'],x['key'])]
            fs=set(x['features'])|req.get(x['key'],set())
            res.append((t,fs,x['defaultFeatures']))
        r=(frozenset(enabled),res);s.cache[key]=r
        return r
    def closure(s,root):
        dft=frozenset(['default']) if 'default' in s.M[root]['features'] else frozenset()
        feats={root:set(dft)};stack=[root];seen=set([root])
        # iterate to a fixed point on features
        while stack:
            c=stack.pop()
            _,res=s.expand(c,frozenset(feats[c]))
            for t,fs,dflt in res:
                add=set(fs)
                if dflt and 'default' in s.M[t]['features']: add.add('default')
                new=t not in feats
                if new: feats[t]=set();
                if new or not add<=feats[t]:
                    feats[t]|=add;stack.append(t);seen.add(t)
        return set(feats)
    def all_reach(s,roots=None):
        roots=roots or list(s.M)
        rev=collections.defaultdict(set);reach=collections.Counter()
        t0=time.time()
        for i,r in enumerate(roots):
            cl=s.closure(r)
            for x in cl:
                if x!=r: reach[x]+=1;rev[x].add(r)
        return reach,rev
if __name__=='__main__':
    R=Resolver();t0=time.time()
    reach,rev=R.all_reach()
    old=json.load(open(GEN+'/reach.json'))
    byname={c['name']:d for d,c in R.C.items()}
    json.dump({R.C[d]['name']:n for d,n in reach.items()},open(OUT+'/reach-feat.json','w'))
    print('done',round(time.time()-t0),'s')
    for h in ['plugin','os-kernel','kernel','job','os-store-sync','plugin-reactor-turn','os-spr','ui-contract','value-control-contract','schema-registry','hash']:
        n='semio-framework-'+h;print('%-28s old %5s feat %5s'%(h,old.get(n),reach.get(byname[n])))
    tot_old=sum(old.values());tot_new=sum(reach.values());print('sum reach old',tot_old,'feat',tot_new)
