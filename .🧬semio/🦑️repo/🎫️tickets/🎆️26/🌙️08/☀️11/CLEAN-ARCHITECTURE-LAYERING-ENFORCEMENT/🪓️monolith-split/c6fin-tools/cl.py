import sys,re,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import modsplit as M
t=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'orig',sys.argv[1]+'.rs'),encoding='utf-8').read()
d=M.Doc(t)
rows=[]
for it in d.items:
    rows.append('%d-%d %s %s %s%s'%(it['s']+1,it['e']+1,it['vis'].replace('pub','p') if it['vis'] else '',it['kind'],it['name'][:34],' [cfg]' if any(a.startswith('#[cfg') or a.startswith('#[test') for a in it['attrs']) else ''))
# regions with lines
for i,l in enumerate(d.lines):
    m=re.match(r'^(\s*)//\s?#(end)?region\b(.*)',l)
    if m and not l.startswith('        '):
        rows.append('%d R%s %s'%(i+1,'-' if m.group(2) else '+',m.group(3).strip()))
rows.sort(key=lambda r:int(re.match(r'\d+',r).group(0)))
print('\n'.join(rows))
