"""🕸️ Inspect actual capital painter constraints without drawing or changing production state."""
import ast
from collections import OrderedDict
import hashlib
import importlib
import json
from pathlib import Path
import sys
import time
import numpy as np
sys.dont_write_bytecode=True
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
TICKET=Path(__file__).parent
INPUT=TICKET/'🗑️generated/capital-order-source'
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
paths=[Path(V.__file__),Path(V._model.__file__),INPUT/'coverage-items.json',INPUT/'coverage-camera.json',Path(__file__)]
start={str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}
rows=json.loads((INPUT/'coverage-items.json').read_text(encoding='utf-8'))
camera=json.loads((INPUT/'coverage-camera.json').read_text(encoding='utf-8'))
items=[]
for row in rows:
    vertices=np.asarray(row['world_vertices'],float)
    lengths=[len(polygon) for polygon in row['pixel_polygons']]
    offsets=np.cumsum([0,*lengths])
    polygons=tuple(vertices[offsets[i]:offsets[i+1]] for i in range(len(lengths)))
    if offsets[-1]!=len(vertices):
        raise ValueError('Exported source polygon boundaries disagree')
    items.append(V.Item(0,vertices.mean(axis=0),vertices,lambda view:None,np.asarray(row['normal'],float),polygons,row['opacity'],row['material'],part_key=row['key']))
text=Path(V.__file__).read_text(encoding='utf-8')
function=next(node for node in ast.parse(text).body if isinstance(node,ast.FunctionDef) and node.name=='_section_order')
probe=ast.get_source_segment(text,function).replace('def _section_order(','def _graph_probe(',1)
probe=probe.replace('    for component in _cyclic_components(edges):','    graph_before=[set(targets) for targets in edges]\n    cycles_before=list(_cyclic_components(edges))\n    for component in _cyclic_components(edges):',1)
probe=probe.replace('    depths = [_order(view, item.centre) for item in visible]','    cycles_after=list(_cyclic_components(edges))\n    depths = [_order(view, item.centre) for item in visible]',1)
probe=probe.replace('    return [items[index] for index in order]','    return order, indices, graph_before, edges, cycles_before, cycles_after, _pivot_log',1)
helper=next(node for node in ast.parse(text).body if isinstance(node,ast.FunctionDef) and node.name=='_dependency_order')
helper_source=ast.get_source_segment(text,helper).replace('            ordered.append(pivot)','            _pivot_log.append(pivot)\n            ordered.append(pivot)',1)
namespace={**vars(V),'_OCCLUSION_ORDERS':OrderedDict(),'_pivot_log':[]}
exec(compile(helper_source,'capital-cycle-pivots','exec'),namespace)
exec(compile(probe,'capital-graph-probe','exec'),namespace)
view=V.make_view((135,24),tuple(camera['point']),camera['scale'],tuple(camera['centre']))
started=time.perf_counter()
print('[DEBUG] Capital graph probe '+str(len(items))+' source Items',flush=True)
order,indices,before,edges,cycles_before,cycles_after,fallback=namespace['_graph_probe'](items,view)
rank={index:position for position,index in enumerate(order)}
violated=[(indices[source],indices[target]) for source,targets in enumerate(edges) for target in targets if rank[indices[source]]>rank[indices[target]]]
result={'seconds':time.perf_counter()-started,'source_items':len(items),'facing_items':len(indices),'edges_before':sum(map(len,before)),'edges_after':sum(map(len,edges)),'cycles_before':[[indices[index] for index in sorted(component)] for component in cycles_before],'cycles_after':[[indices[index] for index in sorted(component)] for component in cycles_after],'fallback':[indices[index] for index in fallback],'violated_edges':[{'source':source,'target':target,'source_key':rows[source]['key'],'target_key':rows[target]['key'],'source_role':rows[source]['role'],'target_role':rows[target]['role'],'source_rank':rank[source],'target_rank':rank[target]} for source,target in violated],'source_start':start,'source_end':{str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}}
result['unchanged']=result['source_start']==result['source_end']
(TICKET/'🗑️generated/capital-order-graph.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Capital graph '+json.dumps({key:result[key] for key in ('seconds','facing_items','edges_before','edges_after','fallback','unchanged')})+' violated='+str(len(violated)),flush=True)
raise SystemExit(0 if result['unchanged'] else 1)
