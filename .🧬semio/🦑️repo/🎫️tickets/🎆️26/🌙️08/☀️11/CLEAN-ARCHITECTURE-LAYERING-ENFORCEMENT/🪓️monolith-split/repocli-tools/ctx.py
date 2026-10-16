import re,sys
sys.path.insert(0,'.')
from sib import src
k=sys.argv[1]; names=sys.argv[2:]
for n in names:
    for i,l in enumerate(src[k].split('\n')):
        if re.search(r'(?<![A-Za-z0-9_])'+n+r'(?![A-Za-z0-9_])',l): print(f"{k} {n} L{i+1}: {l.strip()[:150]}")
