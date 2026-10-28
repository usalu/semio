import json,sys,os
root=sys.argv[1]
d=os.path.join(root,'🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🌐️browser-graph.json')
j=json.load(open(d))
print({k:(v if not isinstance(v,list) else len(v)) for k,v in j.items() if k!='reason'})
for m in j['requireModules']:
    print('OK ' if os.path.exists(os.path.join(root,m)) else 'MISSING', m)
print(j.get('maxModules'))
print(json.dumps(j['alias'],ensure_ascii=False)[:1500])
