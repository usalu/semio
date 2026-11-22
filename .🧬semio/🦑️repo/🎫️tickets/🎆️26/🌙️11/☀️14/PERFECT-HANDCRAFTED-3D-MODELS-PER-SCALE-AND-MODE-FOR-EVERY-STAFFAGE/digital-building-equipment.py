"""🏗️ Preserve the working stock of the balcony guard and thermal equipment."""
from copy import deepcopy
import json,math,sys
from pathlib import Path

T=Path(__file__).resolve().parent
G=T/'🗑️generated'

def catalog(name):
    return next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/'+name+'/🔣️.json'))

def lookup(parts):
    return {p['id']:p for p in parts}

def words(en,de):
    return {'en':en,'de':de}

def balcony(a):
    fine=deepcopy(a['volumes']['50'])
    stock=[s for s in fine['body'] if s['id'].startswith('railing-')]
    steel=[s for s in fine['thermal'] if s['id'].startswith('th-railing-')]
    joints=[s for s in fine['structure'] if 'railing' in s['id']]
    members=[dict(id='member-'+s['id'],of=s['id'],member='railing',part='member') for s in stock if 'balusters-' in s['id'] or '-chord-' in s['id']]
    for k,v in a['volumes'].items():
        if any(s['id']=='member-railing-balusters-l' for s in v['structure']):continue
        if k!='50':
            v['body']=[s for s in v['body'] if not s['id'].startswith('railing-')]+deepcopy(stock)
            v['thermal']=[s for s in v['thermal'] if not s['id'].startswith('th-railing-')]+deepcopy(steel)
            v['structure']=[s for s in v['structure'] if 'railing' not in s['id']]+deepcopy(joints)
            post=lookup(v['body'])['railing-post']
            if k=='500':
                post['profile']=[[-.025,-.025],[.025,-.025],[.025,.025],[-.025,.025]]
                post['holes']=[[[-.021,-.021],[-.021,.021],[.021,.021],[.021,-.021]]]
            if k=='200':
                post['profile']=post['profile'][::2]
                post['holes']=[h[::2] for h in post['holes']]
            heat_post=lookup(v['thermal'])['th-railing-post'];heat_post['profile']=deepcopy(post['profile'])
        v['structure'].extend(deepcopy(members))
        v['decisions']=[d for d in v['decisions'] if k=='50' or not any(q in d['en'].lower() for q in ('rail','infill','guard','panel'))]
        v['decisions'].append(words('Eight 12 mm vertical bars at 106.89 mm pitch weld to two pairs of 40 × 10 mm flats. The 50 mm hollow post carries a 60.3 mm tubular handrail. Its 150 × 150 × 12 mm plate bears on bedded grout and four 110 mm studs anchor it to the concrete.','Acht senkrechte 12-mm-Stäbe im Raster von 106,89 mm sind mit zwei Paar Flachstählen 40 × 10 mm verschweißt. Der 50-mm-Hohlpfosten trägt ein Handlaufrohr von 60,3 mm. Seine 150 × 150 × 12 mm große Fußplatte liegt auf Vergussmörtel. Vier 110 mm lange Anker verbinden sie mit dem Beton.'))
        if k in ('200','500'):
            rep=a['representations'][k]
            rep['geometry']['front']=[s for s in rep['geometry']['front'] if s['id']!='railing-infill']
            if k=='500':
                post_sign=deepcopy(next(p for p in a['representations']['100']['geometry']['front'] if p['id']=='railing-post'))
                post_sign['penMm']=.15
                rep['geometry']['front']=[post_sign if p['id']=='railing-post-line' else p for p in rep['geometry']['front']]
            keep=[s for s in a['representations']['100']['geometry']['front'] if 'chord' in s['id'] or 'baluster-' in s['id']]
            if k=='500':keep=[s for s in keep if 'chord' in s['id'] or abs(abs(s['points'][0][0])-.33967)<.0001]
            for s in deepcopy(keep):
                s['penMm']=.13
                rep['geometry']['front'].append(s)
            rep['decisions']['thinElements']=words('Open ink strokes represent the upper and lower flats and vertical guard bars. The 1:500 convention retains a representative bar on either side of the principal post.','Offene Linien zeigen Gurtflachstähle und Füllstäbe. Bei 1:500 steht auf jeder Seite des Hauptpfostens ein stellvertretender Füllstab.')
            for key in ('retained','omitted','merged'):
                if any(q in rep['decisions'][key]['en'].lower() for q in ('rail','infill','guard','baluster')):
                    rep['decisions'][key]=deepcopy(rep['decisions']['thinElements'])


def trim_loop(points,tolerance):
    result=deepcopy(points)
    errors=[]
    for i,(x,y) in enumerate(result):
        a,b=result[i-1],result[(i+1)%len(result)]
        dx,dy=b[0]-a[0],b[1]-a[1]
        u=max(0,min(1,((x-a[0])*dx+(y-a[1])*dy)/(dx*dx+dy*dy))) if dx or dy else 0
        errors.append(math.hypot(x-a[0]-u*dx,y-a[1]-u*dy))
    return [p for i,p in enumerate(result) if errors[i]>tolerance or i%2==0]

def radiator(a):
    fine=a['volumes']['50']
    stock=lookup(fine['body'])
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if any(s['id']=='grille-plate' for s in v['body']):continue
        retained=['panel-rear','header-rear','panel-front','header-front','fins-rear','fins-front','side-cover-left','side-cover-right','grille-rim','grille-plate']
        parts=deepcopy([stock[id] for id in retained])
        thermal=deepcopy([s for s in fine['thermal'] if s.get('of') in retained or s['id'].startswith('th-band-')])
        for s in parts+thermal:
            if s.get('kind')=='extrude' and len(s['profile'])>150:
                s['profile']=trim_loop(s['profile'],{'100':.00012,'200':.00035,'500':.0012}[k])
            if s['id']=='grille-plate' and k!='100':
                s['holes']=[trim_loop(h,.0006 if k=='200' else .001) for h in s['holes']]
        def fitting(p):
            return any(q in p['id'] for q in ('bracket','valve','connection','pipe','emitter'))
        v['body']=[s for s in v['body'] if fitting(s)]+parts
        v['thermal']=[s for s in v['thermal'] if fitting(s)]+thermal
        for part in ('panel-front','panel-rear'):
            former='member-'+part
            v['structure']=[s for s in v['structure'] if s['id']!=former and not s['id'].startswith(('member-header','member-post'))]
            v['structure'].append(dict(id=former,of=part,part='member',member='radiator'))
        if k=='500':
            group=lookup(fine['structure'])
            v['structure'].extend(deepcopy([group[i] for i in ('joint-nipple-left','joint-nipple-right')]))
        if k=='200':
            rep=a['representations'][k]
            sign=lookup(rep['geometry']['plan'])['radiator-top']
            outer=[[.0335,-.5015],[.1365,-.5015],[.1365,.5015],[.0335,.5015]]
            openings=[[[round(.035-y,6),-z] for y,z in h] for h in stock['grille-plate']['holes'] if -.08<h[0][0]<-.02]
            def path(points):return ' '.join(('M' if j==0 else 'L')+' '+str(x)+' '+str(y) for j,(x,y) in enumerate(points))+' Z'
            for key in ('x','y','w','h'):sign.pop(key)
            sign.update(kind='path',d=' '.join(path(ring) for ring in [outer,*openings]))
            rep['decisions']['retained']=words('The top plan follows the rim and the open central slot runs over the convector air passage.','Der Grundriss zeigt den Deckrahmen und die offenen mittleren Schlitzreihen über dem Konvektorschacht.')
        v['decisions']=[words('Two pressed water panels flank a pair of folded steel convectors. The 32 pierced top slots open above the air gap between the panels. Raised headers close the ends of the water channels. Brackets tie the panel assembly back to the wall.','Zwei gepresste Wasserplatten flankieren die gefalteten Konvektorbleche. Über dem Luftspalt zwischen den Platten liegen 32 ausgestanzte Deckschlitze. Die Sammelkanäle schließen die Wasserzüge ab. Konsolen verbinden die Platten mit der Wand.'),
                        words('The front temperature uses three height bands of the pressed sheet. The forces scene carries both full-height water panels, their nipples and the wall contacts.','Die vordere Temperaturfläche verwendet drei Höhenzonen des Pressblechs. Im Tragwerk liegen beide Wasserplatten, die Verbindungsnippel und die Wandauflager.')]
    v=fine
    if not any(s['id']=='member-pressed-front' for s in v['structure']):
        for part in ('panel-front','panel-rear'):
            id='member-'+part
            v['structure']=[dict(id=id,of=part,member='radiator',part='member') if s['id']==id else s for s in v['structure']]

    fittings=('connection-block','connection-nut-upper','connection-nipple','connection-nut-lower','pipe','bracket-upper','bracket-lower')
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if any(p['id']=='connection-nut-lower' for p in v['body']):continue
        def changed(id):return id.startswith(('connection-','bracket')) or id=='pipe'
        v['body']=[p for p in v['body'] if not changed(p['id'])]+deepcopy([stock[id] for id in fittings])
        v['thermal']=[p for p in v['thermal'] if not changed(p['id'].removeprefix('th-'))]+deepcopy([p for p in fine['thermal'] if p.get('of') in fittings])
        v['structure']=[p for p in v['structure'] if 'member-bracket' not in p['id']]+deepcopy([p for p in fine['structure'] if p['id'].startswith('member-bracket')])
        v['decisions'].append(words('The two 15 mm pipes on 50 mm centres enter the brass connection block through separate union nuts. The four wall clips are folded stock.','Die beiden 15-mm-Rohre im Achsabstand von 50 mm sind mit getrennten Überwurfmuttern am Messingblock befestigt. Die vier Wandhalter sind gekantete Blechprofile.'))

def pump(a):
    fine=a['volumes']['50'];mid=a['volumes']['100']
    stock=lookup(fine['body']); mech=lookup(mid['structure'])
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if any(p['id']=='mounted-fan-motor' for p in v['structure']):continue
        names=['bell-mouth','fan-motor','fan-hub','fan-blades','motor-bracket','motor-plate','side-louvres','guard-rim','guard-ring-1','guard-ring-2','guard-ring-3','guard-ring-4','guard-ring-5','guard-spokes','guard-cap']
        parts=deepcopy([stock[id] for id in names])
        for p in parts:
            if p['id']=='fan-blades':
                ix={'100':[0,1,2,3,4,6,7],'200':[0,1,3,5,7],'500':[0,2,5,7]}[k]
                p['sections']=[p['sections'][i] for i in ix]
            if k=='500' and p['id']=='fan-motor':p['profile']=[p['profile'][i] for i in (0,1,3,4)]
        rem={'guard-frame','fan-guard','fan-hub','fan-disc','side-louvres','side-louvre-frame-top','side-louvre-frame-bottom','fan-drive',*names}
        v['body']=[p for p in v['body'] if p['id'] not in rem]+parts
        th=[p for p in fine['thermal'] if p.get('of') in names]
        v['thermal']=[p for p in v['thermal'] if p.get('of') not in rem]+deepcopy(th)
        v['structure']=[p for p in v['structure'] if 'motor-bracket' not in p['id']]
        for id in ('motor-bracket-bars','compressor'):
            if not any(p['id']==id for p in v['structure']):
                v['structure'].append(deepcopy(mech[id]))
        v['structure'].extend([dict(id='motor-bracket-plate',of='motor-plate',member='unit',part='member'),dict(id='mounted-fan-motor',of='fan-motor',member='unit',part='member')])
        rep=a['representations'][k]
        shapes=rep['geometry']['front']
        keep=deepcopy([p for p in a['representations']['100']['geometry']['front'] if p['id'].startswith('fan-blade-')])
        if k=='500':
            keep.extend(deepcopy([p for p in a['representations']['100']['geometry']['front'] if p['id']=='fan-hub']))
        if k=='500':
            for p in keep:p['role']='body'
        if k!='100':shapes.extend(keep)
        edge=rep['geometry']['elevation']
        if not any(p['id']=='fan-guard' for p in edge):
            curve=deepcopy(lookup(a['representations']['50']['geometry']['elevation'])['fan-guard'])
            points=curve['points']
            curve.update(points=points[1:-1:2] if k=='200' else [points[i] for i in (1,5,9,13,17)],closed=False,filled=False,penMm=.15)
            edge.append(curve)

        rep['decisions']['retained']=words('The front elevation shows the swept three-blade impeller and its central boss within the circular guard.','Die Vorderansicht zeigt das geschwungene Dreiblatt-Laufrad und die zentrale Nabe im runden Schutzgitter.')
        v['decisions']=[words('The three swept fan blades sit on the motor hub inside a bell mouth, behind five circular guard wires and radial spokes. Inclined louvres extend along the side intake. The fan bracket and motor plate carry the shaft from the chassis.','Drei geschwungene Lüfterflügel sitzen auf der Motornabe im Einlaufring hinter fünf Schutzringen mit radialen Speichen. Geneigte Lamellen schützen den seitlichen Einlass. Haltebügel und Motorplatte stützen die Welle am Rahmen.'),
                        words('The outlet, exposed rotor, inlet fins and wire guard retain their temperature surfaces. The structural scene retains the compressor and the motor mounting.','Auslass, Rotor, Einlasslamellen und Schutzgitter behalten ihre Temperaturflächen. Im Tragwerk bleiben Verdichter und Motorlagerung erhalten.')]

    for v in a['volumes'].values():
        if not any(p['id']=='th-discharge-heating' for p in v['thermal']):continue
        v['thermal']=[p for p in v['thermal'] if p['id'] not in ('th-discharge-heating','th-discharge-cooling','th-guard-cap','th-guard-spokes')]
        b=lookup(v['body'])
        for id in ('guard-cap','guard-spokes'):
            exposed=deepcopy(b[id]);exposed.update(id='th-'+id,region='fan-guard',tags=['discharge-heating','discharge-cooling'])
            v['thermal'].append(exposed)
        v['decisions'].append(words('The outlet anchor lies on the centre cap; its spokes mark the discharge aperture without covering the rotor with an opaque temperature disc.','Der Ausblasanker liegt auf der Mittelkappe. Die Speichen markieren den Austrittsquerschnitt und lassen den Rotor in der Temperaturansicht sichtbar.'))

    masses=lookup(fine['structure'])
    for k in ('100','200','500'):
        v=a['volumes'][k]
        if k=='500':
            v['structure']=[deepcopy(masses[p['id']]) if p['id'] in ('mass-unit','mass-lid') else p for p in v['structure']]
        else:
            v['structure']=[deepcopy(masses['compressor']) if p['id']=='compressor' else p for p in v['structure']]

    if not any(p['id']=='mounted-fan-motor' for p in fine['structure']):
        fine['structure'].append(dict(id='mounted-fan-motor',of='fan-motor',member='unit',part='member'))
    coarse=lookup(a['volumes']['500']['body'])['coil']
    mid_coil=lookup(a['volumes']['200']['body'])['coil']
    coarse['profile']=deepcopy(mid_coil['profile'])
    for k in ('200','500'):
        desc=a['representations'][k]['decisions']
        desc['silhouette']=words('Two feet support the casing. Its front opening shows the three swept blades and their hub inside the circular bell mouth.','Zwei Füße tragen das Gehäuse. In der runden Einlauföffnung der Front sind drei geschwungene Flügel und ihre Nabe zu sehen.')
        desc['omitted']=words('The printed front groups the five individual guard wires and eight radial spokes at the circular outlet boundary.','Die gedruckte Frontansicht fasst fünf Schutzringe und acht Radialspeichen an der runden Austrittskontur zusammen.')

def exchanger(a):
    fine=deepcopy(a['volumes']['50']);stock=lookup(fine['body']);ref=lookup(fine['structure'])
    for k,v in a['volumes'].items():
        if any(s['id']=='fin-pack-end-sheets' for s in v['body']):continue
        sides={'50':12,'100':10,'200':8,'500':6}[k]
        holes=[]
        for row in range(12):
            for x,y in [(row*.04+.01,.01732),(row*.04+.03,-.01732)]:
                holes.append([[round(x+.00797*math.cos(-2*math.pi*i/sides),6),round(y+.00797*math.sin(-2*math.pi*i/sides),6)] for i in range(sides)])
        foil={'id':'fin-pack','kind':'extrude','bone':'fins','material':'aluminium-fin','role':'body','tone':.78,'plane':'xy',
              'from':-.1996,'to':-.1994,'profile':[[0,-.03464],[.48,-.03464],[.48,.03464],[0,.03464]],'holes':holes,
              'repeat':{'count':134,'translation':[0,0,.003]},'slender':True}
        stack={'id':'fin-pack','kind':'box','bone':'fins','material':'aluminium-fin','role':'body','tone':.78,'size':[.48,.06928,.0002],'transform':{'translation':[.24,0,-.1965]},'repeat':{'count':132,'translation':[0,0,.003]},'slender':True}
        terminal=deepcopy(foil);terminal['id']='fin-pack-end-sheets';terminal['repeat']={'count':2,'translation':[0,0,.399]}
        v['body']=[s for s in v['body'] if s['id'] not in ('coil-block','fin-pack','fin-edges-ahead','fin-edges-behind')]+[stack,terminal]
        v['thermal']=[s for s in v['thermal'] if s.get('region')!='fins']+[dict(id='th-'+part['id'],of=part['id'],region='fins') for part in (stack,terminal)]
        step,count={'50':(27,5),'100':(36,4),'200':(54,3),'500':(108,2)}[k]
        mechanical=deepcopy(foil)
        mechanical.update(id='member-fin-plates',part='member',member='fins',repeat={'count':count,'translation':[0,0,.003*step]})
        v['structure']=[s for s in v['structure'] if s['id']!='member-fin-plates']+[mechanical]
        if k in ('200','500'):
            v['body']=[s for s in v['body'] if s['id']!='header-block']
            v['thermal']=[s for s in v['thermal'] if s['id'] not in ('th-tube-bank','th-header-block')]
            carry=lookup(a['volumes']['100']['body'])
            parts=['tube-row-entering','tube-row-leaving','header-return','header-supply','hairpins']
            v['body'].extend(deepcopy([carry[id] for id in parts]))
            v['thermal'].extend([dict(id='th-'+id,of=id,region='headers' if id.startswith('header') else 'tubes') for id in parts])
            v['structure']=[s for s in v['structure'] if s['id']!='member-tube-bars']+deepcopy([ref['member-tube-row-entering'],ref['member-tube-row-leaving']])
            if k=='500':
                v['body']=[p for p in v['body'] if p['id']!='casing-bottom'];v['thermal']=[p for p in v['thermal'] if p['id']!='th-casing-bottom'];v['structure']=[p for p in v['structure'] if p['id']!='member-casing-bottom']
                cases=['casing-bottom','casing-top','tube-sheet-left','tube-sheet-right']
                v['body'].extend(deepcopy([carry[id] for id in cases]))
                v['thermal'].extend([dict(id='th-'+id,of=id,region='casing') for id in cases])
                v['structure'].extend([dict(id='member-'+id,of=id,member='frame',part='member') for id in cases])
        if k!='50':v['body']=[deepcopy(stock[p['id']]) if p['id'] in ('header-supply','header-return') else p for p in v['body']]
        for id in ('header-return','header-supply'):
            if not any(s.get('of')==id for s in v['structure']):
                v['structure'].append(dict(id='member-'+id,of=id,member='tubes',part='member'))
        v['decisions']=[words('The 400 mm coil width contains 134 aluminium fins, each 0.2 mm thick at 3 mm pitch. The air gaps extend through the 69.28 mm coil depth. The foils surround 24 copper tubes at 40 mm staggered pitch, supplied by the two headers. The end sheets and channels carry the coil.','Über die 400 mm Breite verteilen sich 134 Aluminiumlamellen mit 0,2 mm Stärke und 3 mm Teilung. Die Luftspalte reichen durch die 69,28 mm tiefe Batterie. Die Bleche umgeben 24 versetzt angeordnete Kupferrohre im Raster von 40 mm. Zwei Sammler versorgen die Rohre, Rohrböden und Randprofile tragen die Batterie.'),
                        words('The mechanical cutaway retains both complete tube rows, the supply and return headers, and selected full-size fins so the tube-to-fin contacts remain visible.','Der Tragwerksschnitt enthält beide vollständigen Rohrreihen, Vor- und Rücklaufsammler sowie ausgewählte Lamellen in Originalstärke, sodass ihre Rohrkontakte erkennbar bleiben.')]
    mid=lookup(a['volumes']['100']['body'])
    coarse=a['volumes']['200']
    if any(p['id']=='end-plate-left' for p in coarse['body']):
        old=('casing-bottom','casing-top','end-plate-left','end-plate-right')
        cases=('casing-bottom','casing-top','tube-sheet-left','tube-sheet-right')
        coarse['body']=[p for p in coarse['body'] if p['id'] not in old]+deepcopy([mid[id] for id in cases])
        coarse['thermal']=[p for p in coarse['thermal'] if p['id'].removeprefix('th-') not in old]+[dict(id='th-'+id,of=id,region='casing') for id in cases]
        coarse['structure']=[p for p in coarse['structure'] if p['id'].removeprefix('member-') not in old]+[dict(id='member-'+id,of=id,member='frame',part='member') for id in cases]
AUTHORS={'cantilever-balcony':balcony,'panel-radiator':radiator,'heat-pump-outdoor-unit':pump,'finned-heat-exchanger':exchanger}
from runpy import run_path
plant=run_path(str(T/'digital-building-plant.py'))
AUTHORS.update({'diesel-generator':plant['generator'],'scroll-compressor':plant['compressor']})
components=run_path(str(T/'digital-component-author.py'))
AUTHORS.update({'etics-brick-wall-12':components['etics'],'etics-brick-wall-30':components['etics'],'external-venetian-blind':components['blind'],'solar-thermal-collector':components['collector']})
joinery=run_path(str(T/'digital-component-joinery.py'))
AUTHORS.update({'glass-balustrade':joinery['balustrade'],'sliding-door':joinery['sliding'],'solid-brick-wall-1960':joinery['brickwork'],'masonry-wall-with-lintel':joinery['lintelwall'],'tied-canopy':joinery['canopy'],'sectional-garage-door':joinery['garage'],'curtain-wall-bay':joinery['insulating_glazing'],'tilt-window':joinery['insulating_glazing']})
repairs=run_path(str(T/'digital-projection-plant.py'))
AUTHORS['gas-boiler']=repairs['boiler']
if __name__=='__main__':
    for name in sys.argv[1:]:
        doc=json.loads(catalog(name).read_bytes());AUTHORS[name](doc['asset'])
        (G/('building-'+name+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
