import json,glob
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
ARM50=[[-0.03,-0.04],[0.0,-0.047],[0.1,-0.05],[0.2,-0.046],[0.228,-0.03],[0.2305,-0.01],[0.228,0.0],[0.2,0.04],[0.1,0.05],[0.0,0.04],[-0.03,0.0]]
HAND50=[[-0.03,-0.032],[0.08,-0.036],[0.17,-0.03],[0.235,-0.015],[0.248,0.0],[0.243,0.012],[0.17,0.026],[0.08,0.03],[0.0,0.036],[-0.03,0.028]]
for s in d['asset']['representations']['50']['geometry']['front']:
    if s['id'] in('near-hand','far-hand','near-arm','far-arm'):
        s['kind']='poly'; s['points']=ARM50 if 'arm' in s['id'] else HAND50; s.pop('followTip',None)
        near=s['id'].startswith('near')
        s['solid']={'kind':'extruded','z0':-0.03,'z1':0.03} if near else {'kind':'extruded','z0':-0.03,'z1':0.03}
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
