import json,os,shutil,glob,subprocess
exec(open('table.py').read())
d=json.load(open('decls.json'))
g=json.load(open('assignment.json'))
L=open('orig.go.txt').read().split('\n')
imp_start=next(i for i,l in enumerate(L) if l.startswith('import ('))
imp_end=next(i for i in range(imp_start,len(L)) if L[i]==')')
import_block='\n'.join(L[imp_start:imp_end+1])
def isreg(l): return l.startswith('// #region') or l.startswith('// #endregion')
texts={}
prev=imp_end+1
for x in d:
    keep=[]
    for ln in range(prev+1,x['start']):
        l=L[ln-1]
        if isreg(l): continue
        if l.startswith('//') and ln>1 and L[ln-2].startswith('// #region'): continue
        keep.append(l)
    body=L[x['start']-1:x['end']]
    j=0
    while j<len(body) and body[j].startswith('//'): j+=1
    head=body[:j]; rest=body[j:]
    nh=[]
    for k,l in enumerate(head):
        if isreg(l): continue
        if k>0 and head[k-1].startswith('// #region'): continue
        nh.append(l)
    body=nh+rest
    pre=[]
    for l in keep:
        if l.strip()=='' and (not pre or pre[-1].strip()==''): continue
        pre.append(l)
    out=pre+body
    while out and out[0].strip()=='': out=out[1:]
    while out and out[-1].strip()=='': out.pop()
    texts[x['idx']]='\n'.join(out)
    prev=x['end']
tail=[l for l in L[prev:] if l.strip() and not isreg(l)]
assert not tail,tail
outdir='out'
shutil.rmtree(outdir,ignore_errors=True); os.makedirs(outdir)
GO="\U0001F439️.go"
files=[]
for e,s,desc,spec in T:
    idxs=sorted(int(k) for k,v in g.items() if v==s)
    assert idxs,s
    hdr=f"// #region 🧲️Header\n\n// 2025-2026 Ueli Saluz <ueli@semio-tech.com>\n\n// {desc}\n\n// #endregion 🧲️Header"
    if s=='helpers': hdr='\n'.join(L[0:7])
    body='\n\n'.join(texts[i] for i in idxs)
    content=f"{hdr}\n\npackage client\n\n{import_block}\n\n{body}\n"
    dn=os.path.join(outdir,e+FE+s)
    os.makedirs(dn)
    p=os.path.join(dn,GO)
    open(p,'w').write(content)
    files.append(p)
assert sum(len(v) for v in [[1]*len([k for k,vv in g.items() if vv==s]) for _,s,_,_ in T])==len(d)
subprocess.check_call(['./prune.bin']+files)
n=0;mx=0
for p in files:
    c=len(open(p).read().split('\n'))-1; n+=c; mx=max(mx,c)
    if c>600: print('OVER',p,c)
print(len(files),'files',n,'lines max',mx)
