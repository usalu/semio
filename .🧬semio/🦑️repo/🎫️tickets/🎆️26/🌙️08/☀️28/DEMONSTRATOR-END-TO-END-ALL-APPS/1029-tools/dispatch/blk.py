import sys,re
f,crate=sys.argv[1],sys.argv[2]
t=open(f,encoding='utf-8',errors='replace').read().split('\n')
start=None
for i,l in enumerate(t):
    if l.startswith('    Checking '+crate+' ') or l.startswith('   Compiling '+crate+' '): start=i
if start is None: sys.exit('no crate')
n=int(sys.argv[3]) if len(sys.argv)>3 else 16
i=start+1
while i<len(t):
    l=t[i]
    if l.startswith('error') :
        print('\n'.join(x[:300] for x in t[i:i+n]))
        print('-----')
        i+=n
        continue
    if l.startswith('    Checking ') or l.startswith('warning: `'): 
        if i>start+1 and l.startswith('warning: `'): break
    i+=1
