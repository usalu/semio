"""🪜 Audit authored and effective L-post floor contact without changing source."""
import hashlib,importlib,json,sys
from pathlib import Path
from copy import deepcopy
from shapely.geometry import Polygon
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();post=api.element('STAIR-RETURN-L',m);t=api.element(post['type'],m);axis=post['geometry']['axis'];shape=Polygon([[p[0]+axis[0],p[1]+axis[1]] for p in t['profile']['outline']]);slab=api.element('SL-1',m);void=next(v for v in slab['geometry']['voids'] if v['tag']=='V-STAIR-1');raw=Polygon(void['outline']);result={'source_hash':hashlib.sha256(Path(api.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'post_area':shape.area,'raw_stair_void_overlap':shape.intersection(raw).area,'outside_raw_stair_void':shape.difference(raw).area,'post_bounds':shape.bounds,'base':post['geometry']['base_level']}
slab['geometry'].pop('column_reserves',None);g=api._slab_geometry(m,slab);rows=[]
for key in ['parts','finish_parts','full_parts','full_finish_parts']:
    hits=[]
    for part in g[key]:
        for ring in api.part_band_footprints(part,part['z0'],part['z1']):
            a=shape.intersection(Polygon(ring)).area
            if a>1e-12:hits.append({'area':a,'z0':part['z0'],'z1':part['z1'],'material':part.get('material'),'region':part.get('region')})
    rows.append({'scope':key,'hits':hits,'projected_union_area':unary_union([shape.intersection(Polygon(ring)) for part in g[key] for ring in api.part_band_footprints(part,part['z0'],part['z1'])]).area})
result['unreserved_geometry']=rows;(Path(__file__).parent/'🗑️generated/stair-L-lower-audit-r92.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result),flush=True)
