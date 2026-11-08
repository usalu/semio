import re,sys
t=open(sys.argv[1],encoding='utf-8',errors='replace').read()
blocks=re.split(r'\n(?=error)',t)
out=[]
for b in blocks:
    if not b.startswith('error'): continue
    m=re.search(r'--> (\S+?):(\d+):(\d+)',b)
    head=b.split('\n')[0][:200]
    loc=''
    if m:
        p=m.group(1).split('../')[-1] if 'rust/../' in m.group(1) else m.group(1)
        loc=p+':'+m.group(2)
    out.append((loc,head))
for l,h in out: print(l,'|',h)
