import json,sys
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/%s/🔣️.json"
a=json.load(open(S%sys.argv[1]))['asset']
for p in a['poses']: print(p['id'],json.dumps(p['bones']),p.get('ground'))
for b in a['rig']['bones']:
    if b['joint']['type']!='fixed': print('joint',b['id'],json.dumps(b['joint']))
