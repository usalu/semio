"""free-function arity drift against artifact_app_laws: usage stale2.py [--list] roots"""
import re,sys,os,json,collections
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from portlib import *
free=json.load(open('/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/scan/laws-arity.json'))
SKIP={'meta','fixture_identity','fixture_mounted_policy','typing_run','fixture_step_context','fixture_progress','fixture_retained_turn','project_and_retire_fixture_tree','decode_fixture_scene','decode_fixture_scene_with_lanes','built_surface_scene','built_carrier_text','fixture_carrier_text','fixture_scene_node','document_input_commit_findings'}
names=[n for n in free if n not in SKIP]
pat=re.compile(r'(?P<pre>::|(?<![A-Za-z0-9_.]))(?P<name>'+'|'.join(sorted(map(re.escape,names),key=len,reverse=True))+r')\s*(?:::<[^()]*>)?\s*\(')
def scan(s):
    out=[]
    for m in pat.finditer(s):
        name=m.group('name')
        if m.group('pre')=='::':
            pre=s[max(0,m.start()-40):m.start()]
            if not re.search(r'(artifact_app_laws|app)\s*$',pre) and not pre.rstrip().endswith('laws'):
                continue
        before=s[max(0,m.start()-16):m.start()]
        if re.search(r'fn\s+$',before): continue
        ls=s.rfind('\n',0,m.start())+1
        if s[ls:m.start()].lstrip().startswith('//'): continue
        i=m.end()-1;j=match_paren(s,i)
        if j<0: continue
        n=len(split_top(s[i+1:j]))
        # nested generic commas could inflate, so only flag when n<want (missing args) or n>want
        if n!=free[name]:
            out.append((s.count('\n',0,m.start())+1,name,n,free[name]))
    return out
if __name__=='__main__':
    lst='--list' in sys.argv
    roots=[a for a in sys.argv[1:] if not a.startswith('--')]
    tot=collections.Counter();byp=collections.defaultdict(collections.Counter)
    for root in roots:
        for dp,dn,fn in os.walk(root):
            if '/node_modules' in dp or '🗑️generated' in dp: continue
            for f in fn:
                if not f.endswith('.rs'):continue
                p=os.path.join(dp,f)
                try:s=read(p)
                except:continue
                for ln,name,n,w in scan(s):
                    tot[name]+=1;byp[p.split('/')[2] if len(p.split('/'))>2 else p][name]+=1
                    if lst:print(f"{p}:{ln}: {name} args={n} want={w}")
    if not lst:
        for k,v in tot.most_common():print(v,k)
        print('--')
        for pl,c in sorted(byp.items(),key=lambda x:-sum(x[1].values())):print(pl,sum(c.values()),dict(c))
