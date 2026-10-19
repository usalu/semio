"""cstep: trait definitions whose methods mention InteractiveJobCloseStep. usage: traits.py listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s)
    for t in re.finditer(r'\bpub(?:\([^)]*\))?\s+trait\s+(\w+)[^{;]*\{|\btrait\s+(\w+)[^{;]*\{',m):
        b=t.end()-1;e=match_brace(m,b)
        blk=m[b:e]
        if 'InteractiveJobCloseStep' in blk:
            for f in re.finditer(r'fn\s+(\w+)[^{;]*',blk):
                if 'InteractiveJobCloseStep' in f.group(0):
                    print(p,t.group(1) or t.group(2),re.sub(r'\s+',' ',f.group(0))[:200],sep='\t')
