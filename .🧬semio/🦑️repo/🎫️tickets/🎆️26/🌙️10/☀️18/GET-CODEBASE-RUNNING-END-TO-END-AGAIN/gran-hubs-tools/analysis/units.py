"""🧱 Unit table of a monolithic file: top-level items grouped by owning name (struct/enum/trait/fn name, `impl [Trait for] Type` joined to Type), with lines, hunk-header churn (commits), references to other units (identifier match on skeleton text) and external production users from the census. `python3 units.py <hub> <file> [N]`"""
from hubenv import *
from rs import *
import re
def unit_name(it,sk):
    kw=it['kw']
    if kw=='impl':
        h=sk[it['start']:it['end']]
        h=h[:h.find('{')] if '{' in h else h
        m=re.search(r'\bimpl\b\s*(?:<[^>]*(?:<[^>]*>[^>]*)*>)?\s*(?:(?:[A-Za-z_:<>\'&, ]+?)\s+for\s+)?(?:&\s*(?:\'\w+\s+)?(?:mut\s+)?)?([A-Za-z_][A-Za-z0-9_]*)',h)
        return m.group(1) if m else 'impl?'
    return it.get('name') or ('use:'+(it.get('use','')[:30]) if kw=='use' else kw)
def header_type(h):
    m=re.match(r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(struct|enum|trait|fn|type|const|static|union|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)',h)
    if m: return m.group(2)
    m=re.match(r'^(?:unsafe\s+)?impl\b\s*(?:<.*?>)?\s*(?:(?:[^{]*?)\s+for\s+)?(?:&\s*(?:\'\w+\s+)?(?:mut\s+)?)?([A-Za-z_][A-Za-z0-9_]*)',h)
    if m: return m.group(1)
    return h[:30] or '<preamble>'
def churn_by_unit(rel):
    out=subprocess.run(['git','log','-p','-U0','--format=@@@%h','--',rel],cwd=ROOT,capture_output=True,text=True).stdout
    per=collections.defaultdict(set);cur=None
    for l in out.splitlines():
        if l.startswith('@@@'): cur=l[3:];continue
        m=re.match(r'@@ -\S+ \+\S+ @@ ?(.*)',l)
        if m: per[header_type(m.group(1).strip())].add(cur)
    return per
if __name__=='__main__':
    hub=sys.argv[1];f=os.path.join(ROOT,sys.argv[2]) if not sys.argv[2].startswith('/') else sys.argv[2];N=int(sys.argv[3]) if len(sys.argv)>3 else 40
    t,sk,items=parse(f)
    units=collections.OrderedDict()
    for it in items:
        nm=unit_name(it,sk)
        u=units.setdefault(nm,{'lines':0,'kinds':collections.Counter(),'text':'','pub':False})
        u['lines']+=t.count('\n',it['start'],it['end'])+1;u['kinds'][it['kw']]+=1;u['text']+=sk[it['start']:it['end']];u['pub']|=it['pub'] and it['kw']!='use'
    ch=churn_by_unit(os.path.relpath(f,ROOT))
    cs=json.load(open(OUT+'/census-%s.json'%hub));R_=cs['resolved'];ID=cs['ident']
    users=collections.defaultdict(set)
    for dd,v in cs['deps'].items():
        for p in v['prod']:
            r=R_[p]
            if r['kind']=='def' and r['owner']==ID and r['file']==os.path.relpath(f,ROOT): users[r['name']].add(dd)
    names=set(units)
    ref={}
    for nm,u in units.items():
        ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',u['text']))
        ref[nm]=sorted((ids&names)-{nm})
    rows=sorted(units,key=lambda n:-len(ch.get(n,())))
    print('units',len(units),'file lines',t.count('\n'))
    for n in rows[:N]:
        u=units[n];print('%-40s lines=%4d churn=%2d users=%4d pub=%d refs=%d %s'%(n[:40],u['lines'],len(ch.get(n,())),len(users.get(n,())),u['pub'],len(ref[n]),dict(u['kinds'])))
    import pickle;pickle.dump({'units':{k:{x:y for x,y in v.items() if x!='text'} for k,v in units.items()},'ref':ref,'churn':{k:sorted(v) for k,v in ch.items()},'users':{k:sorted(v) for k,v in users.items()}},open(OUT+'/units-%s.pkl'%hub,'wb'))
