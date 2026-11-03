import json,glob,sys,copy
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
def load(a):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+a+'/🔣️.json')[0]; return f,json.load(open(f,encoding='utf8'))
EN='The body pen is 0.2 mm here, so that a limb that is foreshortened to a hairline in one view of a pose still prints at the minimum feature.'
DE='Die Körperfeder ist hier 0,2 mm stark, damit ein in einer Ansicht einer Pose zur Haarlinie verkürztes Glied noch mit dem Mindestmerkmal druckt.'
JOBS={'dairy-cow':['500'],'domestic-cat':['200'],'goat':['500','200'],'hedgehog':['100'],'herring-gull':['200','100'],'horse':['500','200'],'labrador-dog':['500']}
for a,scs in JOBS.items():
    f,d=load(a)
    for sc in scs:
        r=d['asset']['representations'][sc]; r['pens']['body']=0.2
        t=r['decisions']['silhouette']
        if EN not in t['en']: t['en']+=' '+EN; t['de']+=' '+DE
    if a=='dairy-cow':
        r=d['asset']['representations']['500']; g=r['geometry']
        el=json.load(open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA1/cow_idx.json',encoding='utf8'))['asset']['representations']['500']['geometry']['elevation']
        u=[s for s in el if s['id']=='udder'][0]
        g['elevation'].append(u)
        g['front'].append({"id":"udder","bone":"udder","role":"body","z":2.9,"filled":True,"tone":0.45,"region":"udder","kind":"poly","points":[[-0.14,-0.4],[-0.12,-0.68],[0.12,-0.68],[0.14,-0.4]],"closed":True})
        g['plan'].append({"id":"udder","bone":"udder","role":"body","z":1.2,"filled":True,"tone":0.45,"region":"udder","kind":"capsule","x0":0.1,"y0":0,"x1":0.28,"y1":0,"r0":0.13,"r1":0.13,"followTip":True})
        t=r['decisions']['silhouette']
        t['en']=t['en'].replace('The udder is left out of all three views at 1:500: at 0.3 mm it is below the minimum feature, and hides behind the chest and legs in front.','The udder stays in all three views at 1:500 as one small block (rigid in front).')
        t['de']=t['de'].replace('Das Euter entfällt bei 1:500 in allen drei Ansichten: Mit 0,3 mm liegt es unter dem Mindestmerkmal und versteckt sich vorn hinter Brust und Beinen.','Das Euter bleibt bei 1:500 in allen drei Ansichten als ein kleiner Block (vorn starr).')
    write_json_atomic(f,d)
