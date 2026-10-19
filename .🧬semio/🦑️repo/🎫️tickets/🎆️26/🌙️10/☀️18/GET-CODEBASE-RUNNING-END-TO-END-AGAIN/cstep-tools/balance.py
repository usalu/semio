"""cstep scanner: lexical bracket balance of Rust files (strings/chars/comments aware). usage: balance.py listfile"""
import sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask
bad=0;n=0
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s);n+=1
    st=[];ok=True;pairs={')':'(',']':'[','}':'{'}
    for i,c in enumerate(m):
        if c in '([{': st.append(c)
        elif c in ')]}':
            if not st or st[-1]!=pairs[c]: ok=False;break
            st.pop()
    if st: ok=False
    if not ok: bad+=1;print('UNBALANCED',p)
print('checked',n,'unbalanced',bad,file=sys.stderr)
