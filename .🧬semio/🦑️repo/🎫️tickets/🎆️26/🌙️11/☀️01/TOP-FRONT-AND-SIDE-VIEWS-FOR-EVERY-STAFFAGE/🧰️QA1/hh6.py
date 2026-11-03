import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
for sc in('50','100'):
    for s in a['representations'][sc]['geometry']['plan']:
        if s['id'].startswith('skirt-'):
            s['cy']=-0.035 if s['id'].endswith('near') else 0.035; s['ry']=0.035
write_json_atomic(f,d)
