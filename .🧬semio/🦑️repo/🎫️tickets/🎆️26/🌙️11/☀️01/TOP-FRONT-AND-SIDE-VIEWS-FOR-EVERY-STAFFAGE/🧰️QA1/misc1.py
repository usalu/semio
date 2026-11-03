import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/miscanthus-grass/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
AX={'plume-1':0.551,'plume-2':0.199,'plume-3':-0.199,'plume-4':-0.551}
P={'50':[[0,0.0],[0.02,0.045],[0.12,0.062],[0.22,0.046],[0.29,0.018],[0.31,0]],
   '100':[[0,0],[0.03,0.05],[0.15,0.06],[0.27,0.03],[0.31,0]],
   '200':[[0,0],[0.12,0.06],[0.31,0]]}
PEN={'50':1.99,'100':1.85,'200':1.0}
for sc in('50','100','200'):
    r=a['representations'][sc]
    for v in('front','plan'):
        for s in r['geometry'][v]:
            if s['id'] in AX:
                for k in('cx','cy','rx','ry','rotationDeg'): s.pop(k,None)
                s['kind']='poly'; s['points']=[list(p) for p in P[sc]]; s['closed']=True
                s['solid']={'kind':'revolved','axisBone':s['bone'],'axisX':AX[s['id']],'axisZ':0,'mirror':True}
                s['penMm']=PEN[sc]
    t=r['decisions']['silhouette']
    t['en']+=' In front and plan each plume is a filled revolved cone about its own plume bone (heavy pen, like the blades), not a hollow loop.'
    t['de']+=' In Vorderansicht und Grundriss ist jede Rispe ein gefüllter Drehkegel um ihren eigenen Rispenknochen (kräftige Feder wie bei den Blättern), keine hohle Schlaufe.'
write_json_atomic(f,d)
print('ok')
