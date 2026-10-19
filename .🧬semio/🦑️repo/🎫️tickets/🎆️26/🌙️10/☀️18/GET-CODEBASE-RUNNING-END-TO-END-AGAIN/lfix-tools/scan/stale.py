"""stale-call scanner: arity-based. usage: stale.py [--list] roots..."""
import re,sys,os,collections
AR={'close_step':2,'render_registered_fixture_app':4,'dispatch_typed':4,'settle_registered_typed_operation':5,'settle_history_verb':6,'settle_extension_invocations':7,'close_registered_fixture_app':3,'load_document':4,'load_document_text':4,'plugin_load_document':5,'plugin_load_document_text':5,'plugin_settle_store_constructor':4,'dispatch_fixture_action':7,'dispatch_fixture_response_action':8,'assert_undo_redo_round_trip':8,'handle_action':5,'handle_response_action':6,'maintenance_step':2,'advance_typed_operation_publication':2,'render':4,'consume_media':3,'produce_media':3,'import_media':4,'ingest_operations':3,'ingest_operations_text':2,'tick_backbone':2,'handle_command_frame':4,'handle_intent_frame':4,'dispatch_config_command':4,'render_registered_fixture_app':4,'drain_maintenance_pressure':2,'document_input_commit_lint':4,'agent_preview':7}
UF={'maintenance_step':3}
def split_args(s):
    n=0;d=0;i=0;cur=False
    while i<len(s):
        c=s[i]
        if c=='"':
            i+=1
            while i<len(s) and s[i]!='"':
                if s[i]=='\\': i+=1
                i+=1
            cur=True
        elif c=="'" and i+2<len(s) and (s[i+2]=="'" or (s[i+1]=='\\' and s[i+3:i+4]=="'")):
            i+=2 if s[i+1]!='\\' else 3; cur=True
        elif c in '([{': d+=1;cur=True
        elif c in ')]}': d-=1
        elif c==',' and d==0:
            n+=1;cur=False
        elif not c.isspace(): cur=True
        i+=1
    return n+(1 if cur else 0)
pat=re.compile(r'(?P<pre>(?:\.|::|\b))(?P<name>'+'|'.join(sorted(AR,key=len,reverse=True))+r')\s*(?:::<[^()]*>)?\s*\(')
def scan(path,s):
    out=[]
    for m in pat.finditer(s):
        name=m.group('name');pre=m.group('pre')
        before=s[max(0,m.start()-14):m.start()]
        if re.search(r'fn\s+$',before): continue
        if name=='import_media' and pre!='.': continue
        if name in('render','handle_action','handle_response_action','maintenance_step','consume_media','produce_media','import_media','ingest_operations','ingest_operations_text','tick_backbone','advance_typed_operation_publication','dispatch_typed','handle_command_frame','handle_intent_frame','dispatch_config_command') and pre=='': 
            continue
        ls=s.rfind('\n',0,m.start())+1
        if s[ls:m.start()].lstrip().startswith('//'): continue
        i=m.end();d=1;j=i;instr=False
        while j<len(s) and d>0:
            c=s[j]
            if c=='"':
                j+=1
                while j<len(s) and s[j]!='"':
                    if s[j]=='\\': j+=1
                    j+=1
            elif c=='(' : d+=1
            elif c==')': d-=1
            j+=1
        args=s[i:j-1]
        n=split_args(args)
        want=AR[name]
        if name=='maintenance_step' and pre=='::': want=3
        if name in('render',) and pre=='::': continue
        cxtok=bool(re.search(r'\bcx\b|StepContext|original_cx|\bcontext\b',args))
        if name=='ingest_operations_text': cxtok='identity' in args
        if name=='close_step':
            rc=pre
            if pre=='.' :
                rcv=s[max(0,m.start()-30):m.start()]
                if not re.search(r'\b(app|self|runtime)\s*$|\*app\s*$',rcv): continue
            elif not s[max(0,m.start()-12):m.start()].endswith('PluginApp'): continue
        if (n!=want) or (not cxtok and name not in('render_registered_fixture_app',)):
            out.append((s.count('\n',0,m.start())+1,name,n,want))
    return out
if __name__=='__main__':
    lst='--list' in sys.argv
    roots=[a for a in sys.argv[1:] if not a.startswith('--')]
    tot=collections.Counter();byp=collections.defaultdict(collections.Counter);files=collections.Counter()
    for root in roots:
        for dp,dn,fn in os.walk(root):
            if '/node_modules' in dp or '🗑️generated' in dp or '/target' in dp: continue
            for f in fn:
                if not f.endswith('.rs'): continue
                p=os.path.join(dp,f)
                try:s=open(p,encoding='utf-8').read()
                except: continue
                r=scan(p,s)
                if not r: continue
                parts=p.split('/')
                plug=parts[2] if len(parts)>2 else p
                files[p]=len(r)
                for ln,name,n,want in r:
                    tot[name]+=1;byp[plug][name]+=1
                    if lst: print(f"{p}:{ln}: {name} args={n} want={want}")
    if not lst:
        for k,v in tot.most_common(): print(v,k)
        print('--')
        for pl,c in sorted(byp.items(),key=lambda x:-sum(x[1].values())): print(pl,sum(c.values()),dict(c))
        print('files',len(files),'sites',sum(tot.values()))
