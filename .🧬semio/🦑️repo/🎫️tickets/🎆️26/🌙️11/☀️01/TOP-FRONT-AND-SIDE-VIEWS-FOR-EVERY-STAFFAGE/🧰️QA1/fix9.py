import json,glob
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/horse/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); reps=d['asset']['representations']
for sc in ('50','100','200','500'):
    for s in reps[sc]['geometry']['plan']:
        if s['id'] in('hindbody','chest'):
            print(sc,s['id'],s['x0'],s['x1'],s['r0'],s['r1'])
            s['x0']=round(s['x0']+s['r0'],4); s['x1']=round(s['x1']-s['r1'],4)
    t=reps[sc]['decisions']['silhouette']
    t['en']+=' The plan trunk capsules end exactly at the body ends of the side view (their round caps no longer overshoot), so a swinging tail or a lowered head keeps the plan and the side view in step.'
    t['de']+=' Die Rumpfkapseln im Grundriss enden genau an den Rumpfenden der Seitenansicht (ihre runden Kappen ragen nicht mehr darüber hinaus), sodass ein schwingender Schweif oder ein gesenkter Kopf Grundriss und Seitenansicht im Gleichschritt hält.'
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
