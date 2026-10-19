"""cstep: list fn items by InteractiveJobCloseStep return shape. usage: sigs.py listfile"""
import re,sys
bare=re.compile(r'\bfn\s+(\w+)\s*(?:<[^{;]*?>)?\s*\(([^{;]*?)\)\s*->\s*((?:\w+::)*InteractiveJobCloseStep)\s*(?:where[^{]*)?\{',re.S)
res=re.compile(r'\bfn\s+(\w+)\s*(?:<[^{;]*?>)?\s*\(([^{;]*?)\)\s*->\s*Result\s*<\s*((?:\w+::)*InteractiveJobCloseStep)\s*,\s*([\w:]+(?:<[^>]*>)?)\s*>\s*(?:where[^{]*)?\{',re.S)
import collections
cnt=collections.Counter()
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    for m in bare.finditer(s):
        ln=s.count('\n',0,m.start())+1
        params=re.sub(r'\s+',' ',m.group(2))
        print(f"BARE\t{p}:{ln}\t{m.group(1)}\t{params[:120]}")
        cnt['bare']+=1
    for m in res.finditer(s):
        ln=s.count('\n',0,m.start())+1
        params=re.sub(r'\s+',' ',m.group(2))
        print(f"RES\t{p}:{ln}\t{m.group(1)}\t{params[:100]}\t{m.group(4)}")
        cnt['res']+=1
print(cnt,file=sys.stderr)
