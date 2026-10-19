import json,sys,os,re,importlib.util
ROOT="/Users/ueli/Documents/semio/"
spec=importlib.util.spec_from_file_location("rt",os.path.dirname(os.path.abspath(__file__))+"/runtime.py"); rt=importlib.util.module_from_spec(spec); spec.loader.exec_module(rt)
closure=json.load(open(sys.argv[1])); mdir=sys.argv[2]
for x in closure:
    p=x['p']
    if not p.endswith('.rs'): continue
    s=open(p,encoding='utf8').read()
    if re.search(r'include_str!\("[^"]+"\)',s):
        rt.run(os.path.relpath(p,ROOT),mdir)
