import sys,os,re,shutil,json
# usage: revert.py <apply-log> <backup-dir> <owner-rows.json>  -- restores the pre-apply working tree of one package from the generator backup
log,bk,rows=sys.argv[1:4]
root="/Users/ueli/Documents/semio"
created=[];restored=[]
for l in open(log,encoding='utf8'):
    m=re.match(r'^  ([+~-]) (.*)$',l.rstrip("\n"))
    if not m: continue
    (created if m.group(1)=="+" else restored).append(m.group(2))
for p in created:
    a=os.path.join(root,p)
    if os.path.exists(a): os.remove(a)
    d=os.path.dirname(a)
    while d.startswith(root+"/✏️s") and os.path.isdir(d) and not os.listdir(d):
        os.rmdir(d); d=os.path.dirname(d)
n=0
for p in restored:
    src=os.path.join(bk,p)
    assert os.path.exists(src),src
    dst=os.path.join(root,p); os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(src,dst); n+=1
names=set(json.load(open(rows,encoding='utf8')))
op=os.path.join(root,"📋️build-owners.json"); doc=json.load(open(op,encoding='utf8'))
b=len(doc["owners"]); doc["owners"]=[o for o in doc["owners"] if o["name"] not in names]
open(op,"w",encoding='utf8').write(json.dumps(doc,indent=2,ensure_ascii=False)+"\n")
print("created removed",len(created),"restored",n,"owner rows removed",b-len(doc["owners"]))
