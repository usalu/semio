import sys,os,re,shutil,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import scan as S
import mksbx
REPO=mksbx.REPO
HERE=os.path.dirname(os.path.abspath(__file__))
def run(label, base_dir, libroot_rel, out_sbx, extra_override=None):
    root=os.path.join(base_dir,libroot_rel)
    mods,order,r=S.load_tree(root)
    base_real=os.path.realpath(base_dir)
    names=set()
    texts={}
    for m in order:
        t=open(m.path,encoding='utf-8').read(); texts[m.key]=t
        for mm in re.finditer(r'\b(?:mod|enum|struct|union|trait|type|fn)\s+(\w+)',t): names.add(mm.group(1))
    keep={'crate','super','self','std','core','alloc'}|names
    rx=re.compile(r'^([ \t]*)((?:#\[[^\]\n]*\][ \t]*)*)(?:pub(?:\([^)\n]*\))?[ \t]+)?use[ \t]+(?:::)?(\w+)((?:::\w+)*)::\*;',re.M)
    rxe=re.compile(r'^(?:[ \t]*#\[[^\]\n]*\][ \t]*\n)*[ \t]*(?:#\[[^\]\n]*\][ \t]*)*(?:pub[ \t]+)?extern[ \t]+crate[ \t]+(\w+)[^;\n]*;',re.M)
    rxn=re.compile(r'^([ \t]*(?:#\[[^\]\n]*\][ \t]*)*(?:pub(?:\([^)\n]*\))?[ \t]+)?use[ \t]+(?:::)?)(\w+)((?:::\w+)*::\{)([^;]*)\};',re.M)
    stats=dict(globs=0,externs=0,nested=0,files=0)
    tmp=os.path.join(os.environ.get("HO_WORK",HERE),"nbuild-"+label); shutil.rmtree(tmp,ignore_errors=True); os.makedirs(tmp)
    ov={}
    for i,m in enumerate(order):
        t=texts[m.key]; t0=t
        def g(mo):
            if mo.group(3) in keep: return mo.group(0)
            stats['globs']+=1
            return mo.group(1)+mo.group(2)+'const _: () = ();'
        t=rx.sub(g,t)
        def e(mo):
            if mo.group(1) in ('std','core','alloc','proc_macro'): return mo.group(0)
            stats['externs']+=1
            return ''
        t=rxe.sub(e,t)
        def nst(mo):
            if mo.group(2) in keep: return mo.group(0)
            body=mo.group(4)
            nb=re.sub(r'(^|,)\s*\*\s*(?=,|$)',lambda q:q.group(1),body)
            if nb!=body: stats['nested']+=1
            return mo.group(1)+mo.group(2)+mo.group(3)+nb+'};'
        t=rxn.sub(nst,t)
        rp=m.key
        if rp.startswith(REPO+"/"): rel=os.path.relpath(rp,REPO)
        elif rp.startswith(base_real+"/"): rel=os.path.relpath(rp,base_real)
        else: raise Exception("outside "+rp)
        f=os.path.join(tmp,"%d.rs"%i); open(f,'w',encoding='utf-8').write(t)
        ov[rel]=f
        if t!=t0: stats['files']+=1
    if extra_override: ov.update(extra_override)
    mksbx.mk(out_sbx,ov)
    print(label,"modules",len(order),stats)
if __name__=="__main__":
    run(sys.argv[1],sys.argv[2],sys.argv[3],sys.argv[4])
