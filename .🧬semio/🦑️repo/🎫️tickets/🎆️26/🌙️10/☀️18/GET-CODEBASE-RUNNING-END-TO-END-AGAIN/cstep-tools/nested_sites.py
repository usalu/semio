"""cstep: inside close_original_granted_step closures list close_step calls (receiver, arg, following line)."""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if 'close_original_granted_step(' not in s: continue
    m=mask(s)
    for c in re.finditer(r'close_original_granted_step\(cx,\|(\w+)\|\{',m):
        b=c.end()-1;e=match_brace(m,b)
        blk=m[b:e]
        for r in re.finditer(r'(\S*?)\.close_step\s*\(|InteractiveJob::close_step\s*\(',blk):
            po=b+r.end()-1;pc=match_brace(m,po)
            ln=s.count('\n',0,po)+1
            arg=s[po+1:pc]
            if arg.strip()==c.group(1): continue
            print(f"{p}:{ln}: {s.splitlines()[ln-1].strip()[:200]}")
