"""usage: apply-launchers.py <birth.tsv> -> writes fileLaunchers into the caching route table for every file that passed under a genuine birth"""
import json,sys
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
p=M+"🗂️suites/🔣️.json"; d=json.load(open(p,encoding='utf8'))
rows={}
for l in open(sys.argv[1],encoding='utf8'):
    f,b=l.rstrip('\n').split('\t')
    if b!='none': rows[f]=b
n=0
for s in d['suites']:
    for f in s.get('files',[]):
        if f in rows:
            s.setdefault('fileLaunchers',{})[f]=rows[f]; n+=1
open(p,'w',encoding='utf8').write(json.dumps(d,ensure_ascii=False,indent=2)+'\n'); print('launchers',n,'of',len(rows))
