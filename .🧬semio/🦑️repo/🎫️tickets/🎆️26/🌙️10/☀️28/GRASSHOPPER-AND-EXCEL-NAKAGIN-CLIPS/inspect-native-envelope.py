"""📦 Measure native physical capsule triangles without CAD control bounds."""
import json
import struct
from pathlib import Path
import numpy as np

root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
assets=root/'🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules'
rows=[]
for source in sorted(assets.glob('*/🧊️*.glb')):
    blob=source.read_bytes()
    size=struct.unpack_from('<I',blob,12)[0]
    doc=json.loads(blob[20:20+size])
    start=28+size
    points=[]
    disks=[]
    for mesh in doc['meshes']:
        for primitive in mesh['primitives']:
            if primitive.get('mode',4)!=4:
                continue
            accessor=doc['accessors'][primitive['attributes']['POSITION']]
            view=doc['bufferViews'][accessor['bufferView']]
            assert accessor['componentType']==5126
            data=np.ndarray((accessor['count'],3),dtype='<f4',buffer=blob,offset=start+view.get('byteOffset',0)+accessor.get('byteOffset',0),strides=(view.get('byteStride',12),4)).astype(float)
            physical=np.column_stack((data[:,0],-data[:,2],data[:,1]))
            span=np.ptp(physical,axis=0)
            if np.count_nonzero(span<1e-6)==1 and np.count_nonzero(np.abs(span-1.3)<1e-5)==2:
                axes=np.flatnonzero(span>1e-6)
                uv=physical[:,axes]
                fitted=np.linalg.lstsq(np.column_stack((2*uv,np.ones(len(uv)))),np.sum(uv*uv,axis=1),rcond=None)[0]
                center=np.mean(physical,axis=0)
                center[axes]=fitted[:2]
                radius=float(np.sqrt(fitted[2]+np.dot(fitted[:2],fitted[:2])))
                disks.append({'vertices':len(physical),'center':center.tolist(),'radius':radius,'plane_axis':int(np.argmin(span))})
            points.extend(physical)
    assert not any(any(key in node for key in ('matrix','translation','rotation','scale')) for node in doc['nodes'])
    points=np.asarray(points)
    q=np.abs(points-[0,0,1.05])-[1.05,1.85,1]
    sdf=np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.max(q,axis=1),0)-.25
    profile=q[:,[0,2]]
    xz=np.linalg.norm(np.maximum(profile,0),axis=1)+np.minimum(np.max(profile,axis=1),0)-.25
    outer=np.abs(sdf)<1e-6
    longitudinal=outer&(np.abs(points[:,1])>1.85001)&(np.abs(points[:,1])<2.09999)&(xz<-.001)
    rows.append({'asset':str(source.relative_to(root)),'physical_vertices':len(points),'minimum':points.min(axis=0).tolist(),'maximum':points.max(axis=0).tolist(),'dimensions':np.ptp(points,axis=0).tolist(),'glazing_disks':disks,'outer_radius25_vertices':int(np.count_nonzero(outer)),'longitudinal_fillet_vertices':int(np.count_nonzero(longitudinal)),'longitudinal_witnesses':points[longitudinal][:5].tolist()})
out=Path(__file__).parent/'🗑️generated/native-envelope-numpy.json'
out.write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps(rows,ensure_ascii=True),flush=True)
