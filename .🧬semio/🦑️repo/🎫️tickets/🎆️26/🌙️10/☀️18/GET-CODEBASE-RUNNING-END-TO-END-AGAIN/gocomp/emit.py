import json,os,re,shutil,sys
exec(open('table.py').read())
d=json.load(open('decls2.json'))
L=open('orig.go.txt').read().split('\n')
imp_end=57
import_block='\n'.join(L[13:imp_end])
assert L[13].startswith('import ('), L[13]
header_orig='\n'.join(L[0:10])
# per-decl text
def isreg(l): return l.startswith('// #region') or l.startswith('// #endregion')
texts={}
prev=imp_end
for x in d:
    gap=[(ln,L[ln-1]) for ln in range(prev+1,x['start'])]
    keep=[]
    for ln,l in gap:
        if isreg(l): continue
        if l.startswith('//') and ln>1 and L[ln-2].startswith('// #region'): continue
        keep.append(l)
    body=L[x['start']-1:x['end']]
    j=0
    while j<len(body) and body[j].startswith('//'):
        j+=1
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
    lines=pre+body
    out=lines
    while out and out[0].strip()=='': out=out[1:]
    while out and out[-1].strip()=='': out.pop()
    texts[x['idx']]='\n'.join(out)
    prev=x['end']
# anything after last decl except region lines?
tail=[l for l in L[prev:] if l.strip() and not isreg(l)]
assert not tail, tail
outdir='out'
shutil.rmtree(outdir,ignore_errors=True); os.makedirs(outdir)
def parse(spec):
    r=[]
    for part in spec.split(','):
        a,_,b=part.partition('-'); b=b or a
        r+=list(range(int(a),int(b)+1))
    return sorted(r)
manifest=[]
for e,s,desc,spec in T:
    fn=e+FE+s+'.go'
    idxs=parse(spec)
    if s=='':
        hdr=header_orig
    else:
        hdr=f"// #region 🧲️Header\n\n// 2025-2026 Ueli Saluz <ueli@semio-tech.com>\n\n// {desc}\n\n// #endregion 🧲️Header"
    body='\n\n'.join(texts[i] for i in idxs)
    content=f"{hdr}\n\npackage client\n\n{import_block}\n\n{body}\n"
    open(os.path.join(outdir,fn),'w').write(content)
    manifest.append(fn)
json.dump(manifest,open('manifest.json','w'),ensure_ascii=False)
print(len(manifest),'files')
