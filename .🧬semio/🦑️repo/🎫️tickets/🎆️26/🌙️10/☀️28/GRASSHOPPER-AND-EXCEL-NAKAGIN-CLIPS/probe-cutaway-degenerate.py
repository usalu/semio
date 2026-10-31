"""🔬 Minimize invalid cutaway boundaries without ordering or painting."""
import importlib
import inspect
import json
from pathlib import Path
import sys
from unittest.mock import patch
import numpy as np

TICKET=Path(__file__).parent
ROOT=next(path for path in TICKET.parents if (path/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
K=V._fragments
output=TICKET/'🗑️generated/cutaway-degenerate.json'
records=[]
original=V._clip_mesh
source_meshes={id(mesh):part.key for part,mesh in V._cap_parts(V._cap(V._model.capsule_type('A')),'full','reference','Slash')}
source_lines,start=inspect.getsourcelines(original)
append_line=start+next(i for i,line in enumerate(source_lines) if 'boundaries.append(polygon)' in line)
def trace(frame,event,arg):
    if frame.f_code is not original.__code__:
        return
    if event=='line' and frame.f_lineno==append_line:
        state=frame.f_locals
        polygon=state['polygon']
        vertices=tuple(map(tuple,polygon))
        epsilon=K._epsilon(vertices)
        clean=K._clean(vertices,epsilon)
        if len(clean)<3:
            pieces=state['pieces']
            all_edges=[(a,b) for piece in pieces for a,b in zip(piece,(*piece[1:],piece[0]))]
            records.append({'part':source_meshes.get(id(state['mesh'])),'face':int(state['index']),'source':state['source'].tolist(),'planes':[(a.tolist(),b) for a,b in state['planes']],'loop_ids':state['loop'],'points':[p.tolist() for p in state['points']],'piece_ids':pieces,'raw':polygon.tolist(),'clean':clean,'epsilon':epsilon,'cancelled_edges':[edge for edge in all_edges if edge[::-1] in all_edges]})
    return trace
def clipped(mesh,planes):
    sys.settrace(trace)
    try:
        return original(mesh,planes)
    finally:
        sys.settrace(None)
items=[]
with patch.object(V,'_clip_mesh',side_effect=clipped),patch.object(V,'_section_order',side_effect=lambda values,view:items.extend(values) or []):
    V.NakaginDrawing().capsule_axon(V._model.capsule_type('A'),cutaway=True)
failures=[]
for source,item in enumerate(items):
    if item.part_key!='lining-back':
        continue
    for polygon_id,polygon in enumerate(item.polygons):
        for facet in V._polygon_facets(polygon):
            normal=V._polygon_normal(facet,item.normal)
            face=K.Face(source,polygon_id,tuple(map(tuple,facet)),tuple(normal))
            try:
                K.ordered_fragments((face,),(0,0,1))
            except ValueError as error:
                failures.append({'source':source,'polygon':polygon_id,'raw':facet.tolist(),'normal':normal.tolist(),'error':str(error),'clean':K._clean(face.vertices,K._epsilon(face.vertices))})
output.write_text(json.dumps({'invalid_emissions':records,'preparation_failures':failures},ensure_ascii=False,indent=2),encoding='utf-8')
print('[DEBUG] Invalid emissions',len(records),'lining preparation failures',len(failures),flush=True)
if not records or not failures:
    raise AssertionError('Expected actual cutaway degeneracy was not captured')
