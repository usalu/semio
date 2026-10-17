import sys,os,shutil,re
# usage: revert.py <apply-log> <backup-dir> : restores every `~` path from the backup and deletes every `+` path (and emptied dirs)
log,bk=sys.argv[1:3]
plus=[];tilde=[]
for l in open(log,encoding='utf8'):
    m=re.match(r'^  ([+~]) (.*)$',l.rstrip('\n'))
    if m: (plus if m.group(1)=='+' else tilde).append(m.group(2))
for p in tilde:
    shutil.copyfile(os.path.join(bk,p),p)
d=set()
for p in plus:
    if os.path.exists(p): os.remove(p); d.add(os.path.dirname(p))
for x in sorted(d,key=len,reverse=True):
    while x and os.path.isdir(x) and not os.listdir(x):
        os.rmdir(x); x=os.path.dirname(x)
print('restored',len(tilde),'deleted',len(plus))
