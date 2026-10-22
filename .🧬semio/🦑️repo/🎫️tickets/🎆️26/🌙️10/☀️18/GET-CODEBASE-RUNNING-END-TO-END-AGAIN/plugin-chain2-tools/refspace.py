#!/usr/bin/env python3
"""usage: refspace.py [--apply] ROOT...  -- strengthens `SpaceMember + MemberFactory` generic bounds to `ReferenceSpaceMember + MemberFactory` and fixes imports. Idempotent."""
import os,re,sys
apply='--apply' in sys.argv
roots=[a for a in sys.argv[1:] if not a.startswith('--')]
bound=re.compile(r'(?<![\w])((?:store::|semio_framework_os_kernel::)?)SpaceMember(\s*\+\s*)((?:store::|semio_framework_os_kernel::)?MemberFactory)')
changed=0
for root in roots:
    for d,_,fs in os.walk(root):
        for f in fs:
            if not f.endswith('.rs'): continue
            p=os.path.join(d,f)
            s=open(p,encoding='utf8').read()
            if not bound.search(s): continue
            t=bound.sub(lambda m:m.group(1)+'ReferenceSpaceMember'+m.group(2)+m.group(3),s)
            lines=t.split('\n')
            code='\n'.join(l for l in lines if not l.lstrip().startswith('use ') and not l.lstrip().startswith('//'))
            still=re.search(r'(?<![\w])SpaceMember(?![\w])',code) is not None
            unq=re.search(r'(?<![\w:])ReferenceSpaceMember(?![\w])',code) is not None
            def fix_use(l):
                m=re.match(r'^(\s*(?:#\[[^\]]*\]\s*)?use (?:store|semio_framework_os_kernel)::)(.*);\s*$',l)
                if not m: return l
                rest=m.group(2)
                if re.search(r'(?<![\w])ReferenceSpaceMember(?![\w])',rest): return l
                if not re.search(r'(?<![\w])SpaceMember(?![\w])',rest): return l
                if rest=='SpaceMember':
                    names=['SpaceMember']; brace=False
                else:
                    names=[n.strip() for n in rest.strip('{}').split(',') if n.strip()]; brace=True
                if unq: names=[('ReferenceSpaceMember' if False else n) for n in names]+['ReferenceSpaceMember']
                if not still: names=[n for n in names if n!='SpaceMember']
                names=sorted(set(names),key=str.lower)
                if not names: return None
                return m.group(1)+('{'+', '.join(names)+'}' if len(names)>1 or brace else names[0])+';'
            lines=[l2 for l2 in (fix_use(l) for l in lines) if l2 is not None]
            t='\n'.join(lines)
            if t!=s:
                changed+=1
                print(p[-100:],'still' if still else 'dropped')
                if apply: open(p,'w',encoding='utf8').write(t)
print('files',changed,'APPLIED' if apply else 'DRY')
