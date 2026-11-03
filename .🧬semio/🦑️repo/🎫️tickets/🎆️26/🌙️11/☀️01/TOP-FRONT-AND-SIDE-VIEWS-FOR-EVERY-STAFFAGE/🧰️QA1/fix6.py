import json,glob,copy
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d): open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
f,d=load('horse'); reps=d['asset']['representations']
base={v:[s for s in reps['50']['geometry'][v] if s['id'] in('tail-1','tail-2','tail-3','tail-4')] for v in('front','plan')}
for sc,k in (('200',1.2),('500',1.5)):
    g=reps[sc]['geometry']
    for v in('front','plan'):
        idx=min(i for i,s in enumerate(g[v]) if s['id'].startswith('tail'))
        g[v]=[s for s in g[v] if not s['id'].startswith('tail')]
        new=[]
        for s in base[v]:
            n=copy.deepcopy(s)
            if v=='front': n['points']=[[round(x*k,4),y] for x,y in n['points']]
            else: n['r0']=round(n['r0']*k,4); n['r1']=round(n['r1']*k,4)
            new.append(n)
        g[v][idx:idx]=new
    t=reps[sc]['decisions']['silhouette']
    t['en']+=f' The tail is drawn in front and plan with one piece per tail bone, {int((k-1)*100)} % wider than true, so it follows the swish and the lateral swing of every tail bone.'
    t['de']+=f' Der Schweif wird in Vorderansicht und Grundriss mit einem Stück je Schweifknochen gezeichnet, {int((k-1)*100)} % breiter als wahr, sodass er dem Schlagen und dem seitlichen Schwingen jedes Schweifknochens folgt.'
if True:
    for sc in ('50','100'):
        t=reps[sc]['decisions']['silhouette']
        t['en']+=' The tail swishes in the graze-and-swish clip: each tail bone carries its own lateral swing, and the ears turn sideways at rest (twist).'
        t['de']+=' Der Schweif schlägt im Clip Grasen und Schweifschlagen: Jeder Schweifknochen schwingt seitlich, und die Ohren drehen sich in Ruhe zur Seite (Verdrehung).'
save(f,d)
