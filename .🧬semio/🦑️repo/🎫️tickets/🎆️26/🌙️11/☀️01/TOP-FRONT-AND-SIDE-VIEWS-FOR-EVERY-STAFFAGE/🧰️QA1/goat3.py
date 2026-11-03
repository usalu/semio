import json,glob,sys,math
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/goat/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
DL,DT=0.163,0.17
by={b['id']:b for b in a['rig']['bones']}
th=math.radians(56.5986)
# remove helper bones
a['rig']['bones']=[b for b in a['rig']['bones'] if b['id'] not in('axis-hind','axis-chest')]
by={b['id']:b for b in a['rig']['bones']}
lum,tho=by['lumbar'],by['thorax']
# lumbar axis moves down DL in world: into pelvis frame
lum['x']=round(lum['x']-DL*math.sin(th),4); lum['y']=round(lum['y']-DL*math.cos(th),4)
lum['mass']['comY']=round(lum['mass']['comY']+DL,4)
# thorax: origin world down DT, lumbar origin down DL
tho['y']=round(tho['y']-DT+DL,4)
tho['mass']['comY']=round(tho['mass']['comY']+DT,4)
for b in a['rig']['bones']:
    if b['parent']=='thorax': b['y']=round(b['y']+DT,4)
for an in a['anchors']:
    if an['bone']=='thorax': an['y']=round(an['y']+DT,4)
    if an['bone']=='lumbar': an['y']=round(an['y']+DL,4)
def shift(s,dy):
    k=s['kind']
    if k=='poly': s['points']=[[x,round(y+dy,5)] for x,y in s['points']]
    elif k=='ellipse': s['cy']=round(s['cy']+dy,5)
    elif k=='capsule': s['y0']=round(s['y0']+dy,5); s['y1']=round(s['y1']+dy,5)
    elif k=='rect': s['y']=round(s['y']+dy,5)
    else: raise Exception(k)
for sc,r in a['representations'].items():
    g=r['geometry']
    for s in g['elevation']:
        if s['bone']=='lumbar': 
            if s['kind']=='path': raise Exception('path')
            shift(s,DL)
        if s['bone']=='thorax': shift(s,DT)
    for v in('front','plan'):
        for s in g[v]:
            if 'solid' in s:
                if s['solid']['axisBone']=='axis-hind': s['solid']['axisBone']='lumbar'; s['bone']='lumbar'
                if s['solid']['axisBone']=='axis-chest': s['solid']['axisBone']='thorax'; s['bone']='thorax'
write_json_atomic(f,d)
print('ok')
