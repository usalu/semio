"""🗺️ Export public occupied-band storey diagnostics without duplicate curtain products."""
import importlib,json,hashlib,time
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
source_path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');source_before=hashlib.sha256(source_path.read_bytes()).hexdigest()
source=api.load_model();started=time.monotonic();g=api.building_geometry(source);registry={e['tag']:e for e in source['elements']}
items=[]
for category in ['slabs','walls','pilotis','screens','fillings','beams','details','furnishings','foundations','railings','services']:
    for item in g[category]:
        tag=item['tag'];parts=item.get('parts',[])
        items.append({'tag':tag,'storey':registry[tag]['storey'],'category':category,'parts':parts})
if g.get('curtain_wall'):
    item=g['curtain_wall'];items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'curtain','parts':item['parts']})
for item in g['stairs']:items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'stairs','parts':item['parts']})
for item in g['ramp']['ramps']:items.append({'tag':item['tag'],'storey':registry[item['tag']]['storey'],'category':'ramps','parts':item['parts']+item['child_parts']})
views=[]
for storey,floor,cut in [('cellar',-2.42,-1.2),('ground',0,1.2),('first',3.17,4.4),('roof',6.6,7.8)]:
    rings=[]
    for item in items:
        visible=item['category'] in ['stairs','ramps'] or item['category'] in ['slabs','furnishings','railings','details'] and item['storey']==storey
        for part in item['parts']:
            if part['z0']<=cut<part['z1']:mode='cut';shapes=api.part_band_footprints(part,cut,cut+1e-7)
            elif visible and part['z0']<cut and (item['category']=='slabs' or part['z1']>floor-.05):mode='below';shapes=api.part_band_footprints(part,part['z0'] if item['category']=='slabs' else max(part['z0'],floor-.05),min(part['z1'],cut))
            else:continue
            for ring in shapes:rings.append({'tag':item['tag'],'category':item['category'],'mode':mode,'points':ring})
    views.append({'storey':storey,'floor':floor,'cut':cut,'rings':rings,'spaces':[s for s in g['spaces'] if s['storey']==storey]})
details=[]
for label,cut,bounds in [('laundry',.85,[3.1,19.7,7.1,21.5]),('guest',4.0,[2.5,10.5,3.8,11.8])]:
    rings=[]
    for item in items:
        for part in item['parts']:
            if part['z0']<=cut<part['z1']:
                for ring in api.part_band_footprints(part,cut,cut+1e-7):
                    rings.append({'tag':item['tag'],'category':item['category'],'mode':'cut','points':ring})
    details.append({'label':label,'cut':cut,'bounds':bounds,'rings':rings})
next(v for v in details if v['label']=='guest')['clear_aperture']=next(v['outline'] for e in source['elements'] if e['tag']=='SL-R' for v in e['geometry']['voids'] if v['tag']=='V-ROOFLIGHT')
source_after=hashlib.sha256(source_path.read_bytes()).hexdigest();assert source_before==source_after,'Source advanced during atlas capture'
result={'detail_views':details,'source_hash':source_before,'views':views,'model_api_hash':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'source_elements':len(source['elements']),'physical_items':[{'tag':item['tag'],'category':item['category'],'parts':len(item['parts'])} for item in items],'seconds':time.monotonic()-started,'limitations':['Below-cut material is projected only within the current floor band, including incoming stair/ramp arrivals irrespective of owner storey.','Below-cut polygons have no depth occlusion and cannot establish a model gap or collision.','This diagnostic uses public native occupied geometry and is not a native clip frame.']}
(Path(__file__).parent/'🗑️generated/physical-review-r86.json').write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print(json.dumps({'source_hash':result['source_hash'],'seconds':result['seconds'],'counts':{v['storey']:len(v['rings']) for v in views}}))
