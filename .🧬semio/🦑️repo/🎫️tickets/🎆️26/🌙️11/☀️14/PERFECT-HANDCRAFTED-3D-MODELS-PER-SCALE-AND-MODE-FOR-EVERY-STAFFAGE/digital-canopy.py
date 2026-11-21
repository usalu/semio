"""☂️ Hand-author the catenary cut of each gore and the two faces of the 0.5 mm polyester cloth."""
from copy import deepcopy
from pathlib import Path
from runpy import run_path
import json
import math

T=Path(__file__).resolve().parent
NOTE={'en':'Eight gores of 0.5 mm polyester bear on the 16 mm ribs. The perimeter has a shallow 12 mm inward cut between neighbouring tips. The fine model follows the curved cut, the intermediate scales simplify the hem, and the coarsest scale joins its tips directly. Both thermal faces follow the same cloth and fold bones.', 'de':'Acht Bahnen aus 0,5 mm starkem Polyester liegen auf den 16-mm-Speichen. Zwischen benachbarten Spitzen ist der Saum 12 mm nach innen geschnitten. Das feine Modell zeichnet den Bogen nach, die mittleren Maßstäbe vereinfachen ihn, der gröbste verbindet die Spitzen direkt. Beide thermischen Oberflächen folgen demselben Stoff und denselben Faltknochen.'}

def cut(triangle,divisions):
    crown,first,last=triangle[0],triangle[1],triangle[-1];pts=[crown,first]
    for j in range(1,divisions):
        f=j/divisions
        q=[(1-f)*x+f*y for x,y in zip(first,last)];d=[x-y for x,y in zip(crown,q)];length=math.hypot(*d);sag=.012*4*f*(1-f)
        pts.append([round(x+sag*y/length,7) for x,y in zip(q,d)])
    return pts+[last]

def author(volumes):
    if any(p["bone"].startswith("pleat-") for p in volumes["50"]["body"]):return volumes
    for d,v in volumes.items():
        top={};bottom={}
        for p in v['body']:
            if not p['id'].startswith('panel-'):continue
            p.update(profile=cut(p['profile'],{'50':8,'100':4,'200':2,'500':1}[d]),**{'from':.008,'to':.0085})
            side=p['id'][6:];up=deepcopy(p);up.update(id='th-panel-'+side,region='canopy-top',layer='cloth-top',sectionFaces=['-v'],**{'from':.00825});top[up['id']]=up
            lo=deepcopy(p);lo.update(id='th-underside-'+side,region='canopy-underside',layer='cloth-under',sectionFaces=['+v'],to=.00825);bottom[lo['id']]=lo
        if d in ('200','500'):
            frame=[deepcopy(s) for s in volumes['100']['body'] if s['id'].endswith('-rod') or s['id'].startswith('stretch-')]
            for bar in frame:
                bar.pop('cornerRadius',None)
                if bar['kind']=='capsule':
                    bar['r0']=bar['r1']=.0055
                    if d=='500':
                        span=bar['p1'][0]-bar['p0'][0];mid=(bar['p1'][0]+bar['p0'][0])/2
                        bar.update(kind='box',size=[span,.011,.011],transform={'translation':[mid,0,0]})
                        for key in ('p0','p1','r0','r1'):bar.pop(key)
            names={bar['id'] for bar in frame}
            v['body']=[s for s in v['body'] if s['id'] not in names]+frame
            for s in v['structure']:
                if s['id'].startswith(('member-rib-','member-stretch-')):
                    ref=s['id'][7:]+('-rod' if s['id'].startswith('member-rib-') else '')
                    if ref in names:
                        item={k:s[k] for k in ('id','member','part')};s.clear();s.update(item,of=ref)
            v['thermal']=[s for s in v['thermal'] if s['id'].removeprefix('th-') not in names]
            v['thermal']+=[{'id':'th-'+bar['id'],'of':bar['id'],'region':'frame-alu'} for bar in frame]
        v['thermal']=[top.get(s['id'],bottom.get(s['id'],s)) for s in v['thermal']]
        v['decisions']=[n for n in v['decisions'] if n!=NOTE];v['decisions'].append(NOTE)
        body=v['decisions'][0]
        for thickness in (8,15,30,75):
            body['en']=body['en'].replace(f'{thickness} mm gores','0.5 mm gores')
            body['de']=body['de'].replace(f'{thickness}-mm-Bahnen','0,5-mm-Bahnen')
        if d in ('200','500'):
            body.update(en='The ballast, mast, crown and eight individually cut gores retain their dimensions. Eight 16 × 10 mm ribs carry the cloth and transfer load through eight stretchers. The rod ends and hubs use fewer facets at this scale.',
                        de='Ballast, Mast, Krone und acht einzeln zugeschnittene Bahnen behalten ihre Maße. Acht Speichen von 16 × 10 mm tragen den Stoff und leiten die Last über acht Spreizen ab. Stabenden und Naben bestehen in diesem Maßstab aus weniger Facetten.')
        model=v['decisions'][1]
        for lang,separator in [('en','Structure:'),('de','Tragwerk:')]:
            original=model[lang]
            body_note=('The canvas uses two 0.25 mm layers meeting at its mid-plane. Only the exterior top and underside exchange heat. The ballast contributes concrete and the frame contributes aluminium.'
                       if lang=='en' else 'Der Stoff besteht aus zwei 0,25-mm-Schichten, die sich in der Mittelebene berühren. Nur die Außenflächen oben und unten tauschen Wärme aus. Der Ballast bringt Beton, das Gestell Aluminium ein.')
            where=original.lower().find(separator.lower());structural=original[where+len(separator):] if where>=0 else None
            model[lang]=body_note + ((' '+separator[0].upper()+separator[1:]+' '+structural.strip()) if structural else '')
    return volumes

def drawings(a):
    if a['rig'].get('radialFolds'):return
    fine=a['representations']['100']['geometry']
    for d,r in a['representations'].items():
        panels={p['id']:p for p in a['volumes'][d]['body'] if p['id'].startswith('panel-')}
        for view,shapes in r['geometry'].items():
            for shape in shapes:
                if shape['id'] in panels:
                    panel=panels[shape['id']];shape['points']=[[x,z] for z,x in panel['profile']]
                    shape['solid'].update(z0=.008,z1=.0085,weights=deepcopy(panel['weights']))
            if d in ('200','500'):
                existing={s['id'] for s in shapes}
                shapes.extend(deepcopy(s) for s in fine[view] if s['id'].startswith(('stretch-','rib-')) and s['id'] not in existing)

def apply():
    lib=run_path(str(T/'r5-range-gaps-lib.py'));catalog=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/market-parasol/🔣️.json'))
    for path in [catalog,T/'volumes/objects/market-parasol.json']:
        before=path.read_text();doc=json.loads(before);a=doc.get('asset',doc);author(a['volumes']);s=before
        for d in ['50','100','200','500']:
            lo,hi=lib['key_value'](s,d,*lib['key_value'](s,'volumes',0,len(s)));line=s[s.rfind('\n',0,lo)+1:lo];s=s[:lo]+lib['render'](a['volumes'][d],len(line)-len(line.lstrip()),2)+s[hi:]
        if 'representations' in a:
            drawings(a)
            for d,r in a['representations'].items():
                lo,hi=lib['key_value'](s,d,*lib['key_value'](s,'representations',0,len(s)));line=s[s.rfind('\n',0,lo)+1:lo];s=s[:lo]+lib['render'](r,len(line)-len(line.lstrip()),2)+s[hi:]
        assert json.loads(s)==doc
        if path.read_text()!=before:raise RuntimeError(path)
        if before!=s:path.write_text(s)

if __name__=='__main__':apply()
