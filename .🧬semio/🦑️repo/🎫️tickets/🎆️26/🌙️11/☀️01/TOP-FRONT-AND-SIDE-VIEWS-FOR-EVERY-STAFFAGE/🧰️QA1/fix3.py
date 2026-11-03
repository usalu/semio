import json,glob
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/horse/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
c=[c for c in a['clips'] if c['id']=='graze-and-swish'][0]
n=0
for k in c['keys']:
    for b in ('tail-1','tail-2','tail-3','tail-4'):
        if b in k['bones']:
            k['bones'][b]['abductionDeg']=round(0.9*k['bones'][b]['angleDeg'],1); n+=1
print('tail keys',n)
p=[p for p in a['poses'] if p['id']=='standing-rest'][0]
p['bones']['ear-n']['twistDeg']=-35; p['bones']['ear-f']['twistDeg']=35
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
