"""🛞 Author separated running tyres, forks and contact anchor bindings for the floor equipment."""
from copy import deepcopy
from pathlib import Path
import json
import math
import sys

T = Path(__file__).resolve().parent
def catalog(name):
    return next(T.parents[6].glob(f'🎓*/*/*staffage/🗂*/*/{name}/🔣️.json'))

def text(en,de):
    return {'en':en,'de':de}

def fit_rims(volume):
    existing={s.get('of') for s in volume['structure']}
    for side in ('l','r'):
        rim='rear-rim-'+side
        if rim not in existing:
            volume['structure'].append(dict(id='load-rim-'+side,of=rim,member='wheel',part='member'))
    if 'caster-fork' not in existing:
        volume['structure'].append(dict(id='castor-yoke',of='caster-fork',member='caster',part='member'))

def vacuum(asset):
    if 'wheel-left-contact' in asset['structure']['supports']:
        for v in asset['volumes'].values():
            fit_rims(v)
        return
    sides = [('left',1),('right',-1)]
    renamed = {bone+'-contact':[bone+'-'+side+'-contact' for side,_ in sides] for bone in ['wheel','caster']}
    anchors = []
    for a in asset['anchors']:
        if a['id'] not in renamed:
            anchors.append(a)
            continue
        rear = a['bone']=='wheel'
        for (side,sign),id in zip(sides,renamed[a['id']]):
            de='links' if side=='left' else 'rechts'
            anchors.append({**a,'id':id,'z':sign*(.1375 if rear else .018),'footprintWidthM':.025 if rear else .012,
                            'title':text(('Rear wheel' if rear else 'Front castor wheel')+', '+side,('Hinterrad' if rear else 'Lenkrolle')+' '+de)})
    asset['anchors']=anchors
    def expand(ids):
        return [x for id in ids for x in renamed.get(id,[id])]
    asset['structure']['supports']=expand(asset['structure']['supports'])
    for pose in asset['poses']:
        pose['contacts']=expand(pose['contacts'])
    for clip in asset['clips']:
        clip['contacts']=[{**span,'anchor':id} for span in clip['contacts'] for id in expand([span['anchor']])]
    fine=deepcopy(asset['volumes']['50'])
    rear=[s for s in fine['body'] if s['id'].startswith(('rear-tyre-','rear-rim-'))]
    casters=[s for s in fine['body'] if s['bone']=='caster']
    fine_paper=asset['representations']['50']['geometry']
    for sc,v in asset['volumes'].items():
        if sc!='50':
            v['body']=[s for s in v['body'] if s['bone'] not in ('wheel','caster')]+deepcopy(rear+casters)
            v['thermal']=[s for s in v['thermal'] if s.get('region')!='wheels']+[{'id':'th-'+s['id'],'of':s['id'],'region':'wheels'} for s in rear+casters]
            for view,items in asset['representations'][sc]['geometry'].items():
                if view=='elevation':
                    if not any(i['id']=='caster-wheel' for i in items):
                        items.append({**next(i for i in fine_paper[view] if i['id']=='caster-wheel'),'penMm':.20})
                elif sc!='500':
                    items[:]=[i for i in items if i['bone']!='wheel']+[
                        {**deepcopy(i),'penMm':.18} for i in fine_paper[view] if i['id'] in ('wheel-left','wheel-right')]
                    if view=='front' and not any(i['bone']=='caster' for i in items):
                        items.append({**deepcopy(next(i for i in asset['representations']['100']['geometry']['front'] if i['id']=='caster-wheel')),'penMm':.20})
            paper=asset['representations'][sc]['decisions']
            if sc=='100':
                paper['merged']=text('Socket and cuff share one collar. The front outline groups the twin castor at this paper size. The wand telescopes are shown as one tube.',
                                     'Stutzen und Muffe teilen sich einen Kragen. Die Vorderansicht fasst die Zwillingsrolle bei dieser Druckgröße zusammen. Das Teleskoprohr erscheint als ein Rohr.')
            if sc=='200':
                paper['omitted']=text('The wheel hubs and exhaust grille are omitted.', 'Radnaben und Ausblasgitter entfallen.')
                paper['thinElements']=text('The rear tyres are 25 mm wide. The side view retains the 50 mm front castor diameter.',
                                           'Die Hinterreifen sind 25 mm breit. In der Seitenansicht misst die vordere Lenkrolle 50 mm im Durchmesser.')
            if sc=='500':
                paper['omitted']=text('Carry handle, inlet collar and wheel hubs are omitted.', 'Tragegriff, Ansaugkragen und Radnaben entfallen.')
                paper['merged']=text('The rear wheel pair reads as a single grouped sign in front and plan. The side elevation shows both axle diameters.',
                                     'Hinterräder erscheinen in Vorder- und Draufsicht als ein gemeinsames Zeichen. Die Seitenansicht zeigt beide Raddurchmesser.')
                paper['thinElements']=text('At the coarse end of the print band the rear axle uses a continuous ink silhouette. The digital model retains two separate tyres.',
                                           'Am groben Ende des Druckbereichs erscheint die Hinterachse als zusammenhängende Silhouette. Das digitale Modell behält zwei getrennte Reifen.')
            v['decisions'][0]=text(
                {'100':'Separate shell and lid profiles carry the handle and intake collar.', '200':'One canister loft carries the handle and intake collar.', '500':'The canister is a single shell with a simplified top.'}[sc]+' Two 25 mm rear tyres and two 12 mm swivel wheels share four floor contacts. The hose, wand and floor head retain their working dimensions.',
                {'100':'Separate Wannen- und Deckelprofile tragen Griff und Ansaugstutzen.', '200':'Ein Kesselkörper trägt Griff und Ansaugstutzen.', '500':'Ein vereinfachter Kesselkörper bildet die Außenhülle.'}[sc]+' Zwei 25 mm breite Hinterräder und zwei 12 mm breite Lenkrollen ergeben vier Rollkontakte. Schlauch, Saugrohr und Bodenkopf behalten ihre Arbeitsmaße.')
        v['structure']=[s for s in v['structure'] if s.get('anchor') not in renamed]
        for side,_ in sides:
            suffix=side[0]
            for bone,tyre in [('wheel','rear-tyre-'+suffix),('caster','caster-wheel-'+suffix)]:
                v['structure'].append(dict(id='support-'+bone+'-'+side,of=tyre,member=bone,part='support',anchor=bone+'-'+side+'-contact'))
        fit_rims(v)

def trolley(asset):
    v=asset['volumes']['500']
    if any(s['id']=='wheel-rear-l' for s in v['body']):
        return
    medium=asset['volumes']['200']
    removed={s['id'] for s in v['body'] if s['id'].startswith('castor-')}
    v['body']=[s for s in v['body'] if s['id'] not in removed]
    v['thermal']=[s for s in v['thermal'] if s.get('of') not in removed]
    v['structure']=[s for s in v['structure'] if s.get('of') not in removed and s.get('part')!='support']
    for a in asset['anchors']:
        if a['kind']!='contact':
            continue
        wheel=a['id'].replace('left','l').replace('right','r')
        fork=wheel.replace('wheel','castor')
        d=next(s for s in medium['body'] if s['id']==wheel)
        v['body'].append({**deepcopy(d),'profile':[[-.016,.056],[-.007,.0625],[.007,.0625],[.016,.056]]})
        v['thermal'].append(dict(id='th-'+wheel,of=wheel,region='wheels'))
        v['structure'].append(dict(id='support-'+wheel,of=wheel,member='chassis',part='support',anchor=a['id']))
        for suffix in ['o','i']:
            leg=deepcopy(next(s for s in asset['volumes']['100']['body'] if s['id']==fork.replace('castor-','castor-fork-')+'-'+suffix))
            v['body'].append(leg)
            v['thermal'].append(dict(id='th-'+leg['id'],of=leg['id'],region='frame-tube'))
            v['structure'].append(dict(id='st-'+leg['id'],of=leg['id'],member='chassis',part='member'))
        block=deepcopy(next(s for s in asset['volumes']['100']['body'] if s['id']==fork.replace('castor-','castor-block-')))
        crown=deepcopy(next(s for s in asset['volumes']['100']['body'] if s['id']==fork.replace('castor-','castor-crown-')))
        axle=deepcopy(next(s for s in asset['volumes']['50']['body'] if s['id']==fork.replace('castor-','castor-axle-')))
        x,y,z=d['origin']
        axle=dict(id=axle['id'],bone='chassis',kind='extrude',material='zinc-tube',role='detail',tone=.8,plane='xy',
                  profile=[[round(x+.005*math.cos(i*math.pi/4),7),round(y+.005*math.sin(i*math.pi/4),7)] for i in range(8)],
                  **{'from':round(z-.031,7),'to':round(z+.031,7)})
        for s in [block,crown,axle]:
            v['body'].append(s)
            v['thermal'].append(dict(id='th-'+s['id'],of=s['id'],region='frame-tube'))
            v['structure'].append(dict(id='st-'+s['id'],of=s['id'],member='chassis',part='member'))
    for marker in v['structure']:
        if not marker['id'].startswith('mass-castor-'):
            continue
        reference=next(s for s in asset['volumes']['50']['structure'] if s['id']==marker['id'])
        radius=reference['radii'][0]
        marker['kind']='polyhedron'
        del marker['radii']
        marker['vertices']=[[radius,0,0],[0,0,radius],[-radius,0,0],[0,0,-radius],[0,radius,0],[0,-radius,0]]
        marker['triangles']=[[4,(i+1)%4,i] for i in range(4)]+[[5,i,(i+1)%4] for i in range(4)]
    rep=asset['representations']['500']
    outlines=asset['representations']['50']['geometry']['elevation']
    rep['geometry']['elevation']=[i for i in rep['geometry']['elevation'] if i['id'] not in ('castor-rear','castor-front')]+[
        {**deepcopy(i),'penMm':.20} for i in outlines if i['id'].startswith(('wheel-','fork-','castor-head-','castor-plate-'))]
    rep['decisions']['merged']=text('The side view separates wheel and fork. The printed front view groups the two wheels on each side. The rail bend and upright share one line.',
                                   'Die Seitenansicht trennt Rad und Gabel. In der gedruckten Vorderansicht bilden die beiden Räder jeder Seite ein gemeinsames Zeichen. Holmbiegung und Stütze bilden eine Linie.')
    rep['decisions']['thinElements']=text('The elevation marks the 8 mm mounting plate with a 0.20 mm pen. The front and plan group the running gear at the coarse end of the print band.',
                                         'Die Seitenansicht zeichnet die 8-mm-Montageplatte mit einem 0,20-mm-Stift. Vorder- und Draufsicht fassen das Fahrwerk am groben Ende des Druckbereichs zusammen.')
    v['decisions'][0]=text('The four mesh faces and rim describe the basket. Straight rails join the uprights and handle. Four 125 mm castors retain their 32 mm tyres between separate fork cheeks.',
        'Vier Gitterflächen und der Rand bilden den Korb. Gerade Holme verbinden die Stützen mit dem Griff. Vier 125-mm-Lenkrollen behalten ihre 32 mm breiten Reifen zwischen getrennten Gabelwangen.')
    v['decisions'][2]=text('The four tyres carry the wheel contacts. Each fork carries its wheel into the chassis. The thermal model separates the polyurethane tread from the steel fork and retains the mesh wire bundles.',
        'Die vier Reifen tragen die Radkontakte. Jede Gabel leitet die Radkraft in das Gestell. Das Thermikmodell trennt die Polyurethanlaufflächen von den Stahlgabeln und behält die Drahtbündel des Korbs.')

if __name__=='__main__':
    for name,author in [('vacuum-cleaner',vacuum),('shopping-trolley',trolley)]:
        doc=json.loads(catalog(name).read_bytes())
        author(doc['asset'])
        (T/'🗑️generated'/('digital-roll-'+name+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
