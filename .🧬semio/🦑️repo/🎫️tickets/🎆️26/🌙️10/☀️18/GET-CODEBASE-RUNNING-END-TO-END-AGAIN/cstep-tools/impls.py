"""cstep: list `impl .. InteractiveJob for` blocks and their close_step/step signatures. usage: impls.py listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
imp=re.compile(r'\bimpl\b[^{;]*?\b(?:InteractiveJob|FixedOperationOwner|ArtifactCommandWork(?:<[^{]*?>)?)\s+for\b[^{;]*?\{',re.S)
fn=re.compile(r'\bfn\s+(step|close_step|borrow_outcome|begin_close)\b[^{;]*?\{',re.S)
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s)
    for im in imp.finditer(m):
        b=im.end()-1;e=match_brace(m,b)
        blk=m[b:e]
        sigs={}
        for f in fn.finditer(blk):
            sigs[f.group(1)]=re.sub(r'\s+',' ',s[b+f.start():b+f.end()-1])[:200]
        ln=s.count('\n',0,im.start())+1
        cs=sigs.get('close_step','-')
        kind='cx' if re.search(r'StepContext',cs) else ('grant' if 'Grant' in cs else '?')
        st=sigs.get('step','-');sk='orig' if 'OriginalTurnFailure' in st else ('valerr' if 'ValueError' in st else '?')
        print(f"{kind}\t{sk}\t{p}:{ln}\t{re.sub(chr(10),' ',s[im.start():im.end()-1])[:90]}")
