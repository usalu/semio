import sys,os,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from carve import *
from verify_installed import family_files,FAM,REPO
def sigs(path,want_trait):
    text,ls,rs=parse_file(path)
    out=[]
    for p in build_pieces(ls,rs):
        if p.kind!='impl':continue
        header,mem,bl,el=impl_piece_members(ls,p)
        hdr=' '.join(h.strip() for h in header)
        is_trait='HubDirectory for' in hdr
        if is_trait!=want_trait and want_trait is not None:continue
        for m in mem:
            if m.kind!='fn':continue
            sig,names,recv,is_async,name=sig_of(m,ls)
            out.append((name,' '.join(s.strip() for s in sig),names,is_trait,hdr))
    return out
def run(b):
    root,kids=family_files(b)
    deleg=sigs(root,True)
    inh=[]
    for k in kids:inh+= [x for x in sigs(k,False)]
    inh_by=collections.defaultdict(list)
    for x in inh:inh_by[x[0]].append(x)
    bad=0
    for name,sig,names,_,_ in deleg:
        cands=inh_by.get(name,[])
        if len(cands)!=1:print('  delegate without unique inherent',name,len(cands));bad+=1;continue
        isig=re.sub(r'^pub\(super\) ','',cands[0][1])
        sg=re.sub(r'\s+',' ',sig).replace('( ','(').replace(' )',')').replace(', )',')')
        ig=re.sub(r'\s+',' ',isig).replace('( ','(').replace(' )',')').replace(', )',')')
        sg=sg.rstrip(' {').strip();ig=ig.rstrip(' {').strip()
        if sg!=ig:print('  signature mismatch',name,'\n   D:',sg[:200],'\n   I:',ig[:200]);bad+=1
        if cands[0][2]!=names:print('  param names mismatch',name);bad+=1
    dup=[n for n,v in inh_by.items() if len(v)>1]
    print(b,'delegates',len(deleg),'inherent fns',len(inh),'bad',bad,'duplicate inherent names',dup)
    # inherent names that equal trait names but are not delegated (pre-existing overlap)
    dnames={d[0] for d in deleg}
    print('  inherent names total',len(inh_by),'delegated',len(dnames&set(inh_by)))
    # top-level name duplicates across children
    top=collections.defaultdict(list)
    for f in kids:
        text,ls,rs=parse_file(f)
        for p in build_pieces(ls,rs):
            if p.kind in('fn','const','static','struct','enum','trait','type','union','macro') and p.name:
                top[(p.kind if p.kind!='macro' else 'macro',p.name)].append(os.path.basename(os.path.dirname(f)))
    d2={k:v for k,v in top.items() if len(v)>1}
    print('  duplicate top-level names across children:',d2)
if __name__=='__main__':
    for b in sys.argv[1:] or ('sqlite','postgres','neo4j'):run(b)
