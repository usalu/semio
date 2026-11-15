"""Raise the tip of every front limb strip of the scots-pine to the bone tip height that the elevation draws (the front strip rose 1.1-1.3 m where the elevation rises 1.6-2.85 m)."""
import re, json, os, shutil
F='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/scots-pine/🔣️.json'
TIP={'stub-w':5.94,'stub-e':6.87,'limb-w3':13.30,'limb-e3':14.30,'limb-w1':16.90,'limb-e1':17.90,'limb-w2':19.00,'limb-e2':19.90}
t=open(F,encoding='utf8').read()
shutil.copy(F,'🗑️generated/author-67/scots-pine.catalog.before.json') if not os.path.exists('🗑️generated/author-67/scots-pine.catalog.before.json') else None
n=0
def fix(m):
    global n
    body=m.group(0)
    lid=m.group(1)
    nums=re.findall(r'\[\s*(-?[\d.]+),\s*(-?[\d.]+)\s*\]',body)
    ys=[float(b) for a,b in nums]
    tipc=(ys[2]+ys[3])/2
    d=TIP[lid]-tipc
    if abs(d)<0.05: return body
    out=body
    # replace third and fourth y
    pos=[mm for mm in re.finditer(r'\[\s*(-?[\d.]+),\s*(-?[\d.]+)\s*\]',body)]
    res=[];last=0
    for i,mm in enumerate(pos):
        res.append(body[last:mm.start(2)])
        y=float(mm.group(2))
        res.append(('%.4f'%(y+d)) if i>=2 else mm.group(2))
        last=mm.end(2)
    res.append(body[last:])
    n+=1
    return ''.join(res)
pat=re.compile(r'"id": "((?:limb|stub)-[we][123]?)",\s*"bone": "(?:limb|stub)-[we][123]?",\s*"role": "body",\s*"z": [\d.]+,\s*"filled": true,\s*"tone": [\d.]+,\s*"kind": "poly",\s*"points": \[(?:\s*\[\s*-?[\d.]+,\s*-?[\d.]+\s*\],?){4}\s*\]')
t2=pat.sub(fix,t)
cur=open(F,encoding='utf8').read()
assert cur==t
open(F+'.tmp','w',encoding='utf8').write(t2); os.replace(F+'.tmp',F)
json.load(open(F,encoding='utf8'))
print('fixed',n)
