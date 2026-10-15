import os,re,sys,json
from plan import *

def read(p): return open(p,encoding='utf-8').read()
def write(p,s): open(p,'w',encoding='utf-8').write(s)

changed=[]
def rewrite_file(p,c):
    s=read(p); o=s
    kit='crate::testkit::' if c=='contract' else ident('contract')+'::testkit::'
    s=s.replace('crate::schema::tests::',kit)
    def sub(m):
        M=m.group(1)
        if M in OWNER and OWNER[M]!=c: return ident(OWNER[M])+'::'+M
        return m.group(0)
    s=re.sub(r'\bcrate::(\w+)\b',sub,s)
    if s!=o:
        write(p,s); changed.append(p)
    return s!=o

def main():
    for m,c in OWNER.items():
        for p in (src(m),tst(m)):
            if os.path.exists(p): rewrite_file(p,c)
    for p in changed: print(p.replace(PETS+'/',''))
    print(len(changed),'files')
main()
