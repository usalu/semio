import re,os,sys
log,base=sys.argv[1:3]
t=open(log,encoding='utf8').read().split('\n')
sites=set()
for i,l in enumerate(t):
    if re.match(r'^error\[E0061\]: this method takes 3 arguments but 0 arguments were supplied',l):
        mm=re.match(r'\s*--> (.*?):(\d+):(\d+)',t[i+1]); sites.add((os.path.normpath(os.path.join(base,mm.group(1))),int(mm.group(2)),int(mm.group(3))))
by={}
for f,ln,col in sites: by.setdefault(f,set()).add((ln,col))
for f,pts in by.items():
    L=open(f,encoding='utf8').read().split('\n')
    for ln,col in sorted(pts,key=lambda x:-x[1]):
        line=L[ln-1];idx=col-1
        if line[idx:idx+6]!='take()':print('skip',f[-50:],ln,line[idx:idx+10]);continue
        pre=line[:idx-1];m=re.search(r'[A-Za-z_][A-Za-z0-9_.]*$',pre)
        L[ln-1]=pre[:len(pre)-len(m.group(0))]+'test_source_custody::take(&mut %s)'%m.group(0)+line[idx+6:]
    open(f,'w',encoding='utf8').write('\n'.join(L))
print(len(sites))
