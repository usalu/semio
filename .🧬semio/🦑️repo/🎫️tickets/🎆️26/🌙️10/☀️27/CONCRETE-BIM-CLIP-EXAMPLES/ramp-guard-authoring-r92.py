"""🛝️ Author continuous source-qualified concrete guards with exact construction and exclusive cast material."""
import copy,importlib.util,json,math,sys,time
from collections import Counter
from fractions import Fraction as Q
from pathlib import Path
sys.dont_write_bytecode=True
ticket=Path(__file__).parent;spec=importlib.util.spec_from_file_location('ramp_author',(ticket/'ramp-stringer-authoring-r92.py'));H=importlib.util.module_from_spec(spec);spec.loader.exec_module(H)
api,F,m,out=H.api,H.F,H.source,H.out;started=time.monotonic();raw=[];raw_rc=[];rows=[]
faces=[[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i] for i in range(4)]
def make(x0,x1,y0,y1,b0,b1,t0,t1,role,material='MAT-RENDERED'):
    """📐 Retain exact affine source coordinates until all material cuts and unions are complete."""
    v=[[Q(x0),Q(y0),Q(b0)],[Q(x1),Q(y0),Q(b0)],[Q(x1),Q(y1),Q(b1)],[Q(x0),Q(y1),Q(b1)],[Q(x0),Q(y0),Q(t0)],[Q(x1),Q(y0),Q(t0)],[Q(x1),Q(y1),Q(t1)],[Q(x0),Q(y1),Q(t1)]]
    return {'mesh':{'vertices':v,'faces':copy.deepcopy(faces)},'material':material,'role':role}

def add(guard,part):
    """🧾 Preserve logical product ownership separately from exact convex construction cells."""
    raw.append((guard,part))

flights={}
q=api._source_fraction
for parent in api._by_class(m,'IfcRamp'):
    for f in api._flights(m,parent):
        source=api._find(m,f['tag']);g=source['geometry'];stock=api._ramp_reference_stock(m,source);top=stock['mesh']['vertices'][4:];lo,hi=q(api._z(m,g['from_level'])),q(api._z(m,g['to_level']));w0,w1=[lo+(hi-lo)*q(portion) for portion in g['portion']]
        by_y={point[1]:point[2] for point in top};pair=sorted([(q(g['path'][0][1]),w0,by_y[q(g['path'][0][1])]),(q(g['path'][1][1]),w1,by_y[q(g['path'][1][1])])]);(y0,w0,r0),(y1,w1,r1)=pair
        flights[f['tag']]={'y0':y0,'y1':y1,'w0':w0,'w1':w1,'r0':r0,'r1':r1,'x0':min(p[0] for p in top),'x1':max(p[0] for p in top)};raw_rc.append(stock)
up=flights['RF-2B'];add('GUARD-RM-2',make(Q('9.44'),Q('9.56'),up['y0'],up['y1'],up['r0'],up['r1'],Q(up['r0'])+Q('.28'),Q(up['r1'])+Q('.28'),'upper_arrival_12x28_upstand','MAT-RC'))
for tag,flight,x in [('GUARD-RM-1','RF-1A','10.898'),('GUARD-RM-1','RF-1A','9.5'),('GUARD-RM-1','RF-1B','8.128'),('GUARD-RM-1','RF-1B','9.5'),('GUARD-RM-2','RF-2A','10.898'),('GUARD-RM-2','RF-2A','9.5'),('GUARD-RM-2','RF-2B','8.128')]:
    f=flights[flight];x=Q(x)
    add(tag,make(x-Q('.03'),x+Q('.03'),f['y0'],f['y1'],f['r0'],f['r1'],Q(f['w0'])+Q('.85'),Q(f['w1'])+Q('.85'),flight+'_continuous_guard_core'))
    add(tag,make(x-Q('.05'),x+Q('.05'),f['y0'],f['y1'],Q(f['w0'])+Q('.85'),Q(f['w1'])+Q('.85'),Q(f['w0'])+Q('.95'),Q(f['w1'])+Q('.95'),flight+'_continuous_guard_cap'))
for ramp,guard in [('RM-1','GUARD-RM-1'),('RM-2','GUARD-RM-2')]:
    g=api._find(m,ramp)['geometry'];mid=next(p for p in g['landings'] if p['ownership']=='ramp_component');walk=[[q(v) for v in p] for p in api._expand(m,mid['walking_outline'])];structure=[[q(v) for v in p] for p in api._expand(m,mid['outline'])];sx0,sx1=min(p[0] for p in structure),max(p[0] for p in structure);wx0,wx1=min(p[0] for p in walk),max(p[0] for p in walk);y0=min(p[1] for p in walk);wy1=max(p[1] for p in walk);sy1=max(p[1] for p in structure);floor=q(api._z(m,g['from_level']))+(q(api._z(m,g['to_level']))-q(api._z(m,g['from_level'])))*q(mid['portion']);rc=floor-q(g['finish_thickness']['value'])
    raw_rc.append({'outline':structure,'z0':rc-q(mid['thickness']['value']),'z1':rc,'material':'MAT-RC','role':'raw_'+ramp+'_landing'})
    for a,b,side in [(sx0,wx0,'west'),(wx1,sx1,'east')]:
        add(guard,make(a,b,y0,wy1,rc,rc,Q(floor)+Q('.85'),Q(floor)+Q('.85'),ramp+'_landing_'+side+'_core'))
        add(guard,make(Q(a)-Q('.02'),Q(b)+Q('.02'),y0,wy1,Q(floor)+Q('.85'),Q(floor)+Q('.85'),Q(floor)+Q('.95'),Q(floor)+Q('.95'),ramp+'_landing_'+side+'_cap'))
    add(guard,make(sx0,sx1,wy1,sy1,rc,rc,Q(floor)+Q('.85'),Q(floor)+Q('.85'),ramp+'_landing_end_core'))
    add(guard,make(Q(sx0)-Q('.02'),Q(sx1)+Q('.02'),Q(wy1)-Q('.02'),Q(sy1)+Q('.02'),Q(floor)+Q('.85'),Q(floor)+Q('.85'),Q(floor)+Q('.95'),Q(floor)+Q('.95'),ramp+'_landing_end_cap'))
f=flights['RF-1A'];raw_rc.append(make(f['x1'],Q(f['x1'])+Q('.12'),f['y0'],f['y1'],Q(f['r0'])-Q('.26'),Q(f['r1'])-Q('.26'),f['r0'],f['r1'],'raw_attested_stringer','MAT-RC'))
primary=[]
for obj in m['elements']:
    cls=obj['class']
    if cls not in ('IfcColumn','IfcBeam','IfcSlab','IfcFooting','IfcStair') or obj['tag']=='BEAM-RAMP-FIRST-STRINGER':continue
    if cls in ('IfcColumn','IfcBeam','IfcFooting'):
        try:parts=api._interface_reference_parts(m,obj['tag'])
        except ValueError:
            if cls=='IfcColumn':parts=api.column_geometry(m,obj)['parts']
            else:
                stock=copy.deepcopy(obj);stock['geometry']['joint_owners']=[];stock['geometry']['embedded_slabs']=[];parts=api._solid_geometry(m,stock,resolve_embedded=False)['parts']
    elif cls=='IfcSlab':
        stock=copy.deepcopy(obj);stock['geometry']['column_reserves']=[];stock['geometry']['finish_voids']=[];parts=api._slab_geometry(m,stock)['parts']
    else:parts=api._stair_geometry(m,obj)['structural_parts']
    for index,part in enumerate(parts):primary.append({'tag':obj['tag'],'class':cls,'index':index,'geometry':part})
owned=[];result={'GUARD-RM-1':[],'GUARD-RM-2':[]}
for index,(tag,part) in enumerate(raw):
    H.check();states=[F.clip_stock(part,[],exact=True)];bb=H.bounds(states[0]);count=0
    for record in primary:
        if not H.intersects(bb,H.bounds(record['geometry'])):continue
        for cutter in H.convex_cells(record):states=H.subtract(states,cutter)
        count+=1
    for cutter in raw_rc:
        if H.intersects(bb,H.bounds(cutter)):states=H.subtract(states,cutter)
    for _,cutter in owned:
        if H.intersects(bb,H.bounds(cutter)):states=H.subtract(states,cutter)
    owned.extend((tag,state) for state in states)
    rows.append({'guard':tag,'role':part['role'],'primary_candidates':count,'parts':len(states)})
    print('[DEBUG] guardstock '+str(index+1)+'/'+str(len(raw))+' '+part['role']+' cells='+str(len(states)),flush=True)
independent=H.importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔻️volume').independent_volume
prepublication=[{'guard':tag,'part':F.publish_stock(state)} for tag,state in owned]
(out/'ramp-guards-prevalidation-r92.json').write_text(json.dumps(prepublication,ensure_ascii=False),encoding='utf8')
volume={tag:0. for tag in result}
for tag,state in owned:
    H.check();part=F.publish_stock(state);native=api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces']);expected=independent(part['mesh']['vertices']);actual=api._polyhedron_volume(native)
    if expected<=0 or actual<=0 or not math.isclose(actual,expected,rel_tol=1e-7,abs_tol=1e-24):
        (out/'ramp-guard-volume-failure-r92.json').write_text(json.dumps({'guard':tag,'part':part,'expected':expected,'actual':actual},ensure_ascii=False),encoding='utf8')
        raise ValueError('A guard cell is not independent positive convex material: '+part['role'])
    edge=Counter(tuple(sorted((a,b))) for face in part['mesh']['faces'] for a,b in zip(face,face[1:]+face[:1]))
    if set(edge.values())!={2}:raise ValueError('A guard cell has an open native edge')
    result[tag].append({'mesh':part['mesh'],'material':part['material'],'role':part['role']});volume[tag]+=actual
report={'guards':result,'stocks':rows,'volume':volume,'source':H.hashlib.sha256(Path(api.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'seconds':time.monotonic()-started,'qualification':['Archive continuous6cm reinforced guard replaces midpoint stairs','Nominal95cm top and10cm cap remain inferred; unreadable groove detail not fabricated','Bases meet actual RC/stringer surfaces; upper12x28upstand height retained and attached to actual exterior RC plane','Actual primary RC owns its material; archive guard RC owns material before inferred masonry, wall interface partitions separately','Exact rational source stock and face planes retained through all cast unions; no per-cut float reseeding','Current original beam/slab structural stocks and physical openings are used before internal column/member ownership partition; rounded internal ownership seams never become guard cuts']}
(out/'ramp-guards-candidates-r92.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'parts':{k:len(v) for k,v in result.items()},'volume':volume,'seconds':report['seconds']}),flush=True)
                                                                                                                                                                                                                           