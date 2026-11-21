"""📏 Author the two generalised jack sections and the real clearance of the closed boards."""

def drawing(a,d):
    rep=a['representations'][str(d)];geom=rep['geometry']
    def rect(i,b,x,y,w,h,region,tone,z=2):
        return dict(id=i,bone=b,kind='rect',role='body',x=x,y=y,w=w,h=h,region=region,tone=tone,z=z,filled=True,material='steel-slides' if region=='slide-rails' else 'oak-timber')
    front=geom['front'];front[:]=[s for s in front if s['id']!='lift-beam-face'];front.insert(0,rect('lift-beam-face','frame',-.31,.636,.62,.018,'slide-rails',.36))
    if d!=500:return
    elev=geom['elevation'];elev[:]=[s for s in elev if s['id'] not in ['slab-left','slab-right','apron-left','apron-right','slide-bar']]
    elev.insert(0,rect('slide-bar','frame',0,-.0105,1,.021,'slide-rails',.35,1))
    for side in ['left','right']:
        elev.extend([rect('slab-'+side,'top-'+side,0,-.015,.8,.03,'table-top',.70,6),rect('apron-'+side,'top-'+side,.1 if side=='left' else 0,-.125,.7,.08,'apron',.55,4)])
    for name in ['front','elevation','plan']:
        for s in geom[name]:
            if s['id']=='leaf-bar':
                s.pop('regions',None);s['region']='leaf-top'
                if name=='front':s.update(x=-.45,y=.69,w=.90,h=.03)
                if name=='elevation':s.update(y=0,h=.03)
                if name=='plan':s.update(y=-.45,h=.9)
            if s['id'].startswith('leg-') and name in ['elevation','front']:s['h']=.72
    front[:]=[s for s in front if s['id'] not in ['slab-right','apron-right']]
    front.extend([rect('slab-right','top-right',-.45,.72,.90,.03,'table-top',.7,6),rect('apron-right','top-right',-.40,.61,.80,.08,'apron',.55,4)])
    for s in geom['plan']:
        if s['id'].startswith('slab-'):s.pop('regions',None);s['region']='table-top'
    rep['decisions']['thinElements']={'en':'The pen carries the 30 mm boards on paper. The 3D insert clears the tops in storage and meets their upper surface when raised.','de':'Die Strichstärke trägt die 30-mm-Platten im Druck. Die Einlage passt unter die Platten und schließt angehoben bündig mit ihnen ab.'}
    rep['decisions']['silhouette']={'en':'Two square-edged oak boards over set-back aprons and square legs. The stored leaf sits below the tops and comes flush between them when extended.','de':'Zwei rechtwinklige Eichenplatten über eingerückten Zargen und Vierkantbeinen. Die Einlage ruht unter den Platten und schließt ausgezogen bündig mit ihnen ab.'}
    rep['decisions']['merged']={'en':'Front and rear legs and side aprons coincide in elevation.','de':'Vordere und hintere Beine sowie seitliche Zargen decken sich im Aufriss.'}
    rep['decisions']['overlays']={'en':'The boards, leaf, legs, aprons and stationary lift crossheads carry their own material regions.','de':'Platten, Einlage, Beine, Zargen und feststehende Hubtraversen tragen eigene Materialbereiche.'}


def coarse(v):
    for s in v['body']:
        sid=s['id']
        if sid.startswith('slab-'):s['from']=-.015
        if sid.startswith('frame-'):
            s.update({'from':-.125,'to':-.045});s['profile']=[[{.325:.375,-.325:-.375}.get(z,z),{.625:.675,-.625:-.675}.get(x,x)] for z,x in s['profile']]
        if sid.startswith('leg-'):
            s['size'][1]=.72;s['transform']['translation'][1]=-.375
        if sid=='leaf':s['from']=0;s['profile']=[[.45 if z>0 else -.45,x] for z,x in s['profile']]
    slide=dict(id='slide',kind='box',bone='frame',material='steel-slides',role='body',tone=.35,size=[1,.021,.012],transform=dict(translation=[.5,0,.369]),repeat=dict(count=2,translation=[0,0,-.738]),slender=True)
    v['body']=[s for s in v['body'] if s['id']!='slide']+[slide]
    for key,id,extra in [('thermal','th-slide',dict(region='slide-rails')),('structure','member-slide',dict(member='frame',part='member'))]:
        v[key]=[dict(id=id,of='slide',**extra) if s['id']==id else s for s in v[key]]
    v['decisions']=[{'en':'Square slab edges, square legs and a single U-shaped apron for each half. The oak boards keep their 30 mm stock thickness and the insert clears both halves while stored. The two stationary steel slide rails connect to the transverse lift mechanism.','de':'Gerade Plattenkanten, Vierkantbeine und je eine U-Zarge. Die Eichenplatten behalten ihre Materialstärke von 30 mm. Verstaut passt die Einlage unter beide Hälften. Zwei feststehende Stahl-Auszugsschienen tragen den querliegenden Hubmechanismus.'}]
