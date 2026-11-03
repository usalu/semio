import json,glob,sys,copy
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/goat/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
def bone(i,parent,y,length,en,de):
    return {"id":i,"parent":parent,"title":{"en":en,"de":de},"x":0,"y":y,"angleDeg":0,"lengthM":length,"joint":{"type":"fixed"}}
bones=[]
for b in a['rig']['bones']:
    bones.append(b)
    if b['id']=='lumbar': bones.append(bone('axis-hind','lumbar',-0.163,0.65,'Axis of the hindquarters barrel','Achse des Hinterleib-Fasses'))
    if b['id']=='thorax': bones.append(bone('axis-chest','thorax',-0.17,0.42,'Axis of the chest barrel','Achse des Brustkorb-Fasses'))
a['rig']['bones']=bones
P={'hindbody':('axis-hind',{
  '50':[[-0.2,0],[-0.18,0.07],[-0.12,0.105],[-0.02,0.12],[0.15,0.135],[0.4,0.16],[0.55,0.155],[0.63,0.12],[0.67,0.05],[0.67,0]],
  '100':[[-0.2,0],[-0.15,0.09],[0.1,0.13],[0.4,0.16],[0.6,0.14],[0.67,0.05],[0.67,0]],
  '200':[[-0.2,0],[-0.12,0.1],[0.4,0.16],[0.64,0.11],[0.67,0]],
  '500':[[-0.2,0],[0.0,0.12],[0.4,0.16],[0.67,0]]}),
 'chest':('axis-chest',{
  '50':[[0.05,0],[0.06,0.09],[0.1,0.14],[0.18,0.17],[0.3,0.17],[0.37,0.13],[0.4,0.07],[0.415,0]],
  '100':[[0.05,0],[0.09,0.13],[0.2,0.17],[0.34,0.15],[0.415,0]],
  '200':[[0.05,0],[0.12,0.15],[0.3,0.17],[0.415,0]],
  '500':[[0.05,0],[0.2,0.17],[0.415,0]]}),
 'neck':('neck',{
  '50':[[-0.079,0],[-0.07,0.075],[0.0,0.095],[0.1,0.09],[0.2,0.075],[0.3,0.065],[0.314,0]],
  '100':[[-0.079,0],[-0.03,0.09],[0.15,0.08],[0.3,0.065],[0.314,0]],
  '200':[[-0.079,0],[0.0,0.09],[0.3,0.065],[0.314,0]],
  '500':[[-0.079,0],[0.1,0.085],[0.314,0]]})}
for sc in('50','100','200','500'):
    g=a['representations'][sc]['geometry']
    for v in('front','plan'):
        for s in g[v]:
            if s['id'] in P:
                bone_,prof=P[s['id']]
                for k in('cx','cy','rx','ry','x0','y0','x1','y1','r0','r1','followTip'): s.pop(k,None)
                s['bone']=bone_ if s['id']!='neck' else 'neck'
                s['kind']='poly'; s['points']=copy.deepcopy(prof[sc]); s['closed']=True
                s['solid']={'kind':'revolved','axisBone':bone_,'axisX':0,'axisZ':0,'mirror':True}
    t=a['representations'][sc]['decisions']['silhouette']
    t['en']+=' Hindquarters, chest and neck are revolved solids about spine axes in front and plan (round cross-section at the true width), so a rearing goat keeps its trunk as one body with the legs and the neck instead of stacked blocks.'
    t['de']+=' Hinterleib, Brustkorb und Hals sind in Vorderansicht und Grundriss um Wirbelsäulenachsen gedrehte Körper (runder Querschnitt in wahrer Breite), sodass eine aufgerichtete Ziege ihren Rumpf als einen Körper mit Beinen und Hals behält statt gestapelter Blöcke.'
write_json_atomic(f,d)
