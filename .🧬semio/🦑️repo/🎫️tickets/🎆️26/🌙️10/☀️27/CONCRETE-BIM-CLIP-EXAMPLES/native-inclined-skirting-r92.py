"""🪵 Author inclined RM1 and middle-landing stock against unchanged native material."""
from collections import defaultdict
from fractions import Fraction as Q
from copy import deepcopy
import math,json
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon,LineString,Point,MultiPoint,box
from shapely.geometry.polygon import orient
from shapely.ops import polygonize,unary_union
from shapely import constrained_delaunay_triangles
from shapely.strtree import STRtree


def fitted_joints(planes,clearance):
    """🪚 Construct inferred timber ends with a declared installation fit, retaining exact source joint planes."""
    if not math.isfinite(clearance) or clearance<0:raise ValueError('Installation fitting must be finite and nonnegative')
    return [{**plane,'clearance':plane.get('clearance',0)-clearance} if clearance else deepcopy(plane) for plane in planes]

def source_part(part):
    """🧬 Keep the schema's single explicit mesh authority, preserving every authored vertex and face."""
    return {key:part[key] for key in ('mesh','material','role') if key in part}

def native_envelope(api,part):
    """📦 Derive missing classification bounds from actual vertices without changing original rational stock."""
    if all(key in part for key in ('z0','z1','outline')):return part
    vertices=api.part_vertices(part)
    return {**part,'z0':min(p[2] for p in vertices),'z1':max(p[2] for p in vertices),'outline':api.part_footprint(part)}

def stock_boundary(points):
    """🪵 Choose continuous inferred stock across exactly collinear subdivision vertices."""
    result=[list(p) for p in points]
    while len(result)>3:
        for i,b in enumerate(result):
            a,c=result[i-1],result[(i+1)%len(result)]
            u=[Q(b[k])-Q(a[k]) for k in (0,1)];v=[Q(c[k])-Q(b[k]) for k in (0,1)]
            if u[0]*v[1]-u[1]*v[0]==0 and sum(x*y for x,y in zip(u,v))>=0:
                result.pop(i);break
        else:break
    return result


def fit_support(finish,stock,support,origin,direction,end_clearance=0.):
    """🪵 Keep whole nominal sections only on longitudinal intervals with complete bearing space."""
    footprint=Polygon(stock['outline']);outside=footprint.difference(support)
    if outside.is_empty:return [stock]
    o=np.asarray(origin[:2],float);d=np.asarray(direction[:2],float);axis_length=float(np.linalg.norm(d));d/=axis_length
    station=lambda p:float((np.asarray(p[:2],float)-o)@d)
    low,high=min(map(station,stock['mesh']['vertices'])),max(map(station,stock['mesh']['vertices']))
    polygons=[outside] if outside.geom_type=='Polygon' else list(outside.geoms)
    intervals=[]
    for polygon in polygons:
        if polygon.geom_type!='Polygon':continue
        values=[station(p) for p in polygon.exterior.coords]
        a,b=max(low,min(values)),min(high,max(values))
        if a<b:intervals.append((a,b))
    merged=[]
    for a,b in sorted(intervals):
        if merged and a<=merged[-1][1]:merged[-1]=(merged[-1][0],max(b,merged[-1][1]))
        else:merged.append((a,b))
    kept=[];cursor=low
    for a,b in merged:
        if cursor<a:kept.append((cursor,a))
        cursor=max(cursor,b)
    if cursor<high:kept.append((cursor,high))
    parts=[]
    for a,b in kept:
        if not max(a,0)<min(b,axis_length):continue
        planes=[]
        for value,sign,bound in ((a,-1,low),(b,1,high)):
            if value==bound:continue
            point=[*(o+value*d),0.];normal=[sign*d[0],sign*d[1],0.]
            planes.append({'normal':normal,'offset':sum(n*v for n,v in zip(normal,point)),'point':point})
        part=finish.clip_stock(stock,fitted_joints(planes,end_clearance),exact=True)
        if part is not None:parts.append(part)
    return parts


def occupied_band(api,part,low,high):
    """🧱 Project actual convex spatial material only over the proposed full stock height."""
    if not part['z0']<high or not part['z1']>low:return Polygon()
    if not any(k in part for k in ('mesh','section','top_outline')):
        return Polygon(part['outline'],part.get('holes',[]))
    vertices=[[Q(float(v)) for v in p] for p in api.part_vertices(part)]
    api._polyhedron_part([[float(v) for v in p] for p in vertices],api.part_faces(part))
    lo,hi=Q(float(low)),Q(float(high))
    points=[p for p in vertices if lo<=p[2]<=hi]
    edges={tuple(sorted((a,b))) for face in api.part_faces(part) for a,b in zip(face,face[1:]+face[:1])}
    for a,b in edges:
        for z in (lo,hi):
            if (vertices[a][2]-z)*(vertices[b][2]-z)<0:
                t=(z-vertices[a][2])/(vertices[b][2]-vertices[a][2])
                points.append([vertices[a][i]+t*(vertices[b][i]-vertices[a][i]) for i in range(3)])
    if not points:return Polygon()
    projection=MultiPoint([[float(p[0]),float(p[1])] for p in points]).convex_hull
    if projection.geom_type!='Polygon':
        xs,ys=[float(p[0]) for p in points],[float(p[1]) for p in points]
        projection=box(math.nextafter(min(xs),-math.inf),math.nextafter(min(ys),-math.inf),math.nextafter(max(xs),math.inf),math.nextafter(max(ys),math.inf))
    return projection


def inclined_occupied(api,part,plane,stock_height):
    """🧱 Reserve the complete nominal stock band above the actual walking plane."""
    n,_=plane
    if n[0]==0 and n[1]==0:
        base=height(plane,0,0)
        return occupied_band(api,part,base,base+stock_height)
    gradient=np.array([-float(n[0]/n[2]),-float(n[1]/n[2])])
    band=stock_height*math.sqrt(1+float(gradient@gradient))
    vertices=[[p[0],p[1],p[2]-height(plane,p[0],p[1])] for p in api.part_vertices(part)]
    transformed={'outline':part['outline'],'z0':min(p[2] for p in vertices),'z1':max(p[2] for p in vertices),'mesh':{'vertices':vertices,'faces':api.part_faces(part)}}
    return occupied_band(api,transformed,0,band)


def host_near_support(footprint,support,thickness):
    """🪵 Include unchanged edge hosts whose receiving stock projects into wearing support."""
    return footprint.intersects(support.buffer(thickness,join_style=2))


def placement_modes(kind):
    """🧭 Keep inferred ramp stock on its wearing incline instead of underneath its structural stringer."""
    return ("inclined",) if kind in ("ramp-stringer","guard") else ("level","inclined")


def affine_plane(points):
    """📐 Retain an actual source face as an exact binary64-rational plane."""
    values=[[Q(v) for v in p] for p in points]
    a=values[0];best=None
    for i in range(1,len(values)-1):
        for j in range(i+1,len(values)):
            u=[values[i][k]-a[k] for k in range(3)];v=[values[j][k]-a[k] for k in range(3)]
            n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
            if n[2] and (best is None or abs(n[2])>abs(best[2])):best=n
    if best is None:raise ValueError("A wearing face must be a height field")
    return best,sum(best[i]*a[i] for i in range(3))


def height(plane,x,y):
    """📏 Evaluate the original source plane before one float publication."""
    n,d=plane
    return float((d-n[0]*Q(float(x))-n[1]*Q(float(y)))/n[2])


def seat_stock(finish,stock,plane):
    """🤝 Construct the inferred bottom exactly on the receiving source face plane."""
    origin=deepcopy(stock._origin);vertices=origin['mesh']['vertices'];count=len(vertices)//2
    base=affine_plane([vertices[0],vertices[1],vertices[count]])
    def exact_z(reference,x,y):
        """📐 Evaluate a retained rational face without intermediate float publication."""
        n,d=reference
        return (d-n[0]*x-n[1]*y)/n[2]
    transformed=[[x,y,z+exact_z(plane,x,y)-exact_z(base,x,y)] for x,y,z in vertices]
    origin['mesh']['vertices']=transformed
    result=type(stock)(finish._stock_mesh([[float(v) for v in p] for p in transformed],origin['mesh']['faces'],origin['material'],origin['role']))
    result._origin=origin;result._planes=();result._exact_mesh=None
    return result


def flight_face_plane(points,flight):
    """📐 Select a wearing face using three independent source walking-plane controls."""
    try:plane=affine_plane(points)
    except ValueError:return None
    start=np.asarray(flight['start'],float);end=np.asarray(flight['end'],float)
    direction=end-start;direction/=np.linalg.norm(direction)
    lateral=start+np.array([-direction[1],direction[0]])
    controls=[(start,flight['z0']),(end,flight['z1']),(lateral,flight['z0'])]
    return plane if all(abs(height(plane,*point)-z)<1e-9 for point,z in controls) else None


def section_xy(api,part,plane,above):
    """✂️ Intersect actual face cycles with a parallel plane without a hull or coordinate snap."""
    vertices=[[Q(float(v)) for v in p] for p in api.part_vertices(part)]
    n,d=plane;d+=n[2]*Q(float(above))
    distances=[sum(n[i]*p[i] for i in range(3))-d for p in vertices]
    if all(v>0 for v in distances) or all(v<0 for v in distances):return Polygon()
    edges={};segments=[];coplanar=[]
    for face in api.part_faces(part):
        if all(distances[i]==0 for i in face):
            coplanar.append(Polygon([[float(v) for v in vertices[i][:2]] for i in face]));continue
        hits=[]
        for i,j in zip(face,face[1:]+face[:1]):
            if distances[i]==0:hits.append(tuple(float(v) for v in vertices[i][:2]))
            if distances[i]*distances[j]<0:
                key=tuple(sorted((i,j)))
                if key not in edges:
                    a,b=key;t=distances[a]/(distances[a]-distances[b])
                    edges[key]=tuple(float(vertices[a][k]+t*(vertices[b][k]-vertices[a][k])) for k in (0,1))
                hits.append(edges[key])
        hits=list(dict.fromkeys(hits))
        if len(hits)==2 and hits[0]!=hits[1]:segments.append(LineString(hits))
        elif len(hits)>2:
            raise ValueError("A native face has a non-simple plane intersection")
    cells=list(polygonize(unary_union(segments))) if segments else []
    if any(p.interiors for p in cells):raise ValueError("Nested native section loops require explicit occupied-shell classification")
    return unary_union([*cells,*coplanar])


def build(model,geometry,api,finish,helpers):
    """🏗️ Preserve actual wearing support and host ownership while constructing inferred stock."""
    section=helpers['section'];material=helpers['material'];report=helpers['progress']
    floors=next(o for o in geometry['details'] if o['tag']=='FIN-RAMP-INTERIOR')
    wearing=[p for p in helpers.get('construction_refs',{}).get(floors['tag'],floors['parts']) if p.get('role')=='wearing']
    flights=[f for f in geometry['ramp']['flights'] if f['ramp']=='RM-1']
    groups=[]
    for flight in flights:
        direction=np.asarray(flight['end'])-flight['start'];direction/=np.linalg.norm(direction)
        candidates=[]
        for part in wearing:
            original=api.part_vertices(part);vertices=np.asarray(original,float)
            for face in api.part_faces(part):
                p=vertices[face];normal=sum((np.cross(p[i]-p[0],p[i+1]-p[0]) for i in range(1,len(p)-1)),np.zeros(3))
                if normal[2]<=0:continue
                plane=flight_face_plane([original[i] for i in face],flight)
                if plane is not None:candidates.append((Polygon(p[:,:2]),plane))
        if not candidates:raise ValueError('Missing actual inclined wearing plane: '+flight['tag'])
        plane=candidates[0][1]
        groups.append((flight['tag'],unary_union([p for p,_ in candidates]),plane,direction))
    mid=(min(f['z0'] for f in flights)+max(f['z1'] for f in flights))/2
    landings=[];landing_planes=[]
    for part in wearing:
        original=api.part_vertices(part);vertices=np.asarray(original,float)
        for face in api.part_faces(part):
            p=vertices[face];normal=sum((np.cross(p[i]-p[0],p[i+1]-p[0]) for i in range(1,len(p)-1)),np.zeros(3))
            if normal[2]>0 and all(abs(v[2]-mid)<1e-9 for v in p):
                landings.append(Polygon(p[:,:2]));landing_planes.append(affine_plane([original[i] for i in face]))
    if landings:groups.append(('RM-1-MID',unary_union(landings),landing_planes[0],None))
    objects=helpers['objects'];allparts=helpers['allparts']
    hosts=[o for o in objects if o in geometry['walls'] or o in geometry['screens'] or o in geometry['pilotis'] or o in geometry['beams'] or o['tag']=='GUARD-RM-1']
    products=[];counts={}
    for group,support,plane,travel in groups:
        report('inclined-hosts',group=group)
        support=support.buffer(-.00005,join_style=2)
        records=[]
        for obj in hosts:
            for part in obj['parts']:
                if not host_near_support(Polygon(api.part_footprint(part)),support,section['thickness']):continue
                shape=inclined_occupied(api,part,plane,section['height'])
                if not shape.is_empty:records.append((obj['tag'],shape))
        occupied=unary_union([p for _,p in records]);polygons=[occupied] if occupied.geom_type=='Polygon' else list(occupied.geoms)
        boundaries=[shape.boundary for _,shape in records];tree=STRtree(boundaries)
        relevant=[(tag,p,Polygon(api.part_footprint(p))) for tag,p in allparts if Polygon(api.part_footprint(p)).intersects(support)]
        cuttertree=STRtree([p for _,_,p in relevant]);cutters={};authored=defaultdict(list)
        support_polygons=[support] if support.geom_type=='Polygon' else list(support.geoms)
        support_cells=[cell for p in support_polygons for cell in ([p] if not p.interiors and p.equals(p.convex_hull) else list(constrained_delaunay_triangles(p).geoms))]
        supporttree=STRtree(support_cells)
        total=0;observations=[]
        for polygon in polygons:
            polygon=orient(polygon,1)
            for ring in [polygon.exterior,*polygon.interiors]:
                points=stock_boundary(list(ring.coords)[:-1])
                for a,b in zip(points,points[1:]+points[:1]):
                    delta=np.asarray(b)-a;length=np.linalg.norm(delta)
                    if not length:continue
                    direction=delta/length
                    if travel is not None and abs(direction[0]*travel[1]-direction[1]*travel[0])>1e-6:continue
                    normal=np.array([direction[1],-direction[0],0.]);middle=Point((a[0]+b[0])/2,(a[1]+b[1])/2)
                    probe={'a':list(a),'b':list(b),'normal':normal.tolist(),'stage':'outside-bearing'};observations.append(probe)
                    if not support.covers(Point(middle.x+normal[0]*.0075,middle.y+normal[1]*.0075)):continue
                    probe['stage']='supported'
                    host=records[min(tree.query_nearest(middle).tolist())][0];probe['host']=host
                    start=[*a,height(plane,*a)];end=[*b,height(plane,*b)]
                    stock=finish.skirting_stock(start,end,normal.tolist(),section,material,extensions=(.03,.03),back_clearance=.0001)
                    stock=seat_stock(finish,stock,plane)
                    joint=[{'normal':[-direction[0],-direction[1],0.],'point':start},{'normal':[direction[0],direction[1],0.],'point':end}]
                    for p in joint:p['offset']=sum(n*v for n,v in zip(p['normal'],p['point']))
                    stock=finish.clip_stock(stock,fitted_joints(joint,.0001),exact=True)
                    if stock is None:probe['stage']='joint-empty';continue
                    probe['stage']='stock'
                    footprint=Polygon(stock['outline']);parts=fit_support(finish,stock,support,start,np.asarray(end)-start,end_clearance=.0001)
                    for hit in cuttertree.query(footprint):
                        tag,cutter,shape=relevant[int(hit)]
                        if not footprint.intersects(shape):continue
                        if int(hit) not in cutters:
                            cutters[int(hit)]=helpers['cutter_planes'](cutter)
                        for planes in cutters[int(hit)]:
                            parts=helpers['subtract'](parts,planes)
                    probe['stage']='retained';probe['parts']=len(parts)
                    for part in parts:
                        try:api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces'])
                        except ValueError:part=finish.replay_stock(part)
                        if part is None:continue
                        helpers.get('trace_stock',lambda *args:None)('SKIRT-'+host+'-'+group,len(authored[host]),part)
                        part=finish.publish_stock(part)
                        try:api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces'])
                        except ValueError:
                            (Path(__file__).parent/'🗑️generated/native-r92/inclined-invalid.json').write_text(json.dumps({'group':group,'host':host,'edge':[a,b],'part':part}),encoding='utf8')
                            raise
                        authored[host].append(part)
                    total+=1
                    if total%20==0:report('inclined-boards',group=group,candidates=total,parts=sum(map(len,authored.values())))
        for host,parts in authored.items():
            tag='SKIRT-'+host+'-'+group
            products.append({'tag':tag,'guid':api.global_id(tag),'class':'IfcCovering','predefined_type':'SKIRTINGBOARD','name':{'en':'Original gray 15×90 mm timber skirting at interior ramp; fit inferred','de':'Ursprüngliche graue 15×90 mm Holzsockelleiste an Innenrampe; Passung abgeleitet'},'storey':'ground','material':material,'geometry':{'kind':'skirting','base_level':'ground','parts':parts},'attributes':{},'psets':{},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']},'historical_evidence':{'state':'original-completed-1930-1931','status':'inferred_reconstruction','source_kind':'mixed','sources':['SAB2024-p190; original FLC L2(17)059/061 photographs'],'uncertainty':{'en':'Interior ramp mapping, endpoints, butt/miter joints,2 mm rounded edge,gray display RGB and0.1 mm installation back/end fits are inferred, not historic measurements. Source15×90 mm timber stock follows the actual wearing plane and unchanged physical host.0.05 mm inward wearing boundary registration supplies positive bearing. No exposed RM2 timber is inferred.','de':'Zuordnung zur Innenrampe,Endpunkte,Fugen,2 mm Rundung,graues Anzeige-RGB und0.1 mm Einpassluft an Rückseite/Enden sind abgeleitet,keine historischen Messungen. Belegte15×90 mm Holzleisten folgen tatsächlichem Belag und unverändertem Host;0.05 mm Einrückung hält die Auflage. Kein Holz an freier Außenrampe RM2.'}}})
        helpers.get('coverage',[]).append({'group':group,'edges':observations})
        counts[group]={'products':len(authored),'parts':sum(map(len,authored.values())),'candidates':total}
        report('inclined-complete',group=group,**counts[group])
    return {'elements':products,'counts':counts}
                                                                                                                