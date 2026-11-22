"""💻 Author the laptop's four desk contacts and the underside clearance in each digital model."""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
CAT = next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/laptop/🔣️.json'))
PADS = [(f'foot-{end}-{side}', x, z, f'{end} {side}', f'{de_end} {de_side}')
        for end, de_end, x in [('rear','hinten',.020),('front','vorne',.198)]
        for side, de_side, z in [('left','links',.134),('right','rechts',-.134)]]

def text(en, de):
    return {'en': en, 'de': de}

def feet(scale):
    profile = [[0,.0052],[.0006,.006],[.0024,.006]]
    if scale == '500':
        profile = [[0,.0052],[.0024,.006]]
    return [dict(id=name, kind='revolve', bone='base', material='rubber-foot', role='ground', tone=.15,
                 profile=profile, axis=[0,1,0], origin=[x,0,z]) for name,x,z,_,_ in PADS]

def author(asset):
    if asset['anchors'][0]['id'] == PADS[0][0]:
        return
    original, *other = asset['anchors']
    assert original['id'] == 'feet-contact'
    names = [p[0] for p in PADS]
    asset['anchors'] = [{**original, 'id':name,'x':x,'z':z, 'footprintLengthM':.0104,'footprintWidthM':.0104,
                         'title':text(f'Rubber foot, {en}', f'Gummifuß {de}')} for name,x,z,en,de in PADS] + other
    asset['structure']['supports'] = names
    area = next(r['areaM2'] for r in asset['thermal']['regions'] if r['id']=='underside')
    for pose in asset['poses']:
        assert pose['contacts'] == ['feet-contact']
        pose['contacts'] = names
        pose['thermal']['regions']['underside']['contact']['fraction'] = round(4*math.pi*.0052**2/area, 7)
    for clip in asset['clips']:
        interval, = clip['contacts']
        clip['contacts'] = [{**interval, 'anchor':name} for name in names]
    for scale, v in asset['volumes'].items():
        v['body'] = feet(scale) + [s for s in v['body'] if s['id'] not in ('foot-l','foot-r')]
        v['thermal'] = [dict(id='th-'+name,of=name,region='underside') for name in names] + [s for s in v['thermal'] if s.get('of') not in ('foot-l','foot-r')]
        v['structure'] = [dict(id='support-'+name,of=name,member='base',part='support',anchor=name) for name in names] + [s for s in v['structure'] if s['id'] != 'support-feet' and s.get('of') not in ('foot-l','foot-r')]
        body = {s['id']:s for s in v['body']}
        thermal = {s['id']:s for s in v['thermal']}
        structure = {s['id']:s for s in v['structure']}
        if scale in ('100','200'):
            shell = body['base']['sections']
            for sec in shell:
                sec['at'][1] = round(.002 + .0105*sec['at'][1]/.0125, 7)
        if scale == '100':
            shell = thermal['th-underside']['sections']
            for sec in shell:
                sec['at'][1] = round(.002 + .0065*sec['at'][1]/.0085, 7)
        if scale == '200':
            thermal['th-underside']['transform']['translation'][1] = .0026
            structure['chassis-frame']['from'] = .0044
            floor = next(part for part in asset['volumes']['50']['structure'] if part['id']=='tray-floor')
            v['structure'].append(dict(floor))
            structure['mass-base']['radii'][1] = .0045
        if scale == '500':
            body['base']['size'][1] = .0105
            body['base']['transform']['translation'][1] = .00725
            for key,y in [('th-underside',.004625),('th-deck',.009875)]:
                thermal[key]['size'][1]=.00525
                thermal[key]['transform']['translation'][1]=y
        if 'tray-floor' in structure:
            structure['tray-floor']['from']=.002
        if scale in ('100','200'):
            geometry = asset['representations'][scale]['geometry']
            for view in ['elevation','front']:
                base = next(p for p in geometry[view] if p['bone']=='base' and p['kind']=='poly')
                top = max(y for _,y in base['points'])
                base['points'] = [[x, round(.002+(top-.002)*y/top,7)] for x,y in base['points']]
                feet2d = [p for p in asset['representations']['50']['geometry'][view] if p['id'].startswith('foot-')]
                geometry[view][:0] = [{**p,'penMm':.21} for p in feet2d]
            paper = asset['representations'][scale]['decisions']
            paper['thinElements'] = text('The 10.5 mm chassis and its 2 mm desk clearance use a separate 0.21 mm outline on the feet.',
                                         'Unter dem 10,5 mm starken Chassis bleiben 2 mm Luft. Die Füße haben eine eigene 0,21-mm-Kontur.')
            if scale=='100':
                paper['silhouette']['en']=paper['silhouette']['en'].replace('its bilge standing on the desk','its bilge above four rubber feet')
                paper['silhouette']['de']=paper['silhouette']['de'].replace('der auf dem Tisch steht','der auf vier Gummifüßen steht')
                paper['merged']=text('The hinge barrel joins the lid profile, the lit panel joins the glass and the keys form one field.',
                                     'Die Scharnierwalze geht im Deckelprofil auf, das leuchtende Panel im Glas und die Tasten in einem Feld.')
            else:
                paper['omitted']=text('Bilge, key field, touchpad, glass and ports.', 'Kimm, Tastenfeld, Touchpad, Glas und Buchsen.')
                paper['retained']=text('The four feet carry a 10.5 mm base. The 6.5 mm lid rotates at its rear hinge.',
                                       'Vier Füße tragen ein 10,5 mm starkes Chassis. Der 6,5 mm starke Deckel schwenkt am hinteren Scharnier.')
                paper['styles']=text('Chassis tone 0.55 and lid tone 0.5; 0.20 mm lid outline and 0.21 mm rubber foot outline.',
                                     'Chassiston 0,55 und Deckelton 0,5. Die Deckelkontur misst 0,20 mm und die Fußkontur 0,21 mm.')
        if scale=='50':
            for lang,old,new in [('en',', the contact pad of 190 x 280 mm lies under the feet','. Four separate rubber soles bear on the desk'),('de',', das Kontaktpolster von 190 x 280 mm liegt unter den Füßen','. Vier einzelne Gummisohlen stehen auf dem Tisch')]:
                v['decisions'][2][lang]=v['decisions'][2][lang].replace(old,new)
        else:
            desc = {
                '100': ('A chamfered hull and hinged glazed lid retain the keyboard field and touchpad.', 'Der abgeschrägte Rumpf und der verglaste Klappdeckel behalten Tastenfeld und Touchpad.'),
                '200': ('A chamfered base and rounded lid retain the keyboard field and display.', 'Die abgeschrägte Basis und der gerundete Deckel behalten Tastenfeld und Display.'),
                '500': ('Minimal: the base and lid are rectangular plates, with a dark inset marking the display face.', 'Minimal: Basis und Deckel sind rechteckige Platten. Eine dunkle Einlage kennzeichnet die Bildschirmseite.')
            }[scale]
            v['decisions'][0]=text(desc[0]+' The 10.5 mm chassis rests 2 mm above the desk on four rubber feet.',desc[1]+' Das 10,5 mm starke Chassis steht auf vier Gummifüßen 2 mm über dem Tisch.')
            v['decisions'][2]=text('The floor plate carries the hinge and rests on four separate rubber soles. The base and lid each retain their mass centre.',
                                    'Die Bodenplatte trägt das Scharnier und ruht auf vier einzelnen Gummisohlen. Basis und Deckel behalten jeweils ihren Massenschwerpunkt.')
        for lang,add in [('en',' The four rubber soles conduct heat across separate desk contacts.'),('de',' Die vier Gummisohlen leiten Wärme über einzelne Kontaktflächen in den Tisch.')]:
            v['decisions'][1][lang] += add

if __name__ == '__main__':
    document=json.loads(CAT.read_bytes())
    author(document['asset'])
    (T/'🗑️generated'/'digital-feet-laptop.json').write_text(json.dumps(document,ensure_ascii=False))
