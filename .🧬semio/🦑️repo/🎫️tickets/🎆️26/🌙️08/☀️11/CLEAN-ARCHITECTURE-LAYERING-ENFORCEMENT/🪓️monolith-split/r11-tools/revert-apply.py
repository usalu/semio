# ⏪️ Reverts one `new crates` apply of this slice: deletes what the apply created, restores what it modified or deleted from the `--backup` directory, removes its owner rows. usage: python3 revert-apply.py <apply-log> <backup-dir>
import json,os,shutil,sys,re
R='/Users/ueli/Documents/semio/'
log,backup=sys.argv[1],sys.argv[2]
created=[];restored=0;removed=0
for line in open(log,encoding='utf8'):
    m=re.match(r'^  ([+~-]) (.*)$',line.rstrip('\n'))
    if not m: continue
    kind,path=m.group(1),m.group(2)
    if kind=='+':
        p=R+path
        if os.path.exists(p): os.remove(p); removed+=1
        created.append(path)
    else:
        src=os.path.join(backup,path)
        if os.path.exists(src):
            os.makedirs(os.path.dirname(R+path),exist_ok=True); shutil.copyfile(src,R+path); restored+=1
        else:
            print('no backup for',path)
rows=json.load(open(backup+'-owner-rows.json',encoding='utf8'))
own=R+'📋️build-owners.json'
doc=json.load(open(own,encoding='utf8'))
before=len(doc['owners'])
doc['owners']=[o for o in doc['owners'] if o['name'] not in set(rows)]
open(own,'w',encoding='utf8').write(json.dumps(doc,indent=2,ensure_ascii=False)+'\n')
for path in created:
    d=os.path.dirname(R+path)
    while d.startswith(R+'✏️s/') and os.path.isdir(d) and not os.listdir(d):
        os.rmdir(d); d=os.path.dirname(d)
print('restored',restored,'removed',removed,'ownerRowsRemoved',before-len(doc['owners']))
