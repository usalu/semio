import json,re,subprocess,sys,os
T=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,T)
from importlib import import_module
subprocess.run(['python3','areaaudit.py'],cwd=T,capture_output=True)
au=json.load(open(T+'/area-audit.json'))
rows={}
for a in au['areas']:
    rows[(a['slug'],a['region'])]=dict(slug=a['slug'],region=a['region'],old=a['old'],cur=a['new'],kind=a['kind'],model=a['model'],src='audit',cls=a['class'],cat=a['category'])
rev=[]
for line in open(T+'/area-review.md'):
    if line.startswith('|') and '---' not in line and 'asset' not in line.split('|')[1]:
        c=[x.strip() for x in line.strip().strip('|').split('|')]
        rev.append(c)
json.dump(dict(rows=[dict(k=list(k),**v) for k,v in rows.items()],review=rev),open(T+'/🗑️generated/areareview/worklist.json','w'),ensure_ascii=False,indent=1)
print(len(rows),len(rev))
