import csv,sys
csv.field_size_limit(10**9)
path=sys.argv[1]; thr=float(sys.argv[2]) if len(sys.argv)>2 else 0.9
rows=[]
for r in csv.DictReader(open(path)):
    def f(x):
        try: return float(x)
        except: return 0.0
    lr,tr=f(r['libRatio']),f(r['testRatio'])
    if max(lr,tr)>=thr:
        rows.append((max(lr,tr),r['name'],r['workspace'],r['libFiles'],r['libBytes'],r['libLines'],lr,r['libBinding'],r['testFiles'],r['testBytes'],r['testLines'],tr,r['testBinding'],r['blocksAny']))
rows.sort(reverse=True)
print("max\tname\tws\tlib f/b/l\tlibR\tbind\ttest f/b/l\ttestR\tbind\tblocksAny")
for m,n,w,lf,lb,ll,lr,lbi,tf,tb,tl,tr,tbi,ba in rows:
    print(f"{m:.3f}\t{n}\t{w}\t{lf}/{lb}/{ll}\t{lr:.3f}\t{lbi}\t{tf}/{tb}/{tl}\t{tr:.3f}\t{tbi}\t{ba}")
