import os,re,sys
root=sys.argv[1]
BODY='4096'
def balanced(s,i):
    d=0;k=i
    while k<len(s):
        c=s[k]
        if c in '([{':d+=1
        elif c in ')]}':
            d-=1
            if d==0:return k
        elif c=='"':
            k+=1
            while s[k]!='"':
                if s[k]=='\\':k+=1
                k+=1
        k+=1
    raise ValueError
def split_args(a):
    out=[];d=0;cur='';k=0
    while k<len(a):
        c=a[k]
        if c in '([{':d+=1
        elif c in ')]}':d-=1
        elif c=='"':
            j=k+1
            while a[j]!='"':
                if a[j]=='\\':j+=1
                j+=1
            cur+=a[k:j+1];k=j+1;continue
        if c==',' and d==0:out.append(cur);cur=''
        else:cur+=c
        k+=1
    if cur.strip():out.append(cur)
    return out
GRANTISH=re.compile(r'^\s*(grant|permit|g|below|turn_grant|RetainedCloneGrant::default\(\)|Default::default\(\)|grant\([^)]*\)|[a-z_]*grant[a-z_]*)\s*$')
def rewrite_advance(s):
    out='';i=0;n=0
    pat=re.compile(r'\.advance\(')
    while True:
        m=pat.search(s,i)
        if not m:out+=s[i:];break
        o=m.end()-1;e=balanced(s,o)
        args=split_args(s[o+1:e])
        if args and GRANTISH.match(args[-1]) and not s[m.start()-9:m.start()]=='.' :
            g=args[-1].strip()
            recv=re.search(r'[A-Za-z_][A-Za-z0-9_.]*$',s[i:m.start()])
            if recv is None:out+=s[i:e+1];i=e+1;continue
            start=m.start()-len(recv.group(0))
            newargs=','.join(args[:-1])+(',' if len(args)>1 else '')+' '+BODY+', effect'
            out+=s[i:start]+'test_source_custody::run(%s, |effect| %s.advance(%s))'%(g,recv.group(0),newargs.strip())
            i=e+1;n+=1
        else:
            out+=s[i:e+1];i=e+1
    return out,n
def rewrite_close(s):
    out='';i=0;n=0
    pat=re.compile(r'\.close_step\(')
    while True:
        m=pat.search(s,i)
        if not m:out+=s[i:];break
        o=m.end()-1;e=balanced(s,o)
        args=split_args(s[o+1:e])
        if len(args)==1 and GRANTISH.match(args[0]):
            g=args[0].strip()
            recv=re.search(r'[A-Za-z_][A-Za-z0-9_.]*$',s[i:m.start()])
            if recv is None:out+=s[i:e+1];i=e+1;continue
            start=m.start()-len(recv.group(0))
            out+=s[i:start]+'test_source_custody::close_turn(%s, |effect| %s.close_step(effect))'%(g,recv.group(0))
            i=e+1;n+=1
        else:out+=s[i:e+1];i=e+1
    return out,n
def rewrite_step(s):
    out='';i=0;n=0
    pat=re.compile(r'\b(owner|retirement)\.step\(')
    while True:
        m=pat.search(s,i)
        if not m:out+=s[i:];break
        o=m.end()-1;e=balanced(s,o)
        args=split_args(s[o+1:e])
        if len(args)==1 and GRANTISH.match(args[0]):
            out+=s[i:m.start()]+'test_source_custody::close_turn(%s, |effect| %s.step(4096, effect))'%(args[0].strip(),m.group(1))
            n+=1
        else:out+=s[i:e+1]
        i=e+1
    return out,n
tot=0
for dp,dn,fn in os.walk(root):
    if '🤖️generated' in dp or '🗑️generated' in dp or '🖌️brush' in dp: continue
    if '🧪️tests' not in dp and 'tests' not in dp: continue
    for f in fn:
        if not f.endswith('.rs') or 'source-custody' in dp: continue
        p=os.path.join(dp,f);s=open(p,encoding='utf8').read()
        if '.advance(' not in s and '.close_step(' not in s and '.step(' not in s: continue
        o=s
        s,a=rewrite_advance(s)
        s,c=rewrite_close(s)
        s,c2=rewrite_step(s)
        if s!=o:
            open(p,'w',encoding='utf8').write(s);tot+=1;print(a,c,p[-80:])
print('files',tot)
