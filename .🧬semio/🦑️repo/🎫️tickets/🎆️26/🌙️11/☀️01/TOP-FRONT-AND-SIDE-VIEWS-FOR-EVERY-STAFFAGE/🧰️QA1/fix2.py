import json,glob,re,copy
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d):
    open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
ARM50=[[-0.03,-0.04],[0.0,-0.047],[0.1,-0.05],[0.2,-0.046],[0.228,-0.03],[0.2305,-0.01],[0.228,0.0],[0.2,0.04],[0.1,0.05],[0.0,0.04],[-0.03,0.0]]
HAND50=[[-0.03,-0.032],[0.08,-0.036],[0.17,-0.03],[0.235,-0.015],[0.248,0.0],[0.243,0.012],[0.17,0.026],[0.08,0.03],[0.0,0.036],[-0.03,0.028]]
ARM100=[[-0.04,-0.044],[0.1,-0.054],[0.22,-0.04],[0.237,-0.01],[0.2,0.044],[0.087,0.053],[-0.04,0.03]]
HAND100=[[-0.04,-0.036],[0.12,-0.038],[0.236,-0.016],[0.258,0.0],[0.248,0.012],[0.12,0.03],[-0.04,0.03]]
ARM200=[[-0.04,-0.05],[0.22,-0.04],[0.237,0.0],[0.2,0.044],[-0.04,0.03]]
HAND200=[[-0.04,-0.036],[0.236,-0.016],[0.258,0.0],[0.12,0.03],[-0.04,0.03]]
FOOT=[[-0.022,0.0],[-0.016,0.0105],[0.065,0.007],[0.088,0.0],[0.088,-0.007],[0.03,-0.0105],[-0.016,-0.0105]]
PROF={'50':(ARM50,HAND50),'100':(ARM100,HAND100),'200':(ARM200,HAND200)}
LAT={'arm':(0.063,0.097),'hand':(0.065,0.095)}
f,d=load('herring-gull'); reps=d['asset']['representations']
for sc,(arm,hand) in PROF.items():
    g=reps[sc]['geometry']
    for s in g['front']:
        m=re.fullmatch(r'(near|far)-(arm|hand)',s['id'])
        if m:
            side,part=m.groups(); lo,hi=LAT[part]
            z0,z1=(-hi,-lo) if side=='near' else (lo,hi)
            pts=arm if part=='arm' else hand
            s['kind']='poly'; s['points']=copy.deepcopy(pts); s['closed']=True
            s.pop('followTip',None); s['solid']={'kind':'extruded','z0':z0,'z1':z1}
        if sc=='50' and s['id'] in('near-foot','far-foot'):
            z0,z1=(-0.074,-0.006) if s['id']=='near-foot' else (0.006,0.074)
            s['kind']='poly'; s['points']=copy.deepcopy(FOOT); s['closed']=True
            s['solid']={'kind':'extruded','z0':z0,'z1':z1}
    for s in g['plan']:
        if s['id']=='tail' and s['kind']=='capsule': s['x0']=round(s['x0']+0.035,4); s['x1']=round(s['x1']+0.035,4)
        if s['id'] in('near-hand','far-hand'):
            sg=-1 if s['id']=='near-hand' else 1
            s['y1']=round(sg*0.075,4); s['r1']=0.02
dec=reps['50']['decisions']['silhouette']
dec['en']=dec['en'].split(' The rig swings the wing only')[0]+' The wings are solids in front: the arm and the hand are extruded profiles of the side view over their folded lateral band, so a spread, raised or swept wing keeps the height and the span of the side view; the plan capsules of arm and hand follow their bones and the wing spread, the tail fan follows its bone, and the feet are extruded profiles so a hanging foot keeps its length in front.'
dec['de']=dec['de'].split(' Das Rig schwenkt den')[0]+' Die Flügel sind in der Vorderansicht Volumenkörper: Arm und Hand sind ausgezogene Profile der Seitenansicht über ihrem gefalteten Seitenband, sodass ein gespreizter, gehobener oder zurückgeschwenkter Flügel Höhe und Spannweite der Seitenansicht behält; die Grundrisskapseln von Arm und Hand folgen ihren Knochen und der Flügelspreizung, der Schwanzfächer folgt seinem Knochen, und die Füße sind ausgezogene Profile, damit ein hängender Fuß in der Vorderansicht seine Länge behält.'
for sc in ('100','200'):
    s=reps[sc]['decisions']['silhouette']
    if sc=='200':
        s['en']=s['en'].replace('the wings stay at the folded lateral position.','the wings are extruded solids in front and capsules in plan that follow the wing spread of their bones.')
        s['de']=s['de'].replace('die Flügel bleiben in der gefalteten seitlichen Lage.','die Flügel sind in der Vorderansicht ausgezogene Volumenkörper und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.')
    else:
        s['en']+=' The wings are extruded solids in front and capsules in plan that follow the wing spread of their bones.'
        s['de']+=' Die Flügel sind in der Vorderansicht ausgezogene Volumenkörper und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.'
save(f,d)
