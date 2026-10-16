import json,sys,os,datetime,collections
G,TAX,SIM=sys.argv[1:4]
d=json.load(open(TAX)); mk=d['semanticDirectoryMemberKinds']
o=json.load(open(SIM)); kinds=o['kinds']
head=set(l.rstrip('\n') for l in open(G+'/tx2-headdirs.txt',encoding='utf8'))
cut=datetime.datetime(2026,10,15).timestamp()
def born(p):
    try: return os.stat(p).st_birthtime>=cut
    except FileNotFoundError: return False
fold=lambda s:s.replace('️','')
allnames={}
for r,s in mk.items():
    for n in s['memberNames']: allnames.setdefault(fold(n),[]).append(r)
dirs=[l.rstrip('\n') for l in open(G+'/tx2-dirs.txt',encoding='utf8')]
new=[p for p in dirs if not p.startswith('.') and (p not in head or born(p))]
c=collections.Counter(); via=collections.Counter(); unr=0
for p in new:
    k=kinds.get(p)
    if k is None: unr+=1; continue
    n=fold(os.path.basename(p))
    if k in mk:
        via['exact:'+k if k in allnames.get(n,[]) else 'pattern:'+k if 'memberSlugPattern' in mk[k] else 'other-row:'+k]+=1
    else: via['structural']+=1
print('born/new dirs',len(new),'unresolved',unr)
for k,v in via.most_common(12): print(v,k)
