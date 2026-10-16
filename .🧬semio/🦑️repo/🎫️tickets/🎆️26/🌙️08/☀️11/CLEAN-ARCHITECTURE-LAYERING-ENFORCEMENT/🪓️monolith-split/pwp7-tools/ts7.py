import sys, re, os, json, shutil
sys.path.insert(0,'.')
from model7 import GEN, ROOT
GONE=['🪜️close-ladder/🦀️.rs','🧵️instance/🔄️full-operation/🦀️.rs','♻️lifecycle/🏪️stores/🦀️.rs','🧵️instance/🚪️ingress/🦀️.rs','🎮️command/🎟️lease/🦀️.rs','🧵️instance/🤝️group/🦀️.rs','🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs','🧵️instance/📨️dispatch/🦀️.rs','🪜️envelope-ladder/🦀️.rs']
def match(s,i,o,c):
    d=0;q=None
    while i<len(s):
        ch=s[i]
        if q:
            if ch=='\\': i+=2; continue
            if ch==q: q=None
        elif ch in '"\'`': q=ch
        elif ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return i
        i+=1
    return -1
def fix(t):
    out=[];i=0;n=0
    while True:
        j=t.find('readFileSync(',i)
        if j<0: out.append(t[i:]); break
        e=match(t,j+len('readFileSync'),'(',')')
        call=t[j:e+1]
        if e>0 and any(g in call for g in GONE):
            out.append(t[i:j]+'familyText()'); i=e+1; n+=1
        else:
            out.append(t[i:j+len('readFileSync(')]); i=j+len('readFileSync(')
    return ''.join(out),n
if __name__=='__main__':
    files=[l.strip() for l in open(GEN+'/pwp7-ts-readers.txt',encoding='utf-8') if l.strip()]
    SNAPR=GEN+'/pwp7-snap-repo'
    tot=0
    for f in files:
        p=ROOT+f
        t=open(p,encoding='utf-8').read()
        t2,n=fix(t)
        if n:
            if 'instance-family' not in t2:
                depth=len([x for x in os.path.dirname(f.split('🔌️plugin/')[-1]).split('/') if x])
                imp='import { familyText } from "%s🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";\n'%('../'*depth)
                last=[m for m in re.finditer(r'(?m)^import[^\n]*\n',t2)]
                pos=last[-1].end() if last else 0
                t2=t2[:pos]+imp+t2[pos:]
            if '--write' in sys.argv:
                sp=SNAPR+'/'+f
                if not os.path.exists(sp): os.makedirs(os.path.dirname(sp),exist_ok=True); shutil.copy2(p,sp)
                open(p,'w',encoding='utf-8').write(t2)
            print(n,f.split('🔌️plugin/')[-1])
            tot+=n
    print('total',tot)
