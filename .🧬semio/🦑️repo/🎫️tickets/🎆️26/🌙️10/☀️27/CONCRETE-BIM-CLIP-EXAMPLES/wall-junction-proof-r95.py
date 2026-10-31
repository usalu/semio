"""🔬 Capture and independently audit every declared actual wall group without altering shared source."""
import importlib,importlib.util,inspect,json,sys,hashlib,time,traceback,math
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/wall-junctions-r95';O.mkdir(parents=True,exist_ok=True);A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');S=Path(A.__file__).with_name('🔣️.json');spec=importlib.util.spec_from_file_location('wall_junction_source',P/'wall-junction-source-r95.py');H=importlib.util.module_from_spec(spec);spec.loader.exec_module(H)
spec=importlib.util.spec_from_file_location('native_proof_observer',P/'native-proof-observer-r95.py');V=importlib.util.module_from_spec(spec);spec.loader.exec_module(V);WATCH=None

def hashes():
    """🔐 Bind source and every actual first-party physical producer to the captured wall bodies."""
    paths={S,Path(A.__file__).parent/'🧬️schema/🔣️.schema.json',Path(H.__file__),Path(V.__file__),Path(__file__)}
    paths.update(Path(inspect.getsourcefile(inspect.unwrap(fn))) for fn in A._geometry_producers() if inspect.getsourcefile(inspect.unwrap(fn)))
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}

def capture():
    """📸 Resolve shared-context raw and owned walls from the same fresh complete source candidate."""
    started=time.monotonic();before=hashes();(O/'capture-start.json').write_text(json.dumps({'before':before,'started':started}),encoding='utf8');m,relations=H.apply(A,json.loads(S.read_text(encoding='utf8')));state=A._wall_junction_backend().context(A,m);tags=sorted(state['index']);rows=[]
    for index,tag in enumerate(tags):
        print('[DEBUG] Actual joined wall '+str(index+1)+'/'+str(len(tags))+' '+tag,flush=True);wall=A._find(m,tag);WATCH.progress({'phase':'actual_wall','tag':tag,'completed':index,'total':len(tags)});g=A._wall_geometry(m,wall,junctions=state,progress=WATCH.progress,cancel=WATCH.cancelled);raw=state['raw'][tag]
        rows.append(A._native_public_value({'tag':tag,'raw':raw['parts'],'owned':g['parts'],'reference':g['interface_reference_parts'],'raw_volume':raw['solid_volume'],'owned_volume':g['solid_volume'],'displacement':g['junction_displacement_volume']}))
        (O/'proof-progress.json').write_text(json.dumps({'stage':'capture','completed':index+1,'total':len(tags),'tag':tag}),encoding='utf8')
    after=hashes();out={'before':before,'after':after,'stable':before==after,'source_sha256':before[str(S.relative_to(R))],'candidate':m,'relations':relations,'rows':rows,'seconds':time.monotonic()-started};(O/'actual-capture.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8');print('[DEBUG] Actual wall capture',len(rows),'stable',out['stable'],'seconds',out['seconds'],flush=True)


def proof():
    """⚖️ Independently test positive contacts, union conservation and convex native volume for all assigned pairs."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    from scipy.spatial import ConvexHull
    import numpy as np
    spec=importlib.util.spec_from_file_location('wall_event_independent',P/'whole-expanded-interference-r92.py');M=importlib.util.module_from_spec(spec);spec.loader.exec_module(M)
    started=time.monotonic();data=json.loads((O/'actual-capture.json').read_text(encoding='utf8'));rows={row['tag']:row for row in data['rows']};solids={};volume_errors=[]
    for tag,row in rows.items():
        solids[tag]=[]
        for index,part in enumerate(row['owned']):
            solid=M.rawsolid(part);solids[tag].append(solid);points=np.asarray(A.part_vertices(part),dtype=float);points-=points.mean(axis=0);native=A._polyhedron_volume(part);hull=ConvexHull(points)
            if not math.isclose(hull.volume,native,rel_tol=1e-9,abs_tol=1e-14):volume_errors.append({'tag':tag,'index':index,'native':native,'independent':hull.volume})
            for face in A.part_faces(part):
                polygon=points[face];normal=next((np.cross(polygon[i]-polygon[0],polygon[i+1]-polygon[0]) for i in range(1,len(polygon)-1) if np.linalg.norm(np.cross(polygon[i]-polygon[0],polygon[i+1]-polygon[0]))>0),None)
                if normal is None:raise ValueError('Degenerate native face '+tag+':'+str(index))
                distances=(points-polygon[0])@(normal/np.linalg.norm(normal))
                if not (distances.max()<1e-10 or distances.min()>-1e-10):raise ValueError('Nonconvex source cannot use hull '+tag+':'+str(index))
    ledger=[tuple(c.strip() for c in line.split('|')[2:5:2]) for line in (P/'complete-conflict-ledger-r95.md').read_text(encoding='utf8').splitlines() if line.startswith('| Native |')];groups=[]
    for first,second in ledger:
        positives=[];errors=[];checked=0
        for i,a in enumerate(solids[first]):
            for j,b in enumerate(solids[second]):
                if not all(max(a.v[:,k].min(),b.v[:,k].min())<min(a.v[:,k].max(),b.v[:,k].max()) for k in (0,1,2)):continue
                checked+=1
                try:
                    result=M.intersection(a,b)
                    if result['volume']>0:positives.append({'source_index':i,'target_index':j,**result})
                except Exception:errors.append({'source_index':i,'target_index':j,'error':traceback.format_exc()})
        groups.append({'source':first,'target':second,'checked':checked,'positive':positives,'unresolved':errors});print('[DEBUG] Wall group '+first+'/'+second+' positive='+str(len(positives))+' unresolved='+str(len(errors)),flush=True);(O/'proof-progress.json').write_text(json.dumps({'stage':'pairs','completed':len(groups),'total':len(ledger),'source':first,'target':second}),encoding='utf8')
    raw=[M.rawsolid(part) for row in rows.values() for part in row['raw']];owned=[part for values in solids.values() for part in values];zs=sorted({float(v[2]) for solid in raw+owned for v in solid.v});baseline=0.;combined=0.;union_errors=[]
    for lo,hi in zip(zs,zs[1:]):
        if hi<=lo:continue
        midpoint=(lo+hi)/2
        for fraction,weight in ((.5-math.sqrt(3)/6,.5),(.5+math.sqrt(3)/6,.5)):
            z=lo+(hi-lo)*fraction;a=unary_union([solid.section(z) for solid in raw if solid.lo<z<solid.hi]);b=unary_union([solid.section(z) for solid in owned if solid.lo<z<solid.hi]);difference=a.symmetric_difference(b).area
            baseline+=a.area*(hi-lo)*weight;combined+=b.area*(hi-lo)*weight
            if difference>1e-10:union_errors.append({'z':z,'area':difference})
    native=sum(row['owned_volume'] for row in rows.values());out={'success':not volume_errors and not union_errors and not any(group['positive'] or group['unresolved'] for group in groups) and math.isclose(native,baseline,rel_tol=1e-9,abs_tol=1e-8),'capture_stable':data['stable'],'current':hashes()==data['after'],'capture_sha256':hashlib.sha256((O/'actual-capture.json').read_bytes()).hexdigest(),'groups':groups,'volume_errors':volume_errors,'union_errors':union_errors,'raw_union_volume':baseline,'owned_union_volume':combined,'native_owned_sum':native,'seconds':time.monotonic()-started};(O/'actual-proof.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items() if k not in ('groups','volume_errors','union_errors')}),flush=True)
    assert out['success']

if __name__=='__main__':
    mode=sys.argv[1];source=json.loads(S.read_text(encoding='utf8'))
    with V.Observer(O,len(source.get('elements',()))+len(source.get('spaces',()))) as WATCH:
        if mode in ('capture','all'):
            try:capture()
            except BaseException:
                (O/'capture-failure.json').write_text(json.dumps({'success':False,'error':traceback.format_exc(),'after':hashes(),'bound':WATCH.reason},indent=2),encoding='utf8');raise
        if mode in ('proof','all'):proof()
