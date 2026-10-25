import re,os,collections
errs=open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/store-b/census-store-errors.txt').read().split('\n')
by=collections.defaultdict(set)
for e in errs:
    m=re.match(r'^(.*?):(\d+):(\d+): error\[E0499\]: cannot borrow `sequence`',e)
    if m:by[os.path.normpath(m.group(1))].add(int(m.group(2)))
BIND=re.compile(r'^(\s*)let (?:mut )?(\w+)\s*=\s*StepContext::new\(')
def indent(s):return len(s)-len(s.lstrip())
for f,lines in sorted(by.items()):
    L=open(f).read().split('\n');plan=[]
    for l in sorted(lines):
        i=l-1
        if 'StepContext::new(' not in L[i] and 'StepContext::new(' in L[i+1]:i+=1
        assert 'StepContext::new(' in L[i],(f,l,L[i][:80])
        # previous binding
        j=i-1;prev=None
        while j>=0:
            if re.match(r'^\s*(pub(\([a-z]+\))? )?(async )?fn \w',L[j]):break
            m=BIND.match(L[j])
            if m:
                name=m.group(2);between='\n'.join(L[j+1:i])
                if not re.search(r'drop\(%s\)'%name,between):prev=(j,name,indent(L[j]));break
            j-=1
        if not prev:print('NOPREV',f[-70:],l);continue
        pj,name,pind=prev;eind=indent(L[i])
        pos=i
        if eind>pind:
            k=i
            while k>pj and not (indent(L[k])==pind and L[k].strip() and not L[k].strip().startswith(('}',')',']'))):k-=1
            header=L[k]
            if k>pj and not re.search(r'\b%s\b'%name,header) and not re.search(r'\b(for|while|loop)\b',''.join(x for x in L[pj+1:k])) :pos=k
        plan.append((pos,name,indent(L[pos]),l))
    for pos,name,ind,l in sorted(plan,reverse=True):
        L.insert(pos,' '*ind+f'drop({name});')
    open(f,'w').write('\n'.join(L));print(f[-90:],[(p+1,n) for p,n,_,_ in plan])
