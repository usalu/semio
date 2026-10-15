import re,os,glob,sys
from collections import Counter
SP='/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pets'
OLD=SP+'/base/🧰️framework/🛍️products/🐾️pets'
NEW='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets'
def norm(text):
    text=re.sub(r'semio_framework_pets_\w+?::','crate::',text)
    text=text.replace('crate::testkit::','crate::schema::tests::')
    out=[]; buf=None
    for l in text.split('\n'):
        t=l.strip()
        if not t: continue
        if buf is not None:
            buf+=' '+t
            if t.endswith(';'): out.append(buf); buf=None
            continue
        if re.match(r'(pub )?use .*[^;]$',t) and not t.endswith(';'):
            buf=t; continue
        out.append(t)
    res=[]
    for l in out:
        m=re.match(r'^((?:pub )?use [^{]*)\{(.*)\};$',l)
        if m:
            items=sorted(x.strip() for x in m.group(2).split(',') if x.strip())
            l=m.group(1)+'{'+', '.join(items)+'};'
        res.append(re.sub(r'\s+',' ',l.replace(', }',' }').replace('( ','(')))
    return res
files=[f for f in glob.glob(OLD+'/🔨️modules/*/🦀️.rs')+glob.glob(OLD+'/🔨️modules/*/🧪️tests/🔬️unit/🦀️.rs')+[OLD+'/🧬️schema/🦀️.rs',OLD+'/🧬️schema/🧪️tests/🔬️unit/🦀️.rs']]
tot=0
for f in sorted(files):
    rel=f[len(OLD)+1:]
    a=Counter(norm(open(f).read())); b=Counter(norm(open(NEW+'/'+rel).read()))
    rem=a-b; add=b-a
    if rem or add:
        tot+=1
        print('==',rel.split('/')[-4:-2] if 'tests' in rel else rel)
        for k,v in rem.items(): print('  -',v,k[:170])
        for k,v in add.items(): print('  +',v,k[:170])
print('files with differences:',tot,'of',len(files))
