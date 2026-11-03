import json,sys
sys.stdout.reconfigure(encoding='utf8')
S='C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
a=json.load(open(S+sys.argv[1]+'/🔣️.json',encoding='utf8'))['asset']
print('BONES');
for b in a['rig']['bones']: print({k:v for k,v in b.items() if k not in('title','mass')})
print('POSES')
for p in a['poses']:
    print(p['id'],json.dumps({k:v for k,v in p.items() if k not in('title','summary','description','id')},ensure_ascii=False)[:700])
print('CLIPS')
for c in a['clips']:
    print(c['id'],json.dumps({k:v for k,v in c.items() if k not in('title','summary','description','id')},ensure_ascii=False)[:1200])
