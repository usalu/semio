import re,os,sys
log,base,pat,old,new=sys.argv[1:6]
t=open(log,encoding='utf8').read().split('\n')
files=[]
for dp,dn,fn in os.walk(base):
    if '🤖️generated' in dp: continue
    for f in fn:
        if f.endswith('.rs'): files.append(os.path.join(dp,f))
sites=set()
for i,l in enumerate(t):
    if re.match(pat,l):
        mm=re.match(r'\s*--> (.*?):(\d+):\d+',t[i+1]); sites.add((mm.group(1),int(mm.group(2))))
done=0
byfile={}
for pth,ln in sites:
    cand=os.path.normpath(os.path.join(base,pth)) if not pth.startswith('/') else pth
    if not os.path.exists(cand): print('missing',cand[-90:]);continue
    byfile.setdefault(cand,set()).add(ln)
for f,lns in byfile.items():
    L=open(f,encoding='utf8').read().split('\n')
    for ln in lns:
        if old in L[ln-1]: L[ln-1]=L[ln-1].replace(old,new);done+=1
    open(f,'w',encoding='utf8').write('\n'.join(L))
print('lines',done,'files',len(byfile))
