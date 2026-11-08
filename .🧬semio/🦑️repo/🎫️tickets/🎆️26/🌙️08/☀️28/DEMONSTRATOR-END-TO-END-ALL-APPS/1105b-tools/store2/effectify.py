import re,sys
N={'step','close_step','step_inner','auxiliary_step','advance_retirement','admit_retirement','origin_step','origin_close_step'}
GR={'g','grant','child','child_grant'}
def match_paren(s,i):
    d=0
    for j in range(i,len(s)):
        c=s[j]
        if c in '([{':d+=1
        elif c in ')]}':
            d-=1
            if d==0:return j
    raise Exception('unbalanced')
def split_args(a):
    out=[];d=0;cur=''
    for c in a:
        if c in '([{<' and c!='<':d+=1
        if c in ')]}':d-=1
        if c==',' and d==0:out.append(cur);cur=''
        else:cur+=c
    out.append(cur);return out
def convert(path):
    s=open(path,encoding='utf8').read()
    # signatures
    pos=0;res='';n=0
    for m in re.finditer(r'\bfn (%s)\b(<[^(]*?>)?\('%'|'.join(sorted(N)),s):
        pass
    out=[];i=0
    pat=re.compile(r'\bfn (%s)\b(<[^(]*?>)?\('%'|'.join(sorted(N)))
    while True:
        m=pat.search(s,i)
        if not m:out.append(s[i:]);break
        po=m.end()-1;pc=match_paren(s,po)
        args=s[po+1:pc]
        mm=re.search(r'\b(g|grant):RetainedCloneGrant',args)
        b=s.index('{',pc)
        if not mm:
            out.append(s[i:b+1]);i=b+1;continue
        name=mm.group(1)
        newargs=args[:mm.start()]+"effect:&mut RetainedAllocationEffect<'_>"+args[mm.end():]
        out.append(s[i:po+1]+newargs+s[pc:b+1]+'let %s=effect.grant();'%name)
        i=b+1;n+=1
    s=''.join(out)
    # call sites
    out=[];i=0
    cp=re.compile(r'\.(%s)\('%'|'.join(sorted(N)))
    while True:
        m=cp.search(s,i)
        if not m:out.append(s[i:]);break
        po=m.end()-1;pc=match_paren(s,po)
        parts=split_args(s[po+1:pc]);changed=False
        for k,a in enumerate(parts):
            t=a.strip()
            if t in ('g','grant'):parts[k]=a.replace(t,'effect');changed=True
            elif t in ('child','child_grant'):parts[k]=a.replace(t,'&mut effect.nested(%s)'%t);changed=True
        out.append(s[i:po+1]+','.join(parts)+')');i=pc+1
    s=''.join(out)
    if not re.search(r'use [^;]*RetainedAllocationEffect',s):
        lines=s.split('\n');idx=next(k for k,l in enumerate(lines) if l.startswith('use '));lines.insert(idx,'use semio_framework_value::RetainedAllocationEffect;');s='\n'.join(lines)
    open(path,'w',encoding='utf8').write(s)
    print(path,n)
for p in sys.argv[1:]:convert(p)
