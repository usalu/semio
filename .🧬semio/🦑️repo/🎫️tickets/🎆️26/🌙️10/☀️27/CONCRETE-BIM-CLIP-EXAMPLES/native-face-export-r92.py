"""🎨 Author the exact neutral native face-treatment samples for an independent IFC backend."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));TICKET=Path(__file__).parent;OUT=TICKET/'🗑️generated/native-r92'
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
test=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧪️tests.🐍️')
oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
files=[Path(api.__file__),Path(test.__file__),Path(test.__file__).with_name('🔣️.json'),ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🐍️.py',ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🐍️.py']
def hashes():
    """🔐 Record only exact producer/input hashes, without copying tracked source."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
before=hashes();m,wall=test.example();g=api._wall_geometry(m,wall);samples=[]
for language in ('en','de'):
    if (TICKET/'native-r92-cancel').exists():raise SystemExit('Native validation cancelled')
    print('[DEBUG] Neutral native face sample '+language,flush=True)
    fragment=api.step_excerpt(wall['tag'],m,language=language);entities=oracle.step_parse(fragment['lines'])['entities']
    breps=[]
    for index,part in enumerate(g['parts']):
        if 'surface_finish' not in part:continue
        item=int(fragment['ids']['solid'+str(index)][1:]);entity,values=entities[item]
        assert entity=='IFCFACETEDBREP'
        shell=values[0][1];faces=oracle.step_refs(entities[shell][1][0])
        styled=[face for face in faces if any(kind=='IFCSTYLEDITEM' and data[0]==('ref',face) for kind,data in entities.values())]
        breps.append({'item':item,'shell':shell,'faces':faces,'styled_faces':styled,'source_part_index':index,'expected_volume':Polygon(part['outline']).area*(part['z1']-part['z0']),'vertices':api.part_vertices(part),'face_indices':api.part_faces(part)})
    samples.append({'tag':wall['tag'],'guid':wall['guid'],'name':wall['name'][language],'language':language,'class':'IfcWall','predefined_type':'STANDARD','text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'expected_volume':test.CASE['expected']['volume'],'expected_treated_face_area':test.CASE['expected']['face_area'],'expected_painted_substrate_volume':test.CASE['expected']['substrate_volume'],'expected_rgb':test.CASE['material']['surface_finish']['display_rgb']['value'],'expected_material':test.CASE['material']['name'][language],'brep_items':breps,'quantity_scope':'existing_substrate_envelope','film_and_mass':'unmeasured','rgb_basis':'inferred_display_only'})
after=hashes();payload={'diagnostic':True,'stable':before==after,'inputs_before':before,'inputs_after':after,'samples':samples}
(OUT/'face-finish-samples.json').write_text(json.dumps(payload,ensure_ascii=True),encoding='utf8')
raise SystemExit(0 if before==after else 1)
