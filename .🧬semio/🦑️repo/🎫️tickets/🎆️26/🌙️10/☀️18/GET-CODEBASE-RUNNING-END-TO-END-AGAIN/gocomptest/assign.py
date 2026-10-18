import json,re,collections
exec(open('table.py').read())
d=json.load(open('decls.json'))
L=open('orig.go.txt').read().split('\n')
def parse(spec):
    r=[]
    for part in spec.split(','):
        if not part: continue
        a,_,b=part.partition('-'); b=b or a
        r+=list(range(int(a),int(b)+1))
    return r
group={}
for e,s,desc,spec in T:
    for i in parse(spec):
        assert i not in group,i
        group[i]=s
for x in d:
    if x['kind']=='method' and x['idx'] not in group:
        pass
byname={}
for x in d:
    if x['kind']=='method': continue
    for n in x['name'].split(','): byname[n]=x['idx']
for x in d:
    if x['kind']=='method':
        group[x['idx']]=group[byname[x['recv']]] if byname[x['recv']] in group else None
text={x['idx']:'\n'.join(L[x['start']-1:x['end']]) for x in d}
unassigned=[x for x in d if x['idx'] not in group and x['kind']!='method']
# resolve helper placement iteratively
def users(name,exclude):
    pat=re.compile(r'\b'+re.escape(name)+r'\b')
    g=collections.Counter()
    for x in d:
        if x['idx'] in exclude: continue
        if pat.search(text[x['idx']]):
            g[group.get(x['idx'])]+=1
    return g
unas=set(x['idx'] for x in unassigned)
for _ in range(5):
    for x in unassigned:
        i=x['idx']
        gs=set()
        for n in x['name'].split(','):
            u=users(n,{i})
            gs|=set(k for k in u if k)
        # unresolved users (other unassigned helpers) ignored
        if len(gs)==1: group[i]=list(gs)[0]
        elif len(gs)>=2: group[i]=H
        elif _==4: group[i]=H
for x in d:
    if x['kind']=='method': group[x['idx']]=group[byname[x['recv']]]
json.dump({str(k):v for k,v in group.items()},open('assignment.json','w'),ensure_ascii=False)
sizes=collections.Counter()
for x in d: sizes[group[x['idx']]]+=x['end']-x['start']+3
for e,s,desc,spec in T: print(s,sizes[s])
print([ (x['idx'],x['name']) for x in d if group.get(x['idx']) is None])
