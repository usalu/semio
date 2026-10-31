"""🏛️ Capture complete public elevations and true vertical sections for physical review."""
import hashlib, importlib, json, time
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file())
import sys
sys.path.insert(0,str(root))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
source_path=Path(api.__file__).with_name('🔣️.json')
source_hash=hashlib.sha256(source_path.read_bytes()).hexdigest()
started=time.monotonic()
print('[DEBUG] Building complete occupied geometry',flush=True)
source=api.load_model();g=api.building_geometry(source);registry={e['tag']:e for e in source['elements']}
items=[]
for category in ['slabs','walls','pilotis','screens','fillings','beams','details','furnishings','foundations','railings','services','stairs']:
    for item in g[category]:
        items.append({'tag':item['tag'],'category':category,'parts':item.get('parts',[])})
if g.get('curtain_wall'):
    item=g['curtain_wall'];items.append({'tag':item['tag'],'category':'curtain','parts':item['parts']})
for item in g['ramp']['ramps']:
    items.append({'tag':item['tag'],'category':'ramps','parts':item['parts']+item['child_parts']})
solids=[];sections=[];plans=[]
for item in items:
    for part in item['parts']:
        vertices=api.part_vertices(part);faces=api.part_faces(part)
        solids.append({'tag':item['tag'],'category':item['category'],'vertices':vertices,'faces':faces,'material':part.get('material')})
for axis,value,label in [(0,6.407145,'Stair / Treppe X=6.407145'),(0,9.0,'Ramp / Rampe X=9.0'),(1,11.5,'Services / Haustechnik Y=11.5'),(1,4.0,'South rooms / Suedraeume Y=4.0')]:
    rings=[]
    for item in items:
        for part in item['parts']:
            for ring in api.part_sections(part,value,axis):
                if len(ring)>=3:rings.append({'tag':item['tag'],'category':item['category'],'points':ring})
    sections.append({'axis':axis,'value':value,'label':label,'rings':rings})
    print('[DEBUG] Section '+label+': '+str(len(rings))+' occupied rings',flush=True)
for z,label in [(-1.32,'Cellar / Keller Z=-1.32'),(1.1,'Ground / Erdgeschoss Z=1.10'),(4.27,'First / Obergeschoss Z=4.27'),(7.7,'Roof / Dach Z=7.70'),(3.5,'Sanitary bodies / Sanitaerkoerper Z=3.50'),(3.9,'Basin and cabinetry / Becken und Moebel Z=3.90')]:
    rings=[]
    for item in items:
        for part in item['parts']:
            if part['z0']<z<part['z1']:
                for ring in api.part_regions(part,z):
                    if len(ring)>=3:rings.append({'tag':item['tag'],'category':item['category'],'material':part.get('material'),'points':ring})
    plans.append({'z':z,'label':label,'rings':rings})
    print('[DEBUG] Plan '+label+': '+str(len(rings))+' occupied rings',flush=True)
source_after=hashlib.sha256(source_path.read_bytes()).hexdigest()
result={'source_hash':source_hash,'source_after':source_after,'source_stable':source_hash==source_after,'api_hash':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'source_products':len(source['elements']),'solids':solids,'sections':sections,'plans':plans,'seconds':time.monotonic()-started,'limits':['Orthographic elevations use painter-sorted actual occupied faces; surfaces crossing in depth may require native render inspection.','Sections use public exact occupied-part vertical slices; planar diagnostics do not prove concealed historical construction.']}
out=Path(__file__).parent/'🗑️generated'/'physical-vertical-r92.json';out.parent.mkdir(exist_ok=True);out.write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print(json.dumps({'source_hash':source_hash,'solids':len(solids),'seconds':result['seconds'],'sections':{s['label']:len(s['rings']) for s in sections}}),flush=True)
