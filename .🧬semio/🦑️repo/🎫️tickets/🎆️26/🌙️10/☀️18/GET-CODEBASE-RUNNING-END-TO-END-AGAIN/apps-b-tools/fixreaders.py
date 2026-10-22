import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rsparse import match_brace
f=sys.argv[1]; s=open(f,encoding='utf-8').read(); out=[];pos=0;n=0
for m in re.finditer(r'^(?:pub(?:\([^)]*\))? )?fn \w+(?:<[^>]*>)?\(',s,re.M):
    if m.start()<pos: continue
    op=m.end()-1; cp=match_brace(s,op)
    params=s[op:cp]
    ob=s.find('{',cp)
    eb=match_brace(s,ob)
    body=s[ob:eb+1]
    nb=body
    for name in ('data','handle_reader','strings'):
        if re.search(r'\b'+name+r':\s*&mut\s+\w+',params):
            nb=re.sub(r'&mut '+name+r'\b(?!\.)',name,nb)
            nb=re.sub(r'^(\s*)'+name+r' = ',r'\1*'+name+' = ',nb,flags=re.M)
    if nb!=body:
        out.append(s[pos:ob]);out.append(nb);pos=eb+1;n+=1
out.append(s[pos:])
open(f,'w',encoding='utf-8').write(''.join(out)); print('fns fixed',n)
