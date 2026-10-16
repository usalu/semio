import sys; sys.path.insert(0,'.')
from ext7 import *
from src7 import *
import difflib
ms=methods()
TBL={}
for m in ms:
    if m['crate'] in ('app','composer-api') or m['recv'] is None: continue
    if m['vis']=='pub' or m['name']=='time_travel_ledger': continue    # kept as composer delegations
    sg=m['sig']; n=len(sg['params'])-(1 if sg['recv'] else 0)
    TBL.setdefault(m['name'],[]).append((m,n))
NAMES=sorted(TBL,key=len,reverse=True)
PAT=re.compile(r'(?<![\w.])([A-Za-z_][\w]*(?:\.[A-Za-z_]\w*)*)\s*\.\s*('+'|'.join(map(re.escape,NAMES))+r')\b\s*(?:::<[^>]*>)?\s*\(')
def rewrite_consumer(text, changed=None):
    top = changed is None
    if top: changed=[]
    M=mask(text); out=[]; i=0
    while True:
        m=PAT.search(M,i)
        if not m: out.append(text[i:]); break
        recv,name=m.group(1),m.group(2)
        op=m.end()-1; cp=match_close(M,op,'(',')')
        argspans=split_args(M,text,op+1,cp)
        ent=[e for e in TBL[name] if e[1]==len(argspans)]
        if not ent or recv in ('self','Self') or recv.startswith('self.'):
            out.append(text[i:m.end()]); i=m.end(); continue
        meth=ent[0][0]
        snake=SNAKE[meth['crate']]
        accessor='%s_view'%snake if meth['recv']=='ref' else '%s_ctx'%snake
        hs=[bool(re.search(r'(?<![\w.])'+re.escape(recv.split('.')[0])+r'\b',M[a:b])) for a,b in argspans]
        out.append(text[i:m.start()])
        rargs=[rewrite_consumer(text[a:b],changed) for a,b in argspans]
        call_name=name+(re.search(r'::\s*<[^>]*>',text[m.start(2):op]) .group(0) if re.search(r'::\s*<[^>]*>',text[m.start(2):op]) else '')
        if any(hs):
            lets=' '.join('let __a%d = %s;'%(k,a.strip()) for k,a in enumerate(rargs) if hs[k])
            callargs=', '.join(('__a%d'%k) if hs[k] else rargs[k].strip() for k in range(len(rargs)))
            after=M[cp+1:cp+14]
            mm=re.match(r'\s*\.\s*await\b',after)
            if mm and meth['is_async']:
                out.append('{ %s %s.%s().%s(%s)%s }'%(lets,recv,accessor,call_name,callargs,text[cp+1:cp+1+mm.end()])); i=cp+1+mm.end()
            else:
                out.append('{ %s %s.%s().%s(%s) }'%(lets,recv,accessor,call_name,callargs)); i=cp+1
        else:
            out.append('%s.%s().%s(%s)'%(recv,accessor,call_name,','.join(rargs))); i=cp+1
        changed.append((recv,name))
    return (''.join(out),changed) if top else ''.join(out)
if __name__=='__main__':
    files=plugin_files()
    fam=set(PLUGREL+'/'+r for r in TIER2_FILES)
    total=0; plan={}
    for f in files:
        if f in fam or '/📥️staging/' in f or not os.path.exists(ROOT+f): continue
        if '/📦️packages/' in f: continue
        t=open(ROOT+f,encoding='utf-8').read()
        nt,ch=rewrite_consumer(t)
        if ch:
            plan[f]=(t,nt,ch); total+=len(ch)
    for f,(t,nt,ch) in plan.items(): print(len(ch),f.replace(PLUGREL+'/',''),collections.Counter(n for _,n in ch).most_common(6))
    print('total',total)
    json.dump({f:nt for f,(t,nt,ch) in plan.items()},open(GEN+'/pwp7-consumers.json','w'),ensure_ascii=False)
