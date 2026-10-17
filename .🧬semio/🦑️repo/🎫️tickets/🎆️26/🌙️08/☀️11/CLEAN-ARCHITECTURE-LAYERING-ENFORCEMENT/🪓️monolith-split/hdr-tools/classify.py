import csv,sys,re,json
csv.field_size_limit(10**9)
OWN=[
 (r'puzzle-(2d|5d)$|puzzle-(2d|5d)-','r8a'),(r'bim-','r9a'),
 (r'raster-','bd10'),
 (r'process-process3d-host','bd10'),
]
def owner(n):
    for p,o in OWN:
        if re.search(p,n): return o
    return ''
rows=[]
for r in csv.DictReader(open(sys.argv[1])):
    f=lambda x: float(x) if x not in ('','refused') else 0.0
    lr,tr=f(r['libRatio']),f(r['testRatio'])
    m=max(lr,tr)
    if m>=float(sys.argv[2]):
        rows.append(dict(name=r['name'],ws=r['workspace'],manifest=r['manifest'],lf=r['libFiles'],lb=r['libBytes'],ll=r['libLines'],lr=lr,lbi=r['libBinding'],tf=r['testFiles'],tb=r['testBytes'],tl=r['testLines'],tr=tr,tbi=r['testBinding'],m=m,own=owner(r['name']),blocks=r['blocksAny']))
rows.sort(key=lambda x:-x['m'])
json.dump(rows,open(sys.argv[3],'w'),ensure_ascii=False,indent=0)
print(len(rows),sum(1 for r in rows if r['own']),sum(1 for r in rows if not r['own']))
