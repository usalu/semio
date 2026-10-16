import sys, re, os, json, shutil
sys.path.insert(0,'.')
from ts7 import match, GEN, ROOT
BANDS=['🚪️ingress/🦀️.rs','📨️dispatch/🦀️.rs','🔄️full-operation/🦀️.rs','🤝️group/🦀️.rs','🧵️instance/🛠️tool-machine/🦀️.rs','🧵️instance/🧾️typed-operations/🦀️.rs','🧵️instance/🖼️render/🦀️.rs','🧵️instance/🧷️command-routes/🦀️.rs','🧵️instance/🔌️plugin-app-impl/🦀️.rs','🪜️close-ladder/🦀️.rs','🪜️envelope-ladder/🦀️.rs']
def arrays(t):
    i=0; out=[]
    while True:
        j=t.find('[',i)
        if j<0: break
        e=match(t,j,'[',']')
        if e<0: i=j+1; continue
        k=e+1
        m=re.match(r'\s*\.map\(',t[k:])
        if m:
            pe=match(t,k+m.end()-1,'(',')')
            k2=pe+1
            m2=re.match(r'\s*\.join\(',t[k2:])
            if m2:
                pj=match(t,k2+m2.end()-1,'(',')')
                out.append((j,pj+1,t[j:e+1],t[k:pe+1]))
        i=j+1
    return out
def fix_arrays(t):
    n=0; res=[]; pos=0
    for (a,b,arr,mp) in arrays(t):
        if a<pos: continue
        if 'readFileSync' not in mp: continue
        els=re.findall(r'"([^"]*)"',arr)
        if not any(any(el.endswith(bd) for bd in BANDS) for el in els): continue
        keep=[el for el in els if not any(el.endswith(bd) for bd in BANDS) and el!='🦀️.rs']
        if keep:
            arr2='['+','.join('"%s"'%el for el in keep)+']'
            repl='[%s.map%s.join("\\n"),familyText()].join("\\n")'%(arr2,mp)
        else: repl='familyText()'
        res.append(t[pos:a]); res.append(repl); pos=b; n+=1
    res.append(t[pos:])
    return ''.join(res),n
def fix_single(t, rules):
    n=0
    for old,new in rules:
        if old in t: t=t.replace(old,new); n+=t.count(new) and 1
    return t,n
SINGLE={
 '🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts':[('readFileSync(resolve(base,"🪜️close-ladder/🦀️.rs"),"utf8")','familyText()')],
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts':[("readFileSync(join(import.meta.dir,'../🦀️.rs'),'utf8')",'familyText()')],
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️lease/🧪️tests/🟦️.ts':[('readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8")','familyText()')],
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🧪️tests/🟦️.ts':[('readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8")','familyText()')],
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests/🟦️.ts':[('readFileSync(join(plugin,ladder,"🦀️.rs"),"utf8")','familyText()')],
}
def main(write):
    files=[l.strip() for l in open(GEN+'/pwp7-ts-readers.txt',encoding='utf-8') if l.strip()]
    SNAPR=GEN+'/pwp7-snap-repo'; tot=0
    for f in files:
        p=ROOT+f
        t=open(p,encoding='utf-8').read()
        t2,n1=fix_arrays(t)
        n2=0
        if f in SINGLE:
            for old,new in SINGLE[f]:
                if old in t2:
                    t2=t2.replace(old,new); n2+=1
        if t2!=t:
            if 'familyText' in t2 and 'instance-family' not in t2:
                depth=len([x for x in os.path.dirname(f.split('🔌️plugin/')[-1]).split('/') if x]) if '🔌️plugin/' in f else None
                if depth is None:
                    # job module test: relative path to the plugin test-support companion
                    rel=os.path.relpath(ROOT+'🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts',os.path.dirname(p))
                else: rel=os.path.relpath(ROOT+'🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts',os.path.dirname(p))
                imp='import { familyText } from "%s";\n'%rel
                last=[m for m in re.finditer(r'(?m)^import[^\n]*\n',t2)]
                pos=last[-1].end() if last else 0
                t2=t2[:pos]+imp+t2[pos:]
            print(n1,n2,f.replace('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/',''))
            tot+=n1+n2
            if write:
                sp=SNAPR+'/'+f
                if not os.path.exists(sp): os.makedirs(os.path.dirname(sp),exist_ok=True); shutil.copy2(p,sp)
                open(p,'w',encoding='utf-8').write(t2)
    print('total',tot)
if __name__=='__main__': main('--write' in sys.argv)
