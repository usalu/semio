"""cstep scanner: remaining `<InteractiveJobCloseStep or alias>::Refused` / `Refused(..)` uses. usage: refleft.py listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask
n=0;files=0
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if 'Refused' not in s: continue
    names={'InteractiveJobCloseStep'}
    for a in re.finditer(r'InteractiveJobCloseStep\s+as\s+(\w+)',s): names.add(a.group(1))
    m=mask(s)
    hits=[x for x in re.finditer(r'\b(?:'+'|'.join(names)+r')::Refused\b',m)]
    if not hits: continue
    files+=1
    for h in hits:
        ln=s.count('\n',0,h.start())+1;n+=1
        print(f"{p}:{ln}: {s.splitlines()[ln-1].strip()[:150]}")
print('files',files,'hits',n,file=sys.stderr)
