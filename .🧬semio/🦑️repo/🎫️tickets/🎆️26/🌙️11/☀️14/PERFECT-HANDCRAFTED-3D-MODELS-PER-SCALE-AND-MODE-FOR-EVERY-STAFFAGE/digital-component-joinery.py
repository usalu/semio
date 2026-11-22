"""🚪 Author the exposed glass joints, glazed sliding leaf and genuine masonry bond."""
from copy import deepcopy
from math import cos, hypot, pi, sin

def words(en,de):
    return {'en':en,'de':de}

def simplified(ring,tolerance):
    if len(ring)<8:return deepcopy(ring)
    def trace(path):
        a,b=path[0],path[-1];dx,dy=b[0]-a[0],b[1]-a[1];den=dx*dx+dy*dy
        def distance(p):
            u=max(0,min(1,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/den)) if den else 0
            return hypot(p[0]-a[0]-u*dx,p[1]-a[1]-u*dy)
        if len(path)<3:return path
        d,i=max((distance(p),i) for i,p in enumerate(path[1:-1],1))
        if d<=tolerance:return [a,b]
        return trace(path[:i+1])[:-1]+trace(path[i:])
    split=max(range(len(ring)),key=lambda j:hypot(ring[j][0]-ring[0][0],ring[j][1]-ring[0][1]))
    result=trace(ring[:split+1])[:-1]+trace(ring[split:]+[ring[0]])[:-1]
    return result if len(result)>2 else deepcopy(ring)

def grade(p,tol):
    if p.get('kind')=='extrude':
        p['profile']=simplified(p['profile'],tol)
        if 'holes' in p:p['holes']=[simplified(h,tol) for h in p['holes']]
    return p

def rounded_rectangle(x0,y0,x1,y1,r,n):
    points=[]
    for x,y,angle in [(x1-r,y1-r,0),(x0+r,y1-r,90),(x0+r,y0+r,180),(x1-r,y0+r,270)]:
        points.extend([[round(x+r*cos((angle+j*90/n)*pi/180),8),round(y+r*sin((angle+j*90/n)*pi/180),8)] for j in range(n+1)])
    return points

def sections_along(s,n):
    lo,hi=s['from'],s['to'];pieces=[]
    for i in range(n):
        p=deepcopy(s);p['id']+='-bay-'+str(i)
        p.update(**{'from':lo+(hi-lo)*i/n,'to':lo+(hi-lo)*(i+1)/n})
        pieces.append(p)
    return pieces

def balustrade(a):
    fine=deepcopy(a['volumes']['50'])
    if fine['body'][0]['kind']=='extrude':return
    for k,v in a['volumes'].items():
        n=24;tol={'50':0,'100':.00016,'200':.00034,'500':.00068}[k]
        body=deepcopy(fine['body'])
        plate=body[0];plate.pop('size');plate.pop('transform')
        corners=[[-.15,-1],[.15,-1],[.15,1],[-.15,1]]
        holes=[[[round(y+.007*cos(-2*pi*i/n),7),round(z+.007*sin(-2*pi*i/n),7)] for i in range(n)] for y in [-.11,.11] for z in [-.8,-.4,0,.4,.8]]
        plate.update(kind='extrude',plane='yz',profile=corners,holes=holes,**{'from':0,'to':.015})
        for s in body:
            if s['id'].startswith('anchor-stud-'):
                s['origin'][0]=0;s['profile'][1][0]=.034
            if tol and s['id'] not in ('base-plate','cheek-fixed','cheek-clamp','pad-fixed','pad-clamp'):grade(s,tol)
        v['body']=body
        heat=deepcopy(fine['thermal']);v['thermal']=[grade(s,tol) if tol else s for s in heat]
        joints=deepcopy(fine['structure']);stations=[s for s in v['structure'] if s.get('part')=='mass']
        if not stations:
            stations=[deepcopy(joints[-1])];stations[0]['profile']=[[pos,rad*1.35] for pos,rad in stations[0]['profile']]
        v['structure']=[s for s in joints if s.get('part')!='mass']+stations
        v['decisions']=[words('Two drilled 10 mm panes and the 1.5 mm PVB interlayer seat between rubber-lined clamping cheeks. Seven sleeved cross bolts pass through both cheeks and the laminate. Two rows of five M12 studs pass through the 15 mm shoe plate. Smaller scales simplify the circular edge facets.','Zwei durchbohrte 10-mm-Scheiben und die 1,5-mm-PVB-Folie sitzen in gummigefütterten Klemmwangen. Sieben ummantelte Querbolzen führen durch Wangen und Verbundglas. Zwei Reihen mit je fünf M12-Ankern durchdringen die 15-mm-Fußplatte. In kleineren Maßstäben sind die Rundungen vereinfacht.'),words('Minimal: the two anchor rows and the complete glass stack establish the support at each scale.','Minimal: Beide Ankerreihen und das vollständige Verbundglas bilden das Auflager in jedem Maßstab.')]

    shape=next(p for p in a['representations']['200']['geometry']['elevation'] if p['id']=='shoe-block')
    for point in shape['points']:
        if point[0]==.03:point[0]=.015

def sliding(a):
    fine=deepcopy(a['volumes']['50'])
    if any(s['id']=='j-wheel-a-ext' for s in fine['structure']):return
    for k,v in a['volumes'].items():
        tol={'50':0,'100':.00038,'200':.00072,'500':.0013}[k]
        body=deepcopy(fine['body'])
        keep=('track-end-caps','leaf-stile-w','leaf-stile-e','leaf-rail-top','leaf-rail-bottom')
        v['body']=[grade(s,tol) if tol and s['id'] not in keep else s for s in body]
        heat=deepcopy(fine['thermal']);v['thermal']=[grade(s,tol) if tol and s.get('region')!='track-steel' else s for s in heat]
        if k!='50':v['thermal']=[p for p in v['thermal'] if p['id'] not in ('th-track-c','th-track-a')]
        joints=deepcopy(fine['structure'])
        markers=[s for s in v['structure'] if s.get('part') in ('mass','support')]
        v['structure']=[(grade(s,tol) if tol and s['id'].removeprefix('m-') not in keep else s) for s in joints if s.get('part') not in ('mass','support') and s['id']!='j-trolleys']+markers
        for cart in ('a','b'):
            for face in ('ext','int'):
                v['structure'].append(dict(id='j-wheel-'+cart+'-'+face,of='trolley-'+cart+'-wheels-'+face,part='joint',member='leaf'))
        for name in ('trolley-chassis','hanger-clevis','hanger-nut-adjust'):
            v['structure'].append(dict(id='j-'+name,of=name,part='joint',member='leaf'))
        v['decisions']=[words('The 10 mm glazed leaf hangs from two adjustable, four-wheel carriages in the open overhead track. Separate stiles, rails and beads capture the glazing. A pull handle on either face, the bottom guide and the sliding leaf share their correct travel. Scale changes simplify the folded and rounded edge profiles.','Das 10-mm-Glasblatt hängt an zwei justierbaren vierrädrigen Laufwagen in der offenen Laufschiene. Holme, Riegel und Glasleisten halten die Verglasung. Zwei Bügelgriffe, Blatt und Bodenführung behalten ihre jeweilige Bewegung. Die Kantungen und Rundungen werden maßstabsabhängig vereinfacht.'),words('Minimal: the complete operable carriage and two hand grips remain at every scale.','Minimal: Die vollständigen Laufwagen und beide Griffe bleiben in jedem Maßstab erhalten.')]

    rep=a['representations']['500']['geometry']
    for p in rep['plan']:
        if p['id'].startswith('jamb-'):p.update(x=-.05 if p['id']=='jamb-west' else 2.95,w=.1)
        if p['id']=='leaf':p.update(y=-.025,h=.05,penMm=.16)
    for p in rep['elevation']:
        if p['id']=='portal':
            p['points']=[[-.05,-2.1],[.05,-2.1],[.05,-.05],[2.95,-.05],[2.95,-2.1],[3.05,-2.1],[3.05,.05],[-.05,.05]]

def brickwork(a):
    fine=deepcopy(a['volumes']['50'])
    if any(p['id'].startswith('th-brick-cell-') for p in fine['thermal']):return
    for k,v in a['volumes'].items():
        steps={'50':3,'100':2,'200':1,'500':0}[k]
        body=deepcopy(fine['body'])
        for p in body:
            if p['id'] not in ('plaster','render'):continue
            x,y,z=p.pop('size');cx,cy,cz=p.pop('transform')['translation'];r=.0015;points=[]
            for yy,zz,angle in [(cy+y/2-r,cz+z/2-r,0),(cy-y/2+r,cz+z/2-r,90),(cy-y/2+r,cz-z/2+r,180),(cy+y/2-r,cz-z/2+r,270)]:
                n=steps+1
                points.extend([[round(yy+r*cos((angle+90*j/n)*pi/180),8),round(zz+r*sin((angle+90*j/n)*pi/180),8)] for j in range(n+1)])
            p.update(kind='extrude',plane='yz',profile=points,**{'from':cx-x/2,'to':cx+x/2})
        v['body']=body
        n={'50':6,'100':4,'200':2,'500':1}[k]
        heat=deepcopy(fine['thermal'])
        brick=heat[1];cells=[]
        for j in range(n):
            cell=deepcopy(brick);cell['id']='th-brick-cell-'+str(j);cell['size'][0]=.365/n;cell['transform']['translation'][0]=.015+.365*(j+.5)/n;cells.append(cell)
        v['thermal']=[heat[0],*cells,heat[-1]]
        markers=[p for p in v['structure'] if p.get('part') in ('mass','support')]
        v['structure']=deepcopy([p for p in fine['structure'] if p.get('part') not in ('mass','support')])+markers
        v['decisions']=[words('Twelve beds carry alternate header and stretcher courses in a 365 mm solid leaf. The cut exposes individual units, transverse and longitudinal joints, 15 mm room plaster and the 20 mm outside render. The smaller scales simplify the 1.5 mm rounded finish arrises. The heat field resolves the homogeneous brick layer with successively wider depth cells.','Zwölf Lagerfugen tragen wechselnde Binder- und Läuferlagen einer 365-mm-Vollziegelwand. Der Schnitt zeigt einzelne Steine, Stoß- und Längsfugen, 15 mm Innenputz und 20 mm Außenputz. Die 1,5-mm-Putzrundungen sind in kleineren Maßstäben vereinfacht. Das Temperaturfeld gliedert die Ziegelschicht in zunehmend breitere Tiefenzellen.'),words('Minimal: every course belongs to the continuous bonded gravity-resisting leaf.','Minimal: Jede Schicht gehört zur durchgehend tragenden Verbandswand.')]

def hole(cy,cz,r,n):
    return [[round(cy+r*cos(-2*pi*i/n),7),round(cz+r*sin(-2*pi*i/n),7)] for i in range(n)]

def canopy(a):
    fine=deepcopy(a['volumes']['50'])
    if any(s['id']=='stud-beam-top' for s in fine['body']):return
    for k,v in a['volumes'].items():
        n=24
        tol={'50':0,'100':.0003,'200':.0007,'500':.0013}[k]
        body=deepcopy(fine['body'])
        if k!='50':body=[p for p in body if not p['id'].startswith('rebar-')]
        joints=deepcopy(fine['structure'])
        for stud in [s for s in joints if s['id'].startswith('j-stud-')]:
            new={key:deepcopy(value) for key,value in stud.items() if key not in ('part','member')}
            new['id']=stud['id'][2:];body.append(new)
        for p in body:
            name=p['id']
            if name=='wall-core':
                dx,dy,dz=p.pop('size');x,y,z=p.pop('transform')['translation']
                p.update(kind='extrude',plane='yz',profile=rounded_rectangle(y-dy/2,z-dz/2,y+dy/2,z+dz/2,.002,{'50':5,'100':3,'200':2,'500':1}[k]),**{'from':x-dx/2,'to':x+dx/2})
            if name.startswith(('plate-','pad-')):
                dx,dy,dz=p.pop('size');x,y,z=p.pop('transform')['translation']
                holes=[hole(y+yy,zz,.009,n) for yy in [-.073,.073] for zz in [-.058,.058]]
                p.update(kind='extrude',profile=[[y-dy/2,-dz/2],[y+dy/2,-dz/2],[y+dy/2,dz/2],[y-dy/2,dz/2]],holes=holes,plane='yz',**{'from':x-dx/2,'to':x+dx/2})
            if name.startswith('nut-'):
                cy=sum(q[0] for q in p['profile'])/6;cz=sum(q[1] for q in p['profile'])/6
                p['holes']=[hole(cy,cz,.008,n)]
            if tol and name=='beam':grade(p,tol)
        v['body']=body
        heat=deepcopy(fine['thermal'])
        for p in heat:
            if p['id']=='th-strut-tube':
                stock=next(q for q in body if q['id']=='strut-tube')
                p['profile']=deepcopy(stock['profile'][:2]) if len(stock['profile'])==2 else [[.15,.03015],[4.37709,.03015]]
        plate=next(q for q in body if q['id']=='plate-strut')
        for i,q in enumerate(heat):
            if q.get('of')=='plate-strut':heat[i]={**deepcopy(plate),**{key:value for key,value in q.items() if key!='of'}}
        wool=next(s for s in body if s['id']=='wall-insulation')
        field=next(q for q in heat if q['id']=='th-wall-wool')
        conductive={**wool,**{key:value for key,value in field.items() if key!='of'}}
        v['thermal']=[q for q in heat if q['id']!='th-wall-wool']+sections_along(conductive,{'50':6,'100':4,'200':2,'500':1}[k])
        if k=='100':
            steel=next(q for q in body if q['id']=='beam')
            joints=[{**deepcopy(steel),'id':'m-beam','part':'member','member':'canopy'} if q['id']=='m-beam' else q for q in joints]
        markers=[p for p in v['structure'] if p.get('part') in ('mass','support')]
        v['structure']=[q for q in joints if q['part'] not in ('mass','support')]+markers
        v['decisions']=[words('Each end of the tie has a bored gusset, two clevis cheeks and a steel pin. Each wall mounting has a pierced 20 mm plate and a separation pad bearing on four 16 mm studs, with separate nuts. Both the IPE beam and the tie continue between actual end fittings.','Beide Zugbandenden haben ein durchbohrtes Knotenblech, zwei Gabelwangen und einen Stahlbolzen. An jedem Wandanschluss sitzen durchbohrte 20-mm-Platte und Trennklotz auf vier 16-mm-Ankern mit separaten Muttern. IPE-Träger und Zugband reichen zwischen den tatsächlichen Endanschlüssen.'),words('Minimal: the two complete clevises and their anchor plates define the reaction path at every scale.','Minimal: Beide vollständigen Gabelköpfe und ihre Ankerplatten bestimmen den Lastweg in jedem Maßstab.')]

    for k in ('100','200','500'):
        v=a['volumes'][k];stock=next(p for p in v['body'] if p['id']=='wall-render')
        field=next(p for p in v['thermal'] if p['id']=='th-wall-render')
        pane={**deepcopy(stock),**{key:value for key,value in field.items() if key!='of'}}
        rings=[pane['profile'],*pane['holes']]
        count={'100':4,'200':2,'500':1}[k]
        low,high=min(q[0] for q in pane['profile']),max(q[0] for q in pane['profile'])
        levels=sorted({round(q[0],7) for ring in rings for q in ring}|{round(low+(high-low)*i/count,7) for i in range(count+1)})
        surface=[]
        for idx,(y0,y1) in enumerate(zip(levels,levels[1:])):
            ym=(y0+y1)/2;cross=[]
            for ring in rings:
                for q,r in zip(ring,ring[1:]+ring[:1]):
                    if min(q[0],r[0])<=ym<max(q[0],r[0]):cross.append(q[1]+(ym-q[0])*(r[1]-q[1])/(r[0]-q[0]))
            cross.sort()
            for s,(z0,z1) in enumerate(zip(cross[::2],cross[1::2])):
                face=deepcopy(pane);face.pop('holes');face.update(id=f'th-render-face-{idx}-{s}',profile=[[y0,z0],[y1,z0],[y1,z1],[y0,z1]]);surface.append(face)
        v['thermal']=[q for q in v['thermal'] if q['region'] not in ('wall-face','wall-plate') or q.get('layer')=='plate-steel']+surface
def garage(a):
    fine=deepcopy(a['volumes']['50'])
    if any(s['id']=='hinge-pin-3' for s in a['volumes']['500']['body']):return
    target=lambda p:p['bone'].startswith('panel-') or p['id'].startswith(('hanger-','ceiling-foot-','brace-foot-','brace-','track-angle-','jamb-bracket-','flag-bracket-','rail-'))
    for k in ['100','200','500']:
        v=a['volumes'][k];tol={'100':.00024,'200':.00060,'500':.0011}[k]
        gear=[grade(deepcopy(p),tol) if p['id'].startswith(('panel-','rail-')) else deepcopy(p) for p in fine['body'] if target(p)]
        ids={p['id'] for p in gear}
        old={p['id'] for p in v['body'] if target(p)}
        v['body']=[p for p in v['body'] if not target(p)]+gear
        thermo=[deepcopy(p) for p in fine['thermal'] if p.get('of') in ids or p.get('bone','').startswith('panel-')]
        cores=[p for p in thermo if p['id'].startswith('th-core-')]
        thermo=[p for p in thermo if p not in cores]+[q for p in cores for q in sections_along(p,{'100':6,'200':4,'500':2}[k])]
        heat_ids={p['id'] for p in thermo}
        v['thermal']=[p for p in v['thermal'] if p['id'] not in heat_ids and p.get('of') not in old and not p.get('bone','').startswith('panel-')]+thermo
        structural=[grade(deepcopy(p),tol) for p in fine['structure'] if p.get('of') in ids or p.get('bone','').startswith('panel-')]
        load_ids={p['id'] for p in structural}
        v['structure']=[p for p in v['structure'] if p['id'] not in load_ids and p.get('of') not in old and not p.get('bone','').startswith('panel-')]+structural
        v['decisions']=[words('Four independent sandwich panels retain the curved 52 mm finger-guard noses and matching sockets. Three pairs of pin hinges link the panels. Ten rollers follow the open track into its braced ceiling hangers. The smaller scales use fewer intervals for the curved panel and steel edge profiles.','Vier unabhängige Sandwichpaneele behalten die gekrümmten 52-mm-Fingerschutznasen und ihre Gegenprofile. Drei Scharnierpaare verbinden die Paneele. Zehn Rollen folgen der offenen Schiene bis zu den ausgesteiften Deckenabhängern. In kleineren Maßstäben verwenden die gekrümmten Blech- und Paneelkanten weniger Abschnitte.')]

    heat=a['volumes']['50']['thermal']
    a['volumes']['50']['thermal']=[q for p in heat for q in (sections_along(p,8) if p['id'].startswith('th-skin-outer-') else sections_along(p,4) if p['id'].startswith('th-core-') else [p])]

    reference=a['representations']['100']['geometry']
    for k in ('100','200','500'):
        v=a['volumes'][k]
        v['thermal']=[p for p in v['thermal'] if p.get('region')!='panel-sandwich' or p['id'].startswith('th-skin-outer-')]
        if k=='100':continue
        drawing=a['representations'][k]
        selection=['ground','jamb-bracket-0','jamb-bracket-1','jamb-bracket-2','flag-bracket','bearing-flag','track-band','hanger-0','hanger-1','brace','track-end-bar','panel-1','panel-2','panel-3','panel-4','drum','roller-2']
        if k=='200':selection+=['track-angle','hinge-1','hinge-2','hinge-3','bottom-seal','roller-0','roller-4']
        elevation=[]
        for q in reference['elevation']:
            if q['id'] not in selection:continue
            q=deepcopy(q)
            if q['id']!='ground':q['penMm']=.20 if k=='500' else .18
            elevation.append(q)
        drawing['geometry']['elevation']=elevation
        drawing['sections']=deepcopy(a['representations']['100']['sections'])
        if k=='200':
            fineplan={p['id']:p for p in reference['plan']}
            for id in ('track-angle-left','track-angle-right','bottom-seal','anchor-bracket','roller-0-left','roller-0-right','roller-4-left','roller-4-right'):
                symbol=deepcopy(fineplan[id]);symbol['penMm']=.17
                drawing['geometry']['plan'].append(symbol)

            plan=drawing['geometry']['plan']
            plan[:]=[p for p in plan if not p['id'].startswith(('hinge-','roller-'))]
            for p in reference['plan']:
                if not p['id'].startswith(('hinge-','roller-')):continue
                q=deepcopy(p);q['penMm']=.18;plan.append(q)
            for i,q in enumerate(drawing['geometry']['plan']):
                if q['id'].startswith('panel-edge-') and q['id'] in fineplan:
                    q=deepcopy(fineplan[q['id']]);q['penMm']=.18;drawing['geometry']['plan'][i]=q
    for k in ('100','200','500'):
        skins=[p for p in a['volumes'][k]['thermal'] if p['id'].startswith('th-skin-outer-')]
        others=[p for p in a['volumes'][k]['thermal'] if p not in skins]
        a['volumes'][k]['thermal']=others+[q for p in skins for q in sections_along(p,{'100':6,'200':4,'500':2}[k])]
def lintelwall(a):
    fine=deepcopy(a['volumes']['50'])
    if any(p['id'].startswith('th-fill-slice-') for p in fine['thermal']):return
    for k,v in a['volumes'].items():
        steps={'50':5,'100':3,'200':2,'500':1}[k]
        body=deepcopy(fine['body'])
        for p in body:
            if p['id']!='sill':continue
            x,y=p['profile'][7];r=.002
            arc=[[round(x+r-r*cos(pi*j/(2*steps)),7),round(y+r-r*sin(pi*j/(2*steps)),7)] for j in range(steps+1)]
            p['profile']=p['profile'][:7]+arc+p['profile'][8:]
        v['body']=body
        heat=[]
        for p in deepcopy(fine['thermal']):
            if p['id']!='th-field-fill':heat.append(p);continue
            count={'50':6,'100':4,'200':2,'500':1}[k];start,end=p['from'],p['to']
            for i in range(count):
                cell=deepcopy(p);cell.update(id='th-fill-slice-'+str(i),**{'from':start+(end-start)*i/count,'to':start+(end-start)*(i+1)/count});heat.append(cell)
        v['thermal']=heat
        markers=[p for p in v['structure'] if p['part'] in ('mass','support')]
        v['structure']=deepcopy([p for p in fine['structure'] if p['part'] not in ('mass','support')])+markers
        v['decisions']=[words('Thirty-three courses of 115 mm facing brick return to the opening, surrounding the 140 mm mineral wool, the inner 175 mm leaf and 15 mm room plaster. The reveals and soffit meet two individually bearing lintels. The external sloping sill retains its drip and a 2 mm rounded leading arris. The smaller scales reduce the number of stations on the arris and through the insulation heat field.','Dreiunddreißig Verblendschichten mit 115 mm Stärke führen zur Öffnung. Dahinter liegen 140 mm Mineralwolle, eine 175-mm-Innenschale und 15 mm Innenputz. Laibungen und Untersicht schließen an zwei separat aufgelagerte Stürze an. Die äußere Sohlbank behält Tropfrille und eine mit 2 mm gerundete Vorderkante. Kleinere Maßstäbe vereinfachen Rundung und die thermische Tiefenauflösung des Dämmstoffs.'),words('Minimal: the opening geometry, facing courses and masonry ties are required at all four scales.','Minimal: Öffnungsgeometrie, Verblendschichten und Maueranker sind in allen vier Maßstäben erforderlich.')]

    for k in ('50','100','200','500'):
        v=a['volumes'][k];heat=v['thermal']
        stock=next(p for p in heat if p['id']=='th-field-facing')
        rings=[stock['profile'],*stock.get('holes',[])]
        levels={round(y,6) for ring in rings for x,y in ring}
        step={'50':1,'100':1,'200':3,'500':33}[k]
        levels.update(round(y/12,6) for y in range(0,34,step))
        if k=='50':levels.update(round(y/12+.01233,6) for y in range(33))
        depths=sorted(levels);surface=[]
        for index,(low,high) in enumerate(zip(depths,depths[1:])):
            if high-low<.00001:continue
            level=(low+high)/2;cross=[]
            for ring in rings:
                for q,r in zip(ring,ring[1:]+ring[:1]):
                    if min(q[1],r[1])<=level<max(q[1],r[1]):cross.append(q[0]+(level-q[1])*(r[0]-q[0])/(r[1]-q[1]))
            cross.sort()
            for side,(left,right) in enumerate(zip(cross[::2],cross[1::2])):
                if right-left<.00001:continue
                tile=deepcopy(stock);tile.pop('holes',None);tile['id']=f'th-facing-{index}-{side}'
                tile['profile']=[[left,low],[right,low],[right,high],[left,high]]
                surface.append(tile)
        v['thermal']=surface+[p for p in heat if (k=='50' and p['id']!='th-field-facing') or p.get('layer') in ('lintel-shell','sill-concrete')]
        v['decisions'].append(words('The exterior temperature field follows the actual facing, projecting sill and exposed lintel. Courses are grouped only within the same heat-transfer surface. The 1:50 thermal section retains the interior passive layers.','Das äußere Temperaturfeld folgt Verblendung, Sohlbank und sichtbarem Sturz. Die Schichten werden nur innerhalb derselben Wärmeübertragungsfläche zusammengefasst. Der thermische Schnitt 1:50 zeigt die inneren passiven Schichten.'))

def insulating_glazing(a):
    curtain=a['id']=='curtain-wall-bay'
    fine=deepcopy(a['volumes']['50'])
    if any(p['id'].startswith('th-gas-depth-') for p in fine['thermal']):return
    if curtain:
        region=next(q for q in a['thermal']['regions'] if q['id']=='glass-centre')
        layer=next(q for q in region['layers'] if q['id']=='cavity')
        medium='argon-cavity'
        value=.016/layer['resistanceM2KW']
        a['materials'].append(dict(id=medium,title=words('16 mm argon cavity, equivalent layer','16-mm-Argonzwischenraum, äquivalente Schicht'),emissivity=.84,solarAbsorptivity=.08,conductivityWmK=round(value,7),source='authored-estimate'))
        layer['material']=medium
    else:medium='glass-unit-cavity'
    for k,v in a['volumes'].items():
        tol={'50':0,'100':.00019,'200':.0005,'500':.0011}[k]
        body=deepcopy(fine['body'])
        if curtain and k!='50':body=[p for p in body if not p['id'].startswith('stub-')]
        for p in body:
            if curtain and p['id'] in ('igu-pane-in','igu-pane-out'):
                xs,ys=zip(*p['profile']);p['profile']=rounded_rectangle(min(xs),min(ys),max(xs),max(ys),.002,{'50':5,'100':3,'200':2,'500':1}[k])
            elif not curtain and p['id'] in ('pane-inner','pane-outer'):
                w,h,d=p.pop('size');x,y,z=p.pop('transform')['translation']
                p.update(kind='extrude',plane='zx',profile=rounded_rectangle(z-d/2,x-w/2,z+d/2,x+w/2,.002,{'50':5,'100':3,'200':2,'500':1}[k]),**{'from':y-h/2,'to':y+h/2})
        v['body']=body
        heat=[]
        for p in deepcopy(fine['thermal']):
            if p['id']!=('th-glass-centre-cavity' if curtain else 'th-centre-cavity'):
                heat.append(grade(p,tol) if tol else p);continue
            count={'50':6,'100':4,'200':2,'500':1}[k]
            for i in range(count):
                cell=deepcopy(p);cell['id']='th-gas-depth-'+str(i);cell['material']=medium
                if curtain:cell.update(**{'from':-.078+.016*i/count,'to':-.078+.016*(i+1)/count})
                else:
                    cell['size'][1]=.016/count;cell['transform']['translation'][1]=-.054+.016*(i+.5)/count
                heat.append(cell)
        if curtain and k!='50':heat=[p for p in heat if p.get('region')!='spandrel' or p['id']=='th-spandrel-enamel']
        v['thermal']=heat
        markers=[p for p in v['structure'] if p['part'] in ('mass','support')]
        v['structure']=deepcopy([p for p in fine['structure'] if p['part'] not in ('mass','support')])+markers
        if not curtain and k=='500':
            arc=next(q for q in body if q['id']=='stay-arc')
            v['structure']=[{**deepcopy(arc),'id':'m-stay-arc','part':'member','member':'sill'} if q['id']=='m-stay-arc' else q for q in v['structure']]
        if curtain:
            detail=words('Two 6 mm panes enclose the clear 16 mm argon cavity. The perimeter seal, mullion and transom profiles, face caps and gaskets seat the glass. The spandrel has separate enamel glass, mineral wool and folded steel backpan.','Zwei 6-mm-Scheiben schließen den freien 16-mm-Argonraum ein. Randverbund, Pfosten- und Riegelprofile, Deckleisten und Dichtungen fassen das Glas. Die Brüstung besteht aus Emailglas, Mineralwolle und einer eigenen Stahlblechwanne.')
        else:
            detail=words('Two 4 mm panes enclose a clear 16 mm insulating cavity with a perimeter edge seal. Both panes, beads and grips follow the tilting sash while the sill, stops and stay arc remain fixed. The smaller scales simplify the curved steel and timber profiles.','Zwei 4-mm-Scheiben schließen einen freien 16-mm-Dämmraum mit umlaufendem Randverbund ein. Beide Scheiben, Glasleisten und der Griff folgen dem Kippflügel; Sohlbank, Anschläge und Ausstellbogen bleiben fest. Kleinere Maßstäbe vereinfachen die gerundeten Stahl- und Holzprofile.')
        v['decisions']=[detail,words('Minimal: both pane surfaces and their physical cavity remain necessary at every scale. The thermal field groups adjoining gas depth cells at smaller scales.','Minimal: Beide Scheiben und ihr tatsächlicher Zwischenraum bleiben in jedem Maßstab erforderlich. Das Temperaturfeld fasst bei kleineren Maßstäben benachbarte Tiefenzellen des Gasraums zusammen.')]
