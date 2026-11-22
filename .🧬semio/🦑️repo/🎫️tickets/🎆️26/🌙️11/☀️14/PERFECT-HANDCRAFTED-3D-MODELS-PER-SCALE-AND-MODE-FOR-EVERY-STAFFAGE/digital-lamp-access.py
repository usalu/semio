"""💡 Seat the bracket in its sleeve and fit the column-door lock and gear-cover spring catch."""
from copy import deepcopy
import json
import math
from pathlib import Path

T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/street-lamp/🔣️.json'))

def text(en,de):
    return {'en':en,'de':de}

def reference(part,region):
    return {'id':'th-'+part,'of':part,'region':region}

def author(a):
    if any(s['id']=='door-lock' for s in a['volumes']['500']['body']):
        return
    fine=deepcopy(a['volumes']['50'])
    original={s['id']:s for s in fine['body']}
    for scale,v in a['volumes'].items():
        by={s['id']:s for s in v['body']}
        st={s['id']:s for s in v['structure']}
        for part in [by['arm'],st['member-arm']]:
            part['profile']=[[.035,.0272],[.82,.0272],[.82,.0302],[.035,.0302],[.035,.0272]]
        sleeve=deepcopy(original['head-spigot'])
        sleeve['profile']=[[-.03,.0303],[.100,.0303],[.100,.038],[-.03,.038],[-.03,.0303]]
        v['body']=[p for p in v['body'] if p['id']!='head-spigot']+[sleeve]
        v['structure']=[p for p in v['structure'] if p['id']!='joint-head-spigot']+[dict(id='joint-head-spigot',of='head-spigot',member='head',part='joint')]
        if scale!='50':
            v['thermal'].append(reference('head-spigot','lamp-housing'))
            v['body'].append(deepcopy(original['pole-hatch']))
        v['thermal'].append(reference('pole-hatch','pole-galv'))
        radius=.085-(.085-.040)*.99/5.88+.003
        outer=[[round(.99+.005*math.cos(i*math.pi/8),8),round(.005*math.sin(i*math.pi/8),8)] for i in range(16)]
        drive=[[.988,-.002],[.988,.002],[.992,.002],[.992,-.002]]
        lock=dict(id='door-lock',bone='pole',kind='extrude',material='galv-steel',role='detail',tone=.22,
                  plane='xy',profile=outer,holes=[drive],**{'from':round(-radius-.002,7),'to':round(-radius+.006,7)})
        shell=original['head-housing']
        rings=shell['sections']
        def roof(x):
            for left,right in zip(rings,rings[1:]):
                x1=left['at'][0];x2=right['at'][0]
                if x1<=x<=x2:
                    q=(x-x1)/(x2-x1)
                    return (1-q)*(left['at'][1]+left['ry'])+q*(right['at'][1]+right['ry'])
            raise ValueError(x)
        if scale!='50':
            housing=by.get('head-housing',by.get('head'))
            def shape_section(shell):
                for left,right in zip(shell['sections'],shell['sections'][1:]):
                    if left['at'][0]<=.17<=right['at'][0]:
                        f=(.17-left['at'][0])/(right['at'][0]-left['at'][0])
                        return tuple((1-f)*l+f*r for l,r in zip((left['at'][1],left['rx'],left['ry']),(right['at'][1],right['rx'],right['ry'])))
            fine_y,fine_w,fine_h=shape_section(original['head-housing'])
            y,w,h=shape_section(housing)
            seam=deepcopy(original['head-hatch-joint'])
            seam['profile']=[[round(y+(u-fine_y)*h/fine_h,7),round(v*w/fine_w,7)] for u,v in seam['profile']]
            v['body'].append(seam)
            v['thermal'].append(reference('head-hatch-joint','lamp-housing'))
        seat=roof(.170)
        curve=[(-.020,-.0015),(-.014,-.0015),(-.011,.003),(-.004,.006),(.008,.006),(.011,.003),(.019,.003),
               (.019,.005),(.012,.005),(.009,.008),(-.005,.008),(-.013,.004),(-.016,.001),(-.020,.001)]
        latch=dict(id='head-latch',bone='head',kind='extrude',material='galv-steel',role='detail',tone=.26,
                   plane='xy',profile=[[round(.17+x,7),round(seat+y,7)] for x,y in curve],**{'from':-.006,'to':.006})
        keeper=dict(id='head-latch-keeper',bone='head',kind='box',material='aluminium-anodised',role='detail',tone=.44,
                    size=[.008,.009,.017],transform={'translation':[.186,round(seat-.0015,7),0]})
        pivot=dict(id='head-latch-pivot',bone='head',kind='box',material='aluminium-anodised',role='detail',tone=.43,
                   size=[.009,.012,.016],transform={'translation':[.153,round(seat-.005,7),0]})
        for part,region,joint in [(pivot,'lamp-housing','joint-head-latch-pivot'),(lock,'pole-galv','joint-door-lock'),(latch,'lamp-housing','joint-head-latch'),(keeper,'lamp-housing','joint-head-keeper')]:
            v['body'].append(part)
            v['thermal'].append(reference(part['id'],region))
            v['structure'].append(dict(id=joint,of=part['id'],member=part['bone'],part='joint'))
        rep=a['representations'][scale]
        if scale=='50':
            rep['geometry']['elevation'].append(dict(id='door-lock',bone='pole',role='detail',kind='ellipse',cx=.99,cy=0,rx=.005,ry=.005,
                z=4,filled=True,tone=.22,region='pole-galv',material='galv-steel',penMm=.16))
        v['decisions'].append(text('A 60.4 mm bracket with a 3 mm wall enters a 60.6 mm bored side-entry sleeve for 130 mm. A recessed square-drive lock secures the terminal door. A galvanised spring catch spans the gear-cover joint and engages its keeper.',
                                   'Der 60,4-mm-Ausleger mit 3 mm Wandstärke greift 130 mm in eine seitliche Hülse mit 60,6 mm Bohrung. Ein versenkter Vierkantverschluss sichert die Masttür. Ein verzinkter Federverschluss überbrückt die Gehäusefuge und greift ins Gegenlager.'))
        if scale=='500':
            v['decisions'][0]=text('At 1:500 the tapered column carries a 60.4 mm tubular arm and a five-station luminaire. The access panel, lock, gear catch and sleeve remain at their installed dimensions.',
                                    'Bei 1:500 trägt der konische Mast ein 60,4-mm-Auslegerrohr und einen Leuchtenkopf mit fünf Querschnitten. Zugangstür, Verschluss, Federriegel und Hülse behalten ihre Einbaumaße.')
            v['decisions'][2]=text('The hollow arm enters the bored sleeve at the head. The lumped masses remain at their authored centres and the foundation bears on the ground pad.',
                                    'Der Hohlausleger greift in die Hülse am Leuchtenkopf. Die konzentrierten Massen bleiben an ihren Schwerpunkten und das Fundament liegt auf dem Bodenlager.')


if __name__=='__main__':
    d=json.loads(CAT.read_bytes())
    author(d['asset'])
    (T/'🗑️generated/digital-access-street-lamp.json').write_text(json.dumps(d,ensure_ascii=False))
