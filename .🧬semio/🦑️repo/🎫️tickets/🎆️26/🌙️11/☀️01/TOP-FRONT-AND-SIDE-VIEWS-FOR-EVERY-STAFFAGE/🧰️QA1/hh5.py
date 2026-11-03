import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
H={'50':[[-0.022,0.02],[-0.013,0.027],[0.015,0.032],[0.041,0.022],[0.064,-0.004],[0.044,-0.018],[0.015,-0.025],[-0.014,-0.021],[-0.025,0.005]],
   '100':[[-0.03,0.022],[0,0.033],[0.041,0.028],[0.065,-0.009],[0.03,-0.024],[-0.02,-0.026],[-0.031,0.011]]}
for sc in('50','100'):
    for s in a['representations'][sc]['geometry']['front']:
        if s['id'].startswith('skirt-'):
            s['points']=[[x,(-0.075 if y==-0.105 else (-0.06 if y==-0.09 else y))] for x,y in s['points']]
        if s['id']=='head':
            for k in('cx','cy','rx','ry'): s.pop(k,None)
            s['kind']='poly'; s['points']=H[sc]; s['closed']=True; s['solid']={'kind':'extruded','z0':-0.035,'z1':0.035}
    t=a['representations'][sc]['decisions']['silhouette']
    t['en']+=' In front the head is an extruded profile of the side outline across the head width, so a head tucked down in rolling up keeps the height of the side view.'
    t['de']+=' In der Vorderansicht ist der Kopf ein ausgezogenes Profil der Seitenkontur über die Kopfbreite, sodass ein beim Einrollen eingezogener Kopf die Höhe der Seitenansicht behält.'
write_json_atomic(f,d)
