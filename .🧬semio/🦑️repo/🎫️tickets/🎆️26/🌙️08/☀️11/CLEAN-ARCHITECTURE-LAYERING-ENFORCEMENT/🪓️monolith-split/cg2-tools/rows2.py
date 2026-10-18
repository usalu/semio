import json,sys
d=json.load(open(sys.argv[1]))
rows=[]
for c in d['crates']:
    def r(x): return max(x[0]/32,x[1]/262144,x[2]/4000) if x else 0
    rows.append((max(r(c['lib']),r(c['test'])),c['name'].replace('semio-s-artifact-bim-model-',''),c['lib'],c['test']))
rows.sort(reverse=True)
print(len(rows),'over0.9',sum(1 for x in rows if x[0]>0.9),'over0.85',sum(1 for x in rows if x[0]>0.85))
for x in rows[:int(sys.argv[2])]: print('%.3f'%x[0],x[1],x[2],x[3])
print('problems',d['problems'][:5])
