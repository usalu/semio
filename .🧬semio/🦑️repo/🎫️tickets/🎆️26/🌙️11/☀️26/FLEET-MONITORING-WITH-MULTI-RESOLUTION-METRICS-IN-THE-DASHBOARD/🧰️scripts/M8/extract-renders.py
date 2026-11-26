import re,sys
def sections(path):
    text=open(path,encoding='utf8').read()
    parts=re.split(r'(?m)^## ',text)[1:]
    out={}
    for part in parts:
        head,_,rest=part.partition('\n')
        out[head]=rest
    return out
def block(rest,limit=None):
    body=rest.split('```')[1].strip('\n').split('\n')
    if limit: body=body[:limit]+(['  …'] if len(body)>limit else [])
    return '```\n'+'\n'.join(line.rstrip() for line in body)+'\n```\n'
en=sections('live-render-en.md');de=sections('live-render-de.md')
out=[]
for head,rest in en.items():
    if head.startswith('Overview'): out.append('### '+head+'\n\n'+block(rest)+'\n')
for key,limit in [('Drill-down g1-m01, range key 1',34),('Drill-down g1-m05, range key 2',24),('Drill-down g1-m01, range key 3',24)]:
    head=next(h for h in en if h.startswith(key))
    out.append('### '+head+' (first lines)\n\n'+block(en[head],limit)+'\n')
for head,rest in de.items():
    if head.startswith('Drill-down'): out.append('### '+head+' (first lines)\n\n'+block(rest,16)+'\n')
open('renders.md','w',encoding='utf8').write(''.join(out))
