"""🎨 Capture source garage and neutral rounded-stock whole-material native appearances for independent backend verification."""
from copy import deepcopy
import hashlib,importlib,json,sys
from pathlib import Path
from scipy.spatial import ConvexHull
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));OUT=Path(__file__).parent/'🗑️generated/native-r92'
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
test=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧪️tests.🐍️')
oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),Path(api._surface_finishes().__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py']
def hashes():
    """🔐 Bind exact native input/producer bytes without copying tracked source."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();m=api.load_model();garage=api._find(m,'GARAGE-SLIDING');samples=[]
stock=api._surface_finishes().skirting_stock([0,0,0],[2,0,0],[0,1,0],test.CASE['skirting']['section'],'TEST-GRAY-WOOD')
wood=deepcopy(api._find(m,'MAT-PAINTED-WOOD'));wood.update(tag='TEST-GRAY-WOOD',guid=api.global_id('TEST-GRAY-WOOD'),name={'en':'Neutral gray timber; display colour inferred','de':'Neutrales graues Holz; Anzeigefarbe abgeleitet'})
wood['surface_finish']={'binder':'unmeasured_paint','colour_family':'gray','substrate':'MAT-PAINTED-WOOD','quantity_scope':'existing_substrate_envelope','film_thickness':'unmeasured','basis':'assumed','source':'Synthetic original-sized board; SAB2024-p190 profile only','display_rgb':{'value':[.47,.48,.49],'basis':'inferred','source':'Illustrative gray; original RGB unmeasured'}}
m['materials'].append(wood)
board={'tag':'TEST-ROUNDED-BOARD','guid':api.global_id('TEST-ROUNDED-BOARD'),'class':'IfcCovering','predefined_type':'SKIRTINGBOARD','name':{'en':'Neutral rounded timber board','de':'Neutrale gerundete Holzsockelleiste'},'storey':'ground','material':wood['tag'],'geometry':{'kind':'skirting','base_level':'ground','parts':[stock]},'attributes':{},'psets':{},'metadata':{'author':'Synthetic acceptance','revision':1,'domains':['architecture']}}
m['elements'].append(board)
for obj,rgb in [(garage,[.28,.42,.31]),(board,[.47,.48,.49])]:
    geometry=api._filling_geometry(m,obj) if obj['class']=='IfcDoor' else api.solid_geometry(obj['tag'],m)
    volume=sum(ConvexHull(api.part_vertices(part)).volume for part in geometry['parts'])
    vertices=[v for part in geometry['parts'] for v in api.part_vertices(part)]
    bounds=[[min(v[k] for v in vertices) for k in range(3)],[max(v[k] for v in vertices) for k in range(3)]]
    for language in ('en','de'):
        if (OUT/'uniform.cancel').exists():raise KeyboardInterrupt('Native uniform capture cancelled')
        fragment=api.step_excerpt(obj['tag'],m,language=language);entities=oracle.step_parse(fragment['lines'])['entities']
        items=[int(fragment['ids']['solid'+str(i)][1:]) for i in range(len(geometry['parts']))]
        styles=[{'style_item':key,'target':args[0][1],'style':oracle.step_refs(args[1])[0]} for key,(kind,args) in entities.items() if kind=='IFCSTYLEDITEM']
        assert {s['target'] for s in styles}==set(items)
        samples.append({'tag':obj['tag'],'guid':obj['guid'],'name':obj['name'][language],'language':language,'class':obj['class'],'predefined_type':obj['predefined_type'],'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'expected_volume':volume,'expected_bounds':bounds,'expected_rgb':rgb,'physical_items':items,'style_ownership':styles,'quantity_scope':'existing_substrate_envelope','film_and_mass':'unmeasured','rgb_basis':'inferred_display_only'})
after=hashes();payload={'stable':before==after,'inputs_before':before,'inputs_after':after,'samples':samples}
(OUT/'uniform-finish-samples.json').write_text(json.dumps(payload,ensure_ascii=True),encoding='utf8')
print('[DEBUG] '+json.dumps({'stable':payload['stable'],'samples':len(samples)}),flush=True)
raise SystemExit(0 if payload['stable'] else 1)
