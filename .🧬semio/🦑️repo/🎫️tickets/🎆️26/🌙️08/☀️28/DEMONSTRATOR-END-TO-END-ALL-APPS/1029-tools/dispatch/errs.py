import sys,re
t=open(sys.argv[1],encoding='utf-8',errors='replace').read().split('\n')
pre='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/'
i=0;out=[]
while i<len(t):
    l=t[i]
    if l.startswith('error') and not l.startswith('error: aborting') :
        msg=l[:200];loc=''
        for j in range(i+1,min(i+8,len(t))):
            m=re.match(r'\s*--> (.*)',t[j])
            if m: loc=m.group(1).replace(pre,'~/'); break
        out.append((msg,loc))
    i+=1
for m,l in out: print(m,'\n    ',l[:250])
