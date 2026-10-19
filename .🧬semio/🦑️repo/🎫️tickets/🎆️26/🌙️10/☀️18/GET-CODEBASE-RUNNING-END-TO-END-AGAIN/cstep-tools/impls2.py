"""cstep: print raw fn signatures inside trait impls. usage: impls2.py traitregex fnregex listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
tr=re.compile(r'\bimpl\b[^{;]*?\b(?:'+sys.argv[1]+r')\s+for\b[^{;]*?\{',re.S)
fn=re.compile(r'\bfn\s+('+sys.argv[2]+r')\b[^{;]*?\{',re.S)
for p in open(sys.argv[3]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s)
    for im in tr.finditer(m):
        b=im.end()-1;e=match_brace(m,b);blk=m[b:e]
        for f in fn.finditer(blk):
            sg=re.sub(r'\s+',' ',s[b+f.start():b+f.end()-1])
            sg=re.sub(r'semio_framework_\w+::(\w+::)*','',sg)
            print(sg[:230])
