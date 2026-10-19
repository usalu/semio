"""cstep: append `?` to `let VAR = RECV.METHOD(args);` call statements. usage: addq.py [--write] 'recvRegex' 'method' listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
write='--write' in sys.argv
a=[x for x in sys.argv[1:] if not x.startswith('--')]
recv,meth,lst=a
pat=re.compile(r'\blet\s+(?:mut\s+)?\w+\s*=\s*(?:'+recv+r')\.'+meth+r'\s*\(')
tot=0
for p in open(lst).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    m=mask(s);out=s;n=0
    for r in reversed(list(pat.finditer(m))):
        po=r.end()-1;pc=match_brace(m,po)
        if pc<0: continue
        j=pc+1
        if m[j:j+1]=='?': continue
        if m[j:j+1]!=';': print('SKIP',p,s.count('\n',0,r.start())+1,s[r.start():pc+2].replace('\n',' ')[:100]);continue
        out=out[:pc+1]+'?'+out[pc+1:];n+=1
    if n:
        print(p.split('/')[1],p[-60:],n);tot+=n
        if write: open(p,'w',encoding='utf-8').write(out)
print('total',tot)
