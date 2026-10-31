"""🔑 Census exact opaque-cache keys without culling or painting."""
from pathlib import Path
import sys,importlib,hashlib,json,gc,time,collections
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root))
import numpy as np
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️');M=V._model
paths=[Path(V.__file__),Path(M.__file__),Path(M.__file__).with_name('🔣️.json')]
hashes=lambda:{p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
start=hashes(); began=time.monotonic();view=V.make_view((55,24),(0,0,0),1,(0,0,0));basis,towards=V._basis(view);padding=V._raster_footprint(.09,.45)
rows=[]
for index,slot in enumerate(M.slots()):
    parts=V._slot_parts(slot,None);groups={};items=V._assembly_items(parts,base=V.CAPSULE,groups=groups,plane=slot.plane,lod=True,glass_opacity=1)
    digest=hashlib.blake2b(digest_size=20);digest.update(np.asarray(basis,float).tobytes());digest.update(np.asarray([padding],float).tobytes());anchor=items[0].centre
    topology=hashlib.sha256();intrinsic=hashlib.sha256()
    for item in items:
        topo=np.asarray([item.tier,len(item.verts),*(len(p) for p in item.polygons)],np.int64).tobytes();digest.update(topo);topology.update(topo)
        if item.geometry_key is not None: digest.update(item.geometry_key);intrinsic.update(item.geometry_key)
        else:
            digest.update(np.asarray(item.verts-anchor,float).tobytes());digest.update(np.asarray(item.normal if item.normal is not None else [0,0,0],float).tobytes())
        digest.update(np.asarray([-1 if item.opacity is None else item.opacity],float).tobytes());digest.update(str(item.material).encode())
    axes=np.asarray((slot.plane.x,slot.plane.y,slot.plane.z),float)
    rows.append({'index':index,'tower':slot.tower,'storey':slot.storey,'type':slot.type,'family':slot.layout,'plane_origin':list(slot.plane.origin),'plane_axes':axes.tolist(),'axes_bytes':axes.tobytes().hex(),'axes_rounded12':np.round(axes,12).tobytes().hex(),'signature':digest.hexdigest(),'topology':topology.hexdigest(),'intrinsic':intrinsic.hexdigest(),'items':len(items)})
    del items,groups,parts;gc.collect()
    if index%10==9:print('[DEBUG] signatures '+str(index+1)+'/140',flush=True)
end=hashes();out={'source_start':start,'source_end':end,'unchanged':start==end,'elapsed':time.monotonic()-began,'view':[55,24],'padding':padding,'signature_algorithm':'exact _opaque_visible digest; no culling called','distinct_signatures':len({r['signature'] for r in rows}),'distinct_axes_bytes':len({r['axes_bytes'] for r in rows}),'distinct_axes_rounded12':len({r['axes_rounded12'] for r in rows}),'rows':rows}
p=Path(__file__).parent/'🗑️generated'/(sys.argv[1] if len(sys.argv)>1 else 'visibility-signatures.json');p.write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in out.items() if k!='rows'}),flush=True)
