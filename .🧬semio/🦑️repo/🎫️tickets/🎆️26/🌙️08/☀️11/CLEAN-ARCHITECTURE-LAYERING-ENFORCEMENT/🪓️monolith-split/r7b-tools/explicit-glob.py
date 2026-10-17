# 🔤️ Replaces a glob import of the old artifact root by explicit owner-path imports of exactly the mapped names the file uses: usage `python3 explicit-glob.py <map.json> '<glob line>' <crate prefix> <file>...`.
import json,re,sys
MAP=json.load(open(sys.argv[1])); line=sys.argv[2]; prefix=sys.argv[3]; files=sys.argv[4:]
SKIP={'artifact','definition','core','host','standards','editor','viewer','examples','graph_query','content_read','graph_manifest','ast','executor','lexer','language_service'}
for f in files:
    t=open(f,encoding='utf8').read()
    assert t.count(line)==1,(f,t.count(line))
    body=t.replace(line,'')
    ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body))
    groups={}
    for n in sorted(ids&set(MAP)):
        if n in SKIP: continue
        v=MAP[n]
        full=(v[1:] if v.startswith('@') else prefix+'::'+v)
        mod,_,leaf=full.rpartition('::')
        groups.setdefault(mod,[]).append(leaf)
    stmts=[]
    for mod,leaves in groups.items():
        stmts.append(f'use {mod}::{leaves[0]};' if len(leaves)==1 else f'use {mod}::{{{", ".join(leaves)}}};')
    t=t.replace(line,'\n'.join(stmts))
    open(f,'w',encoding='utf8').write(t)
    print(f.split('/')[-3:],len(stmts))
