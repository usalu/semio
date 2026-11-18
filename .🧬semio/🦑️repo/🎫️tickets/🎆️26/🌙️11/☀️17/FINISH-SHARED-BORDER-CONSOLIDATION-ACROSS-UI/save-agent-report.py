import json,sys
p,out,title=sys.argv[1],sys.argv[2],sys.argv[3]
last=None
for l in open(p):
    try: m=json.loads(l).get('message',{})
    except Exception: continue
    if m.get('role')=='assistant' and isinstance(m.get('content'),list):
        t=''.join(x.get('text','') for x in m['content'] if x.get('type')=='text')
        if t.strip(): last=t
open(out,'w').write(f'# {title}\n\n{last}')
print(len(last))
