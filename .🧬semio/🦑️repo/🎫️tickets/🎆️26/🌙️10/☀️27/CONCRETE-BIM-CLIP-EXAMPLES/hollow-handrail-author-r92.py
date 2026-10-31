"""🪈️ Author source-qualified hollow handrails with exact shared miters, network air and actual RC priority."""
import copy,hashlib,importlib,json,math,sys,time,traceback
from fractions import Fraction as Q
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');profiles=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️');m=api.load_model();start=time.monotonic();q=api._source_fraction
source=Path(api.__file__).with_name('🔣️.json');out={'source_before':hashlib.sha256(source.read_bytes()).hexdigest(),'api_before':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'products':[],'contacts':[]}
def intersects(a,b):return all(x[0]<y[1]and y[0]<x[1]for x,y in zip(a,b))
def center(ring,z):return [sum(q(p[k])for p in ring)/len(ring)for k in (0,1)]+[q(z)]
registered=[];constructions={}
for tag in ('TUBE-ST-GF','TUBE-ST-FR','TUBE-RM-2'):
    obj=api._find(m,tag);g=obj['geometry'];paths=[[center(p['outline'],p['z0']),center(p.get('top_outline',p['outline']),p['z1'])]for p in g['parts']];shift=0.
    if tag!='TUBE-RM-2':
        points=[paths[0][0]]
        for a,b in zip(paths,paths[1:]):
            joint=[(a[1][k]+b[0][k])/2 for k in range(3)];shift=max(shift,math.dist(joint,a[1]),math.dist(joint,b[0]));points.append(joint)
        points.append(paths[-1][1]);routes=[{'id':'continuous_stair_rail','role':'stair_handrail','points':api._native_public_value(points)}]
    else:
        nominal=json.loads((ticket/'ramp-handrail-nominal-controls-r92.json').read_text(encoding='utf8'));x=q(nominal['axis_x']);y0,y1=map(q,nominal['y_interval']);routes=[]
        for i,(z0,z1)in enumerate(nominal['rail_z']):routes.append({'id':'rail_'+str(i+1),'role':'ramp_handrail','points':api._native_public_value([[x,y0,q(z0)],[x,y1,q(z1)]])})
        upstand=next(p for p in api._guard_reference_stocks(m,api._find(m,nominal['bearing_owner']))if p['role']==nominal['bearing_role']);by_y={p[1]:p[2]for p in upstand['mesh']['vertices'][4:]};(a,za),(b,zb)=sorted(by_y.items());top0,top1=map(q,nominal['rail_z'][1]);base=q(api._z(m,g['base_level']))
        for i,t in enumerate(map(q,nominal['post_stations'])):
            y=y0+(y1-y0)*t
            if i==len(nominal['post_stations'])-1:y=y1+q(nominal['terminal_inset'])
            t=(y-y0)/(y1-y0);bottom=za+(zb-za)*(y-a)/(b-a)-base;top=top0+(top1-top0)*t
            routes.append({'id':'post_'+str(i+1),'role':'ramp_guard_post','points':api._native_public_value([[x,y,bottom],[x,y,top]]),'bearing':{'owner':nominal['bearing_owner'],'role':nominal['bearing_role']},'end_anchor':{'route':'rail_2','axis':'Z'}})
            if i==len(nominal['post_stations'])-1:routes[-1]['terminal_inset']={'value':nominal['terminal_inset'],'unit':'m','basis':'inferred','source':'Unmeasured terminal station inset for full annular bearing within the actual finite upstand'}
        shift=max(math.sqrt(float(sum((q(value)-old[k])**2 for k,value in enumerate(point))))for route,path in zip(routes,paths)for point,old in zip(route['points'],path))
    constructions[tag]={'kind':'hollow_miter_network','diameter':{'value':.035,'unit':'m','basis':'sourced'if tag!='TUBE-RM-2'else'inferred','source':'CSIC2022-Annex2-3'if tag!='TUBE-RM-2'else'Existing source-qualified arrival railing envelope'},'wall_thickness':{'value':.002,'unit':'m','basis':'inferred','source':'Hidden tube gauge unmeasured; bounded teaching reconstruction'},'segments':24,'registration_max_shift':shift,'routes':routes}
    registered.append({'tag':tag,'previous_paths':api._native_public_value(paths),'shared_station_max_shift':shift})
(ticket/'handrail-centerline-registration-r92.json').write_text(json.dumps(registered,ensure_ascii=False,indent=2),encoding='utf8')
primary=[]
for obj in m['elements']:
    cls=obj['class'];parts=[]
    if cls in ('IfcColumn','IfcBeam','IfcFooting'):
        if obj['geometry'].get('interface_construction',{}).get('kind')=='ramp_stringer':parts=[api._ramp_stringer_reference_stock(m,obj)]
        else:
            try:parts=api._interface_reference_parts(m,obj['tag'])
            except ValueError:
                if cls=='IfcColumn':parts=api.column_geometry(m,obj)['parts']
                else:
                    stock=copy.deepcopy(obj);stock['geometry']['joint_owners']=[];stock['geometry']['embedded_slabs']=[];parts=api._solid_geometry(m,stock,resolve_embedded=False)['parts']
    elif cls=='IfcSlab':
        stock=copy.deepcopy(obj);stock['geometry']['column_reserves']=[];stock['geometry']['finish_voids']=[];parts=api._slab_geometry(m,stock)['parts']
    elif cls=='IfcStair':parts=api._stair_geometry(m,obj)['structural_parts']
    elif cls=='IfcRampFlight':parts=[api._ramp_reference_stock(m,obj)]
    elif obj['geometry'].get('interface_construction',{}).get('kind')=='ramp_guard_bands':parts=api._guard_reference_stocks(m,obj)
    for part in parts:primary.append((obj['tag'],part,api._material_bounds(part)))
print('[DEBUG] Structural interface stocks '+str(len(primary)),flush=True)
from scipy.spatial import ConvexHull
import numpy as np
def occupied(part,point):
    bounds=api._material_bounds(part)
    if any(not lo<point[k]<hi for k,(lo,hi)in enumerate(bounds)):return False
    if not any(key in part for key in ('mesh','section','top_outline')):
        from shapely.geometry import Point,Polygon
        return Polygon(part['outline'],part.get('holes',())).contains(Point(point[:2]))
    vertices=np.asarray(api.part_vertices(part),dtype=float);origin=vertices.min(axis=0);span=vertices.max(axis=0)-origin;eq=ConvexHull((vertices-origin)/span).equations
    distance=(eq[:,:3]@((np.asarray(point)-origin)/span)+eq[:,3])/np.linalg.norm(eq[:,:3]/span,axis=1)
    return bool(np.max(distance)<-1e-9)
checkpoint=None
if '--resume-stairs'in sys.argv:
    checkpoint=json.loads((ticket/'🗑️generated'/'hollow-handrail-progress-r92.json').read_text(encoding='utf8'))
    if any(checkpoint[key]!=out[key]for key in ('source_before','api_before')):raise ValueError('Handrail checkpoint source/producers changed')
for tag,construction in constructions.items():
    cached=next((p for p in checkpoint['products']if p['tag']==tag),None)if checkpoint and tag!='TUBE-RM-2'else None
    if cached:
        if cached['construction']!=construction:raise ValueError('Handrail checkpoint construction operands changed')
        out['products'].append(cached);print('[DEBUG] Reused validated exact-source family '+tag,flush=True);continue
    obj=api._find(m,tag);base=q(api._z(m,obj['geometry']['base_level']));routes=[]
    for route in profiles.tube_routes(construction['routes'],exact=True):
        points=[[q(x),q(y),base+q(z)]for x,y,z in route['points']]
        if 'bearing'in route:
            bearing=route['bearing'];host=next(p for p in api._guard_reference_stocks(m,api._find(m,bearing['owner']))if p['role']==bearing['role']);by_y={p[1]:p[2]for p in host['mesh']['vertices'][:4]};(a,za),(b,zb)=sorted(by_y.items());y=points[0][1];points[0][2]=za+(zb-za)*(y-a)/(b-a)
        stock=profiles.hollow_polyline(points,.035,.002,24,exact=True);routes.append((route,points,stock))
    cores=[part for _,_,stock in routes for part in stock['core']];preceding=[];parts=[];raw_volume=0.;removed_volume=0.;contacts=[]
    caps={id(part):planes for _,_,stock in routes for collection in ('core','outer')for part,planes in zip(stock[collection],stock['cap_planes'])}
    member_bounds={id(part):api._material_bounds(part)for _,_,stock in routes for collection in ('core','outer')for part in stock[collection]}
    for route,points,stock in routes:
        for index,outer in enumerate(stock['outer']):
            box=api._material_bounds(outer);rc=[(owner,p)for owner,p,bounds in primary if intersects(box,bounds)];cutters=[p for _,p in rc]+[p for p in cores+preceding if p is not stock['core'][index] and intersects(box,member_bounds[id(p)]) and not api._surface_finishes().stock_separated(outer,caps[id(p)])]
            metal=[{**p,'material':obj['material'],'role':route['role'],'route':route['id'],'segment':index}for p in stock['metal'][index*24:(index+1)*24]]
            owned,removed=api._subtract_native_material(metal,cutters);raw_volume+=sum(api._polyhedron_volume(p)for p in metal);removed_volume+=removed
            for p in owned:
                try:api._polyhedron_part(api.part_vertices(p),api.part_faces(p))
                except Exception:
                    (ticket/'🗑️generated'/'hollow-handrail-failure-r92.json').write_text(json.dumps({'tag':tag,'route':route['id'],'segment':index,'part':p,'stock':metal,'cutters':cutters,'rc':sorted(set(owner for owner,_ in rc)),'error':traceback.format_exc()},default=api._native_cache_scalar),encoding='utf8')
                    raise
                vertices=np.asarray(api.part_vertices(p));origin=vertices.min(axis=0);spans=vertices.max(axis=0)-origin
                if np.any(spans<=0):raise ValueError('A handrail material cell has zero world extent')
                expected=ConvexHull((vertices-origin)/spans).volume*float(np.prod(spans));actual=api._polyhedron_volume(p)
                if expected<=0 or actual<=0 or not math.isclose(actual,expected,rel_tol=1e-7,abs_tol=1e-24):raise ValueError('A handrail material cell failed independent positive volume')
                parts.append(p)
            preceding.append(outer);contacts.append({'route':route['id'],'segment':index,'candidate_rc':sorted(set(owner for owner,_ in rc)),'removed':removed,'parts':len(owned)})
            print('[DEBUG] '+tag+' '+route['id']+'/'+str(index)+' parts='+str(len(owned))+' removed='+str(removed),flush=True)
    local=[{'mesh':{'vertices':[[x,y,z-float(base)]for x,y,z in p['mesh']['vertices']],'faces':p['mesh']['faces']},'material':p['material'],'role':p['role']}for p in parts]
    probes=[]
    for route,points,_ in routes:
        probes.extend(api._native_public_value(points));probes.extend([[float((a[k]+b[k])/2)for k in range(3)]for a,b in zip(points,points[1:])])
    if tag=='TUBE-RM-2':
        lower=routes[0][1]
        for _,points,_ in routes[2:]:
            x,y=float(points[0][0]),float(points[0][1]);a,b=lower;t=(q(y)-a[1])/(b[1]-a[1]);z=float(a[2]+(b[2]-a[2])*t)
            for height in (z-.0165,z,z+.0165,float(points[1][2])-.0165):
                if float(points[0][2])<=height<=float(points[1][2]):probes.append([x,y,height])
    air=[];occluded=[]
    for point in probes:
        blockers=sorted({owner for owner,p,bounds in primary if all(lo<point[k]<hi for k,(lo,hi)in enumerate(bounds)) and occupied(p,point)})
        if blockers:occluded.append({'point':point,'owners':blockers});continue
        if any(occupied(p,point)for p in parts):raise ValueError('A handrail network air reservation contains metal')
        if point not in air:air.append(point)
    out['products'].append({'tag':tag,'construction':construction,'parts':local,'raw_volume':raw_volume,'removed_volume':removed_volume,'retained_volume':sum(api._polyhedron_volume(p)for p in parts),'contacts':contacts,'air_controls':air,'rc_occluded_centerline_samples':occluded})
    (ticket/'🗑️generated'/'hollow-handrail-progress-r92.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8')
out.update(source_after=hashlib.sha256(source.read_bytes()).hexdigest(),api_after=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),seconds=time.monotonic()-start);(ticket/'🗑️generated'/'hollow-handrail-candidates-r92.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({'seconds':out['seconds'],'products':[{k:v for k,v in row.items()if k not in('parts','construction','contacts')}|{'parts':len(row['parts'])}for row in out['products']]}),flush=True)
