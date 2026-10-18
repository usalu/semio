import json,collections,re,sys
d=json.load(open(sys.argv[1]))
rows=d['refused']
print(len(rows), d['rustAbsent'])
c=collections.Counter()
art=collections.Counter()
for r in rows:
    kind=r.split(' ')[0]
    c[kind]+=1
    m=re.search(r'✏️s/🔌️plugins/([^/]+)/🗿️artifacts/([^/]+)/',r)
    art[(m.group(1)+'/'+m.group(2)) if m else 'other:'+r[:80]]+=1
print(c)
for k,v in art.most_common(100): print(v,k)
