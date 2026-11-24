import json,glob,sys
sys.path.insert(0,'.')
import audit as A
def walk(n):
    if isinstance(n,dict):
        yield n
        for v in n.values(): yield from walk(v)
    elif isinstance(n,list):
        for v in n: yield from walk(v)
for fam in ['en1990','en1992','en1996','en1995','en1998','din16798']:
    fd=[x for x in glob.glob(A.ROOT+'*') if x.endswith(fam)][0].split('/')[-1]
    sd=glob.glob(A.ROOT+fd+'/🏅️standards/*/🪆️subsets/*/🧬️schema')[0]
    snap=json.load(open(sd+'/📸️snapshot/🔣️.json'))
    cands={}
    for n in walk(snap):
        if 'enum' in n and isinstance(n['enum'],list) and (n.get('x-semio-ui') or {}).get('options'):
            cands.setdefault(tuple(n['enum']),n['x-semio-ui']['options'])
    done=set()
    for o in A.audit(fd):
        if not o[2].startswith('OptionLabel') or (o[0],o[1]) in done: continue
        done.add((o[0],o[1]))
        p=sd+'/🧬️mutations/'+o[0]+'/🧬️schema/🔣️.json'
        s=open(p).read(); d=json.loads(s)
        assert json.dumps(d,indent=2,ensure_ascii=False)+"\n"==s,p
        node=d
        parts=[x for x in o[1].split('/') if x]
        # resolve through $ref (for embedded records)
        def res(n):
            while '$ref' in n and 'properties' not in n:
                t=snap
                for q in n['$ref'].split('#/')[1].split('/'): t=t[q]
                n=t
            return n
        ok=True
        node=res(node)
        for i,q in enumerate(parts):
            node=res(node)['properties'][q] if q!='-' else res(node)['items']
        node=res(node) if 'enum' not in node else node
        enum=tuple(node['enum'])
        opts=cands.get(enum)
        if not opts: print(fam,o[0],o[1],'NO CANDIDATE',enum); continue
        if parts and len(parts)==1:
            ui=d['properties'][parts[0]].setdefault('x-semio-ui',{})
            ui['options']=opts
            open(p,'w').write(json.dumps(d,indent=2,ensure_ascii=False)+"\n"); print(fam,o[0],o[1],'applied')
        else: print(fam,o[0],o[1],'NESTED (snapshot edit)',enum)
