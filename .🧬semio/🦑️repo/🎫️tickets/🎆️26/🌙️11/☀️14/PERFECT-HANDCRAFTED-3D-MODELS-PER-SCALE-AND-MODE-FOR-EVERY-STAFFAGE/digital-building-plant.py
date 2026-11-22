"""🛠️ Model the acoustic ventilation passages and the four compressor isolation mounts."""
from copy import deepcopy
import math

def rect(x0,y0,x1,y1):
    return [[x0,y0],[x1,y0],[x1,y1],[x0,y1]]

def text(en,de):
    return {'en':en,'de':de}

def path(shape,rings):
    for key in ('points','closed','hatch'):shape.pop(key,None)
    shape.update(kind='path',d=' '.join(' '.join(('M' if j==0 else 'L')+' '+str(round(x,6))+' '+str(round(y,6)) for j,(x,y) in enumerate(r))+' Z' for r in rings))

def generator(a):
    if not any(m['id']=='acoustic-wool' for m in a['materials']):
        a['materials'].append(dict(id='acoustic-wool',title=text('Acoustic mineral wool','Akustik-Mineralwolle'),emissivity=.9,solarAbsorptivity=.6,densityKgM3=80,conductivityWmK=.04,specificHeatJKgK=1000,source='authored-estimate',reference='ashrae-2021'))
        region=next(r for r in a['thermal']['regions'] if r['id']=='enclosure-walls')
        next(l for l in region['layers'] if l['id']=='wool')['material']='acoustic-wool'
        region['areaM2']=round(2*(6.06+2.44)*2.59-4*1.3*1.5+4*2*(1.3+1.5)*.052+4*2*(1.4+1.6)*.03,2)

    fine={p['id']:deepcopy(p) for p in a['volumes']['50']['body']}
    openings=[rect(x,.55,x+1.3,2.05)[::-1] for x in (.23,2.83)]
    contour=rect(.03,.2,6.03,2.47)
    frames=[fine['louvre-frame-'+side] for side in ('r','l')]
    blades=[p for p in fine.values() if p['id'].endswith('-slat')]
    for p in blades:
        if p['kind']=='extrude':continue
        p.pop('texture',None)
        width,gauge,depth=p.pop('size')
        p.update(kind='extrude',plane='zx',slender=True,profile=rect(-depth/2,-width/2,depth/2,width/2),**{'from':-gauge/2,'to':gauge/2})
    for k,v in a['volumes'].items():
        if any(p['id']=='wall-skin-r' for p in v['body']):continue
        walls=[];baffles=[]
        for side,sign in [('l',1),('r',-1)]:
            for layer,lo,hi,tone in [('skin',1.188,1.19,.55),('wool',1.138,1.188,.22)]:
                walls.append(dict(id='wall-'+layer+'-'+side,bone='container',material='acoustic-wool' if layer=='wool' else 'container-steel',kind='extrude',role='body',tone=tone,plane='xy',profile=contour,holes=deepcopy(openings),
                                  **{'from':min(sign*lo,sign*hi),'to':max(sign*lo,sign*hi)}))
            for bay,x in enumerate((.445,3.045)):
                ring=[[x-.04,sign*1.135],[x+.04,sign*1.135],[x+.065,sign*1.10],[x+.065,sign*.89],[x+.04,sign*.85],[x-.04,sign*.85],[x-.065,sign*.89],[x-.065,sign*1.10]]
                baffles.append(dict(id=f'attenuator-{side}-{bay+1}',bone='container',material='container-steel',kind='extrude',role='body',tone=.15,plane='zx',profile=[[z,y] for y,z in ring],
                                    **{'from':.55,'to':2.05},repeat=dict(count=3,translation=[.435,0,0])))
        for side,x in [('a',.056),('b',6.004)]:
            walls.append(dict(id='wall-end-'+side,bone='container',material='container-steel',kind='box',role='body',tone=.55,size=[.052,2.27,2.276],transform=dict(translation=[x,1.335,0])))
        stock=walls+deepcopy(frames+blades)+baffles
        removed={p['id'] for p in v['body'] if p['id']=='wall' or p['id'].startswith(('inlet-louvre','attenuator','louvre-frame'))}
        v['body']=[p for p in v['body'] if p['id'] not in removed]+stock
        v['thermal']=[p for p in v['thermal'] if not p['id'].startswith(('th-wall','th-attenuator','th-louvre','th-inlet-louvre'))]
        for p in stock:
            id=p['id'];z=dict(id='th-'+id,of=id,region='inlet-louvres' if id.endswith('slat') else 'enclosure-walls')
            if id.endswith('slat'):z['layer']='alu'
            if id.startswith('wall-'):
                z['layer']='wool' if '-wool-' in id else 'skin'
                z['sectionFaces']=[{'l':'-w','r':'+w','a':'+u','b':'-u'}[id[-1]]]
            if id.startswith('attenuator-'):
                z['sectionFaces']=['-v','+v','-w','+w']
            v['thermal'].append(z)
        for p in frames:
            v['structure'].append(dict(id='intake-jamb-'+p['id'][-1],of=p['id'],member='container',part='member'))
        roof=next(p for p in v['body'] if p['id']=='roof')
        roofcut=rect(-.82,5.06,.82,5.92)[::-1]
        roofbase=dict(id='roof',bone='container',kind='extrude',material='container-steel',role='body',tone=.5,plane='zx',profile=rect(-1.16,.06,1.16,6.0),holes=[roofcut],**{'from':2.47,'to':2.576})
        roofbase['from']=2.574
        roof.clear();roof.update(roofbase)
        eave=deepcopy(roofbase);eave.update(id='roof-eave',holes=[rect(-1.157,.063,1.157,5.997)[::-1]],**{'from':2.47,'to':2.574})
        v['body'].append(eave)
        for side,c,sz,omit in (
            ('left',[3.03,2.522,-1.1585],[5.94,.104,.003],'+w'),
            ('right',[3.03,2.522,1.1585],[5.94,.104,.003],'-w'),
            ('front',[.0615,2.522,0],[.003,.104,2.314],'+u'),
            ('rear',[5.9985,2.522,0],[.003,.104,2.314],'-u')):
            v['thermal'].append(dict(id='th-roof-eave-'+side,kind='box',bone='container',role='body',material='container-steel',tone=.5,size=sz,transform=dict(translation=c),region='roof',layer='sheet',sectionFaces=[omit,'-v','+v']))
        for solid in v['structure']:
            if solid['id']=='mass-container':solid['radii']=[.04,.04,.04]

        for p in v['thermal']:
            if p['id']=='th-roof-sheet':
                p.clear();p.update(id='th-roof-sheet',of='roof',region='roof',layer='sheet',sectionFaces=['-v'])
        if k in ('200','500'):
            rails=['casting-top-a','casting-top-b','top-rail-r','top-rail-l','top-end-rail-a','top-end-rail-b']
            v['body'].extend(deepcopy([fine[id] for id in rails]))
            v['thermal'].extend([dict(id='th-'+id,of=id,region='enclosure-walls') for id in rails])
            hood=[p for p in fine.values() if p['id'] in ('hood-side-l','hood-side-r','hood-top','hood-slope','hood-back','hood-frame')]
            v['body']=[p for p in v['body'] if p['id']!='discharge-hood']+deepcopy(hood)
            v['thermal']=[p for p in v['thermal'] if p['id']!='th-discharge-hood']+[dict(id='th-'+p['id'],of=p['id'],region='discharge-hood',sectionFaces=['-w'] if 'side-' in p['id'] else ['-v']) for p in hood]
        for p in v['thermal']:
            if k!='50' and p.get('region')=='enclosure-walls' and 'layer' not in p:
                p['layer']='skin'
        rep=a['representations'][k]
        for shape in rep['geometry']['elevation']:
            id=shape['id']
            if id=='wall':path(shape,[shape['points'],*openings])
            if id in ('inlet-louvre-1','inlet-louvre-2'):
                x=.23 if id.endswith('1') else 2.83
                frame=.075 if k=='500' else .05
                left,right=x-.05+frame,x+1.35-frame
                bottom,top=.5+frame,2.1-frame
                pitch,width={'50':(.1,.019),'100':(.1,.024),'200':(.1,.026),'500':(.3,.075)}[k]
                ribs=[(x+.15+i*.435,x+.28+i*.435) for i in range(3)]
                spaces=list(zip([left,*[hi for lo,hi in ribs]],[*[lo for lo,hi in ribs],right]))
                slats=[.6+i*pitch for i in range(15) if bottom+width/2<.6+i*pitch<top-width/2]
                rings=[shape['points']]+[rect(lo,bottom,hi,top)[::-1] for lo,hi in spaces]
                path(shape,rings)
                blade=deepcopy(shape)
                blade.update(id=id+'-blades',filled=False,z=2.21,penMm=round(width*1000/int(k),3),d=' '.join(f'M {left:.3f} {y:.3f} L {right:.3f} {y:.3f}' for y in slats))
                rep['geometry']['elevation'].append(blade)
        v['decisions']=[d for d in v['decisions'] if not any(q in d['en'].lower() for q in ('louvre','attenuator','wall','hood block'))]
        r=rep['decisions']
        r['retained']=text('Four inlet banks occupy two matching pairs of wall openings. The long elevation shows the louvre blades, acoustic splitters and their surrounding frames.','Vier Einlassfelder liegen in zwei gegenüberliegenden Öffnungspaaren. Die Längsansicht zeigt Lamellen, Schalldämpferkulissen und die umgebenden Rahmen.')
        r['omitted']=text('At the smaller printed scales selected blades represent the fifteen physical blades per intake.','In den kleineren Druckmaßstäben vertreten ausgewählte Linien die fünfzehn Lamellen jedes Einlasses.')
        r['merged']=text('The drawing groups the steel skins and insulated cores into one wall outline around the four openings.','Die Zeichnung fasst Stahlhaut und Dämmkern in einer Wandkontur um die vier Öffnungen zusammen.')
        r['overlays']=text('The walls, inlet banks, roof and discharge plenum retain separate surface readings.','Wände, Einlassfelder, Dach und Ausblashaube behalten getrennte Oberflächenwerte.')
        v['decisions'].append(text('Four framed openings pierce the steel skin and the 50 mm acoustic lining. Each inlet has fifteen separate inclined blades ahead of three deep splitters. The roof is pierced below the open discharge plenum.','Vier gerahmte Öffnungen durchdringen Stahlhaut und 50-mm-Schalldämmung. Jeder Einlass enthält fünfzehn geneigte Lamellen vor drei tiefen Kulissen. Das Dach ist unter der Ausblashaube geöffnet.'))

    for v in a['volumes'].values():
        if any(p['id']=='intake-upright-l-1' for p in v['body']):continue
        for side,sign in [('l',1),('r',-1)]:
            for corner,x in enumerate((.18,1.53),1):
                profile=[[1.19,x],[1.19,x+.05],[1.125,x+.05],[1.125,x+.047],[1.187,x+.047],[1.187,x+.003],[1.125,x+.003],[1.125,x]]
                id=f'intake-upright-{side}-{corner}'
                v['body'].append(dict(id=id,kind='extrude',bone='container',material='galvanised-steel',role='body',tone=.25,plane='zx',profile=[[sign*z,round(u,5)] for z,u in profile],**{'from':.19,'to':2.472},repeat=dict(count=2,translation=[2.6,0,0])))
                v['structure'].append(dict(id='member-'+id,of=id,member='container',part='member'))
        v['decisions'].append(text('The inlet frames bear on 50 × 65 × 3 mm folded steel uprights between the lower bed rail and the top rail.','Die Einlassrahmen liegen auf gekanteten Stahlpfosten 50 × 65 × 3 mm zwischen unterem und oberem Rahmenriegel.'))

def compressor(a):
    fine={p['id']:deepcopy(p) for p in a['volumes']['50']['body']}
    for k,v in a['volumes'].items():
        if any(p['id']=='mount-sleeves' for p in v['body']):continue
        rubber=deepcopy(fine['grommets'])
        rubber['profile']=[[0,.0065],[0,.02],[.017,.02],[.022,.017],[.022,.0154],[.0295,.0154],[.0295,.0185],[.034,.0185],[.034,.0065],[0,.0065]]
        bolt=deepcopy(fine['mount-bolts'])
        bolt['profile']=[[0,0],[0,.0048],[.034,.0048],[.034,.012],[.036,.012],[.036,.0075],[.041,.0075],[.041,0]]
        sleeve=deepcopy(bolt);sleeve.update(id='mount-sleeves',profile=[[0,.0049],[0,.00645],[.034,.00645],[.034,.0049],[0,.0049]])
        plate=deepcopy(fine['base-plate'])
        sides=32
        plate['holes']=[[[round(x+.0156*math.cos(2*math.pi*j/sides),7),round(z+.0156*math.sin(2*math.pi*j/sides),7)] for j in range(sides)] for x in [-.09525,.09525] for z in [-.09525,.09525]]
        stock=[rubber,plate,sleeve,bolt];ids={'feet','grommets','mount-bolts','base-plate'}
        v['body']=[p for p in v['body'] if p['id'] not in ids]+stock
        v['thermal']=[p for p in v['thermal'] if p.get('of') not in ids]+[dict(id='th-'+p['id'],of=p['id'],region='feet') for p in stock]
        v['structure']=[p for p in v['structure'] if p['id'] not in ('contact-pad','contact-grommets','foot-frame','base-plate-member')]
        v['structure'].extend([dict(id='contact-grommets',of='grommets',member='shell',part='support',anchor='base-contact'),dict(id='base-plate-member',of='base-plate',member='shell',part='member'),dict(id='mount-sleeve-joints',of='mount-sleeves',member='shell',part='joint'),dict(id='mount-bolt-joints',of='mount-bolts',member='shell',part='joint')])
        if k=='500':
            cap=next(p for p in v['structure'] if p['id']=='top-cap-shell')
            cap['profile']=[p for i,p in enumerate(cap['profile']) if i not in (2,7)]
        v['decisions']=[d for d in v['decisions'] if not any(q in d['en'].lower() for q in ('grommet','foot','feet','base plate','base-plate','contact pad'))]
        v['decisions'].append(text('The pressure vessel bears on the welded plate. The structural load passes from each pierced ear through its elastomer mount to the support. The sleeves limit rubber compression under bolt preload.','Der Druckbehälter steht auf der angeschweißten Platte. Die Last gelangt durch jede gebohrte Lasche und ihr Elastomerlager zum Auflager. Distanzhülsen begrenzen die Gummikompression beim Vorspannen der Bolzen.'))
        v['decisions'].append(text('Four 40 mm isolators sit on a 190.5 mm square. Each has a 13 mm bore with a 12.9 mm sleeve around the mounting bolt. The 7.5 mm stamped foot plate captures the rubber waist through a 31.2 mm aperture.','Vier Schwingungsdämpfer von 40 mm Durchmesser liegen im Quadrat von 190,5 mm. In jeder 13-mm-Bohrung führt eine 12,9-mm-Hülse den Befestigungsbolzen. Das 7,5 mm dicke Fußblech greift mit einer 31,2-mm-Bohrung in die Gummitülle.'))
