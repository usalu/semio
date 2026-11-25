import glob,json,os,sys
R='/Users/ueli/Documents/semio'
S=R+'/🎓️teaching/🏛️architecture/🧍️staffage'
B=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))+'/batches'
owned=set()
for f in glob.glob(B+'/M*.tsv'):
    if f.endswith('/MU.tsv'): continue
    for l in open(f):
        p=l.rstrip('\n').split('\t')
        owned.update(x for x in p[:2] if x)
cats={}
for d in sorted(glob.glob(S+'/🗂️catalog/*/*/🔣️.json')):
    i=d.split('/')[-2]; c=d.split('/')[-3]
    a=json.load(open(d)); a=a.get('asset',a)
    cats[i]=(a.get('category',c),c,os.path.getsize(d),len(a.get('poses',[])),len(a.get('clips',[])),len(a.get('representations',{})))
rows=[(cats[i][0],i) for i in cats if i not in owned and i!='electric-fan-heater']
rows.sort()
print(len(cats),len(owned),len(rows),file=sys.stderr)
open(B+'/MU.tsv','w').write(''.join(f'{c}\t{i}\t\t\t\t\t\n' for c,i in rows))
for c,i in rows: print(c,i,*cats[i][2:])
