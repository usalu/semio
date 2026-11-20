"""🌿 Models the gardener in the two digital overview scales. Distances are metres in the individual bone frames."""
from pathlib import Path
from copy import deepcopy
import json
from runpy import run_path

ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob("🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/*"))
doc=json.loads(path.read_text())
asset=doc['asset']
ref=asset['volumes']['100']
parts={p['id']:p for p in ref['body']}
rot=[0,-0.70711,0,0.70711]

def rings(rows):
    return [dict(at=[x,y,z],rx=w,ry=d,exponent=e[0] if e else 2,rotation=rot) for x,y,z,w,d,*e in rows]

def model(scale):
    mid=scale==200
    body=[]
    thermal=[]
    structure=[]
    def solid(id,bone,material,tone,**form):
        return dict(id=id,bone=bone,material=material,role='body',tone=tone,**form)
    def add(id,bone,material,tone,region,**form):
        body.append(solid(id,bone,material,tone,**form))
        if region:thermal.append(dict(id='th-'+id,of=id,region=region))
    def loft(id,bone,material,tone,region,rows):
        add(id,bone,material,tone,region,kind='loft',sections=rings(rows),capStart=True,capEnd=True)
    def member(id,bone,rows):
        structure.append(dict(**solid(id,bone,'skin',.57,kind='loft',sections=rings(rows),capStart=True,capEnd=True),member=bone,part='member'))
    def refmember(id,bone):
        structure.append(dict(id='sk-'+id,of=id,member=bone,part='member'))
    add('pad','pad','eva-foam',.55,'pad',kind='box',size=[.76,.04,.4],cornerRadius=.012 if mid else .004,transform={'translation':[-.18,.02,0]})
    add('trug','trug','polyethylene',.72,'trug',kind='revolve',axis=[0,1,0],origin=[0,0,0],profile=([[0,0],[0,.174],[.02,.19],[.205,.249],[.211,.247],[.202,.235],[.016,.177],[.011,0]] if mid else [[0,0],[0,.176],[.21,.248],[.21,.231],[.014,.173],[.014,0]]))
    if mid:
        for side in ('l','r'):
            p=deepcopy(parts['trug-handle-'+side])
            p['sections']=[p['sections'][i] for i in sorted({0,len(p['sections'])//4,len(p['sections'])//2,3*len(p['sections'])//4,len(p['sections'])-1})]
            body.append(p)
            thermal.append(dict(id='th-'+p['id'],of=p['id'],region='trug'))
    add('weeds','trug','polyethylene',.26,None,kind='ellipsoid',radii=[.203,.054 if mid else .045,.19],transform={'translation':[0,.174,0]})
    if mid:
        loft('trowel-grip','trowel','steel',.3,'trowel',[[-.057,0,0,.012,.012],[-.033,0,0,.018,.017],[.071,0,0,.016,.016],[.098,-.009,0,.015,.015]])
    tool=[([.081,-.003,0,.015,.015] if mid else [-.055,0,0,.019,.019]),[.106,-.018,0,.018,.015],[.12,-.023,0,.04,.016],[.196,-.024,0,.038,.014],[.253,-.017,0,.009,.009]]
    if not mid:tool.insert(1,[.073,0,0,.018,.018])
    loft('trowel-blade','trowel','steel',.7,'trowel',tool)
    loft('seat','pelvis','canvas',.45,'trousers',[[-.099,.015,0,.046,.038],[-.058,.026,0,.146,.109],[-.005,.025,0,.171,.124],[.125,0,0,.147,.098]] if mid else [[-.098,.015,0,.056,.047],[-.025,.025,0,.171,.123],[.124,0,0,.149,.10]])
    shirt=[[-.072,-.02,0,.169,.116,2.45],[.065,-.018,0,.151,.123,2.4],[.261,-.012,0,.163,.139,2.3],[.396,.011,0,.168,.1],[.453,.012,0,.079,.065],[.472,.009,0,.057,.055]]
    loft('shirt','torso','linen',.88,'shirt-torso',shirt if mid else [shirt[0],[.12,-.017,0,.155,.124],shirt[2],[.401,.012,0,.164,.096],shirt[-1]])
    face=[[-.027,.012,0,.05,.05],[.031,-.015,0,.05,.074],[.065,-.025,0,.06,.082],[.128,-.013,0,.071,.093],[.175,.002,0,.074,.091],[.229,.005,0,.044,.052]]
    loft('head','head','skin',.38,None,face if mid else [face[i] for i in [0,1,3,4,5]])
    brim=[[.147,.186],[.16,.19],[.175,.09],[.212,.093],[.25,.08],[.265,.045],[.265,0],[.162,0],[.154,.08],[.147,.186]]
    add('sunhat','head','straw',.72,'hat',kind='revolve',axis=[1,0,0],origin=[0,.002,0],profile=brim if mid else [[.145,.19],[.164,.19],[.176,.093],[.248,.085],[.266,.047],[.266,0],[.159,0],[.145,.19]])
    for side,sign in [('l',1),('r',-1)]:
        def limb(id,bone,material,tone,region,rows):
            loft(id+'-'+side,bone+'-'+side,material,tone,region,[[x,y,round(sign*z,4),w,d,*rest] for x,y,z,w,d,*rest in rows])
        thigh=[[-.048,0,-.004,.063,.074],[.034,.006,0,.089,.099],[.151,.01,0,.083,.09],[.322,.008,0,.065,.072],[.416,.005,0,.058,.062],[.461,.002,0,.034,.035]]
        limb('thigh','thigh','canvas',.45,'trousers',thigh if mid else [thigh[i] for i in [0,1,3,4,5]])
        shin=[[-.05,.004,0,.034,.034],[.006,.004,0,.059,.063],[.13,-.009,0,.057,.065],[.299,-.004,0,.047,.053],[.351,-.002,0,.045,.051],[.406,0,0,.034,.039]]
        limb('shank','shank','canvas',.45,'trousers',shin if mid else [shin[i] for i in [0,1,2,5]])
        clog=[[-.073,-.053,0,.028,.021],[-.035,-.035,0,.04,.04],[.007,-.035,.002,.041,.041],[.084,-.035,.002,.048,.04],[.132,-.042,.002,.052,.033]]
        limb('clog','foot','rubber-clog',.32,'clogs',clog if mid else [clog[i] for i in [0,1,3,4]])
        limb('clog-toe','toe','rubber-clog',.32,'clogs',[[-.005,.033,.002,.052,.033],[.046,.031,-.003,.047,.03],[.087,.022,-.007,.019,.014]])
        sleeve=[[-.054,0,-.011,.021,.023],[-.013,0,0,.054,.058],[.037,.002,.003,.058,.061],[.15,.002,.003,.052,.057],[.228,.002,.002,.054,.057]]
        if mid:
            limb('sleeve','upper-arm','linen',.88,'shirt-sleeves',sleeve)
            limb('elbow','upper-arm','skin',.38,'forearms',[[.213,.002,.001,.044,.047],[.292,-.003,0,.041,.04],[.327,-.002,0,.026,.026]])
        else:
            limb('upper-arm','upper-arm','linen',.88,None,[sleeve[0],sleeve[2],sleeve[-1],[.295,-.003,0,.042,.04],[.326,-.002,0,.03,.03]])
            for id,region,rows in [('sleeve','shirt-sleeves',[sleeve[0],sleeve[2],sleeve[-1]]),('elbow','forearms',[[.225,.002,.002,.044,.047],[.292,-.003,0,.041,.04],[.326,-.002,0,.03,.03]])]:
                thermal.append(dict(**solid('th-'+id+'-'+side,'upper-arm-'+side,'linen' if id=='sleeve' else 'skin',.5,kind='loft',sections=rings([[x,y,z*sign,w,d] for x,y,z,w,d in rows]),capStart=True,capEnd=True),region=region))
        wrist=-.036 if side=='l' else 0
        loft('forearm-'+side,'forearm-'+side,'skin',.38,'forearms',[[-.032,0,0,.026,.025],[.031,.003,wrist*.1,.042,.042],[.135,.002,wrist*.54,.035,.035],[.254,0,wrist,.029 if side=='l' else .023,.023 if side=='l' else .029]])
        if side=='l':
            loft('glove-l','hand-l','glove',.54,'gloves',[[-.015,0,wrist,.03,.023],[.043,0,-.039,.047,.022],[.133,-.006,wrist,.046,.016],[.18,-.019,wrist,.032,.012]])
        else:
            add('glove-r','hand-r','glove',.54,'gloves',kind='ellipsoid',radii=[.06,.047,.038],transform={'translation':[.046,-.002,-.005],'rotation':[0,0,.15643,.98769]})
        if mid:
            add('thumb-'+side,'hand-'+side,'glove',.54,'gloves',**{k:v for k,v in parts['thumb-'+side].items() if k in ('kind','p0','p1','r0','r1')})
    for p in ref['thermal']:
        if p['id'] in ('th-face','th-neck') or 'emitter' in p.get('tags',[]):
            p=deepcopy(p)
            if 'emitter' in p.get('tags',[]):
                p['radii']=[max(x,.015 if mid else .025) for x in p['radii']]
            elif p['id']=='th-face':
                p['sections']=[p['sections'][i] for i in ([0,1,2,4] if mid else [0,2,4])]
            thermal.append(p)
    for id,bone in [('pad','pad'),('trug','trug'),('trowel-blade','trowel')]:
        refmember(id,bone)
    if mid:
        refmember('trowel-grip','trowel')
        for side in ('l','r'):refmember('trug-handle-'+side,'trug')
    member('sk-pelvic-ring','pelvis',[[-.061,.019,0,.055,.037],[.002,.009,0,.112,.061],[.09,.023,0,.127,.067],[.119,.029,0,.095,.047]])
    member('sk-spine','torso',[[-.013,.041,0,.025,.022],[.13,.024,0,.021,.019],[.278,.055,0,.019,.019],[.443,.032,0,.017,.016]])
    member('sk-ribcage','torso',[[.128,.012,0,.078,.056],[.215,.009,0,.127,.091],[.293,.009,0,.14,.095],[.374,.011,0,.115,.074],[.407,.019,0,.071,.055]] if mid else [[.137,.012,0,.084,.058],[.27,.009,0,.136,.092],[.394,.017,0,.085,.058]])
    member('sk-skull','head',[[.017,-.044,0,.032,.024],[.069,-.021,0,.052,.061],[.139,.001,0,.067,.084],[.195,.006,0,.062,.072],[.222,.006,0,.032,.037]])
    for side,sign in [('l',1),('r',-1)]:
        def skeletal(id,bone,rows):
            member('sk-'+id+'-'+side,bone+'-'+side,[[x,y,z*sign,w,d] for x,y,z,w,d in rows])
        if mid:
            skeletal('clavicle','upper-arm',[[-.011,-.006,-.104,.016,.018],[-.004,-.002,-.048,.015,.014],[0,0,0,.02,.017]])
        skeletal('femur','thigh',[[-.018,0,0,.02,.02],[.036,.002,.032,.022,.023],[.226,.009,.018,.016,.017],[.405,0,0,.039,.028]])
        skeletal('tibia','shank',[[-.01,.001,0,.038,.03],[.082,.006,-.002,.019,.018],[.28,.006,-.004,.015,.016],[.4,0,-.004,.021,.018]])
        skeletal('humerus','upper-arm',[[-.012,0,0,.023,.022],[.044,0,0,.016,.015],[.215,.002,0,.012,.014],[.3,-.001,0,.028,.018]])
        offset=-.036 if side=='l' else 0
        member('sk-forearm-'+side,'forearm-'+side,[[-.014,0,0,.023,.021],[.074,.003,offset*.25,.019,.017],[.249,0,offset,.023,.016]])
        member('sk-metacarpus-'+side,'hand-'+side,([[-.002,0,offset,.023,.012],[.096,0,offset,.036,.013],[.171,-.017,offset,.028,.01]] if side=='l' else [[0,0,0,.016,.016],[.048,.002,-.004,.031,.025],[.08,-.012,0,.021,.016]]))
        skeletal('tarsus','foot',[[-.056,-.034,0,.017,.015],[.004,-.019,0,.026,.022],[.127,-.045,0,.038,.014]])
        skeletal('phalanges','toe',[[0,.029,0,.037,.014],[.073,.025,-.005,.021,.013]])
    structure.extend(deepcopy(p) for p in ref['structure'] if p['part']=='mass' or (mid and p['part']=='joint' and any(n in p['member'] for n in ('thigh','shank','upper-arm','forearm'))))
    desc=('The rolled sleeves end above the bare elbows. Separate thumb forms, tub handles and tool grip retain the working hand and contact with the trowel.','Die Ärmel enden über den freien Ellbogen. Daumen, Tragegriffe und Werkzeugheft zeichnen die Arbeitshand und ihren Griff um die Schaufel.') if mid else ('The hat profile unites crown and brim. A single tapered tool follows the gloved fist. The knee and pointed clog remain articulated against the mat.','Krone und Krempe bilden ein Hutprofil. Eine durchgehende Schaufel folgt der behandschuhten Faust. Knie und Schuhspitze bleiben über der Matte beweglich.')
    return dict(status='authored',denominator=scale,body=body,thermal=thermal,structure=structure,decisions=[dict(en=desc[0],de=desc[1]),dict(en='Exposed face, neck, forearms, hat, sleeves, gloves and footwear each follow their heat regions. Breath leaves the face and the buoyant plume leaves the hat.',de='Gesicht, Hals, Unterarme, Hut, Ärmel, Handschuhe und Schuhe folgen ihren Wärmeregionen. Atemquellen liegen im Gesicht und der Auftriebsaustritt auf dem Hut.'),dict(en='The pelvic ring, curved spine and tapered rib cage connect to the crouched limbs. The tarsus and toes pivot independently inside each clog.',de='Beckenring, geschwungene Wirbelsäule und sich verjüngender Brustkorb schließen an die gebeugten Glieder an. Fußwurzel und Zehen drehen sich getrennt in den Gartenschuhen.')])

models={str(s):model(s) for s in (200,500)}
for s in models:
    assert asset['representations'][s]['status']=='unavailable'
    old=asset['volumes'][s]
    if old['status']!='unavailable' and old.get('body',[{}])[0].get('id')!='pad':raise ValueError('another gardener model is present')
    asset['volumes'][s]=models[s]
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
text=path.read_text()
current=json.loads(text)['asset']['volumes']
current.update(models)
start=text.index('"volumes": ')+len('"volumes": ')
end=sp['member_end'](text,start)
voltxt=text[start:end]
for key,v in models.items():
    marker='    "'+key+'": '
    i=voltxt.rindex(marker)+len(marker)
    j=sp['member_end'](voltxt,i)
    voltxt=voltxt[:i]+sp['dump'](v,3,'compact')+voltxt[j:]
new=text[:start]+voltxt+text[end:]
assert json.loads(new)['asset']['volumes']==current
assert path.read_text()==text
path.write_text(new)
