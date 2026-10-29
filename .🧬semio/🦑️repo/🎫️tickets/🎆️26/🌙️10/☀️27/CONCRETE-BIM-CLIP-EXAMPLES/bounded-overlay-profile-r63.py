import ast,json,math,time
from pathlib import Path
from shapely.geometry import Polygon,box as realbox,shape
from shapely.strtree import STRtree
import shapely.ops as ops
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests');started=time.monotonic();events=[];state={'cell':None,'union':0}
def log(phase,**data):
 e={'phase':phase,'seconds':time.monotonic()-started,'cell':state['cell'],**data};events.append(e);(p/'🗑️generated/bounded-overlay-profile-r63.json').write_text(json.dumps(e));(p/'🗑️generated/bounded-overlay-events-r63.json').write_text(json.dumps(events))
def bbox(*args):
 state['cell']=list(args);log('cell-start');return realbox(*args)
original=ops.unary_union
def union(shapes):
 values=list(shapes);state['union']+=1;log('local-union-start',index=state['union'],count=len(values));(p/'🗑️generated/bounded-overlay-current-local-r63.json').write_text(json.dumps({'cell':state['cell'],'index':state['union'],'geometries':[v.__geo_interface__ for v in values]}));s=time.monotonic();value=original(values);log('local-union-end',index=state['union'],count=len(values),call_seconds=time.monotonic()-s);return value
ops.unary_union=union
tree=ast.parse((root/'🚗️parking.py').read_text());node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='_tiled_xor_area');scope={'math':math,'Polygon':Polygon,'box':bbox,'STRtree':STRtree};exec(compile(ast.Module(body=[node],type_ignores=[]),'actual parking helper','exec'),scope);fixture=json.loads((root/'🚗️overlays.json').read_text());source=[shape(x) for x in fixture['source_group']['geometries']];log('captured-source-start',count=len(source));area=scope['_tiled_xor_area'](source,[]);log('captured-source-end',area=area);(p/'🗑️generated/bounded-overlay-profile-result-r63.json').write_text(json.dumps({'area':area,'seconds':time.monotonic()-started}))
