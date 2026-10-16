import sys; sys.path.insert(0,'.')
from src7 import *
RANK={c:i for i,c in enumerate(ORDER)}
PURE_HOOKS='reserved_action_refusal mutation_refusal freezes_document document_refusal transaction_refusal holds_receipt terminal_is_empty'.split()
STATIC_HOOKS='claims_action reads_history admitted_while_loading'.split()
CTX_HOOKS='dispatch_action drive_turn render_body after_app_render has_pending_work'.split()
WHOLE=set(FIELDS)-{'mounted_policy'}
class Table:
    def __init__(self, ms):
        self.by={}
        for m in ms:
            if m['crate']=='composer-api': continue
            self.by.setdefault(m['name'],[]).append(m)
        self.free={}   # assoc fn name -> generics suffix
    def get(self,name):
        v=self.by.get(name)
        if not v: return None
        fam=[m for m in v if m['crate']!='app'] or v
        return fam[0]
def call_span(M, raw, name_end):
    q=name_end
    while q<len(M) and M[q].isspace(): q+=1
    turbo=''
    if M.startswith('::',q):
        r=q+2
        while M[r].isspace(): r+=1
        if M[r]=='<':
            e=match_angle(M,r); turbo=raw[q:e+1]; q=e+1
            while q<len(M) and M[q].isspace(): q+=1
    if q<len(M) and M[q]=='(': return turbo,q
    return None
RX_SELF=re.compile(r'(?<![\w.])self\s*\.\s*(\w+)')
RX_HOOK=re.compile(r'\bInstanceExtensions\s*::\s*(\w+)')
RX_ASSOC=re.compile(r'(?<![\w:])(?:Self|VcsArtifactApp(?:\s*::\s*<[^>]*>)?)\s*::\s*(\w+)')
def rewrite(raw, caller, T, flags):
    """caller: dict(crate, view:bool, name)"""
    M=mask(raw); cc=caller['crate']; cview=caller['view']
    out=[]; i=0
    while True:
        cands=[m for m in (RX_SELF.search(M,i),RX_HOOK.search(M,i),RX_ASSOC.search(M,i)) if m]
        if not cands: out.append(raw[i:]); break
        m=min(cands,key=lambda x:x.start())
        out.append(raw[i:m.start()])
        name=m.group(1)
        call=call_span(M,raw,m.end())
        txt_m=M[m.start():m.end()]
        if txt_m.startswith('InstanceExtensions'):
            if call is None:
                flags.append('%s: hook %s without call'%(caller['name'],name)); out.append(raw[m.start():m.end()]); i=m.end(); continue
            turbo,op=call; cp=match_close(M,op,'(',')')
            args=[rewrite(raw[a:b],caller,T,flags).strip() for a,b in split_args(M,raw,op+1,cp)]
            rest=args[1:] if args and args[0] in ('self','&mut self','&self') else None
            HP='InstanceExtensions' if cc=='app' else 'X'
            LT='&self.time_travel' if cc=='app' else '&*self.time_travel'
            LR='&self.tool_runs' if cc=='app' else '&*self.tool_runs'
            EC='&mut self.extension_ctx()' if cc=='app' else '&mut self.as_extension()'
            EV='&self.extension_ref()'
            if name in STATIC_HOOKS: out.append('%s::%s(%s)'%(HP,name,', '.join(args)))
            elif rest is None: flags.append('%s: hook %s: first argument %r is not self'%(caller['name'],name,args[:1])); out.append(raw[m.start():cp+1])
            elif name in PURE_HOOKS: out.append('%s::%s(%s)'%(HP,name,', '.join([LT,LR]+rest)))
            elif name=='window_config_published': out.append('%s::window_config_published(%s)'%(HP,'&mut self.time_travel, &mut self.tool_runs' if cc=='app' else '&mut *self.time_travel, &mut *self.tool_runs'))
            elif name=='has_pending_work': out.append('%s::has_pending_work(%s)'%(HP,EV))
            elif name in CTX_HOOKS: out.append('%s::%s(%s)'%(HP,name,', '.join([EC]+rest)))
            else: flags.append('%s: unknown hook %s'%(caller['name'],name)); out.append(raw[m.start():cp+1])
            i=cp+1; continue
        if txt_m.startswith('Self') or txt_m.startswith('VcsArtifactApp'):
            callee=T.get(name)
            if callee is not None and callee['recv'] is None and callee['crate']!='app':
                gen=T.free.get(name,'')
                out.append('%s%s'%(name,gen if call is not None or True else '')); i=m.end()
                if call is not None and call[0]:   # explicit turbofish on the call itself
                    flags.append('%s: assoc call %s with its own turbofish'%(caller['name'],name))
                continue
            flags.append('%s: %s'%(caller['name'],txt_m)); out.append(raw[m.start():m.end()]); i=m.end(); continue
        callee=T.get(name)
        if call is None or callee is None or callee['crate']=='composer-api':
            out.append(raw[m.start():m.end()]); i=m.end(); continue
        turbo,op=call; cp=match_close(M,op,'(',')')
        ccr=callee['crate']
        if callee['recv'] is None:
            flags.append('%s: self.%s(..) on an associated function'%(caller['name'],name)); out.append(raw[m.start():m.end()]); i=m.end(); continue
        recv_txt=None
        if ccr==cc:
            if callee['recv']=='ref' and cc in VIEW and not cview: recv_txt='self.shared()'
            elif callee['recv']!='ref' and cview: flags.append('%s: view member calls &mut member %s'%(caller['name'],name))
        elif cc=='app' and ccr!='app' or (ccr!='app' and RANK[ccr]<RANK[cc]):
            snake=SNAKE[ccr]
            if callee['recv']=='ref': recv_txt='self.%s_ref()'%snake
            else:
                recv_txt='self.%s_ctx()'%snake if cc=='app' else 'self.as_%s()'%snake
                if cview: flags.append('%s: view member needs &mut bridge to %s'%(caller['name'],name))
            if cc=='app' and callee['recv']=='ref': recv_txt='self.%s_ref()'%snake
        else:
            flags.append('%s: UPWARD call %s -> %s (%s)'%(caller['name'],cc,ccr,name))
        argspans=split_args(M,raw,op+1,cp)
        rargs=[rewrite(raw[a:b],caller,T,flags) for a,b in argspans]
        if recv_txt is None:
            out.append(raw[m.start():m.end()]+turbo+'('+','.join(rargs)+')'); i=cp+1; continue
        hs=[bool(re.search(r'\bself\b',M[a:b])) for a,b in argspans]
        need=any(hs)
        if need:
            lets=' '.join('let __a%d = %s;'%(k,a.strip()) for k,a in enumerate(rargs) if hs[k])
            call_txt='%s.%s%s(%s)'%(recv_txt,name,turbo,', '.join(('__a%d'%k) if hs[k] else rargs[k].strip() for k in range(len(rargs))))
            mm=re.match(r'\s*\.\s*await\b',M[cp+1:cp+14])
            if mm and callee['is_async']:
                out.append('({ %s %s%s })'%(lets,call_txt,raw[cp+1:cp+1+mm.end()])); i=cp+1+mm.end()
            else:
                out.append('({ %s %s })'%(lets,call_txt)); i=cp+1
        else:
            out.append('%s.%s%s(%s)'%(recv_txt,name,turbo,','.join(rargs))); i=cp+1
    return ''.join(out)
def whole_borrows(txt):
    M=mask(txt); pos=0; res=[]
    rx=re.compile(r'&\s*(mut\s+)?self\s*\.\s*(\w+)\b(?!\s*[.\[(])')
    for m in rx.finditer(M):
        f=m.group(2)
        if f in WHOLE:
            res.append(txt[pos:m.start()]); res.append('&%sself.%s'%('mut *' if m.group(1) else '*',f)); pos=m.end()
    res.append(txt[pos:])
    return ''.join(res)
