import sys; sys.path.insert(0,'.')
from model7 import *
from plan7 import *
HOOKS_PURE='reserved_action_refusal mutation_refusal freezes_document document_refusal transaction_refusal holds_receipt terminal_is_empty window_config_published admitted_while_loading reads_history claims_action'.split()
HOOKS_CTX='dispatch_action drive_turn has_pending_work render_body after_app_render'.split()
def recv_of(txt):
    mm=re.search(r'\bfn\s+\w+\s*(?:<[^{(]*>)?\s*\(\s*(&\s*mut\s+self|&\s*self|mut\s+self|self)\b', txt)
    return re.sub(r'\s+','',mm.group(1)) if mm else None
def analyse():
    ms=load_methods()
    cm={x['name']:crate_of(x) for x in nodes}
    meth=collections.defaultdict(dict)   # crate -> name -> dict
    for m in ms:
        c=cm.get(m['name'])
        if c is None or m['file'].endswith('composer-api/🦀️.rs'): continue
        used=set(re.findall(r'\bself\s*\.\s*(\w+)', m['masked']))&set(FIELDS)
        hooks=set(re.findall(r'InstanceExtensions\s*::\s*(\w+)', m['masked']))
        rv=recv_of(m['masked'])
        calls=set()
        for x in nodes:
            if x['name']==m['name']: calls=set(x['calls']); break
        meth[c][m['name']]=dict(fields=used,hooks=hooks,recv=rv,calls=calls,m=m)
    return meth,cm
if __name__=='__main__':
    meth,cm=analyse()
    need={c:set() for c in ORDER}; hk={c:set() for c in ORDER}
    # direct
    for c in ORDER:
        for n,d in meth[c].items(): need[c]|=d['fields']; hk[c]|=d['hooks']
    # bridged closure (callee crates below)
    for c in ORDER:
        for n,d in meth[c].items():
            for callee in d['calls']:
                cc=cm.get(callee)
                if cc and cc!=c: pass
    changed=True
    while changed:
        changed=False
        for c in ORDER:
            for n,d in meth[c].items():
                for callee in d['calls']:
                    cc=cm.get(callee)
                    if cc and cc!=c and cc!='app':
                        if not need[cc] <= need[c]: need[c]|=need[cc]; changed=True
                        if not hk[cc] <= hk[c]: hk[c]|=hk[cc]; changed=True
    for c in ORDER:
        print('%-11s fields: %s'%(c,','.join(sorted(need[c],key=FIELDS.index))))
        print('%-11s hooks : %s'%('',','.join(sorted(hk[c]))))
