import sys,os,re,glob,collections,unicodedata
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import items,name_kind
from carve import impl_piece_members,parse_file,build_pieces
REPO='/Users/ueli/Documents/semio/'
T=os.path.dirname(os.path.abspath(__file__))
R_='🧰️framework/🛍️products/🦑️repo/🔨️modules/'
FAM={
 'sqlite':'🌎️hub/📇️directory/🪶️sqlite',
 'postgres':'🌎️hub/📇️directory/🐘️postgres',
 'neo4j':'🌎️hub/📇️directory/🌐️neo4j',
 'model':R_+'📐️model/📦️packages/🦀️rust',
 'tickets':R_+'🎫️tickets/📦️packages/🦀️rust',
 'gateway':'🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway',
}
def family_files(name):
    d=REPO+FAM[name]
    root=d+'/🦀️.rs'
    mounted=re.findall(r'#\[path = "([^"]+)/🦀️\.rs"\]',open(root,encoding='utf-8').read())
    kids=[d+'/'+m+'/🦀️.rs' for m in mounted if '🧪️tests' not in m]
    return root,kids
def norm(l):
    return l.strip().replace('pub(super) ','')
def members(path):
    text=open(path,encoding='utf-8').read()
    _,ls,rs=parse_file(path)
    out=collections.Counter()
    for p in build_pieces(ls,rs):
        if p.kind in('endmark','loose'):continue
        if p.kind=='impl':
            try:
                header,mem,bl,el=impl_piece_members(ls,p)
                hdr=' '.join(h.strip() for h in header).rstrip('{').strip()
                if 'HubDirectory for' in hdr:hdr='impl HubDirectory-port'
                for m in mem:
                    if m.kind in('endmark','loose'):continue
                    out[('member',m.kind,m.name)]+=1
            except Exception as e:
                out[('impl-unparsed',p.name,'')]+=1
        else:
            out[(p.kind,p.name,'')]+=1
    return out
def run(name):
    root,kids=family_files(name)
    snap=os.path.join(T,'orig/%s.rs.txt'%name)
    O=collections.Counter(norm(l) for l in open(snap,encoding='utf-8').read().split('\n') if l.strip())
    N=collections.Counter()
    allf=[root]+kids
    for f in allf:
        for l in open(f,encoding='utf-8').read().split('\n'):
            if l.strip():N[norm(l)]+=1
    miss=O-N;extra=N-O
    print('=== %s: files %d, orig non-blank %d, new non-blank %d, missing %d, extra %d'%(name,len(allf),sum(O.values()),sum(N.values()),sum(miss.values()),sum(extra.values())))
    for l,c in sorted(miss.items(),key=lambda x:-x[1])[:60]:print('  - %dx %s'%(c,l[:170]))
    cat=collections.Counter();unc=[]
    for l,c in extra.items():
        if l.startswith('//!'):cat['docstring']+=c
        elif l=='use super::*;':cat['use super']+=c
        elif l.startswith('#[path = "') or re.fullmatch(r'(pub )?mod \w+;',l) or re.fullmatch(r'pub(\(crate\))? use \w+::\*;',l) or l=='#[macro_use]':cat['mount']+=c
        elif re.fullmatch(r'impl \w+ \{',l):cat['impl wrapper open']+=c
        elif l=='}':cat['closing brace']+=c
        elif re.fullmatch(r'self\.\w+\(.*\)\.await',l):cat['delegate call']+=c
        elif re.match(r'(pub )?async fn \w+\(',l) and l.endswith('{'):cat['delegate/sig']+=c
        elif l.startswith('use '):cat['import']+=c
        else:unc.append((c,l))
    print('  extras by category:',dict(cat))
    for c,l in sorted(unc,key=lambda x:-x[0])[:40]:print('  + %dx %s'%(c,l[:170]))
    ro=sum(open(snap,encoding='utf-8').read().count(m) for m in('//#region',))
    rn=sum(open(f,encoding='utf-8').read().count('//#region') for f in allf)
    eo=open(snap,encoding='utf-8').read().count('//#endregion');en=sum(open(f,encoding='utf-8').read().count('//#endregion') for f in allf)
    print('  region markers orig/new: %d/%d, endregion %d/%d'%(ro,rn,eo,en))
    mo=members(snap);mn=collections.Counter()
    for f in allf:mn+=members(f)
    lost=mo-mn;gained=mn-mo
    print('  items orig %d new %d lost %d gained %d'%(sum(mo.values()),sum(mn.values()),sum(lost.values()),sum(gained.values())))
    print('  lost:',sorted(lost.items())[:30])
    print('  gained (first 12 of %d):'%sum(gained.values()),sorted(gained.items())[:12])
    sizes=[(os.path.relpath(f,REPO+FAM[name]),sum(1 for _ in open(f,encoding='utf-8'))) for f in allf]
    print('  sizes:',sizes)
if __name__=='__main__':
    for n in (sys.argv[1:] or FAM):run(n)
