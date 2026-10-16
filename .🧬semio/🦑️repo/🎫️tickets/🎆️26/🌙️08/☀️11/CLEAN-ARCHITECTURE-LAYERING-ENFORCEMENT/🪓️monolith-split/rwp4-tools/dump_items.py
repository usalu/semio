import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
want = sys.argv[1:] 
for d, p in shell_files():
    if want and not any(w in d for w in want): continue
    src = Src(open(p, encoding='utf-8').read())
    print('=====', d, len(src.lines))
    for it in top_items(src):
        if it['kind'] == 'use': continue
        s = it['first']
        while s > 0 and src.lines[s-1].lstrip().startswith(('///', '#[', '//!', '//')) and not REGION.match(src.lines[s-1]): s -= 1
        nm = item_name(it['head'], it['kind']) if it['kind'] != 'impl' else it['head'][:90]
        print(f"{s+1}-{it['last']+1} {it['last']-s+1} {it['kind']} {nm}")
