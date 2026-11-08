import re,sys
t=open(sys.argv[1],encoding='utf8').read()
bl=re.split(r'\n(?=error)',t)
for b in bl:
    if not b.startswith('error'): continue
    m=re.search(r'--> .*?/(?:\.\./)+([^/\s]+\.rs):(\d+):(\d+)',b)
    first=b.split('\n')[0][:150]
    extra=[l.strip()[:200] for l in b.split('\n') if l.strip().startswith('= note') or 'expected' in l or 'found' in l][:2]
    print(first,'@',m.group(1,2,3) if m else '', ' | '.join(extra))
