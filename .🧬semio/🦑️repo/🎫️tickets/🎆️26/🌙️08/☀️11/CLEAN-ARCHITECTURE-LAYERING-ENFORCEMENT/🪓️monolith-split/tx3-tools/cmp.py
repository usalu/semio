import json,sys,os,datetime,collections
G,INV,SIM=sys.argv[1:4]
inv=json.load(open(INV))
v=inv['violations']
c=collections.Counter(x['code'] for x in v)
print('codes',{k:c[k] for k in ('directory-kind-unresolved','directory-kind-ambiguous','fixed-directory-contract-ambiguous')})
real=set(x['path'] for x in v if x['code'] in('directory-kind-unresolved','directory-kind-ambiguous'))
o=json.load(open(SIM)); sim=set(o['unres'])|set(o['ambig'])
head=set(l.rstrip('\n') for l in open(G+'/tx3-headdirs.txt',encoding='utf8'))
cut=datetime.datetime(2026,10,15).timestamp()
def born(p):
    try: return os.stat(p).st_birthtime>=cut
    except FileNotFoundError: return False
def isnew(p): return (not p.startswith('.')) and (p not in head or born(p))
rn=sorted(p for p in real if isnew(p))
print('real total',len(real),'real nondot',len([p for p in real if not p.startswith('.')]),'real born/new',len(rn))
print('sim total',len(sim),'real-not-sim',len(real-sim),'sim-not-real',len(sim-real))
for p in rn: print(' REAL-NEW',p, p in sim)
rns=sorted(p for p in real-sim)
for p in rns[:40]: print(' REAL-NOT-SIM',p)
json.dump(sorted(real),open(G+'/tx2-real-paths-before.json','w'),ensure_ascii=False)
