import json,math,time
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated'
exec((ticket/'facade-local-reference-r80.py').read_text())
cases=json.loads((ticket/'facade-local-cases-r80.json').read_text())['cases'];results=[]
for case in cases:
 raw,owned,primary=([Polygon(r[0],r[1:]) for r in case[k]] for k in ('raw','owned','primary'))
 expected=unary_union(raw).difference(unary_union(primary));overlap=unary_union(owned).intersection(unary_union(primary)).area
 actual,common=_local_primary_reference(raw,owned,primary)
 assert actual.symmetric_difference(expected).area<1e-8
 assert abs(actual.area-case['difference_area'])<1e-8 and abs(common-case['overlap_area'])<1e-8
 assert abs(common-overlap)<1e-8
 results.append({'id':case['id'],'xor_area':actual.symmetric_difference(expected).area,'difference_area':actual.area,'overlap_area':common})
(gen/'facade-local-proof-r80.json').write_text(json.dumps(results));print(results)
