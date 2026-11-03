import json,glob,re
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d): open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
f,d=load('herring-gull'); reps=d['asset']['representations']
ARM=[[-0.097,0.094],[-0.097,-0.014],[-0.063,-0.014],[-0.063,0.094]]
HAND=[[-0.095,0.082],[-0.09,-0.002],[-0.07,-0.002],[-0.065,0.082]]
mir=lambda pts:[[-x,y] for x,y in reversed(pts)]
for sc in ('50','100','200'):
    for s in reps[sc]['geometry']['front']:
        m=re.fullmatch(r'(near|far)-(arm|hand)',s['id'])
        if m:
            side,part=m.groups(); pts=ARM if part=='arm' else HAND
            s['points']=[list(p) for p in (pts if side=='near' else mir(pts))]
            s.pop('solid',None); s['followTip']=True
    t=reps[sc]['decisions']['silhouette']
    if sc=='50':
        t['en']=t['en'].replace('The wings are solids in front: the arm and the hand are extruded profiles of the side view over their folded lateral band, so a spread, raised or swept wing keeps the height and the span of the side view; the plan capsules of arm and hand follow their bones and the wing spread, the tail fan follows its bone, and the feet are extruded profiles so a hanging foot keeps its length in front.','The wings are columns of arm and hand bound to their bones (the bones sit 0.068 m beside the axis), so a spread, raised or swept wing carries its folded outline with it; the plan capsules of arm and hand follow their bones, the tail fan follows its bone, and the feet are extruded profiles of the side view so a hanging foot keeps its length in front.')
        t['de']=t['de'].replace('Die Flügel sind in der Vorderansicht Volumenkörper: Arm und Hand sind ausgezogene Profile der Seitenansicht über ihrem gefalteten Seitenband, sodass ein gespreizter, gehobener oder zurückgeschwenkter Flügel Höhe und Spannweite der Seitenansicht behält; die Grundrisskapseln von Arm und Hand folgen ihren Knochen und der Flügelspreizung, der Schwanzfächer folgt seinem Knochen, und die Füße sind ausgezogene Profile, damit ein hängender Fuß in der Vorderansicht seine Länge behält.','Die Flügel sind Säulen aus Arm und Hand, an ihre Knochen gebunden (die Knochen sitzen 0,068 m neben der Achse), sodass ein gespreizter, gehobener oder zurückgeschwenkter Flügel seine gefaltete Kontur mitnimmt; die Grundrisskapseln von Arm und Hand folgen ihren Knochen, der Schwanzfächer folgt seinem Knochen, und die Füße sind ausgezogene Profile der Seitenansicht, damit ein hängender Fuß in der Vorderansicht seine Länge behält.')
    else:
        t['en']=t['en'].replace('The wings are extruded solids in front and capsules in plan that follow the wing spread of their bones.','The wings are columns in front and capsules in plan that follow the wing spread of their bones.').replace('the wings are extruded solids in front and capsules in plan that follow the wing spread of their bones.','the wings are columns in front and capsules in plan that follow the wing spread of their bones.')
        t['de']=t['de'].replace('Die Flügel sind in der Vorderansicht ausgezogene Volumenkörper und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.','Die Flügel sind in der Vorderansicht Säulen und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.').replace('die Flügel sind in der Vorderansicht ausgezogene Volumenkörper und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.','die Flügel sind in der Vorderansicht Säulen und im Grundriss Kapseln, die der Flügelspreizung ihrer Knochen folgen.')
save(f,d)
