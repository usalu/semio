import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/goat/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
for sc,r in d['asset']['representations'].items():
    for v in('front','plan'):
        for s in r['geometry'][v]:
            if s['id']=='neck' and 'solid' in s: s['solid']['axisX']=0.7397
write_json_atomic(f,d)
