import sys,os,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import scan as S
def main(root,pfile,famre,extra=None):
    mods,order,r=S.load_tree(root)
    P=next(m for m in order if os.path.realpath(m.path)==os.path.realpath(pfile))
    fam=re.compile(famre)
    inP=[m for m in order if P in S.ancestors(m) or (extra and re.search(extra,m.path))]
    n=0;hits=0
    for m in order:
        if not fam.search(m.path): continue
        tup={}
        for d in m.defs:
            if d['kind']=='struct' and d.get('tuple'): tup[d['name']]=[]
            if d['kind']=='field' and d.get('tuple') and d['owner'] in tup: tup[d['owner']].append(d['vis'])
        for name,vis in tup.items():
            priv=[v for v in vis if v=='' or v=='pub(self)']
            if not priv: continue
            n+=1
            for u in inP:
                if u is m or S.in_scope(m,u): continue
                T=u.toks
                for i,t in enumerate(T):
                    if t[0]=='id' and t[1]==name and i+1<len(T) and T[i+1][1]=='(' and not (i>0 and T[i-1][1] in ('struct','fn','enum','impl','trait','type')):
                        hits+=1; print("  TUPLE-CTOR",name,m.path.split('/')[-2],'<-',u.path.split('semio/')[-1][-60:],t[2]+1)
    print("private-field tuple structs in family:",n,"external ctor/pattern uses:",hits)
if __name__=="__main__": main(*sys.argv[1:5])
