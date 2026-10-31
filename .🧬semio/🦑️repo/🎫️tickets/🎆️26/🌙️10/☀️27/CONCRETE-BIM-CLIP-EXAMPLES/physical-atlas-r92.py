"""🗺️ Export public occupied-band storey diagnostics without duplicate curtain products."""
import importlib,json,hashlib,time,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
source_path=Path(api.__file__).with_name('🔣️.json');source_before=hashlib.sha256(source_path.read_bytes()).hexdigest()
producer_paths=[p for p in Path(api.__file__).parent.rglob('*.py') if '🧪️tests' not in str(p)]+[Path(api.__file__).parent/'🧬️schema/🔣️.schema.json'];producer_before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producer_paths}
source=api.load_model();started=time.monotonic();cache_path=Path(__file__).parent/'🗑️generated/physical-atlas-geometry-r92.json';cached=json.loads(cache_path.read_text(encoding='utf8')) if 'reuse' in sys.argv and cache_path.is_file() else None
if cached and cached['source']==source_before and cached['producers']==producer_before:g=cached['geometry'];cache_mode='exact-input-reuse'
else:
    g=api.building_geometry(source);cache_mode='cold'
    if hashlib.sha256(source_path.read_bytes()).hexdigest()==source_before and producer_before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in producer_paths}:cache_path.write_text(json.dumps({'source':source_before,'producers':producer_before,'geometry':g},ensure_ascii=False),encoding='utf8')
registry={e['tag']:e for e in source['elements']}
items=[]
for category in ['slabs','walls','pilotis','screens','fillings','beams','details','furnishings','foundations','railings','services']:
    for item in g[category]:
        tag=item['tag'];parts=item.get('parts',[])
        items.append({'tag':tag,'storey':registry[tag]['storey'],'category':category,'kind':item.get('kind'),'parts':parts})
if g.get('curtain_wall'):
    item=g['curtain_wall'];items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'curtain','parts':item['parts']})
for item in g['stairs']:items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'stairs','parts':item['parts']})
for item in g['ramp']['ramps']:items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'ramps','parts':item['parts']+item['child_parts']})
views=[]
for storey,floor,cut in [('cellar',-2.40,-1.2),('ground',0,1.2),('first',3.17,4.4),('roof',6.6,7.8)]:
    rings=[]
    for item in items:
        for index,part in enumerate(item['parts']):
            low,high=part['z0'],part['z1'];own_floor=item['category']=='details' and item.get('kind')=='flooring' and item['storey']==storey
            if low<=cut<high:mode='cut';bottom,top=cut,cut+1e-7
            elif low<cut and (high>floor-.05 or own_floor or item['category']=='slabs' and item['storey']==storey):
                mode='below';bottom,top=(low if own_floor or item['category']=='slabs' else max(low,floor-.05)),min(high,cut)
            else:continue
            shapes=api.part_band_footprints(part,bottom,top)
            for ring in shapes:rings.append({'tag':item['tag'],'category':item['category'],'part':index,'role':part.get('role'),'material':part.get('material'),'visible_z':min(high,cut),'mode':mode,'points':ring})
    rings.sort(key=lambda row:(row['mode']=='cut',row['visible_z']))
    views.append({'storey':storey,'floor':floor,'cut':cut,'rings':rings,'spaces':[s for s in g['spaces'] if s['storey']==storey]})
source_after=hashlib.sha256(source_path.read_bytes()).hexdigest();producer_after={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producer_paths};stable=source_before==source_after and producer_before==producer_after
result={'cache_mode':cache_mode,'source_hash':source_before,'source_after':source_after,'stable':stable,'producer_before':producer_before,'producer_after':producer_after,'views':views,'model_api_hash':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'source_elements':len(source['elements']),'physical_items':[{'tag':item['tag'],'category':item['category'],'parts':len(item['parts'])} for item in items],'seconds':time.monotonic()-started,'limitations':['All physical categories use actual world-height occupied bands irrespective of owner storey; current-storey flooring includes exterior surfaces below its indoor FFL.','Below-cut polygons have no depth occlusion and cannot establish a model gap or collision.','This diagnostic uses public native occupied geometry and is not a native clip frame.']}
(Path(__file__).parent/'🗑️generated/physical-atlas-r92.json').write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print('[DEBUG] '+json.dumps({'source_hash':result['source_hash'],'seconds':result['seconds'],'counts':{v['storey']:len(v['rings']) for v in views}}))

if 'final' in sys.argv and not stable:raise SystemExit('Final atlas source/producers changed during generation')
