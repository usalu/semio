import re, collections, sys
from common import *
import model
from rs import blank
o = model.owners()
for k in sys.argv[1:]:
    cnt = collections.Counter(); sup = 0; glob = 0
    for f in o[k]:
        if '/🧪️tests/' in f or f.endswith('🧪️tests/🦀️.rs'): continue
        t = blank(read(PLUG + '/' + f))
        for m in re.finditer(r'(?<![A-Za-z0-9_$:])crate::(\{|\*|[A-Za-z_]\w*)', t): cnt[m.group(1)] += 1
        sup += len(re.findall(r'(?<![A-Za-z0-9_$:])super::', t)); glob += len(re.findall(r'use super::\*', t))
    print(k, 'files', len(o[k]), 'super::', sup, 'use super::*', glob, dict(cnt.most_common(14)))
