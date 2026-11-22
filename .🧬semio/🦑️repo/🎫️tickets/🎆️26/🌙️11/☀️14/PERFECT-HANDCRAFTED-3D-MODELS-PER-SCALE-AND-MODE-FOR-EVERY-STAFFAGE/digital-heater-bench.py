"""🪑 Restore the fan heater's airflow, carrying grip and feet, and the bench's load-bearing oak boards."""
from copy import deepcopy
import json
import math
from pathlib import Path
import sys

T=Path(__file__).resolve().parent

def catalog(name):
    return next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/'+name+'/🔣️.json'))

def by_id(parts):
    return {part['id']:part for part in parts}

def replace(parts,removed,replacements):
    return [p for p in parts if p['id'] not in removed]+deepcopy(replacements)

def heater(a):
    fine={mode:by_id(a['volumes']['50'][mode]) for mode in ('body','thermal','structure')}
    mid={mode:deepcopy(by_id(a['volumes']['100'][mode])) for mode in fine}
    for k in ('100','200','500'):
        v=a['volumes'][k]
        b=by_id(v['body'])
        if 'grille-frame' in b:
            continue
        aperture=deepcopy(fine['body']['grille-frame']['holes'])
        if k=='100':
            for id in ('shell-block','front-cap','rear-cap'):
                b[id]['holes']=deepcopy(aperture)
        else:
            b['block']['holes']=deepcopy(aperture)
            b['block']['from']=-.092
            casing=by_id(v['thermal'])['th-casing']
            casing['holes']=deepcopy(aperture)
            casing['from']=-.092
            by_id(v['thermal'])['th-top-cover']['from']=-.092
        carry=deepcopy(mid['body']['handle'])
        if k!='100':
            for section in carry['sections']:
                if section['at'][1] < .27:
                    section['at'][1]=.27
        fine_stock=['grille-frame','louvres','intake-panel','fan-hub','fan-blades','motor-bridge','motor-stack','fan-shroud','heater-box','coil-stage-1','coil-stage-2','heater-brackets']
        body_parts=[fine['body'][i] for i in fine_stock]+[carry,mid['body']['foot-back'],mid['body']['foot-front']]
        v['body']=replace(v['body'],{'grille','grille-plate','louvres','handle','handle-block','intake-plate','base','foot-back','foot-front'},body_parts)
        thermal_parts=[fine['thermal'][i] for i in ('th-grille-frame','th-louvres','th-warm-air','th-intake-panel')]+[mid['thermal'][i] for i in ('th-handle','th-foot-back','th-foot-front')]
        air=next(p for p in thermal_parts if p['id']=='th-warm-air')
        air=deepcopy(air)
        air['origin'][0]=.058
        air['profile']=[[0,0],[0,.049],[.001,.05],[.003,.05],[.004,.049],[.004,0]]
        thermal_parts=[air if p['id']=='th-warm-air' else p for p in thermal_parts]
        if k=='100':
            th=by_id(v['thermal']);th['th-front-cap']['sectionFaces']=['-u'];th['th-rear-cap']['sectionFaces']=['+u']
            v['thermal']=[p for p in v['thermal'] if p['id']!='th-shell-block']
            for id in ('th-shell-left','th-shell-right'):
                outer=deepcopy(fine['thermal'][id]);outer['from']=-.077;outer['to']=.075;v['thermal'].append(outer)
        if k=='200':
            geo=a['representations'][k]['geometry'];paper=a['representations']['100']['geometry']
            for view in ('elevation','front'):
                block=geo[view][0];pts=block['points'];block['points']=[p for p in pts if .016<=p[1]<=.272 and (view!='front' or abs(p[0])>=.092) and (view!='elevation' or abs(p[0])>=.09)]
                block['regions']=['casing','top-cover','intake','control','feet']
                cx=.058 if view=='elevation' else .075
                floor=[[round(x,4),y] for x,y in [(-cx-.016,.016),(-cx-.016,0),(-cx+.016,0),(-cx+.016,.016),(cx-.016,.016),(cx-.016,0),(cx+.016,0),(cx+.016,.016)]]
                if view=='elevation':block['points']=[[-.092,.016],*floor,[.09,.016],[.09,.272],[-.092,.272]]
                else:block['points']=[[-.092,.016],*floor,*[p for p in block['points'] if p!=[-.092,.016]]]
                geo[view].extend(deepcopy([p for p in paper[view] if p['id']=='handle']))
        v['thermal']=replace(v['thermal'],{'th-grille','th-grille-plate','th-louvres','th-warm-air','th-intake','th-intake-plate','th-handle','th-feet','th-foot-back','th-foot-front'},thermal_parts)
        supports=[dict(id='support-foot-'+side,of='foot-'+side,member='body',part='support',anchor='foot-'+side) for side in ('back','front')]
        front=dict(id='member-cassette',of='grille-frame',member='body',part='member')
        rear=dict(id='member-intake',of='intake-panel',member='body',part='member')
        v['structure']=replace(v['structure'],{'member-handle','support-foot-back','support-foot-front'},[dict(id='member-handle',of='handle',member='body',part='member'),front,rear,*supports])
        v['decisions']=[{'en':'The moulded housing surrounds an open air passage. Eight inclined steel louvres sit in the outlet cassette, opposite seven rear intake slots. The fan hub and pitched blades sit on the shaft behind the grille. The carrying bar clears the cover, and four rubber pads support the case.','de':'Das Formgehäuse umschließt einen offenen Luftkanal. Acht geneigte Stahllamellen sitzen im Auslassrahmen gegenüber sieben Ansaugschlitzen. Hinter dem Gitter sitzen Nabe und angestellte Lüfterflügel auf der Welle. Der Tragebügel lässt Platz über der Haube, vier Gummifüße tragen das Gehäuse.'},
                        {'en':'The thermal surface follows the slats, intake, feet and grip. The air emitter remains at the outlet and the structural cassette, intake panel and feet follow their mounted parts.','de':'Die thermische Oberfläche folgt Lamellen, Ansauggitter, Füßen und Griff. Die Warmluftquelle bleibt am Auslass, die Tragwerksansicht zeigt Rahmen, Ansauggitter und Füße in ihren Einbaulagen.'}]

    for k in ('100','200','500'):
        v=a['volumes'][k]
        control_parts=['dial','dial-grip','mode-switch']
        thermal_parts=['th-'+part for part in control_parts]
        v['body']=replace(v['body'],set(control_parts),[fine['body'][part] for part in control_parts])
        v['thermal']=replace(v['thermal'],{'th-control',*thermal_parts},[fine['thermal'][part] for part in thermal_parts])
        if len(v['decisions'])==2:
            v['decisions'].append({'en':'The 44 mm dial and the separate rocker protrude through the control side. Their temperature covers the exposed plastic.','de':'Der 44-mm-Drehknopf und die separate Wippe ragen aus der Bedienseite heraus. Ihre thermische Oberfläche erfasst den freiliegenden Kunststoff.'})

def simplify(loop,tolerance):
    result=deepcopy(loop)
    while len(result)>4:
        errors=[]
        for i,(x,y) in enumerate(result):
            ax,ay=result[i-1];bx,by=result[(i+1)%len(result)]
            dx,dy=bx-ax,by-ay
            u=max(0,min(1,((x-ax)*dx+(y-ay)*dy)/(dx*dx+dy*dy)))
            errors.append(math.hypot(x-ax-u*dx,y-ay-u*dy))
        if min(errors)>tolerance:break
        result.pop(errors.index(min(errors)))
    return result

def ink_bands(shapes):
    result={key:deepcopy(value) for key,value in shapes[0].items() if key not in ('x','y','w','h','points','closed','penMm')}
    paths=[]
    for part in shapes:
        if part['kind']=='rect':
            x,y,w,h=(part[key] for key in ('x','y','w','h'))
            points=[(x,y),(x+w,y),(x+w,y+h),(x,y+h)]
        else:
            points=part['points']
        paths.append(' '.join(('M' if i==0 else 'L')+' '+str(round(x,6))+' '+str(round(y,6)) for i,(x,y) in enumerate(points))+' Z')
    result.update(id='three-back-slats',kind='path',d=' '.join(paths))
    return result

def bench(a):
    medium={mode:by_id(a['volumes']['100'][mode]) for mode in ('body','thermal','structure')}
    frames=by_id(a['volumes']['200']['body'])
    for k in ('200','500'):
        v=a['volumes'][k]
        if 'back-slat-3' in by_id(v['body']):
            continue
        for mode,ids in [('body',['back-slat-']),('thermal',['th-back-slat-']),('structure',['m-back-slat-','bolt-back-'])]:
            parts=[p for id,p in medium[mode].items() if any(id.startswith(part) for part in ids)]
            remove={p['id'] for p in v[mode] if 'back-slab' in p['id'] or 'back-board' in p['id'] or p['id'].startswith('bolt-back-')}
            if k=='500' and mode=='body':
                remove.update(['l-frame','r-frame'])
                parts.extend([frames['l-frame'],frames['r-frame']])
            v[mode]=replace(v[mode],remove,parts)
        if k=='500':
            for side in ('l','r'):
                frame=by_id(v['body'])[side+'-frame']
                frame['profile']=simplify(frame['profile'],.003)
                frame['holes']=[simplify(h,.003) for h in frame['holes']]
            fine_back=by_id(a['volumes']['50']['body'])
            v['body']=[deepcopy(fine_back[p['id']]) if p['id'].startswith('back-slat-') else p for p in v['body']]
            joints=by_id(a['volumes']['50']['structure'])
            v['structure']=[deepcopy(joints[p['id']]) if p['id'].startswith('bolt-back-') else p for p in v['structure']]
        rep=a['representations'][k]
        for view,shapes in rep['geometry'].items():
            slim=[p for p in a['representations']['100']['geometry'][view] if p['bone']=='backrest']
            keep=[p for p in shapes if p['bone']!='backrest' and not(k=='500' and p['bone'] in ('end-left','end-right'))]
            if k=='500':
                keep+=[deepcopy(p) for p in a['representations']['200']['geometry'][view] if p['bone'] in ('end-left','end-right')]
            rep['geometry'][view]=keep+[ink_bands(slim)]
        rep['decisions']['silhouette']={'en':'Separate lumbar, middle and upper slats span the two cast frames above the continuous seat. The side view shows the rake and the open arms.','de':'Lenden-, Mittel- und Oberlatte überspannen die Gussgestelle über der durchgehenden Sitzfläche. Die Seitenansicht zeigt die Neigung und die offenen Armlehnen.'}
        v['decisions']=[{'en':'Three separate raked oak slats span the two cast end frames. Their 90 and 32 mm gaps clear the armrest and divide the back. The one-piece seating surface retains its scale-specific section.','de':'Drei einzeln geneigte Eichenlatten überspannen die beiden Gussgestelle. Ihre 90 und 32 mm breiten Abstände lassen Platz für die Armlehne und gliedern den Rücken. Die durchgehende Sitzfläche behält ihren maßstabsabhängigen Querschnitt.'},
                        {'en':'Each slat carries its own timber temperature surface and bears on the cast posts through two bolts. The iron frames retain the open arm and leg bracing.','de':'Jede Latte besitzt eine eigene thermische Holzoberfläche und liegt mit zwei Bolzen an den Gusspfosten. Die Eisengestelle behalten die offenen Arm- und Beinstreben.'}]

if __name__=='__main__':
    for name,author in [('electric-fan-heater',heater),('park-bench',bench)]:
        doc=json.loads(catalog(name).read_bytes())
        author(doc['asset'])
        (T/'🗑️generated'/('digital-art-'+name+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
