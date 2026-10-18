import json,glob,collections
exec(open('table.py').read())
d=json.load(open('decls2.json'))
L=open('orig.go.txt').read().split('\n')
def isreg(l): return l.startswith('// #region') or l.startswith('// #endregion')
def parse(spec):
    r=[]
    for part in spec.split(','):
        a,_,b=part.partition('-'); b=b or a
        r+=list(range(int(a),int(b)+1))
    return r
file_of={}
for e,s,desc,spec in T:
    for i in parse(spec): file_of[i]=e+FE+s+'.go'
bad=0
cache={}
for x in d:
    body=L[x['start']-1:x['end']]
    j=0
    while j<len(body) and body[j].startswith('//'): j+=1
    head=[l for k,l in enumerate(body[:j]) if not isreg(l) and not (k>0 and body[k-1].startswith('// #region'))]
    txt='\n'.join(head+body[j:])
    fn=file_of[x['idx']]
    if fn not in cache: cache[fn]=open('out/'+fn).read()
    if txt not in cache[fn]:
        bad+=1; print('NOT VERBATIM',x['idx'],x['name'][:40])
print('decls',len(d),'not verbatim',bad)
# exactly-once counts of decl first lines
tot=sum(len(open(f).read().split('\n')) for f in glob.glob('out/*.go'))
print('total lines out',tot)
