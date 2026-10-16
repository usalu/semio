import sys,os,shutil,glob,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from backend_split import assemble,T,SC
from rsitems import items,name_kind
import widen
from engine import ident_set
REPO='/Users/ueli/Documents/semio/'
REAL={'sqlite':'🌎️hub/📇️directory/🪶️sqlite','postgres':'🌎️hub/📇️directory/🐘️postgres','neo4j':'🌎️hub/📇️directory/🌐️neo4j'}
DEDUPE_NAMES={'now_ms','backend','AdminEffectPreflightFailure','admin_effect_preflight','actor_kind_to_str','actor_kind_from_str'}
CONTRACT_IMPORT='use semio_hub_directory_contract::{AdminEffectPreflightFailure, actor_kind_from_str, actor_kind_to_str, admin_effect_preflight, backend, now_ms};'

def remove_items(text,pred):
    ls=text.split('\n')
    kill=[]
    for it in items(text):
        v,k,nm=name_kind(it['toks'])
        if pred(k,nm):
            a=it['first_tok_line'];b=it['end_line']
            kill.append((a,b))
    removed=[]
    for a,b in sorted(kill,reverse=True):
        removed.append('\n'.join(ls[a:b+1]))
        end=b+1
        if end<len(ls) and ls[end].strip()=='':end+=1
        del ls[a:end]
    return '\n'.join(ls),removed

def dedupe_pred(k,nm):
    if k in('fn','enum') and nm in DEDUPE_NAMES:return True
    if k=='impl' and 'From<AdminEffectPreflightFailure>' in nm.replace(' ',''):return True
    return False

def main(backend,do_widen=True):
    out,names=assemble(backend)
    stage=os.path.join(SC,'rhs-stage',backend)
    shutil.rmtree(stage,ignore_errors=True)
    dedup_report=[]
    if backend in('sqlite','postgres'):
        rows=out['🧱️rows']
        rows,removed=remove_items(rows,dedupe_pred)
        out['🧱️rows']=rows
        dedup_report=removed
    parent=out['🦀️.rs']
    if backend in('sqlite','postgres'):
        anchor='use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};\n'
        assert parent.count(anchor)==1
        parent=parent.replace(anchor,anchor+CONTRACT_IMPORT+'\n')
    else:
        m=re.search(r'^use semio_hub_directory_neo4j_rows::\{([^}]*)\};$',parent,flags=re.M)
        assert m
        names_=[x.strip() for x in m.group(1).split(',')]
        for n in ('actor_kind_to_str','admin_effect_preflight','backend','now_ms'):
            assert n in names_,n
            names_.remove(n)
        parent=parent.replace(m.group(0),'use semio_hub_directory_neo4j_rows::{%s};'%', '.join(names_))
        anchor='use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};\n'
        assert parent.count(anchor)==1
        parent=parent.replace(anchor,anchor+'use semio_hub_directory_contract::{actor_kind_to_str, admin_effect_preflight, backend, now_ms};\n')
    out['🦀️.rs']=parent
    for k,v in out.items():
        p=os.path.join(stage,k) if k.endswith('.rs') else os.path.join(stage,k,'🦀️.rs')
        os.makedirs(os.path.dirname(p),exist_ok=True)
        open(p,'w',encoding='utf-8').write(v)
    files={}
    for p in glob.glob(stage+'/**/🦀️.rs',recursive=True):files[p]=open(p,encoding='utf-8').read()
    ext={}
    for p in glob.glob(REPO+REAL[backend]+'/🧪️tests/**/*.rs',recursive=True):ext[p]=ident_set(open(p,encoding='utf-8').read())
    idents={p:ident_set(t) for p,t in files.items()}
    rep=[]
    root=os.path.join(stage,'🦀️.rs')
    if do_widen:
        for p in sorted(files):
            if p==root:continue
            others=set()
            for q,s in idents.items():
                if q!=p:others|=s
            for s in ext.values():others|=s
            ed=widen.plan(files[p],others)
            files[p]=widen.apply(files[p],ed)
            open(p,'w',encoding='utf-8').write(files[p])
            for e in ed:rep.append((os.path.relpath(p,stage),)+e[2])
    # prune glob re-exports nobody needs
    idents={p:ident_set(t) for p,t in files.items()}
    parent=files[root]
    pruned=[]
    for m in list(re.finditer(r'^(pub(?:\(crate\))? )use (\w+)::\*;\n',parent,flags=re.M)):
        modname=m.group(2)
        folder=re.search(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod %s;'%modname,parent).group(1)
        cp=os.path.join(stage,folder,'🦀️.rs')
        text=files[cp]
        defined=set();haspub=False
        for it in items(text):
            v,k,nm=name_kind(it['toks'])
            if k in('fn','const','static','struct','enum','trait','type','union','macro') and nm:
                defined.add(nm)
                if v=='pub':haspub=True
        others=set()
        for q,s in idents.items():
            if q!=cp:others|=s
        for s in ext.values():others|=s
        used=defined&others
        if not used and not haspub:
            parent=parent.replace(m.group(0),'');pruned.append(modname)
        else:
            want='pub use %s::*;\n'%modname if haspub else 'pub(crate) use %s::*;\n'%modname
            parent=parent.replace(m.group(0),want)
    files[root]=parent
    open(root,'w',encoding='utf-8').write(parent)
    open(os.path.join(SC,'rhs-stage',backend+'.widen.txt'),'w',encoding='utf-8').write('\n'.join('\t'.join(r) for r in rep))
    open(os.path.join(SC,'rhs-stage',backend+'.dedupe.txt'),'w',encoding='utf-8').write('\n\n'.join(dedup_report))
    print(backend,'children',len(out)-1,'widened',len(rep),'delegates',len(names),'pruned reexports',pruned,'deduped',len(dedup_report))
if __name__=='__main__':
    main(sys.argv[1],len(sys.argv)<3 or sys.argv[2]!='nowiden')
    import fmt_widened
    print('formatted hunks',fmt_widened.run(os.path.join(SC,'rhs-stage',sys.argv[1])))
