import csv,collections,re,os,sys
R='/Users/ueli/Documents/semio/'
rows=list(csv.DictReader(open(sys.argv[2] if len(sys.argv)>2 else 'ftc-baseline-overfiles.tsv',encoding='utf8'),delimiter='\t'))
c=sys.argv[1]
for r in rows:
    if r['label']!='native' or r['crate']!=c: continue
    f=r['file']
    tag=('T' if r['notInLibNormal']=='true' else 'N')
    print(tag,r['kind'][:3],r['bytes'].rjust(7),r['lines'].rjust(5),f.replace('🧰️framework/','')[-100:])
    if r['kind']!='source': continue
    L=open(R+f,encoding='utf8').read().split('\n')
    for i,l in enumerate(L):
        if re.search(r'cfg\((any\()?(all\()?(not\()?test|cfg\(feature = "testkit"|cfg\(any\(test',l) :
            nxt=' / '.join(x.strip() for x in L[i+1:i+3])
            print('      L%d %s | %s'%(i+1,l.strip()[:90],nxt[:120]))
