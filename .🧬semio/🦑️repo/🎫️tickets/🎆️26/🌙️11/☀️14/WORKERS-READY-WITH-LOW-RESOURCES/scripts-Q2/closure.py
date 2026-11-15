import sys,os,re,json
sys.path.insert(0,'/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/Q2')
import resolve as R
pk={}
for dp,dn,fn in os.walk(R.ROOT+'/🧰️framework'):
    dn[:]=[d for d in dn if d not in R.SKIP and d not in('dist','🤖️generated')]
    if 'package.json' in fn:
        try:j=json.load(open(dp+'/package.json',encoding='utf8'))
        except:continue
        if str(j.get('name','')).startswith('@semio'): pk[j['name']]=(dp,j.get('exports') or {})
def bare(spec):
    for n,(d,ex) in pk.items():
        if spec==n or spec.startswith(n+'/'):
            sub='.'+spec[len(n):]
            t=ex.get(sub) if isinstance(ex,dict) else None
            if isinstance(t,dict): t=t.get('default') or t.get('import') or next(iter(t.values()),None)
            if isinstance(t,str): return os.path.normpath(os.path.join(d,t))
    return None
dyn=re.compile(r'import\(\s*[\'"]([^\'"]+)[\'"]\s*\)')
starts=sys.argv[1:] or [R.ROOT+'/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts']
seen=set();q=list(starts);bad=[]
while q:
    p=q.pop()
    if p in seen or not p.endswith(R.EXT) or not os.path.isfile(p): continue
    seen.add(p)
    t=R.read(p) or ''
    specs=[s for _,_,s,_ in R.imports(p)]+dyn.findall(t)+re.findall(r'(?:^|\n)\s*import\s*[\'"]([^\'"]+)[\'"]',t)
    for s in specs:
        r=R.resolve(p,s) if s.startswith('.') else bare(s)
        if r: q.append(r)
    bad+=R.check(p)
print('closure files',len(seen))
for b in bad: print(b[0][len(R.ROOT)+1:],b[1],b[2],b[3])
