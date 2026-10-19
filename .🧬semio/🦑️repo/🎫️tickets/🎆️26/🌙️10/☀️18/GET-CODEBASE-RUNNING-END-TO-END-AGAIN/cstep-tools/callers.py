"""cstep scanner: close_step calls with a non-cx argument whose result is consumed as an InteractiveJobCloseStep.
usage: callers.py listfile  -> prints file:line: text"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
cx=re.compile(r'^\s*(?:&mut\s+)?\*?(?:cx|_cx|context|ctx|original_cx|step_cx|issuer_cx|turn_cx|scope\.cx\(\)|[\w.]*\.cx\(\)|\w*cx\w*)\s*$')
tot=0;files=0
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if 'InteractiveJob' not in s and 'CloseStep' not in s: continue
    names={'InteractiveJobCloseStep','Step','Close','ICS','CloseStep','S'}
    for a in re.finditer(r'InteractiveJobCloseStep\s+as\s+(\w+)',s): names.add(a.group(1))
    m=mask(s);lines=s.split('\n');hit=False
    for r in re.finditer(r'(?:\.|::)close_step\s*\(',m):
        po=r.end()-1;pc=match_brace(m,po)
        if pc<0: continue
        arg=s[po+1:pc]
        if cx.match(arg): continue
        ln=s.count('\n',0,r.start())+1
        before=m[max(0,s.rfind('\n',0,r.start())):r.start()]
        if re.search(r'fn\s+$',m[max(0,r.start()-4):r.start()]): continue
        ctx='\n'.join(lines[max(0,ln-2):ln+6])
        if re.search(r'\b(?:'+'|'.join(names)+r'|InteractiveJobCloseStep)::(?:Pending|Complete|Blocked|Refused)',ctx) or 'InteractiveJob::close_step' in lines[ln-1]:
            print(f"{p}:{ln}: {lines[ln-1].strip()[:170]}");tot+=1;hit=True
    files+=hit
print('files',files,'sites',tot,file=sys.stderr)
