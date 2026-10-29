import ast,json,math,time
from pathlib import Path
from shapely.geometry import Polygon,box,shape
from shapely.strtree import STRtree
from shapely.ops import unary_union
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests')
tree=ast.parse((root/'🚗️parking.py').read_text());node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='_tiled_xor_area');scope={'math':math,'Polygon':Polygon,'box':box,'STRtree':STRtree};exec(compile(ast.Module(body=[node],type_ignores=[]),'actual parking helper','exec'),scope);helper=scope['_tiled_xor_area'];fixture=json.loads((root/'🚗️overlays.json').read_text());rows=[]
for case in fixture['cases']:
 a=[shape(x) for x in case['actual']];b=[shape(x) for x in case['expected']];value=helper(a,b);oracle=unary_union(a).symmetric_difference(unary_union(b)).area;rows.append({'id':case['id'],'actual':value,'global_oracle':oracle,'expected':case['xor_area']});assert abs(value-oracle)<1e-12;assert abs(value-case['xor_area'])<1e-10
extra=[('expected-only-negative',[],[box(-2.5,-1.25,-.5,.75)]),('overlap-negative',[box(-1.5,-.5,.5,1.5),box(-.5,-.5,1.5,1.5)],[box(-1.5,-.5,1.5,1.5)]),('tiny-positive-sliver',[box(0,0,1,1)],[box(0,0,1+1e-10,1)]),('both-empty',[],[])]
for name,a,b in extra:
 value=helper(a,b);oracle=unary_union(a).symmetric_difference(unary_union(b)).area;rows.append({'id':name,'actual':value,'global_oracle':oracle});assert abs(value-oracle)<1e-15
 if name=='tiny-positive-sliver':assert value>0
source=[shape(x) for x in fixture['source_group']['geometries']];started=time.monotonic();area=helper(source,[]);same=helper(source,source);seconds=time.monotonic()-started;reference=.4594722475880584;assert abs(area-reference)<1e-8;assert same<1e-8
result={'portable':rows,'captured_count':len(source),'all_valid':all(x.is_valid for x in source),'tiled_union_area':area,'independent_node_area':reference,'same_input_xor':same,'seconds':seconds,'total_tolerance':fixture['total_xor_tolerance']};(p/'🗑️generated/bounded-overlay-independent-r63.json').write_text(json.dumps(result));print(json.dumps(result))
