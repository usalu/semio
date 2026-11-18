#!/usr/bin/env python3
"""🧬 Lists every `generate` then `graph-generate` target of the workspace as `name<TAB>cwd<TAB>command` rows for native-play.sh."""
import json,os,sys
root=sys.argv[1];rows=[]
for base,dirs,files in os.walk(root):
    dirs[:]=[d for d in dirs if d not in('node_modules','.cache','.git','.🧬semio','dist','target')]
    if '📋️project.json' not in files: continue
    try: d=json.load(open(os.path.join(base,'📋️project.json')))
    except Exception: continue
    for t in('generate','graph-generate'):
        v=(d.get('targets') or {}).get(t)
        if not v: continue
        o=v.get('options') or {};c=o.get('command','')
        if not c or 'catalog/📣️publication' in c: continue
        cwd=o.get('cwd') or os.path.relpath(base,root)
        cwd=cwd.replace('{projectRoot}',os.path.relpath(base,root))
        rows.append((t,f"{d.get('name','').replace('/','_').replace('@','')}-{t}",cwd,c))
for t,n,cwd,c in sorted(rows,key=lambda r:r[0]!='generate'): print(f"{n}\t{cwd}\t{c}")
