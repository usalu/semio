import json,glob,sys,copy
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
def load(a):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+a+'/🔣️.json')[0]; return f,json.load(open(f,encoding='utf8'))
def shift(s,dy):
    k=s['kind']
    if k=='poly': s['points']=[[x,round(y+dy,5)] for x,y in s['points']]
    elif k=='ellipse': s['cy']=round(s['cy']+dy,5)
    elif k=='capsule': s['y0']=round(s['y0']+dy,5); s['y1']=round(s['y1']+dy,5)
    else: raise Exception(k)
def repivot(a,DL,DT):
    by={b['id']:b for b in a['rig']['bones']}
    lum,tho=by['lumbar'],by['thorax']
    lum['y']=round(lum['y']-DL,4); lum['mass']['comY']=round(lum['mass']['comY']+DL,4)
    tho['y']=round(tho['y']-DT+DL,4); tho['mass']['comY']=round(tho['mass']['comY']+DT,4)
    for b in a['rig']['bones']:
        if b['parent']=='thorax': b['y']=round(b['y']+DT,4)
    for an in a['anchors']:
        if an['bone']=='thorax': an['y']=round(an['y']+DT,4)
        if an['bone']=='lumbar': an['y']=round(an['y']+DL,4)
    for r in a['representations'].values():
        if r['status']!='authored': continue
        for s in r['geometry']['elevation']:
            if s['bone']=='lumbar': shift(s,DL)
            if s['bone']=='thorax': shift(s,DT)
def solidify(a,spec,note_en,note_de):
    # spec: shape id -> (bone, axisX, {scale: profile})
    for sc,r in a['representations'].items():
        if r['status']!='authored': continue
        for v in('front','plan'):
            for s in r['geometry'][v]:
                if s['id'] in spec and sc in spec[s['id']][2]:
                    bone,ax,prof=spec[s['id']]
                    for k in('cx','cy','rx','ry','x0','y0','x1','y1','r0','r1','followTip'): s.pop(k,None)
                    s['bone']=bone; s['kind']='poly'; s['points']=copy.deepcopy(prof[sc]); s['closed']=True
                    s['solid']={'kind':'revolved','axisBone':bone,'axisX':ax,'axisZ':0,'mirror':True}
        r['decisions']['silhouette']['en']+=' '+note_en; r['decisions']['silhouette']['de']+=' '+note_de
def rel(pts,x0): return [[round(x-x0,4),r] for x,r in pts]
EN='Trunk and neck are revolved solids about their spine bones in front and plan (round cross-section at the true width), so a sitting or rearing body keeps one connected trunk.'
DE='Rumpf und Hals sind in Vorderansicht und Grundriss um ihre Wirbelsäulenknochen gedrehte Körper (runder Querschnitt in wahrer Breite), sodass ein sitzender oder aufgerichteter Körper einen zusammenhängenden Rumpf behält.'
# dog
f,d=load('labrador-dog'); a=d['asset']
repivot(a,0.03,0.072)
H=[(-0.125,0),(-0.11,0.07),(-0.06,0.1),(0.1,0.115),(0.27,0.13),(0.38,0.12),(0.40,0)]
C=[(0.26,0),(0.28,0.09),(0.34,0.14),(0.46,0.135),(0.56,0.1),(0.62,0)]
N=[[-0.02,0],[0,0.075],[0.1,0.072],[0.2,0.062],[0.21,0]]
sp={'hindbody':('lumbar',0.14,{'50':rel(H,0.14),'100':rel([H[0],H[2],H[4],H[6]],0.14),'200':rel([H[0],H[4],H[6]],0.14),'500':rel([H[0],H[4],H[6]],0.14)}),
    'chest':('thorax',0.31,{'50':rel(C,0.31),'100':rel([C[0],C[2],C[4],C[5]],0.31),'200':rel([C[0],C[2],C[5]],0.31),'500':rel([C[0],C[2],C[5]],0.31)}),
    'neck':('neck',0.55,{'50':N,'100':[N[0],N[1],N[3],N[4]],'200':[N[0],N[2],N[4]],'500':[N[0],N[2],N[4]]})}
solidify(a,sp,EN,DE); write_json_atomic(f,d)
# cat
f,d=load('domestic-cat'); a=d['asset']
Hc=[(-0.055,0),(-0.045,0.04),(0.02,0.056),(0.12,0.06),(0.2,0.05),(0.215,0)]
Ma=[(0.118,0),(0.13,0.05),(0.18,0.062),(0.25,0.064),(0.3,0.05),(0.305,0)]
Cc=[(0.208,0),(0.22,0.05),(0.27,0.064),(0.32,0.055),(0.342,0)]
Nc=[[-0.01,0],[0,0.042],[0.05,0.041],[0.08,0.035],[0.09,0]]
sp={'hindbody':('lumbar',0.06,{'50':rel(Hc,0.06),'100':rel([Hc[0],Hc[2],Hc[3],Hc[5]],0.06),'200':rel([Hc[0],Hc[3],Hc[5]],0.06)}),
    'midbody':('thorax-a',0.145,{'50':rel(Ma,0.145),'100':rel([Ma[0],Ma[2],Ma[3],Ma[5]],0.145)}),
    'chest':('thorax-b',0.215,{'50':rel(Cc,0.215),'100':rel([Cc[0],Cc[2],Cc[4]],0.215),'200':rel([Cc[0],Cc[2],Cc[4]],0.215)}),
    'neck':('neck',0.285,{'50':Nc,'100':[Nc[0],Nc[1],Nc[3],Nc[4]],'200':[Nc[0],Nc[2],Nc[4]]})}
solidify(a,sp,EN,DE); write_json_atomic(f,d)
print('ok')
