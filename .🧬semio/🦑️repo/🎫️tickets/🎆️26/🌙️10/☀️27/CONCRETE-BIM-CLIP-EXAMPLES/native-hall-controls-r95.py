"""📏 Independently inspect retained negative access and complete operated door parts."""
from pathlib import Path
import json,sys,importlib
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/native-r95-hall';A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');g=json.loads((O/'skirting-geometry.json').read_text(encoding='utf8'))['geometry'];case=json.loads((Path(A.__file__).parent/'🧪️tests/🏠ground.json').read_text(encoding='utf8'))['removed_unsupported_access'];flight=next(obj for obj in g['ramp']['flights'] if obj['tag']==case['ramp']);route=LineString(case['rejected_route']).buffer(case['body_width']/2,cap_style=2)
parts=[part for part in flight['parts'] if part['z0']<1.8 and part['z1']>0];projection=[Polygon(part['outline']) for part in parts];bands=[Polygon(ring) for part in parts for ring in A.part_band_footprints(part,0,1.8)]
report={'ramp':case['ramp'],'old_expected_area':case['expected_overlap_area'],'sum_projection_area':sum(route.intersection(shape).area for shape in projection),'union_projection_area':route.intersection(unary_union(projection)).area,'actual_band_union_area':route.intersection(unary_union(bands)).area,'parts':[(part.get('role'),part['z0'],part['z1'],route.intersection(Polygon(part['outline'])).area) for part in parts]}
(O/'controls.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(report),flush=True)
