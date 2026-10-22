"""🧩 Every `$crate::path` of the exported macros of a hub with its resolution: first non-facade crate and the path inside it, or own (hub-defined) / unresolved. `python3 macro_paths.py <hub> [macro]`; library: macro_paths(hub) -> {macro: {path: result}}"""
from hubenv import *
import re
FAC={'semio_framework','semio_framework_os_kernel','semio_framework_plugin'}
def macro_paths(hub,world=None,only=None):
    d=json.load(open(OUT+'/census-%s.json'%hub));ID=d['ident']
    world=world or make_world()[0]
    m=world.model_for(ID);out=collections.OrderedDict()
    for f in ptree(hub):
        if not isprod(f): continue
        t=open(f,encoding='utf-8',errors='ignore').read()
        if 'macro_rules!' not in t: continue
        ms=list(re.finditer(r'macro_rules!\s+(\w+)\s*\{',t))
        for i,mm in enumerate(ms):
            if only and mm.group(1)!=only: continue
            end=ms[i+1].start() if i+1<len(ms) else len(t)
            body=t[mm.end():end];res=collections.OrderedDict()
            for x in re.finditer(r'\$crate::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)',body):
                p=x.group(1)
                if p in res: continue
                segs=p.split('::');q=list(segs);r=None;suf=[]
                while q:
                    r=world.resolve(m,q)
                    if r: break
                    suf.insert(0,q.pop())
                if not r: res[p]=('unresolved',);continue
                if r['kind']=='external': res[p]=('external',r['owner']);continue
                st=[s for s in r.get('steps',[]) if s[0] not in FAC and s[0]!=ID]
                if st: res[p]=('rewrite',st[0][0],'::'.join(st[0][1:]+suf))
                else: res[p]=('own',r.get('file','')[-50:],r.get('name'))
            out[mm.group(1)]=dict(res,**{'__file':os.path.relpath(f,ROOT)})
    return out
if __name__=='__main__':
    for k,v in macro_paths(sys.argv[1],only=sys.argv[2] if len(sys.argv)>2 else None).items():
        print(k,v['__file'][-60:])
        for p,r in v.items():
            if p!='__file': print('    $crate::%-60s %s'%(p,r))
