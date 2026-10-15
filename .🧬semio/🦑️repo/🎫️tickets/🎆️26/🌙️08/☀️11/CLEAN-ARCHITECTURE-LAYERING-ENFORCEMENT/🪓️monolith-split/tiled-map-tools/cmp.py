import re,sys,collections
def errs(path):
    txt=open(path,encoding='utf-8').read()
    out=[]
    blocks=re.split(r'\n(?=(?:error|warning)(?:\[|:))',txt)
    for b in blocks:
        if not b.startswith('error'): continue
        head=b.split('\n',1)[0]
        m=re.search(r'\n\s*-->\s*(.*?):(\d+):(\d+)',b)
        f=m.group(1) if m else '?'
        lab=re.search(r'\n\s*\d+ \|.*\n\s*\|\s*([\^\-]+)\s*(.*)',b)
        out.append((head,lab.group(2) if lab else '',f))
    return out
a=errs(sys.argv[1]); b=errs(sys.argv[2])
ca=collections.Counter((h,l) for h,l,f in a); cb=collections.Counter((h,l) for h,l,f in b)
print('before',len(a),'after',len(b))
for k in sorted(set(ca)|set(cb)):
    if ca[k]!=cb[k]:
        print('%+d'%(cb[k]-ca[k]),k[0][:150],'|',k[1][:80])
