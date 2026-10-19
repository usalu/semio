"""cstep: classify every fn with bare InteractiveJobCloseStep return. usage: classify.py listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace,fn_items,enclosing_impl
bare=re.compile(r'->\s*(?:\w+::)*(?:InteractiveJobCloseStep|Step|Close|ICS)\s*$')
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if 'InteractiveJobCloseStep' not in s: continue
    m=mask(s)
    for name,st,bo,be,sig in fn_items(s,m):
        sg=re.sub(r'\s+',' ',sig)
        if not re.search(r'->\s*(?:[\w:]*::)?(InteractiveJobCloseStep|Step|Close|ICS)\s*(where.*)?$',sg): continue
        if re.search(r'->.*Result\s*<',sg): continue
        body=s[bo:be]
        ref='Refused' in m[bo:be]
        grant=bool(re.search(r'[gG]rant',sg))
        im=enclosing_impl(m,st)
        hdr=re.sub(r'\s+',' ',im[0])[:80] if im and im[3]>=be else '-'
        trait='InteractiveJob for' in hdr
        ln=s.count('\n',0,st)+1
        print(f"{'T' if trait else 'I'}\t{'R' if ref else '-'}\t{'G' if grant else '?'}\t{p}:{ln}\t{name}\t{hdr}")
