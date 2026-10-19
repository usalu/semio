"""extra method-call arity scanner: usage stale3.py [--list] roots"""
import re,sys,os,json,collections
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from portlib import *
S='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/scan'
M=json.load(open(S+'/methods-arity.json'))
import importlib.util
spec=importlib.util.spec_from_file_location('stale',S+'/stale.py'); st=importlib.util.module_from_spec(spec); spec.loader.exec_module(st)
skip=set(st.AR)|{'close_step','maintenance_step','render','handle_action','advance_typed_operation_publication','dispatch_typed','ingest_operations','tick_backbone','import_media','poll_document_archive_load'}
names=[n for n in M if n not in skip]
EXTRA={'export_media':[3],'submit_media_export':[4],'document_load_archive':[1]}
pat=re.compile(r'\.\s*(?P<name>'+'|'.join(sorted(map(re.escape,names),key=len,reverse=True))+r')\s*(?:::<[^()]*>)?\s*\(')
pat2=re.compile(r'PluginApp::(?P<name>'+'|'.join(sorted(map(re.escape,names+['render']),key=len,reverse=True))+r')\s*\(')
def scan(s):
    out=[]
    for m in pat.finditer(s):
        name=m.group('name')
        ls=s.rfind('\n',0,m.start())+1
        if s[ls:m.start()].lstrip().startswith('//'):continue
        before=s[max(0,m.start()-14):m.start()]
        i=m.end()-1;j=match_paren(s,i)
        if j<0:continue
        n=len(split_top(s[i+1:j]))
        if n not in M[name]:
            out.append((s.count('\n',0,m.start())+1,name,n,M[name]))
    for m in pat2.finditer(s):
        name=m.group('name')
        i=m.end()-1;j=match_paren(s,i)
        if j<0:continue
        n=len(split_top(s[i+1:j]))-1
        want=M.get(name,[4] if name=='render' else None)
        if want is None:continue
        if n not in want: out.append((s.count('\n',0,m.start())+1,'PluginApp::'+name,n,want))
    return out
if __name__=='__main__':
    lst='--list' in sys.argv
    tot=collections.Counter();byp=collections.defaultdict(collections.Counter)
    for root in [a for a in sys.argv[1:] if not a.startswith('--')]:
        for dp,dn,fn in os.walk(root):
            if '/node_modules' in dp or '🗑️generated' in dp: continue
            for f in fn:
                if not f.endswith('.rs'):continue
                p=os.path.join(dp,f)
                try:s=read(p)
                except:continue
                for ln,name,n,w in scan(s):
                    tot[name]+=1;byp[p.split('/')[2]][name]+=1
                    if lst:print(f"{p}:{ln}: {name} args={n} want={w}")
    if not lst:
        for k,v in tot.most_common():print(v,k)
        print('--')
        for pl,c in sorted(byp.items(),key=lambda x:-sum(x[1].values())):print(pl,sum(c.values()),dict(c))
