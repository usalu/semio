#!/usr/bin/env python3
"""usage: nested.py [--apply] <abs value root> ; rewrites X.reborrow(G, X.maximum_work_units()[.saturating_sub(..)]).expect(..)|? to X.nested(G) inside the scoped dirs only; every other reborrow is listed as MANUAL."""
import os,re,sys
apply='--apply' in sys.argv;args=[a for a in sys.argv[1:] if not a.startswith('--')];root=args[0];extra=args[1:]
scope=extra or ['🧬️retained-clone','🗂️ordered','🛬️decode/🏭️construction/🧳️slots','🛬️decode/🧳️owned/📥️default','♻️retirement/📋️queue','🔗️read','🌳️tree/🟰️equal','📦️paged','📝️text/📦️paged','📋️list/🔺️edit']
second=re.compile(r'^(\w+)\.maximum_work_units\(\)(?:\.saturating_sub\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\))?$')
param=re.compile(r'^maximum_work_units$')
def split(args):
    parts=[];d=0;cur='';
    for c in args:
        if c in '([{':d+=1
        if c in ')]}':d-=1
        if c==',' and d==0:parts.append(cur);cur=''
        else:cur+=c
    parts.append(cur);return parts
def close(s,i):
    d=0
    for j in range(i,len(s)):
        if s[j]=='(':d+=1
        elif s[j]==')':
            d-=1
            if d==0:return j
    return -1
changed=0;manual=[];sites=0
for scope_dir in scope:
    for dp,_,fs in os.walk(os.path.join(root,scope_dir)):
        if '🧪' in dp:continue
        for f in fs:
            if not f.endswith('.rs'):continue
            p=os.path.join(dp,f);s=open(p).read()
            if '.reborrow(' not in s:continue
            out='';i=0;n=0
            while True:
                k=s.find('.reborrow(',i)
                if k<0:out+=s[i:];break
                o=k+len('.reborrow');e=close(s,o)
                recv=re.search(r'(\w+)$',s[:k]);args=split(s[o+1:e])
                line=s.count('\n',0,k)+1
                ok=False
                if recv and len(args)==2:
                    m=second.match(args[1].strip());pm=param.match(args[1].strip())
                    if pm and not args[0].count('-1') and not re.search(r'\w-\w',args[0]):
                        rest=s[e+1:]
                        if rest.startswith('?'):ok=True;tail=1;fn='.nested_within('+args[0].strip()+',maximum_work_units)'
                    elif m and m.group(1)==recv.group(1):
                        rest=s[e+1:];mm=re.match(r'\.expect\("[^"]*"\)',rest)
                        if mm:ok=True;tail=len(mm.group(0))
                        elif rest.startswith('?'):ok=True;tail=1
                        fn='.nested('+args[0].strip()+')'
                if ok:
                    out+=s[i:k]+fn;i=e+1+tail;n+=1
                else:
                    manual.append((p[len(root)+1:],line,s[k:k+140].replace('\n',' ')));out+=s[i:e+1];i=e+1
            sites+=n
            if n and apply:open(p,'w').write(out)
            if n:changed+=1
print('files',changed,'sites',sites,'manual',len(manual))
for m in manual:print('MANUAL',*m)
