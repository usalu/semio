"""🦶 Independently verify annular post feet meet actual RC along every sampled material generator."""
import importlib,json,sys,math,functools,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Point,Polygon
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();candidate=json.loads((ticket/'🗑️generated'/'hollow-handrail-candidates-r92.json').read_text(encoding='utf8'));row=next(p for p in candidate['products']if p['tag']=='TUBE-RM-2');base=api._z(m,api._find(m,row['tag'])['geometry']['base_level']);cursor=0;groups={}
for contact in row['contacts']:
    group=[]
    for part in row['parts'][cursor:cursor+contact['parts']]:
        p={**part,'mesh':{**part['mesh'],'vertices':[[x,y,z+base]for x,y,z in part['mesh']['vertices']]}};group.append(p)
    groups[contact['route']]=group;cursor+=contact['parts']
cache={}
def interval(part,x,y):
    bounds=api._material_bounds(part)
    if not(bounds[0][0]<=x<=bounds[0][1]and bounds[1][0]<=y<=bounds[1][1]):return None
    if not any(k in part for k in ('mesh','section','top_outline')):
        return (float(part['z0']),float(part['z1']))if Polygon(part['outline'],part.get('holes',())).covers(Point(x,y))else None
    key=id(part)
    if key not in cache:
        v=np.asarray(api.part_vertices(part),dtype=float);origin=v.min(axis=0);span=v.max(axis=0)-origin;eq=ConvexHull((v-origin)/span).equations;normals=eq[:,:3]/span;cache[key]=np.column_stack([normals,eq[:,3]-normals@origin])
    lo,hi=-math.inf,math.inf
    for nx,ny,nz,d in cache[key]:
        offset=nx*x+ny*y+d
        if nz==0:
            if offset>1e-10:return None
        elif nz>0:hi=min(hi,-offset/nz)
        else:lo=max(lo,-offset/nz)
    return (float(lo),float(hi))if hi>lo+1e-12 else None
owners={}
for tag in sorted({tag for contact in row['contacts']for tag in contact['candidate_rc']}):
    obj=api._find(m,tag)
    if obj['geometry'].get('interface_construction',{}).get('kind')=='ramp_guard_bands':parts=api._guard_reference_stocks(m,obj)
    elif obj['class']=='IfcRampFlight':parts=[api._ramp_reference_stock(m,obj)]
    elif obj['class']in('IfcColumn','IfcBeam','IfcFooting'):
        try:parts=api._interface_reference_parts(m,tag)
        except ValueError:parts=api._interface_owner_parts(m,tag)
    else:parts=api._interface_owner_parts(m,tag)
    owners[tag]=parts
out={'routes':[],'pass':True};started=time.monotonic()
for route in row['construction']['routes']:
    if route['role']!='ramp_guard_post':continue
    parts=groups[route['id']];checks=[]
    if not parts:out['routes'].append({'route':route['id'],'status':'ENTIRELY_RC_OCCLUDED','checks':[]});continue
    cx,cy,_=route['points'][0]
    for i in range(24):
        angle=(i+.5)*2*math.pi/24;x,y=cx+.0165*math.cos(angle),cy+.0165*math.sin(angle);material=[v for p in parts if(v:=interval(p,x,y))is not None]
        if not material:continue
        foot=min(v[0]for v in material);supports=[(tag,v[1])for tag,records in owners.items()for p in records if(v:=interval(p,x,y))is not None and v[1]<=foot+1e-8]
        nearest=min(supports,key=lambda pair:abs(pair[1]-foot))if supports else(None,None);gap=abs(foot-nearest[1])if nearest[1]is not None else math.inf;checks.append({'angle_index':i,'xy':[x,y],'metal_foot':foot,'support':nearest[0],'support_top':nearest[1],'gap':gap});out['pass']=out['pass']and gap<=1e-8
    out['routes'].append({'route':route['id'],'status':'ANNULAR_BEARING_PROVEN'if checks and all(c['gap']<=1e-8 for c in checks)else'BEARING_FAILURE','checks':checks});out['pass']=out['pass']and bool(checks)
out['seconds']=time.monotonic()-started;(ticket/'🗑️generated'/'hollow-handrail-bearing-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'pass':out['pass'],'routes':[{k:v for k,v in r.items()if k!='checks'}|{'controls':len(r['checks']),'max_gap':max((c['gap']for c in r['checks']),default=None)}for r in out['routes']],'seconds':out['seconds']}),flush=True);raise SystemExit(not out['pass'])
