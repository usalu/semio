"""🚗 Construct one explicitly inferred gravel approach from original reserved XY operands."""
from __future__ import annotations
import importlib,json,math,sys,time,traceback,os,hashlib,platform
import importlib.metadata
from fractions import Fraction as Q
from pathlib import Path
from copy import deepcopy

ROOT=Path.cwd();TICKET=Path(__file__).parent;sys.path.insert(0,str(ROOT))
API=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
F=API._surface_finishes()
CASE=json.loads((Path(API.__file__).parent/'🧪️tests/🌿️gravel.json').read_text(encoding='utf-8'))
INPUT=json.loads((TICKET/'garage-approach-input-r94.json').read_text(encoding='utf-8'))
OUT=TICKET/'🗑️generated/native-r92';OUT.mkdir(parents=True,exist_ok=True)
START=time.monotonic();PEAK=0


def guard():
    """🌡️ Bound only this small authoring process with portable OS counters and cooperative cancellation."""
    global PEAK
    if os.name=='nt':
        import ctypes
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD)]+[(name,ctypes.c_size_t) for name in ('peak','rss','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile','private')]
        counters=Counters();counters.cb=ctypes.sizeof(counters);kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=wintypes.HANDLE;psapi=ctypes.WinDLL('psapi',use_last_error=True);psapi.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.c_void_p,wintypes.DWORD]
        if not psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(counters),counters.cb):raise OSError(ctypes.get_last_error())
        peak=counters.peak
    else:
        import resource
        peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024)
    PEAK=max(PEAK,peak)
    if PEAK>1932735283:raise MemoryError('Own1.8GiB authoring bound')
    if time.monotonic()-START>600:raise TimeoutError('Own600second authoring bound')
    if (OUT/'garage-approach-cancel').exists():raise InterruptedError('Own cooperative authoring cancellation')


def q(value):
    """🔢 Preserve declared decimal source operands before any affine operation."""
    return value if isinstance(value,Q) else Q(str(value))


def area(ring):
    """📐 Exact occupied area of one oriented convex source ring."""
    return abs(sum(a[0]*b[1]-a[1]*b[0] for a,b in zip(ring,ring[1:]+ring[:1])))/2


def bounds(ring):
    """📦 Bound only actual planar vertices for a conservative exact rejection."""
    return tuple(min(p[k] for p in ring) for k in range(2))+tuple(max(p[k] for p in ring) for k in range(2))


def overlaps(a,b):
    """🤝 Reject only an empty-area bounding intersection, without a tolerance."""
    return all(a[k+2]>b[k] and b[k+2]>a[k] for k in range(2))


def planes(ring):
    """✂️ Keep the exact CCW patch interior using original edge points and signed planes."""
    result=[]
    for a,b in zip(ring,ring[1:]+ring[:1]):
        normal=[b[1]-a[1],a[0]-b[0],Q(0)]
        result.append({'normal':normal,'offset':normal[0]*a[0]+normal[1]*a[1],'point':[a[0],a[1],Q(0)]})
    return result


def patch(name,ring,heights):
    """🧩 Derive one affine surface from its original three noncollinear controls."""
    a,b,c=ring[:3];u=[b[k]-a[k] for k in range(2)];v=[c[k]-a[k] for k in range(2)];den=u[0]*v[1]-u[1]*v[0]
    if den<=0:raise ValueError('Approach patches must be strictly positive CCW controls')
    h0,h1,h2=heights[:3];ax=((h1-h0)*v[1]-(h2-h0)*u[1])/den;by=(u[0]*(h2-h0)-v[0]*(h1-h0))/den
    return {'name':name,'ring':ring,'plane':(ax,by,h0-ax*a[0]-by*a[1]),'bounds':bounds(ring),'cuts':planes(ring)}


def construction():
    """🌀 Use a central two-sided grade and continuous semicircular end fans, never a cliff-edged apron."""
    spec=CASE['approach'];x,y0,y1=map(q,(spec['portal']['x'],spec['portal']['y0'],spec['portal']['y1']));run,rise=map(q,(spec['run'],spec['rise']));n=spec['sector_count'];base=q(CASE['reference_elevation'])
    if n%4 or y1<=y0 or run<=0 or rise<=0:raise ValueError('Invalid declared capsule construction')
    result=[patch('central-west',[[x-run,y0],[x,y0],[x,y1],[x-run,y1]],[base,Q(0),Q(0),base]),patch('central-east',[[x,y0],[x+run,y0],[x+run,y1],[x,y1]],[Q(0),base,base,Q(0)])]
    def direction(i):
        return tuple(q(v) for v in INPUT['directions'][i%n])
    for label,y,start in (('north',y0,n//2),('south',y1,0)):
        for i in range(start,start+n//2):
            u,v=direction(i),direction(i+1);ring=[[x,y],[x+run*u[0],y+run*u[1]],[x+run*v[0],y+run*v[1]]]
            result.append(patch(label+'-'+str(i),ring,[Q(0),base,base]))
    error=float(rise)*(1-math.cos(math.pi/n))
    if error>spec['maximum_vertical_facet_error']:raise ValueError('Declared angular surface approximation exceeds its height bound')
    return result,error


def original_stock(ring):
    """🪨 Retain one accepted convex reserved footprint and vertical40mm reference stock exactly."""
    if sum(a[0]*b[1]-a[1]*b[0] for a,b in zip(ring,ring[1:]+ring[:1]))<0:ring=list(reversed(ring))
    count=len(ring);base=q(CASE['reference_elevation']);depth=q(CASE['depth']);vertices=[[*p,base-depth] for p in ring]+[[*p,base] for p in ring]
    faces=[list(reversed(range(count))),list(range(count,2*count)),*[[i,(i+1)%count,(i+1)%count+count,i+count] for i in range(count)]]
    return F.clip_stock({'material':'MAT-GRAVEL','role':'wearing','mesh':{'vertices':vertices,'faces':faces}},[],exact=True)


def top_ring(state):
    """🧾 Read original exact top controls without inheriting a floating preview."""
    mesh=state._exact_mesh;base=q(CASE['reference_elevation'])
    face=next(face for face in mesh['faces'] if all(mesh['vertices'][index][2]==base for index in face))
    return [[mesh['vertices'][index][0],mesh['vertices'][index][1]] for index in face]


def difference(state,cuts):
    """🧵 Partition one convex stock by complementary ordered planes in one exact continuation."""
    cursor=state;outside=[]
    for plane in cuts:
        guard()
        inverse={**plane,'normal':[-v for v in plane['normal']],'offset':-plane['offset']}
        other=F.clip_stock(cursor,[inverse],exact=True)
        if other is not None:outside.append(other)
        cursor=F.clip_stock(cursor,[plane],exact=True)
        if cursor is None:break
    return outside,cursor


def graded(state,surface):
    """📤 Shear matching top/bottom vertices by one original affine field, then publish exactly once."""
    mesh=state._exact_mesh;a,b,c=surface;reference=q(CASE['reference_elevation'])
    vertices=[[x,y,z+a*x+b*y+c-reference] for x,y,z in mesh['vertices']]
    shifted=F.clip_stock({'material':'MAT-GRAVEL','role':'wearing','mesh':{'vertices':vertices,'faces':deepcopy(mesh['faces'])}},[],exact=True)
    if shifted is None:raise ValueError('A positive exact grading cell cannot disappear at publication')
    published=F.publish_stock(shifted)
    return {key:published[key] for key in ("mesh","material","role")}


def conforming_parts(cells):
    """🧵 Publish shared exact boundary vertices together, without moving any source plane."""
    def line(a,b):
        nx,ny=b[1]-a[1],a[0]-b[0]
        scale=nx if nx else ny
        return nx/scale,ny/scale,(nx*a[0]+ny*a[1])/scale
    nodes={}
    for ring,surface,row in cells:
        for a,b in zip(ring,ring[1:]+ring[:1]):
            if a==b:raise ValueError('Repeated exact source edge')
            nodes.setdefault(line(a,b),set()).update((tuple(a),tuple(b)))
    parts=[];rows=[]
    for ring,surface,row in cells:
        guard();refined=[]
        for a,b in zip(ring,ring[1:]+ring[:1]):
            axis=0 if abs(b[0]-a[0])>=abs(b[1]-a[1]) else 1
            selected=[p for p in nodes[line(a,b)] if min(a[axis],b[axis])<=p[axis]<=max(a[axis],b[axis])]
            selected.sort(key=lambda p:p[axis],reverse=b[axis]<a[axis])
            refined.extend([list(p) for p in selected[:-1]])
        if area(refined)!=area(ring):raise ValueError('Conforming edge subdivision changed exact material')
        centre=[sum(p[k] for p in refined)/len(refined) for k in range(2)]
        triangles=[refined] if len(refined)==3 else [[centre,a,b] for a,b in zip(refined,refined[1:]+refined[:1])]
        if sum(area(t) for t in triangles)!=area(ring):raise ValueError('Exact fan changed occupied material')
        for triangle in triangles:
            if not area(triangle)>0:raise ValueError('Positive exact material fan collapsed')
            parts.append(graded(original_stock(triangle),surface))
            rows.append({**row,'area':float(area(triangle)),'volume':float(area(triangle)*q(CASE['depth']))})
    return parts,rows


def build():
    """🏗️ Partition accepted source material; do not alter the original reservations or aggregate volume."""
    patches,error=construction();parts=[];rows=[];exact_cells=[];started=time.monotonic();reference=q(CASE['reference_elevation']);plane=(Q(0),Q(0),reference)
    for source_index,raw in enumerate(INPUT['reserved_cells']):
        guard()
        ring=[[q(v) for v in point] for point in raw];initial=area(ring);near=[p for p in patches if overlaps(bounds(ring),p['bounds'])]
        if not near:
            exact_cells.append((ring,plane,{'source':source_index,'patch':'flat'}));continue
        remaining=[original_stock(ring)];accepted=[]
        for patch_row in near:
            next_remaining=[]
            for state in remaining:
                local=top_ring(state)
                if not overlaps(bounds(local),patch_row['bounds']) or F.stock_separated(state,patch_row['cuts']):next_remaining.append(state);continue
                outside,inside=difference(state,patch_row['cuts']);next_remaining.extend(outside)
                if inside is not None:accepted.append((inside,patch_row))
            remaining=next_remaining
        accepted.extend((state,{'name':'flat','plane':plane}) for state in remaining)
        if sum(area(top_ring(state)) for state,_ in accepted)!=initial:raise ValueError(('Exact source area not conserved',source_index))
        for state,patch_row in accepted:
            exact_cells.append((top_ring(state),patch_row['plane'],{'source':source_index,'patch':patch_row['name']}))
        print('[DEBUG] garage source '+str(source_index)+' cells '+str(len(accepted)),flush=True)
    parts,rows=conforming_parts(exact_cells)
    return {'parts':parts,'base_level':0.,'cells':rows,'source_sha256':INPUT['original_source_sha256'],'source_product':CASE['product'],'depth':CASE['depth'],'reference_elevation':CASE['reference_elevation'],'facet_height_error':error,'maximum_declared_facet_height_error':CASE['approach']['maximum_vertical_facet_error'],'approach':CASE['approach'],'seconds':time.monotonic()-started,'peak_rss_bytes':PEAK,'publication':'CANDIDATE ONLY; native geometry/C0/volume/ENDE SDK and public quantity gates required'}


def apply_candidate(model,result):
    """📝 Apply only one candidate product and its localized inference metadata to an owned in-memory model."""
    obj=API._find(model,CASE['product']);obj['geometry']['parts']=result['parts'];obj['geometry']['base_level']=0.
    terrain=obj['psets']['Pset_TerrainReconstruction'];terrain['UniformExtension']['value']=False;terrain['UniformExtension']['source']='Actual local grading is nonuniform; the source-local reference remains−0.15m and no surveyed estate grade is established'
    spec=CASE['approach'];source='Explicitly inferred functional fit at the current inferred garage FFL0; run, rise, stations, fans and subdivision are not surveyed original grading'
    values={'GarageApproachInferred':(True,'IfcBoolean'),'GarageApproachRun':(spec['run'],'IfcReal'),'GarageApproachRise':(spec['rise'],'IfcReal'),'GarageApproachFanSegments':(spec['sector_count'],'IfcReal'),'GarageApproachMaximumHeightError':(spec['maximum_vertical_facet_error'],'IfcReal')}
    labels={'GarageApproachInferred':('Inferred local garage approach','Abgeleitete örtliche Garagenzufahrt'),'GarageApproachRun':('Inferred garage approach run (m)','Abgeleitete Zufahrtslänge zur Garage (m)'),'GarageApproachRise':('Inferred garage approach rise (m)','Abgeleiteter Zufahrtshöhenunterschied (m)'),'GarageApproachFanSegments':('Inferred full-circle fan sectors','Abgeleitete Fächersektoren je Vollkreis'),'GarageApproachMaximumHeightError':('Declared maximum vertical facet error (m)','Deklarierter maximaler vertikaler Facettenfehler (m)')}
    for key,(value,kind) in values.items():
        terrain[key]={'value':value,'ifc_type':kind,'basis':'inferred','source':source};model['property_names'][key]={'name':dict(zip(('en','de'),labels[key])),'unit':'m' if key in ('GarageApproachRun','GarageApproachRise','GarageApproachMaximumHeightError') else ''}
    model['property_names']['UniformExtension']['name']={'en':'Uniform actual terrain surface','de':'Einheitliche tatsächliche Geländeoberfläche'}
    model['property_names']['UniformExtension']['meaning']={'en':'Whether the actual represented terrain has a uniform top elevation; false when an explicitly inferred local approach is present','de':'Ob die tatsächlich dargestellte Geländeoberseite eine einheitliche Höhe hat; bei ausdrücklich abgeleiteter örtlicher Zufahrt falsch'}
    obj['attributes']['nominal_depth']['source']='Inferred40mm vertical gravel wearing depth, retained equally below the flat reference and local graded approach; not a measured normal-to-slope thickness'
    obj['historical_evidence']['uncertainty']={'en':'Gravel beneath the pilotis circumdrive is attested. Two Annex6 transverse sections mark local exterior terrain at−0.15m. The bounded19×21.5m extension, perimeter,40mm vertical depth, grain, subgrade and drainage remain inferred or unmeasured. A functional local garage approach joins this reference to the current inferred garage FFL0 using an inferred1.8m central1:12 transition and continuous semicircular planar fans, with at most100µm declared vertical faceting error. Actual historic garage grading and surveyed estate levels are not established. All original column/footing/wall/ground-slab reservations and product identity are retained; analytic circle reserves remain conservatively bounded by0.1mm. No estate surface, rail, drain or3D vehicle contact trajectory is reconstructed.','de':'Kies auf der Umfahrt unter den Pilotis ist belegt. Zwei Querschnitte in Anhang6 markieren das örtliche Außengelände bei−0,15m. Die begrenzte Fortsetzung über19×21,5m, der Rand,40mm vertikale Schichtdicke, Körnung, Untergrund und Entwässerung bleiben abgeleitet oder unvermessen. Eine örtliche funktionale Garagenzufahrt verbindet die Referenz mit dem derzeit abgeleiteten Garagen-Fertigfußboden0: angenommener1,8m langer Mittelstreifen mit1:12 und stetige halbkreisförmige ebene Fächer, höchstens100µm deklarierter vertikaler Facettenfehler. Ursprünglicher Zufahrtsverlauf und vermessene Grundstückshöhen sind nicht belegt. Alle bisherigen Stützen-/Fundament-/Wand-/Erdgeschossdeckenaussparungen und die Produktidentität bleiben erhalten; analytische Kreisreserven sind konservativ auf0,1mm begrenzt. Keine Grundstücksfläche, Schiene, Entwässerung oder3D-Fahrzeug-Aufstandsbahn wird rekonstruiert.'}
    return obj


def main():
    """🚦 Retain an actual old-flat RED before producing a single-product candidate."""
    metadata={'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'pid':os.getpid(),'versions':{name:importlib.metadata.version(name) for name in ('numpy','scipy','shapely')},'lock_sha256':hashlib.sha256((ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock').read_bytes()).hexdigest(),'normal_profile_mib':2048,'bounds':{'seconds':600,'rss_bytes':1932735283},'loaded_model_source_sha256':hashlib.sha256(API._source().encode('utf-8')).hexdigest(),'producer_source_sha256':{str(Path(module.__file__).relative_to(ROOT)):hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest() for module in (API,F)}}
    (OUT/'garage-approach-start-r94.json').write_text(json.dumps(metadata,ensure_ascii=True,indent=2),encoding='utf-8')
    print('[DEBUG] garage start '+json.dumps(metadata),flush=True)
    tests=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌿️gravel')
    try:tests.test_gravel_has_continuous_explicitly_inferred_garage_approach_at_actual_portal()
    except AssertionError:
        (OUT/'garage-approach-old-flat-red-r94.json').write_text(json.dumps({'rejected':True,'traceback':traceback.format_exc()},ensure_ascii=True,indent=2),encoding='utf-8')
    else:raise ValueError('The original flat source unexpectedly passes the new approach contract')
    result=build();result['runtime']=metadata;(OUT/'garage-approach-candidate-r94.json').write_text(json.dumps(result,ensure_ascii=True,indent=2),encoding='utf-8');print('[DEBUG] garage candidate '+json.dumps({key:value for key,value in result.items() if key not in ('parts','cells','approach')}),flush=True)


if __name__=='__main__':
    try:main()
    except BaseException:
        (OUT/'garage-approach-failure-r94.json').write_text(json.dumps({'error':traceback.format_exc(),'peak_rss_bytes':PEAK,'elapsed':time.monotonic()-START},ensure_ascii=True,indent=2),encoding='utf-8');raise
