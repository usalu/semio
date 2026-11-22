import re,os,collections,sys
base='/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d'
t=open(sys.argv[1],encoding='utf8').read().split('\n')
seen={}
for i,l in enumerate(t):
    m=re.match(r'^error(\[E\d+\])?: (.*)',l)
    if m and not m.group(2).startswith('could not'):
        mm=re.match(r'\s*--> (.*?):(\d+):\d+',t[i+1]) if i+1<len(t) else None
        if not mm: continue
        f=os.path.normpath(os.path.join(base,mm.group(1)))
        src=[x for x in t[i+2:i+9] if re.match(r'\s*\d+ \|',x)]
        seen.setdefault((f,mm.group(2)),(m.group(2)[:90],src[0].strip()[:210] if src else ''))
byf=collections.defaultdict(list)
for (f,ln),v in seen.items(): byf[f[len(base)+1:]].append((ln,v))
print(len(seen),'sites',len(byf),'files')
for f,v in sorted(byf.items()):
    print('##',len(v),f[-95:])
    for ln,(m,s) in v[:int(sys.argv[2])]: print('  ',ln,m,'|',s[:int(sys.argv[3])])
