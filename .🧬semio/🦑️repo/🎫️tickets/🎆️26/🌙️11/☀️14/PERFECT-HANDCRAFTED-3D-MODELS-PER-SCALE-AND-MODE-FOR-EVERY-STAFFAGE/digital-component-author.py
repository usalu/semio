"""🧱 Preserve the physical cut edges, lift gear, and collector support stock in the component series."""
from copy import deepcopy
import math

def byid(parts):
    return {p['id']:p for p in parts}

def words(en,de):
    return {'en':en,'de':de}

def etics(a):
    fine=deepcopy(a['volumes']['50']);stock=byid(fine['body']);heat=byid(fine['thermal']);load=byid(fine['structure'])
    foil=stock['wool-board-low-left'];thickness=foil['size'][0]
    def course(id,base,height):
        s=deepcopy(foil);s.update(id=id,size=[thickness,height,1.0],transform={'translation':[foil['transform']['translation'][0],base+height/2,0]});return s
    lower=course('insulation-lower-course',0,.4);upper=course('insulation-upper-course',.4,.6)
    boards={'50':[s for s in fine['body'] if s['id'].startswith('wool-board-')],
            '100':[stock['wool-board-low-left'],stock['wool-board-low-right'],upper],
            '200':[lower,upper],'500':[course('insulation',0,1)]}
    bond=[s for s in fine['body'] if not s['id'].startswith('wool-board-') and s['id']!='render']
    for k,v in a['volumes'].items():
        if any(s['id']=='th-face-render' for s in v['thermal']):continue
        v['body']=deepcopy(bond+boards[k]+[stock['render']])
        q=deepcopy(heat['th-insulation']);q={key:q[key] for key in ('region','layer','sectionFaces')}
        v['thermal']=deepcopy([heat['th-plaster'],heat['th-brick']])+[dict(deepcopy(p),**q,id='th-'+p['id']) for p in boards[k]]+[dict(deepcopy(heat['th-render']),id='th-face-render')]
        v['structure']=[p for p in v['structure'] if not p['id'].startswith('joint-anchor-')]+deepcopy([p for p in fine['structure'] if p['id'].startswith('joint-anchor-')])
        v['decisions']=[words('The cut sample contains actual 15 mm room plaster, a 365 mm bonded brick leaf with mortar courses, mineral wool and 20 mm outside render. Larger digital scales consolidate adjacent wool boards within one material; each physical interface stays at its measured depth.','Der Schnitt zeigt 15 mm Innenputz, 365 mm im Verband gemauertes Ziegelmauerwerk mit Mörtelfugen, Mineralwolle und 20 mm Außenputz. In kleineren Darstellungen werden benachbarte Dämmplatten desselben Materials zusammengefasst; jede Materialgrenze bleibt in ihrer gemessenen Tiefe.'),
                        words('Four 8 mm anchor shafts embed 50 mm into the masonry. Their 60 mm heads retain the insulation beneath the render. The brick leaf carries the gravity load.','Vier 8-mm-Dübel verankern sich 50 mm tief im Ziegel. Die 60-mm-Teller halten die Dämmung unter dem Putz. Die Ziegelschale trägt die Eigenlast.'),
                        words('Minimal: all four insulation anchors, the masonry and the bearing and mass indicators are required at every scale.','Minimal: Alle vier Dämmstoffdübel, das Mauerwerk sowie die Auflager- und Massensymbole werden in jedem Maßstab benötigt.')]

def blind(a):
    fine=deepcopy(a['volumes']['50']);stock=byid(fine['body']);structure=byid(fine['structure'])
    mechanical=[p for p in fine['body'] if p['id'].startswith(('lift-tape','guide-pin')) or p['id'] in ('tape-bearing','head-rail','tilt-shaft','guide-left','guide-right','bottom-rail','bottom-rail-pin')]
    ids={p['id'] for p in mechanical};joints=[p for p in fine['structure'] if p.get('of') in ids or p['id'].startswith('member-slat-')]
    names={p['id'] for p in joints};heated=[p for p in fine['thermal'] if p.get('of') in ids]
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if any(p['id']=='member-slat-17' for p in v['structure']):continue
        v['body']=[p for p in v['body'] if p['id'] not in ids]+deepcopy(mechanical)
        v['thermal']=[p for p in v['thermal'] if p.get('of') not in ids and p['id'] not in {s['id'] for s in heated}]+deepcopy(heated)
        v['structure']=[p for p in v['structure'] if p['id'] not in names]+deepcopy(joints)
        if k=='100':
            rail=deepcopy(stock['head-rail'])
            rail.update(id='member-head-rail',part='member',member='box',profile=[[.056,.148],[.0575,.148],[.0575,.2025],[.1125,.2025],[.1125,.148],[.114,.148],[.114,.204],[.056,.204]])
            v['structure']=[rail if p['id']=='member-head-rail' else p for p in v['structure']]

        v['decisions']=[d for d in v['decisions'] if not any(q in d['en'].lower() for q in ('slat','tape','bearing','guide','head rail'))]
        v['decisions'].append(words('Eighteen individual aluminium slats rotate on their own axes and retract with their slide bones. Two 8 mm lift tapes travel through the head bearing. Side pins locate the slats in the open guide channels, and the suspended bottom rail closes the blind.','Achtzehn Aluminiumlamellen drehen sich um ihre eigenen Achsen und ziehen sich mit den Gleitelementen ein. Zwei 8-mm-Aufzugsbänder laufen über das Kopflager. Seitliche Stifte führen die Lamellen in offenen Führungen; die hängende Unterschiene schließt den Behang ab.'))

def collector(a):
    fine=deepcopy(a['volumes']['50']);stock=byid(fine['body'])
    parts=[p for p in fine['body'] if p['id'].startswith(('rail-','post-','knee-','brace-rising','bolt-brace','cleat-'))]
    stock_ids={p['id'] for p in parts}
    def folded_tray(segments):
        tray=deepcopy(stock['tray']);ring=tray['profile'];points=[]
        for i,(x,y) in enumerate(ring):
            if i not in (0,1,6,7):points.append([x,y]);continue
            radius=.003 if i in (0,1) else .0015
            prev,nextp=ring[i-1],ring[(i+1)%len(ring)]
            def leg(p):
                dx,dy=p[0]-x,p[1]-y
                mag=math.hypot(dx,dy)
                return dx/mag,dy/mag
            before,after=leg(prev),leg(nextp)
            cx,cy=x+radius*(before[0]+after[0]),y+radius*(before[1]+after[1])
            start=math.atan2(-after[1],-after[0]);end=math.atan2(-before[1],-before[0]);sweep=(end-start+math.pi)%(2*math.pi)-math.pi
            points += [[round(cx+radius*math.cos(start+sweep*j/segments),7),round(cy+radius*math.sin(start+sweep*j/segments),7)] for j in range(segments+1)]
        tray['profile']=points;return tray
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if any(p['id']=='st-bolt-brace-rising-left' for p in v['structure']):continue
        merged={'side-frame','rail','post-rear','post-front','knee','brace-rising-left','brace-rising-right'}
        v['body']=[p for p in v['body'] if p['id'] not in merged|stock_ids]+deepcopy(parts)
        v['thermal']=[p for p in v['thermal'] if p.get('of') not in merged|stock_ids and p['id'].removeprefix('th-') not in merged|stock_ids]+[dict(id='th-'+p['id'],of=p['id'],region='stand') for p in parts]
        v['structure']=[p for p in v['structure'] if p.get('of') not in merged|stock_ids and p['id'].removeprefix('st-') not in merged|stock_ids]
        v['structure']+=deepcopy([p for p in fine['structure'] if p.get('of') in stock_ids])
        for s in parts:
            if s['id'].startswith(('bolt-brace','cleat-')) and not any(p.get('of')==s['id'] for p in v['structure']):
                v['structure'].append(dict(id='st-'+s['id'],of=s['id'],member='stand',part='joint'))
        v['decisions']=[d for d in v['decisions'] if not any(q in d['en'].lower() for q in ('brace','side frame','stand','knee'))]
        v['decisions'].append(words('Two 2 × 20 mm steel straps cross behind the collector. Their ends bolt to the posts. Side knees join the real base channels and posts, with cleats carrying the reactions into the feet.','Zwei Stahlbänder 2 × 20 mm kreuzen sich hinter dem Kollektor. Ihre Enden sind an den Stützen verschraubt. Seitliche Kniebleche verbinden Fußschienen und Stützen; Laschen übertragen die Auflagerkräfte.'))

    for k,v in a['volumes'].items():
        if k=='50' or any(p['id']=='th-working-absorber' for p in v['thermal']):continue
        common=['tray-side-left','tray-side-right','insulation','glass','absorber','header-upper','header-lower']
        extras={'100':['cover-side-left','cover-side-right','cover-end-upper','cover-end-lower','riser-end-upper','riser-end-lower'],'200':['cover-end-upper','cover-end-lower','riser-end-upper','riser-end-lower'],'500':[]}[k]
        panel=[folded_tray({'100':3,'200':2,'500':1}[k])]+deepcopy([stock[id] for id in common+extras])
        v['body']=[p for p in v['body'] if p['bone']=='stand']+panel
        hmap={p.get('of'):p['region'] for p in fine['thermal']}
        v['thermal']=[p for p in v['thermal'] if p['region']=='stand' or p.get('tags')]+[dict(id='th-working-absorber' if p['id']=='absorber' else 'th-'+p['id'],of=p['id'],region=hmap[p['id']]) for p in panel]
        current={p['id'] for p in v['body']}
        v['structure']=[p for p in v['structure'] if 'of' not in p or p['of'] in current]
        for id in ('glass','header-upper','header-lower'):
            aliases={'glass':('st-glass','glass-pane'),'header-upper':('st-header-upper','header-pipe-upper'),'header-lower':('st-header-lower','header-pipe-lower')}[id]
            v['structure']=[p for p in v['structure'] if p['id'] not in aliases]
            v['structure'].append(dict(id='st-'+id,of=id,member=stock[id]['bone'],part='member'))
        v['decisions'].append(words('The collector contains 3.2 mm glazing, the embossed absorber, mineral-wool backing and two copper headers inside the 1.5 mm folded tray. The smaller scales simplify the bend radius stations and edge trims.','Im 1,5-mm-Blechgehäuse liegen 3,2-mm-Glas, geprägter Absorber, Mineralwolle und zwei Kupfersammler. In den kleineren Maßstäben sind die Biegeradien und die Kanteneinfassungen vereinfacht.'))

    r=a['representations'];fine=r['100']['geometry']['elevation'];coarse=r['500']['geometry']['elevation']
    if not any(p['id']=='knee-centre' for p in coarse):
        strokes=[]
        for p in fine:
            if p['bone']!='stand':continue
            s=deepcopy(p);s['id']=p['id'].replace('brace','centre')
            if p['kind']=='rect':
                x,y,w,h=(p[key] for key in ('x','y','w','h'));points=[[x,y+h/2],[x+w,y+h/2]]
            else:
                ring=p['points']
                if p['id']=='knee-brace':
                    points=[[(ring[0][j]+ring[3][j])/2 for j in (0,1)],[(ring[1][j]+ring[2][j])/2 for j in (0,1)]]
                else:points=[[(ring[0][j]+ring[1][j])/2 for j in (0,1)],[(ring[2][j]+ring[3][j])/2 for j in (0,1)]]
            s={key:val for key,val in s.items() if key not in ('x','y','w','h','points','closed','filled')}
            s.update(kind='poly',closed=False,filled=False,points=points,penMm=.13)
            strokes.append(s)
        r['500']['geometry']['elevation']=strokes+[s for s in coarse if s['bone']=='housing']

    for p in a['volumes']['500']['body']:
        if p['id'].startswith(('rail-','post-','knee-')) and p['bone']=='stand':p['slender']=True

    for q in r['500']['geometry']['elevation']:
        if q['bone']=='stand':
            stock=next(p for p in fine if p['id']==q['id'].replace('centre','brace'))
            q.clear();q.update(deepcopy(stock));q['penMm']=.18
    r['500']['geometry']['elevation'][-1]['id']='collector-envelope'
