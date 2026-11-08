import sys,re,collections
t=open(sys.argv[1],errors='replace').read().split('\n')
out=[];i=0
while i<len(t):
    if t[i].startswith('error') and 'aborting' not in t[i] and 'could not compile' not in t[i]:
        m=t[i]
        j=i+1
        while j<len(t) and '-->' not in t[j]: j+=1
        loc=t[j].strip() if j<len(t) else ''
        out.append((loc,m))
    i+=1
c=collections.Counter()
for l,m in out:
    f=re.sub(r':\d+:\d+$','',l.replace('--> ','')); c[f.split('🔨️modules/🌱️value/')[-1][:90]]+=1
for f,n in c.most_common(): print(n,f)
if len(sys.argv)>2:
    for l,m in out:
        if sys.argv[2] in l: print(l.replace('--> ','')[-110:],'|',m[:170])
