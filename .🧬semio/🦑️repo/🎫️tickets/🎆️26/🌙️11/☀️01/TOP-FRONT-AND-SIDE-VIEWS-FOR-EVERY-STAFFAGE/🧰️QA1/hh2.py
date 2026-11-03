import json,glob
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
for sc in('50','100'):
    for s in a['representations'][sc]['geometry']['front']:
        if s['id'].startswith('skirt-'): s['rx']=0.025; s['ry']=0.065; s['cy']=-0.04
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
