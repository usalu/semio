import re,sys
t=open(sys.argv[1],errors='replace').read()
pat=sys.argv[2] if len(sys.argv)>2 else ''
bl=re.split(r'\n(?=error(?:\[E\d+\])?: |warning: )',t)
n=0
for b in bl:
    if not b.startswith('error') or 'could not compile' in b[:40]: continue
    m=re.search(r'-->\s+(\S+?:\d+:\d+)',b)
    loc=m.group(1) if m else '?'
    if pat in loc or pat in b[:300]:
        n+=1;print(b.split('\n')[0][:130],'@',loc.split('/')[-3] + '/' + loc.split('/')[-2]+'/'+loc.split('/')[-1][:30] if loc!='?' else '?')
print(n)
