"""cstep scanner: find `close_step(<non-cx>)` calls whose receiver is (heuristically) a type implementing InteractiveJob /
ArtifactCommandWork / FixedOperationOwner, or one of the inherent fns that now answer Result<ICS,ValueError>.
usage: typed_calls.py indexlist scanlist"""
import re,sys,os,collections
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace,fn_items,enclosing_impl
TRAITS=r'(?:InteractiveJob|FixedOperationOwner|ArtifactCommandWork(?:<[^{]*?>)?)'
IMPLRE=re.compile(r'\bimpl\b(?P<g><[^{]*?>)?\s*(?:[\w:]*::)?'+TRAITS+r'\s+for\s+(?P<t>[^{]*?)(?:\swhere\b[^{]*)?\{',re.S)
def tname(t):
    t=re.sub(r'\s+',' ',t).strip()
    m=re.match(r'(?:dyn )?(?:[\w]+::)*(\w+)',t)
    return m.group(1) if m else t
ijtypes=set();structs=collections.defaultdict(dict)
files=[l for l in open(sys.argv[1]).read().split('\n') if l]
cache={}
def load(p):
    if p not in cache:
        s=open(p,encoding='utf-8').read();cache[p]=(s,mask(s))
    return cache[p]
for p in files:
    s,m=load(p)
    for im in IMPLRE.finditer(m):
        ijtypes.add(tname(im.group('t')))
    for st in re.finditer(r'\bstruct\s+(\w+)\s*(?:<[^{;]*?>)?\s*(?:where[^{;]*)?\{',m):
        b=st.end()-1;e=match_brace(m,b)
        for f in re.finditer(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*([^,]+(?:<[^{}]*?>)?)\s*(?:,|$)',m[b+1:e]):
            structs[st.group(1)][f.group(1)]=f.group(2).strip()
        # re-extract with depth awareness
        body=s[b+1:e];depth=0;cur='';fields=[]
        for ch in body:
            if ch in '<([': depth+=1
            elif ch in '>)]': depth-=1
            if ch==',' and depth==0: fields.append(cur);cur=''
            else: cur+=ch
        fields.append(cur)
        for fl in fields:
            mm=re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)$',re.sub(r'(?m)^\s*//.*$','',fl).strip(),re.S)
            if mm: structs[st.group(1)][mm.group(1)]=re.sub(r'\s+',' ',mm.group(2)).strip()
IJ=ijtypes|{'ToolRunJob','ErasedToolJob'}
def is_ij(ty):
    if not ty: return False
    if re.search(r'dyn\s+(?:[\w:]*::)?(?:InteractiveJob|FixedOperationOwner|ArtifactCommandWork)',ty): return True
    for n in re.findall(r'\b[A-Z]\w*',ty):
        if n in IJ: return True
    return False
cxre=re.compile(r'^\s*(?:&mut\s+)?\*?(?:[\w.]*cx[\w]*|context|ctx|[\w.]*\.cx\(\)|same|nested|issuer_cx)\s*$')
res=[]
for p in [l for l in open(sys.argv[2]).read().split('\n') if l]:
    s,m=load(p)
    fns=list(fn_items(s,m))
    for r in re.finditer(r'(?:\.|::)close_step\s*\(',m):
        po=r.end()-1;pc=match_brace(m,po)
        arg=s[po+1:pc]
        if cxre.match(arg) or cxre.match(arg.rsplit(',',1)[-1]): continue
        ln=s.count('\n',0,r.start())+1
        if re.search(r'fn\s*$',m[max(0,r.start()-6):r.start()]): continue
        # receiver text
        k=r.start()
        recv=''
        # walk back over a path/field/method chain
        j=k
        depth=0
        while j>0:
            c=m[j-1]
            if c in ')]': depth+=1
            elif c in '([':
                if depth==0: break
                depth-=1
            elif depth==0 and not (c.isalnum() or c in '_.:&*'): 
                break
            j-=1
        recv=s[j:k].strip().lstrip('&*').strip()
        if r.group(0).startswith('::'):
            # UFCS: first argument is receiver
            first=arg.split(',')[0].strip()
            recv=first.lstrip('&').replace('mut ','').strip()
            tpre=s[max(0,r.start()-60):r.start()]
            if re.search(r'(?:InteractiveJob|FixedOperationOwner|ArtifactCommandWork[^:]*)\s*(?:as\s+[^>]*>)?$',tpre) or '>' in tpre[-3:]:
                res.append((p,ln,'ufcs',recv));continue
        # find enclosing fn + impl
        enc=None
        for name,st,bo,be,sig in fns:
            if bo<r.start()<be: enc=(name,st,bo,be,sig)
        ty=None
        base=re.sub(r'\.as_mut\(\)|\.as_ref\(\)|\.take\(\)|\.unwrap\(\)|\.expect\([^)]*\)|\.as_deref_mut\(\)|\.as_deref\(\)','',recv)
        if base.startswith('self.'):
            im=enclosing_impl(m,r.start())
            if im:
                hd=re.sub(r'\s+',' ',im[0])
                mt=re.search(r'\bfor\s+(.+)$',hd) or re.search(r'impl(?:<[^>]*>)?\s+(.+)$',hd)
                st=tname(mt.group(1)) if mt else None
                fld=base[5:].split('.')[0]
                ty=structs.get(st,{}).get(fld)
        else:
            ident=base.split('.')[0]
            if enc:
                sig=enc[4]
                pm=re.search(r'\b'+re.escape(ident)+r'\s*:\s*([^,)]+)',sig)
                if pm: ty=pm.group(1)
                if ty is None:
                    # binding patterns before the call
                    pre=s[enc[2]:r.start()]
                    bm=None
                    for b in re.finditer(r'(?:Some\(\s*'+re.escape(ident)+r'\s*\)\s*=\s*([^{;]+?)\s*(?:\{|else|;)|let\s+(?:mut\s+)?'+re.escape(ident)+r'\s*(?::\s*([^=]+?))?\s*=\s*([^;]+);|for\s+'+re.escape(ident)+r'\s+in\s+([^{]+)\{)',pre):
                        bm=b
                    if bm:
                        expr=bm.group(1) or bm.group(3) or bm.group(4) or ''
                        if bm.group(2): ty=bm.group(2)
                        e2=re.sub(r'\.as_mut\(\)|\.as_ref\(\)|\.take\(\)|\.unwrap\(\)|\.expect\([^)]*\)|\.as_deref_mut\(\)|\.as_deref\(\)|\.iter_mut\(\)|\.flatten\(\)|&mut |&','',expr).strip()
                        if e2.startswith('self.'):
                            im=enclosing_impl(m,r.start())
                            if im:
                                hd=re.sub(r'\s+',' ',im[0])
                                mt=re.search(r'\bfor\s+(.+)$',hd) or re.search(r'impl(?:<[^>]*>)?\s+(.+)$',hd)
                                st=tname(mt.group(1)) if mt else None
                                ty=structs.get(st,{}).get(e2[5:].split('.')[0].split('[')[0].split(')')[0])
                        elif re.match(r'^\w+(::\w+)*',e2):
                            ty=ty or e2
                if ty:
                    hdr=s[enc[1]:enc[2]]+(' '+s[enc[1]:enc[2]])
                    for g in re.findall(r'\b([A-Z]\w?)\b',ty):
                        if re.search(r'\b'+g+r'\s*:\s*[^,>{]*(?:InteractiveJob|ArtifactCommandWork|FixedOperationOwner)',hdr+(s[enc[2]:enc[2]+0])): ty=g+': InteractiveJob'
        if ty and is_ij(ty) or (ty and 'InteractiveJob' in ty):
            res.append((p,ln,'ij',recv+' : '+(ty or '?')[:60]))
for p,ln,k,t in res:
    print(f"{p}:{ln}: [{k}] {t} | {load(p)[0].splitlines()[ln-1].strip()[:120]}".replace('\n',' '))
print('ijtypes',len(ijtypes),'sites',len(res),file=sys.stderr)
