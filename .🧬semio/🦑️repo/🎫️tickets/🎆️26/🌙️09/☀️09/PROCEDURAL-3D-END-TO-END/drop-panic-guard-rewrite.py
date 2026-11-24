import os,re,sys
apply=sys.argv[1]=='1'
pat=re.compile(r'impl(?:<[^{]*?>)?\s+(?:[\w:]+::)?Drop\s+for\s')
def body_range(s,start):
    i=s.find('{',start);d=0;j=i
    try:
      while j<len(s):
        c=s[j]
        if c=='"':
            j+=1
            while s[j]!='"':
                if s[j]=='\\':j+=1
                j+=1
        elif c=='{':d+=1
        elif c=='}':
            d-=1
            if d==0:break
        j+=1
    except IndexError:return None
    return i,j
def first_arg_end(s,k):
    d=0;j=k
    while True:
        c=s[j]
        if c=='"':
            j+=1
            while s[j]!='"':
                if s[j]=='\\':j+=1
                j+=1
        elif c in '([{':d+=1
        elif c in ')]}':
            if d==0:return j
            d-=1
        elif c==',' and d==0:return j
        j+=1
residual=[];changed=0
for root in ('🧰️framework','✏️s'):
  for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in('.cache','.git','target','node_modules') and '🧪' not in d]
    for f in fn:
        if not f.endswith('.rs') or '🧪' in dp:continue
        p=os.path.join(dp,f)
        try:s=open(p,encoding='utf8').read()
        except:continue
        edits=[]
        for m in pat.finditer(s):
            r=body_range(s,m.end())
            if not r:continue
            i,j=r;b=s[i:j+1]
            if re.search(r'panicking|unsettled_drop_allowed|abandoning',b):continue
            if not re.search(r'\bassert(_eq|_ne)?!|panic!|unreachable!',b):continue
            ok=True
            for a in re.finditer(r'(?<![\w_])assert!\(',b):
                k=i+a.end();e=first_arg_end(s,k)
                edits.append((k,e))
            for a in re.finditer(r'\bassert_(eq|ne)!|panic!|unreachable!',b):
                residual.append((p,s.count('\n',0,i+a.start())+1,a.group(0)))
        if edits and apply:
            for k,e in sorted(set(edits),reverse=True):
                s=s[:k]+'std::thread::panicking()||('+s[k:e]+')'+s[e:]
            open(p,'w',encoding='utf8').write(s);changed+=1
        elif edits: changed+=1
print('files',changed)
for r in residual:print(*r)
