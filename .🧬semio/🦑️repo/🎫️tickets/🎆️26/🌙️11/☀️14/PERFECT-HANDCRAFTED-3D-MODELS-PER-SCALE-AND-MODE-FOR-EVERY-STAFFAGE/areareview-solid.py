import json,sys
sys.path.insert(0,'.')
from importlib import import_module
d=import_module('areareview-dump')
slug=sys.argv[1]; ids=sys.argv[2:]
a,p=d.load(slug)
for s in a['volumes']['50']['body']+a['volumes']['50']['structure']:
    if s['id'] in ids or any(s['id'].startswith(i+'*') for i in ids):
        print(json.dumps(s,ensure_ascii=False)[:1500])
