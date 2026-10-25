import re,os,sys,collections
src=sys.argv[1]
by=collections.defaultdict(set)
for e in open(src).read().split('\n'):
    m=re.match(r'^(.*?):(\d+):(\d+): error\[E0499\]',e)
    if m:by[os.path.normpath(m.group(1))].add(int(m.group(2)))
BIND=re.compile(r'^(\s*)let (?:mut )?(\w+)\s*=\s*StepContext::new\(')
def indent(s):return len(s)-len(s.lstrip())
for f,lines in sorted(by.items()):
    L=open(f).read().split('\n');plan=[]
    for l in sorted(lines):
        i=l-1
        if 'StepContext::new(' not in L[i] and i+1<len(L) and 'StepContext::new(' in L[i+1]:i+=1
        if 'StepContext::new(' not in L[i]:print('NOCALL',f[-60:],l,L[i][:60]);continue
        depth=0;prev=None;j=i-1
        while j>=0:
            line=L[j]
            if re.match(r'^\s*(pub(\([a-z]+\))? )?(async )?fn \w',line):break
            depth+=line.count('}')-line.count('{')
            m=BIND.match(line)
            if m and depth<=0:
                name=m.group(2);between='\n'.join(L[j+1:i])
                if not re.search(r'drop\(%s\)'%name,between):prev=(j,name,indent(line));break
            j-=1
        if not prev:print('NOPREV',f[-60:],l);continue
        pj,name,pind=prev;eind=indent(L[i]);pos=i
        if eind>pind:
            k=i
            while k>pj and not (indent(L[k])==pind and L[k].strip() and not L[k].strip().startswith(('}',')',']'))):k-=1
            header=L[k]
            if k>pj and not re.search(r'\b%s\b'%name,header) and not re.search(r'\b(for|while|loop)\b','\n'.join(L[pj+1:k])):pos=k
        plan.append((pos,name,indent(L[pos]),l))
    for pos,name,ind,l in sorted(plan,reverse=True):
        L.insert(pos,' '*ind+f'drop({name});')
    open(f,'w').write('\n'.join(L));print(f[-90:],[(p+1,n) for p,n,_,_ in plan])
