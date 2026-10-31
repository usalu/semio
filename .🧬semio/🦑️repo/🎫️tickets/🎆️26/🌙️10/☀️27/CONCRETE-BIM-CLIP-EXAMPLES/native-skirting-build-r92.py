"""🪵 Build source-qualified proud timber boards against exact current native material; publish separately after audit."""
from collections import defaultdict
from copy import deepcopy
from fractions import Fraction
import hashlib,importlib,json,math,time,sys,cProfile,pstats,io,threading,_thread
from pathlib import Path
import numpy as np
from shapely import constrained_delaunay_triangles
from shapely.geometry import Polygon,Point,LineString
from shapely.geometry.polygon import orient
from shapely.ops import unary_union
from shapely.strtree import STRtree
BASE=Path(__file__).parent
MODEL_HELPER=Path(sys.argv[sys.argv.index('--model-helper')+1]).resolve() if '--model-helper' in sys.argv else None
OUT=BASE/'🗑️generated'/sys.argv[sys.argv.index('--output-subfolder')+1] if '--output-subfolder' in sys.argv else BASE/'🗑️generated/native-r92'
assert OUT.resolve().is_relative_to((BASE/'🗑️generated').resolve())
OUT.mkdir(parents=True,exist_ok=True)
sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
finish=api._surface_finishes()
START=time.monotonic()
SECTION={'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8}
MATERIAL='MAT-SKIRTING-GRAY'
def snapshot():
    """🧾 Bind results to actual source and first-party producer bytes."""
    domain=Path(api.__file__).parent
    paths=[p for p in domain.rglob('*.py') if not any('🧪' in part or part=='__pycache__' for part in p.relative_to(domain).parts)]
    paths.extend([Path(api.__file__).with_name('🔣️.json'),Path(api.__file__).parent/'🧬️schema/🔣️.schema.json',Path(__file__),BASE/'native-cupboard-skirting-r92.py',BASE/'cupboard-skirting-register-r92.json',BASE/'native-inclined-skirting-r92.py',BASE/'inclined-skirting-cases-r92.json',Path.cwd()/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml',Path.cwd()/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock'])
    if MODEL_HELPER:
        import runpy
        paths.append(MODEL_HELPER);paths.extend(runpy.run_path(str(MODEL_HELPER)).get('INPUTS',()))
    return {str(p.relative_to(Path.cwd())):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))}
def progress(phase,**values):
    """⏳ Persist bounded progress and respect the existing cancellation file."""
    if (OUT/'skirting.cancel').exists():raise KeyboardInterrupt('Skirting authoring cancelled')
    (OUT/'skirting-progress.json').write_text(json.dumps({'phase':phase,'elapsed':time.monotonic()-START,**values}),encoding='utf8')
    print('[DEBUG] '+phase+' '+json.dumps(values),flush=True)
def planes(part):
    """🔻 Obtain verified convex native face half-spaces without replacing a nonconvex source body by its hull."""
    original=api.part_vertices(part);vertices=np.asarray(original,float);result=[]
    for face in api.part_faces(part):
        p=vertices[face];normal=sum((np.cross(p[i]-p[0],p[i+1]-p[0]) for i in range(1,len(p)-1)),np.zeros(3));norm=np.linalg.norm(normal)
        if norm==0:continue
        normal/=norm;offset=float(normal@p[0])
        if np.max(vertices@normal-offset)>1e-9:raise ValueError('Native cutter is not convex')
        triple=max(((face[0],face[i],face[i+1]) for i in range(1,len(face)-1)),key=lambda q:np.linalg.norm(np.cross(vertices[q[1]]-vertices[q[0]],vertices[q[2]]-vertices[q[0]])))
        result.append({'normal':normal.tolist(),'offset':offset,'points':[original[i] for i in triple]})
    return result
def planar_planes(polygon,subject_boundary=None):
    """📐 Keep exactly one convex GEOS cell through first-party vertical half-spaces."""
    points=list(orient(polygon,1).exterior.coords)[:-1];result=[]
    for a,b in zip(points,points[1:]+points[:1]):
        if subject_boundary is not None and subject_boundary.covers(LineString([a,b])):continue
        dx,dy=b[0]-a[0],b[1]-a[1];length=math.hypot(dx,dy)
        result.append({'normal':[dy/length,-dx/length,0.],'offset':(dy*a[0]-dx*a[1])/length,'points':[[*a,0.],[*b,0.],[*a,1.]]})
    return result
def keep_plan(stock,shape):
    """🧩 Partition exact support/joint domains into nonoverlapping convex cells."""
    if shape.is_empty:return []
    if shape.covers(Polygon(stock["outline"])):return [stock]
    if not shape.is_valid:raise ValueError('Invalid authored support polygon')
    polygons=[shape] if shape.geom_type=='Polygon' else list(shape.geoms)
    result=[]
    for polygon in polygons:
        if polygon.area==0:continue
        cells=[polygon] if not polygon.interiors and polygon.equals(polygon.convex_hull) else list(constrained_delaunay_triangles(polygon).geoms)
        for cell in cells:
            if not polygon.covers(cell):raise ValueError('Independent triangulation leaves its source domain')
            try:part=finish.clip_stock(stock,planar_planes(cell,Polygon(stock["outline"]).boundary))
            except ValueError:
                (OUT/"skirting-invalid-domain.json").write_text(json.dumps({"stock":stock,"cell":list(cell.exterior.coords),"planes":planar_planes(cell)},indent=2),encoding="utf8");raise
            if part is not None:result.append(part)
    return result
def subtract(parts,cutter_planes):
    """✂️ Retain the exact complement of one convex physical material cutter."""
    result=[]
    for part in parts:
        if finish.stock_separated(part,cutter_planes):
            result.append(part)
            continue
        remaining=part
        for plane in cutter_planes:
            if remaining is None:break
            outside=finish.clip_stock(remaining,[{'normal':[-n for n in plane['normal']],'offset':-plane['offset'],**({'point':plane['point']} if 'point' in plane else {}),**({'points':plane['points']} if 'points' in plane else {}),**({'clearance':-plane['clearance']} if 'clearance' in plane else {})}])
            if outside is not None:result.append(outside)
            remaining=finish.clip_stock(remaining,[plane])
    return result
def cutter_planes(cutter):
    """🔻 Decompose only planar nonconvex prisms while retaining actual spatial cutter cells."""
    try:return [planes(cutter)]
    except ValueError:
        if 'mesh' in cutter or 'top_outline' in cutter or 'section' in cutter:raise ValueError('Nonconvex spatial cutter requiring exact decomposition')
        polygon=Polygon(cutter['outline'],cutter.get('holes',[]))
        cells=list(constrained_delaunay_triangles(polygon).geoms)
        if not all(polygon.covers(cell) for cell in cells):raise ValueError('Cutter triangulation left occupied material')
        return [planar_planes(cell)+[{'normal':[0,0,-1],'offset':-cutter['z0']},{'normal':[0,0,1],'offset':cutter['z1']}] for cell in cells]

before=snapshot();m=api.load_model()
if MODEL_HELPER:
    import runpy
    m=runpy.run_path(str(MODEL_HELPER))['candidate'](api,m)
model_signature=hashlib.sha256(json.dumps(m,sort_keys=True,separators=(',',':'),allow_nan=False).encode()).hexdigest()
progress('geometry')
cache_path=OUT/'skirting-geometry.json'
cache=json.loads(cache_path.read_text()) if cache_path.exists() else {}
diagnostic_cache='--diagnostic-cache' in sys.argv
geometry_inputs=lambda inputs:{key:value for key,value in inputs.items() if not Path(key).is_relative_to(BASE.relative_to(Path.cwd())) and not Path(key).name.endswith('.schema.json')}
if geometry_inputs(cache.get('snapshot',{}))==geometry_inputs(before) and cache.get('model_canonical_sha256')==model_signature or diagnostic_cache and cache.get('geometry'):
    g=cache['geometry'];progress('cached-geometry',diagnostic=geometry_inputs(cache.get('snapshot',{}))!=geometry_inputs(before))
    if not diagnostic_cache:cache_path.write_text(json.dumps({'snapshot':before,'model_canonical_sha256':model_signature,'geometry':g},ensure_ascii=False),encoding='utf8')
else:
    stopped=threading.Event()
    def observe_geometry():
        """⏳ Expose cold build progress and forward the user's cancellation request without publishing incomplete geometry."""
        tick=time.monotonic()
        while not stopped.wait(.5):
            if (OUT/'skirting.cancel').exists():_thread.interrupt_main();return
            if time.monotonic()-tick>=10:progress('geometry');tick=time.monotonic()
    monitor=threading.Thread(target=observe_geometry,daemon=True);monitor.start()
    profiler=cProfile.Profile() if '--profile-geometry' in sys.argv else None
    if profiler:profiler.enable()
    try:g=api.building_geometry(m)
    finally:
        if profiler:profiler.disable()
        stopped.set();monitor.join(timeout=1)
    if profiler:
        profiler.dump_stats(str(OUT/'skirting-geometry.pstats'))
        stream=io.StringIO();pstats.Stats(profiler,stream=stream).sort_stats('cumulative').print_stats(40)
        (OUT/'skirting-geometry-profile.txt').write_text('\n'.join('[DEBUG] '+line for line in stream.getvalue().splitlines()),encoding='utf8')
    cache_path.write_text(json.dumps({'snapshot':before,'model_canonical_sha256':model_signature,'geometry':g},ensure_ascii=False),encoding='utf8')
progress('geometry-complete',products=len(m['elements']))
objects=[o for name in ('walls','screens','pilotis','slabs','fillings','stairs','details','furnishings','beams','foundations','railings','services') for o in g[name]]+[g['curtain_wall']]+g['ramp']['ramps']+g['ramp']['flights']
reference_key={'kinds':['ramp_finish','owned_ramp'],'model':hashlib.sha256(json.dumps(m,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest(),'producers':{key:value for key,value in before.items() if Path(key).suffix=='.py' and Path(key).is_relative_to(Path(api.__file__).parent.relative_to(Path.cwd()))}}
reference_path=OUT/'skirting-floor-refs.json'
def decode_reference(value):
    """🔢 Read only exact rational data from a source-bound generated artifact."""
    return Fraction(*map(int,value['$rational'])) if set(value)=={'$rational'} else value
reference_cache=json.loads(reference_path.read_text(encoding='utf8'),object_hook=decode_reference) if reference_path.exists() else {}
if reference_cache.get('key')==reference_key:
    construction_refs=reference_cache['values'];progress('exact-floor-reference-cache',products=len(construction_refs))
else:
    ref_done=threading.Event()
    def observe_refs():
        """⏳ Keep original floor construction observable without a native traceback watchdog."""
        while not ref_done.wait(10):
            print('[DEBUG] original-Q floor references working',flush=True)
            if (OUT/'skirting.cancel').exists():_thread.interrupt_main();return
    ref_monitor=threading.Thread(target=observe_refs,daemon=True);ref_monitor.start()
    try:
        construction_refs={}
        for obj in objects:
            source=api._find(m,obj['tag'])
            if source['geometry'].get('kind')=='ramp_finish':construction_refs[obj['tag']]=api._ramp_finish_reference_parts(m,source)
            elif source['class'] in ('IfcRamp','IfcRampFlight'):
                progress('exact-ramp-reference',tag=source['tag'])
                construction_refs[obj['tag']]=api._ramp_owned_reference_parts(m,source,cancel=lambda:(OUT/'skirting.cancel').exists())
    finally:ref_done.set();ref_monitor.join()
    reference_path.write_text(json.dumps({'key':reference_key,'values':construction_refs},default=lambda value:{'$rational':[str(value.numerator),str(value.denominator)]}),encoding='utf8')
    progress('exact-floor-reference-complete',products=len(construction_refs))
allparts=[(obj['tag'],part) for obj in objects for part in construction_refs.get(obj['tag'],obj.get('parts',[]))]
errors=[];products=[];counts={};probes=[];traces=[]
trace_targets={}
if '--trace-native' in sys.argv:
    for item in json.loads((OUT/'skirting-candidate-native-r94.json').read_text())['failures']:trace_targets.setdefault(item['tag'],set()).add(item['source_item_index'])
def trace_stock(tag,index,stock):
    """🔬 Retain only diagnosed native-cell construction origins and ordered exact planes."""
    if index in trace_targets.get(tag,set()):traces.append({'tag':tag,'index':index,'origin':stock._origin,'planes':stock._planes})
import runpy
inclined_module=runpy.run_path(str(BASE/'native-inclined-skirting-r92.py'))
allparts=[(tag,inclined_module['native_envelope'](api,part)) for tag,part in allparts]
partial_inclined='--inclined-only' in sys.argv
for level in (() if partial_inclined else ('ground','first')):
    floor=g['storeys'][level];progress('storey',level=level)
    rooms=unary_union([Polygon(s['outline']) for s in g['spaces'] if s['storey']==level and s['predefined_type']=='INTERNAL'])
    wearing=unary_union([Polygon(p['outline']) for o in g['details'] if o['storey']==level and o.get('predefined_type')=='FLOORING' for p in o['parts'] if p.get('role')=='wearing' and abs(p['z1']-floor)<1e-9])
    supports=rooms.intersection(wearing).buffer(-.00005,join_style=2)
    bearing_tags={o['tag'] for o in g['details'] if o['storey']==level and o.get('predefined_type')=='FLOORING' and any(p.get('role')=='wearing' and abs(p['z1']-floor)<1e-9 for p in o['parts'])}
    host_tags={o['tag'] for o in g['walls']+g['screens']+g['beams']+g['pilotis'] if 'level' in inclined_module['placement_modes'](api._find(m,o['tag']).get('geometry',{}).get('kind'))}
    occupied=[]
    for obj in objects:
        if obj['tag'] in bearing_tags:continue
        for part in obj.get('parts',[]):
            band=inclined_module['occupied_band'](api,part,floor,floor+SECTION['height'])
            if not band.is_empty:occupied.append(band if obj['tag'] in host_tags else band.buffer(.0001,join_style=2))
    if occupied:supports=supports.difference(unary_union(occupied))
    support_polygons=[supports] if supports.geom_type=='Polygon' else list(supports.geoms)
    support_cells=[cell for polygon in support_polygons for cell in ([polygon] if not polygon.interiors and polygon.equals(polygon.convex_hull) else list(constrained_delaunay_triangles(polygon).geoms))]
    support_tree=STRtree(support_cells)
    wallrecords=[]
    for obj in g['walls']+g['screens']+g['beams']+g['pilotis']:
        if 'level' not in inclined_module['placement_modes'](api._find(m,obj['tag']).get('geometry',{}).get('kind')):continue
        for part in obj['parts']:
            shape=inclined_module['occupied_band'](api,part,floor,floor+SECTION['height'])
            if shape.is_empty:continue
            if part.get('profile',{}).get('shape')=='circle':
                centre=part['centroid'];radius=part['radius']/math.cos(math.pi/128)+2e-9
                shape=Polygon([[centre[0]+radius*math.cos(2*math.pi*i/128),centre[1]+radius*math.sin(2*math.pi*i/128)] for i in range(128)])
            for polygon in [shape] if shape.geom_type=='Polygon' else shape.geoms:
                if not polygon.is_empty:wallrecords.append((obj['tag'],polygon))
    joint_edges=defaultdict(list)
    for host,polygon in wallrecords:
        coords=list(polygon.exterior.coords)[:-1]
        for a,b in zip(coords,coords[1:]+coords[:1]):
            for p in (a,b):joint_edges[(host,tuple(p))].append((a,b))
    union=unary_union([p for _,p in wallrecords]);wallpolygons=[union] if union.geom_type=='Polygon' else list(union.geoms)
    candidates=[]
    hosttree=STRtree([polygon.boundary for _,polygon in wallrecords])
    pathframes={}
    for obj in g['walls']+g['screens']:
        path=obj.get('path',[obj.get('start'),obj.get('end')])
        pathframes[obj['tag']]=[(LineString([a,b]),(np.asarray(b)-a)/np.linalg.norm(np.asarray(b)-a)) for a,b in zip(path,path[1:]) if a!=b]
    for polygon in wallpolygons:
        polygon=orient(polygon,1)
        for ring in [polygon.exterior,*polygon.interiors]:
            pts=inclined_module['stock_boundary'](list(ring.coords)[:-1]);runs=[]
            for a,b in zip(pts,pts[1:]+pts[:1]):
                delta=np.asarray(b)-a;length=np.linalg.norm(delta)
                if length==0:continue
                direction=delta/length;normal=[direction[1],-direction[0],0.]
                origin=Point((a[0]+b[0])/2,(a[1]+b[1])/2)
                if not supports.covers(Point(origin.x+normal[0]*.0075,origin.y+normal[1]*.0075)):continue
                host=wallrecords[min(hosttree.query_nearest(origin).tolist())][0]
                if host in pathframes:
                    frame=min(pathframes[host],key=lambda row:row[0].distance(origin))[1]
                    if abs(direction[0]*frame[1]-direction[1]*frame[0])>1e-6:continue
                square=lambda p,n:{'plane':{'normal':list(n),'offset':sum(x*y for x,y in zip(p,n)),'point':list(p)},'extension':SECTION['thickness']}
                runs.append({'host':host,'axis_a':[*a,floor],'axis_b':[*b,floor],'a':np.array([*a,floor]),'b':np.array([*b,floor]),'d':np.array([*direction,0.]),'normal':normal,'start':square([*a,floor],np.array([*direction,0.])),'end':square([*b,floor],np.array([*direction,0.]))})
            for index,run in enumerate(runs):
                following=runs[(index+1)%len(runs)];d0,d1=run['d'][:2],following['d'][:2];delta=following['a'][:2]-run['b'][:2]
                cross=lambda a,b:a[0]*b[1]-a[1]*b[0]
                determinant=cross(d0,d1);point=None
                if abs(determinant)>1e-10:point=run['b'][:2]+d0*cross(delta,d1)/determinant
                elif d0@d1>0 and abs(cross(delta,d0))<1e-9:point=(run['b'][:2]+following['a'][:2])/2
                if point is None or np.linalg.norm(point-run['b'][:2])>2*SECTION['thickness'] or np.linalg.norm(point-following['a'][:2])>2*SECTION['thickness']:continue
                point=np.array([*point,floor]);joint=finish.skirting_joint(run['a'].tolist(),point.tolist(),following['b'].tolist(),SECTION['thickness'])
                edges=joint_edges.get((run['host'],tuple(point[:2])),[])
                matched=[]
                for a,b in edges:
                    delta=np.asarray(b)-a;normal=np.array([delta[1],-delta[0],0.]);normal/=np.linalg.norm(normal)
                    agreement=float(normal@np.asarray(joint['plane']['normal']))
                    if abs(agreement)>.99:matched.append((abs(agreement),a,b,normal if agreement>0 else -normal))
                if matched:
                    _,a,b,normal=max(matched,key=lambda x:x[0]);joint['plane']={'normal':normal.tolist(),'offset':float(normal@point),'points':[[*a,floor],[*b,floor],[*a,floor+1]]}
                run['b']=point;following['a']=point;run['end']=joint;following['start']=joint
            candidates.extend((r['host'],r['a'].tolist(),r['b'].tolist(),r['normal'],r['start'],r['end'],r['axis_a'],r['axis_b']) for r in runs)
    relevant=[(tag,p,Polygon(api.part_footprint(p))) for tag,p in allparts if p['z0']<floor+.09 and p['z1']>floor]
    tree=STRtree([p for _,_,p in relevant]);cache={};authored=defaultdict(list)
    for index,(host,a,b,normal,start_joint,end_joint,axis_a,axis_b) in enumerate(candidates):
        if index%20==0:progress('boards',level=level,index=index,total=len(candidates),parts=sum(map(len,authored.values())))
        direction=(np.asarray(b)-a)/np.linalg.norm(np.asarray(b)-a)
        extended_a=np.asarray(a)-direction*start_joint['extension'];extended_b=np.asarray(b)+direction*end_joint['extension']
        extended=extended_b-extended_a;extended/=np.linalg.norm(extended);normal=[extended[1],-extended[0],0.]
        axis=(np.asarray(axis_b)-axis_a);axis/=np.linalg.norm(axis)
        stock=finish.skirting_stock(axis_a,axis_b,[axis[1],-axis[0],0.],SECTION,MATERIAL,extensions=(start_joint["extension"]+float(np.linalg.norm(np.asarray(axis_a)-a)),end_joint["extension"]+float(np.linalg.norm(np.asarray(axis_b)-b))),back_clearance=.0001)
        stock=finish.clip_stock(stock,inclined_module['fitted_joints']([{'normal':[-n for n in start_joint['plane']['normal']],'offset':-start_joint['plane']['offset'],**({'point':start_joint['plane']['point']} if 'point' in start_joint['plane'] else {}),**({'points':start_joint['plane']['points']} if 'points' in start_joint['plane'] else {})},end_joint['plane']],.0001),exact=True)
        if stock is None:continue
        footprint=Polygon(stock['outline'])
        parts=inclined_module['fit_support'](finish,stock,supports,axis_a,np.asarray(axis_b)-axis_a,end_clearance=.0001)
        for hit in tree.query(footprint):
            tag,cutter,outline=relevant[int(hit)]
            if footprint.intersection(outline).area==0:continue
            key=int(hit)
            if key not in cache:cache[key]=cutter_planes(cutter)
            for cutter_faces in cache[key]:
                parts=subtract(parts,cutter_faces)
        if parts:
            validated=[]
            for part in parts:
                try:api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces'])
                except ValueError:
                    part=finish.replay_stock(part)
                    if part is None:continue
                constructed=part;trace_stock('SKIRT-'+host+'-'+level,len(authored[host])+len(validated),constructed);part=finish.publish_stock(constructed)
                part['mesh']['vertices']=[[v[0],v[1],v[2]-floor] for v in part['mesh']['vertices']];part['z0']-=floor;part['z1']-=floor
                try:api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces'])
                except ValueError:
                    (OUT/'skirting-invalid-part.json').write_text(json.dumps({'host':host,'candidate':index,'start':a,'end':b,'normal':normal,'part':part,'origin':constructed._origin,'planes':constructed._planes},ensure_ascii=False,indent=2,default=str),encoding='utf8');raise
                validated.append(part)
            authored[host]+=validated
    for host,parts in authored.items():
        tag='SKIRT-'+host+'-'+level
        obj={'tag':tag,'guid':api.global_id(tag),'class':'IfcCovering','predefined_type':'SKIRTINGBOARD','name':{'en':'Original 15×90 mm gray painted timber skirting at '+host+'; joints inferred','de':'Ursprüngliche graue 15×90 mm Holzsockelleiste an '+host+'; Fugen abgeleitet'},'storey':level,'material':MATERIAL,'geometry':{'kind':'skirting','base_level':level,'parts':parts},'attributes':{},'psets':{},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']},'historical_evidence':{'state':'original-completed-1930-1931','status':'inferred_reconstruction','source_kind':'mixed','sources':['SAB2024-p190; original FLC L2(17)059/061 photographs'],'uncertainty':{'en':'Painted timber 15×90 mm and slightly rounded upper edge are source-attested.15 mm timber thickness plus 0.1 mm installation back-fitting and end clearances plus 0.05 mm inset from wearing-floor support boundaries (inferred, not measured historic gaps), 2 mm radius/eight facets, gray display RGB, coordinated mitre joints and exact host endpoints are inferred. Circular shaft contacts use a circumscribed 128-facet timber contour, up to 0.15 mm including installation clearance from the analytic shaft; actual wearing floor supplies bearing. Physical wall, door, furniture and floor contacts derive from current native solids; original fasteners and pigment codes are unmeasured.','de':'Gestrichenes Holz 15×90 mm mit leicht gerundeter Oberkante ist belegt.15 mm Holzdicke mit 0.1 mm abgeleiteter Einpassluft an Rückseite und Enden (keine historisch gemessene Fuge), 2 mm Radius/acht Facetten, graues Anzeige-RGB, Fugen und genaue Host-Endpunkte sind abgeleitet. Wand-, Tür-, Möbel- und Bodenkontakte folgen aktuellen nativen Körpern; ursprüngliche Befestigungen und Pigmentcodes sind ungemessen.'}}}
        products.append(obj)
    counts[level]={'candidates':len(candidates),'products':len(authored),'parts':sum(map(len,authored.values()))}
    progress('storey-complete',level=level,**counts[level])
import runpy
front_module=runpy.run_path(str(BASE/'native-cupboard-skirting-r92.py'))
fronts=front_module['build'](m,api,finish)
products.extend(fronts['elements']);counts['cupboard_fronts']={'products':len(fronts['elements']),'parts':sum(len(obj['geometry']['parts']) for obj in fronts['elements'])}
inclined_module=runpy.run_path(str(BASE/'native-inclined-skirting-r92.py'))
inclined=inclined_module['build'](m,g,api,finish,{'section':SECTION,'material':MATERIAL,'progress':progress,'objects':objects,'allparts':allparts,'planar_planes':planar_planes,'cutter_planes':cutter_planes,'subtract':subtract,'coverage':probes,'construction_refs':construction_refs,'trace_stock':trace_stock})
products.extend(inclined['elements']);counts['interior_ramp']=inclined['counts']
for obj in products:obj['geometry']['parts']=[inclined_module['source_part'](part) for part in obj['geometry']['parts']]
after=snapshot();report={'stable':before==after and not diagnostic_cache and not partial_inclined and MODEL_HELPER is None,'source_stable':before==after,'unpublished_model_helper':str(MODEL_HELPER) if MODEL_HELPER else None,'model_canonical_sha256':model_signature,'partial_inclined_only':partial_inclined,'geometry_snapshot':cache.get('snapshot') if diagnostic_cache else before,'diagnostic_cache':diagnostic_cache,'before':before,'after':after,'counts':counts,'inclined_coverage':probes,'elements':products,'errors':errors,'elapsed':time.monotonic()-START,'scope':'Interior combined wall/beam/pilotis contours, actual inclined RM1 and middle-landing wearing planes, and continuous visible cupboard fronts; final physical/source publication gates required'}
if traces:(OUT/'skirting-native-origins-r94.json').write_text(json.dumps(traces,default=str,indent=2),encoding='utf8')
(OUT/'skirting-candidate.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');progress('complete',stable=report['stable'],counts=counts)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        