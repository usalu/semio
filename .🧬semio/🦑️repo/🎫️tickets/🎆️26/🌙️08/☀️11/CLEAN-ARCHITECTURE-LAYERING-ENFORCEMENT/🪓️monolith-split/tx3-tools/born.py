import json,sys,os,datetime,collections
G,SIM,OUT=sys.argv[1:4]
o=json.load(open(SIM))
head=set(l.rstrip('\n') for l in open(G+'/tx3-headdirs.txt',encoding='utf8'))
cut=datetime.datetime(2026,10,15).timestamp()
def born(p):
    try: return os.stat(p).st_birthtime>=cut
    except FileNotFoundError: return False
items=list(o['unres'])+list(o['ambig'].keys())
res=[p for p in items if not p.startswith('.')]
nd=sorted(p for p in res if p not in head or born(p))
json.dump(nd,open(OUT,'w'),ensure_ascii=False)
print('unresolved+ambiguous(non-dot)=',len(res),'born-or-new=',len(nd),'old=',len(res)-len(nd))
c=collections.Counter()
for p in nd:
    parts=p.split('/'); c['/'.join(parts[:4])]+=1
for k,v in c.most_common(40): print(v,k)
