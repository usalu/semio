"""🔍 Review the exact current parking geometry delta before changing the portable fixture."""
import importlib,json,math
from collections import Counter,defaultdict
from pathlib import Path
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.';overlay=importlib.import_module(prefix+'🧱️model.🧪️tests.🚗️overlay');ticket=Path(__file__).parent;generated=ticket/'🗑️generated';root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');old=json.loads((root/'🧱️model/🧪️tests/🚗️parking.json').read_text());new=json.loads((generated/'parking-review-r77.json').read_text());moved={'FIX-LAUNDRY','TAP-LAUNDRY-1','TAP-LAUNDRY-2'}
def group(records):
    result=defaultdict(list)
    for record in records:result[(record['tag'],record['role'])].append(record)
    return result
a,b=group(old['physical_obstacles']),group(new['physical_obstacles']);assert set(a)==set(b);different=[];cases=[];translations=0
def numeric(value):
    if isinstance(value,bool) or value is None:return value
    if isinstance(value,(int,float)):return ['binary64',float(value).hex()]
    if isinstance(value,list):return [numeric(v) for v in value]
    if isinstance(value,dict):return {k:numeric(v) for k,v in value.items()}
    return value
for key in sorted(a):
    literal=lambda rows:Counter(json.dumps(numeric(r),sort_keys=True,separators=(',',':')) for r in rows)
    if literal(a[key])==literal(b[key]):continue
    if key[0] in moved:
        expected=[{**p,'outline':[[x,y+.5] for x,y in p['outline']]} for p in a[key]];assert literal(expected)==literal(b[key]),key;translations+=len(expected)
    else:
        cases.append({'id':':'.join(key),'actual':[[p['outline']+[p['outline'][0]]] for p in b[key]],'expected':[[p['outline']+[p['outline'][0]]] for p in a[key]]})
    different.append({'tag':key[0],'role':key[1],'before':len(a[key]),'after':len(b[key])})
def ring_key(ring):
    values=[tuple(point) for point in ring]
    if values[0]==values[-1]:values.pop()
    candidates=[]
    for direction in (values,list(reversed(values))):
        start=min(range(len(direction)),key=lambda i:direction[i]);candidates.append(tuple(direction[start:]+direction[:start]))
    return min(candidates)
reductions=[]
for row in cases:
    left={ring_key(polygon[0]):polygon for polygon in row['actual']};right={ring_key(polygon[0]):polygon for polygon in row['expected']};common=set(left)&set(right)
    reductions.append({'id':row['id'],'actual_unique':len(left),'expected_unique':len(right),'exact_shared':len(common),'actual_residual':len(set(left)-common),'expected_residual':len(set(right)-common)})
    row['actual']=[polygon for key,polygon in left.items() if key not in common];row['expected']=[polygon for key,polygon in right.items() if key not in common]
(generated/'parking-delta-r77-reduction.json').write_text(json.dumps(reductions))
result=json.loads(overlay.overlay_json(json.dumps({'cases':cases})));assert all(row['xor_area']<1e-8 for row in result['cases']);assert translations==18
scenario_identical=old['expected_results']==new['expected_results']
record={'different_groups':different,'rigidly_translated_records':translations,'unchanged_scenario_outputs':scenario_identical,'conservative_common_polygon_reduction':reductions,'fraction_union_proof':result,'before_obstacles':len(old['physical_obstacles']),'after_obstacles':len(new['physical_obstacles'])};(generated/'parking-delta-r77.json').write_text(json.dumps(record));print(record)
