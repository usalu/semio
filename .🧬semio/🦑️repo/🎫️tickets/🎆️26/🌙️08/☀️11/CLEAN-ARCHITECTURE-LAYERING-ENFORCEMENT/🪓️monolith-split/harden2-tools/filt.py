import json,sys,collections
r=json.load(open(sys.argv[1]))
f=[x for x in r if x.get('owner_in_use') or x['kind'] in ('trait','struct')]
print("total",len(r),"owner_in_use",len(f))
g=collections.defaultdict(list)
for x in f: g[(x['kind'],x.get('owner'),x['name'],x['def_file'].split('/')[-2],x['def_line'])].append((x['use_file'].split('/')[-2],x['hits'][:2]))
for k,v in sorted(g.items(),key=lambda kv:str(kv[0])): print(k,v[:3])
