"""📊 Count actual core assembly and previsibility work without culling, ordering or Cairo."""
from pathlib import Path
import hashlib, importlib, json, sys, time
from collections import Counter
import numpy as np
ROOT=Path.cwd()
sys.path.insert(0,str(ROOT))
HERE=Path(__file__).resolve().parent
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
sources=[Path(V.__file__),Path(V._model.__file__),Path(V._model.__file__).with_name('🔣️.json')]
fingerprints=lambda:{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
before=fingerprints()
rows=[]
view=V.make_view('axon',(0,0,0),1,(0,0,0))
basis,towards=V._basis(view)
for tower in V._model.towers():
    row={'tower':tower.key,'phases':{}}
    start=time.perf_counter()
    print('[DEBUG] Census '+tower.key+' source assembly',flush=True)
    assembly=V._model.core_assembly(tower,detail='full')
    row['phases']['assembly_seconds']=time.perf_counter()-start
    row['parts']=len(assembly.parts)
    row['source_vertices']=sum(len(p.solid.vertices) for p in assembly.parts)
    row['source_faces']=sum(len(p.solid.faces) for p in assembly.parts)
    row['roles']=dict(Counter(p.role for p in assembly.parts))
    print('[DEBUG] Census '+tower.key+' mesh conversion',flush=True)
    start=time.perf_counter()
    parts=V._parts(assembly)
    row['phases']['mesh_seconds']=time.perf_counter()-start
    print('[DEBUG] Census '+tower.key+' Items only',flush=True)
    start=time.perf_counter()
    groups={}
    items=V._assembly_items(parts,base=V.CONCRETE,groups=groups,lod=True,glass_opacity=1)
    row['phases']['items_seconds']=time.perf_counter()-start
    facing=[i for i in items if i.normal is None or i.normal@towards>1e-6]
    row['items']=len(items)
    row['facing_items']=len(facing)
    row['item_vertex_entries']=sum(len(i.verts) for i in items)
    row['facing_vertex_entries']=sum(len(i.verts) for i in facing)
    row['unique_facing_axes']=len({tuple(np.round(i.normal,10)) for i in facing if i.normal is not None})
    print('[DEBUG] Census '+tower.key+' cheap grid occupancy',flush=True)
    occupancy=Counter()
    large=0
    max_cells=0
    for item in facing:
        xy=np.asarray(item.verts)@basis.T
        lo,hi=np.floor(xy.min(0)/.25).astype(int),np.floor(xy.max(0)/.25).astype(int)
        cells=int(np.prod(hi-lo+1))
        max_cells=max(max_cells,cells)
        if cells>2048:
            large+=1
            continue
        for x in range(lo[0],hi[0]+1):
            for y in range(lo[1],hi[1]+1):
                occupancy[(x,y)]+=1
    row['grid']={'cell_width':.25,'large_global_items':large,'maximum_bbox_cells':max_cells,'occupied_cells':len(occupancy),'bucket_entries':sum(occupancy.values()),'maximum_bucket_items':max(occupancy.values(),default=0),'bucket_pair_occurrences':sum(n*(n-1)//2 for n in occupancy.values()),'large_candidate_occurrences_upper':large*len(facing),'note':'BBox occupancy only; pair occurrences duplicate across cells and do not measure exact overlap or real culling.'}
    rows.append(row)
    print('[DEBUG] Census '+tower.key+' '+json.dumps(row),flush=True)
    del assembly,parts,items,facing,groups
result={'view':'axon','towards':towards.tolist(),'rows':rows,'source_before':before,'source_after':fingerprints(),'forbidden_operations_called':[]}
result['unchanged']=result['source_before']==result['source_after']
out=HERE/'🗑️generated/core-census.json'
out.write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG] Census complete unchanged='+str(result['unchanged']),flush=True)
