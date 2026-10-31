"""🪵 Independently validate complete authored stock, native cells and actual floor bearing before source publication."""
import hashlib,importlib,json,runpy,sys,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely.strtree import STRtree
from jsonschema import Draft202012Validator
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated'/sys.argv[sys.argv.index('--output-subfolder')+1] if '--output-subfolder' in sys.argv else BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');helper=runpy.run_path(str(BASE/'native-skirting-source-r92.py'))
model_binder=BASE/'native-hall-bind-r95.py' if '--unpublished-hall' in sys.argv else None
validation_paths=[Path(__file__),BASE/'native-skirting-source-r92.py']+([model_binder,BASE/'native-hall-source-r95.py',BASE/'roof-layer-binding-source-r95.py'] if model_binder else [])
validation_inputs={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in validation_paths}
candidate=json.loads((OUT/'skirting-candidate.json').read_text());assert (candidate['source_stable'] and bool(candidate['unpublished_model_helper']) and not candidate['stable'] if model_binder else candidate['stable']) and not candidate['partial_inclined_only'] and not candidate['diagnostic_cache'] and not candidate['errors']
for name,value in candidate['after'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==value,('changed-input',name)
cache=json.loads((OUT/'skirting-geometry.json').read_text());assert cache['snapshot']==candidate['before'];g=cache['geometry'];source=a.load_model();m=runpy.run_path(str(model_binder))['prepare'](a,source,candidate) if model_binder else helper['with_skirting'](a,source,candidate['elements']);schema=json.loads((Path(a.__file__).parent/'🧬️schema/🔣️.schema.json').read_text());validation_errors=sorted(Draft202012Validator(schema).iter_errors(m),key=lambda error:str(list(error.absolute_path)))
if validation_errors:
    error=validation_errors[0];(OUT/'skirting-schema-error.json').write_text(json.dumps({'path':list(error.absolute_path),'schema_path':list(error.absolute_schema_path),'message':error.message,'context':[{'path':list(child.absolute_path),'message':child.message} for child in error.context]},ensure_ascii=False,indent=2),encoding='utf8');raise ValueError('Skirting schema rejected at '+str(list(error.absolute_path))+': '+error.message[:500])

def face(part,indices):
    """📐 Keep true emitted face coordinates and a stable anchored oriented plane."""
    points=np.asarray(a.part_vertices(part),float)[indices];origin=points[0]
    normal=max((np.cross(points[i]-origin,points[i+1]-origin) for i in range(1,len(points)-1)),key=np.linalg.norm)
    normal/=np.linalg.norm(normal)
    return points,normal,float(normal@origin)

supports=[]
for obj in g['slabs']+[item for item in g['details'] if item.get('predefined_type')=='FLOORING']:
    for index,part in enumerate(obj['parts']):
        if obj.get('class')=='IfcCovering' and part.get('role')!='wearing':continue
        for indices in a.part_faces(part):
            points,normal,offset=face(part,indices)
            if normal[2]<=0:continue
            polygon=Polygon(points[:,:2])
            if polygon.area>0:supports.append((obj['tag'],index,polygon,normal,offset))
tree=STRtree([item[2] for item in supports]);rows=[];failures=[];start=time.monotonic()
for obj in candidate['elements']:
    if (OUT/'skirting.cancel').exists():raise KeyboardInterrupt('Skirting validation cancelled')
    body=a._solid_geometry(m,obj);volume=0.;native=0.;bearing=0.;contacts=set();minimum=float('inf')
    for part in body['parts']:
        a._polyhedron_part(a.part_vertices(part),a.part_faces(part));points=np.asarray(a.part_vertices(part),float);span=np.ptp(points,axis=0);assert np.all(span>0),(obj['tag'],'flat-span')
        independent=ConvexHull((points-np.min(points,axis=0))/span).volume*float(np.prod(span));actual=a._polyhedron_volume(part)
        assert independent>0 and actual>0 and abs(independent-actual)<=max(1e-24,independent*1e-8),(obj['tag'],actual,independent)
        volume+=independent;native+=actual;minimum=min(minimum,independent)
        for indices in a.part_faces(part):
            vertices,normal,offset=face(part,indices)
            if normal[2]>=0:continue
            footprint=Polygon(vertices[:,:2])
            if footprint.area==0:continue
            contact=[]
            for hit in tree.query(footprint):
                tag,index,polygon,up,plane=supports[int(hit)]
                if float(normal@up)>-1+1e-10 or np.max(np.abs(vertices@up-plane))>1e-9:continue
                overlap=footprint.intersection(polygon)
                if overlap.area>0:contact.append(overlap);contacts.add(tag)
            if contact:bearing+=unary_union(contact).area
    if not bearing>0:failures.append({'tag':obj['tag'],'error':'no positive actual wearing/slab bearing'})
    rows.append({'tag':obj['tag'],'parts':len(body['parts']),'independent_volume':volume,'native_volume':native,'minimum_cell_volume':minimum,'bearing_area':bearing,'bearing_owners':sorted(contacts)})
    if len(rows)%10==0:print('[DEBUG] source stock validation '+json.dumps({'products':len(rows),'seconds':time.monotonic()-start}),flush=True)
for name,value in candidate['after'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==value,('changed-input',name)
for name,value in validation_inputs.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==value
publication={'material':a._find(m,'MAT-SKIRTING-GRAY'),'material_relation':m['relations'][-1]}
(OUT/'skirting-publication-input.json').write_text(json.dumps(publication,ensure_ascii=False,indent=2),encoding='utf8')
report={'success':not failures,'unpublished_hall_candidate':bool(model_binder),'failures':failures,'validation_inputs':validation_inputs,'publication_sha256':hashlib.sha256((OUT/'skirting-publication-input.json').read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256((OUT/'skirting-candidate.json').read_bytes()).hexdigest(),'source_inputs':candidate['after'],'products':len(rows),'parts':sum(row['parts'] for row in rows),'rows':rows,'seconds':time.monotonic()-start,'scope':'Checks require all cells closed/convex/positive with independent affine-conditioned SciPy volume and positive actual native wearing/slab face bearing per product via GEOS; failures remain explicit; no independent all-pair Boolean audit is claimed'}
(OUT/'skirting-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] source stock validation '+('GREEN ' if not failures else 'RED ')+json.dumps({key:value for key,value in report.items() if key not in ('rows','source_inputs')}),flush=True)

if failures:raise AssertionError(failures)
