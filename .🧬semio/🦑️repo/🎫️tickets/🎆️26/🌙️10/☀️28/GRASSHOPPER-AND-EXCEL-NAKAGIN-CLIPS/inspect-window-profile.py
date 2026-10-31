"""🪟 Measure actual native aperture radii and recess planes without control-point bounds."""
import json
from pathlib import Path
import struct
import numpy as np

root=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
assets=root/"🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules"
rows=[]
for source in sorted(assets.glob("*/🧊️*.glb")):
    name=source.stem.casefold()
    if not any(key in name for key in ("slash","_p","_s")):
        continue
    blob=source.read_bytes()
    size=struct.unpack_from("<I",blob,12)[0]
    doc=json.loads(blob[20:20+size])
    records=[]
    side=name.endswith(("_p","_s"))
    for mesh in doc["meshes"]:
        for primitive in mesh["primitives"]:
            if primitive.get("mode",4)!=4:
                continue
            accessor=doc["accessors"][primitive["attributes"]["POSITION"]]
            view=doc["bufferViews"][accessor["bufferView"]]
            values=np.ndarray((accessor["count"],3),dtype="<f4",buffer=blob,offset=28+size+view.get("byteOffset",0)+accessor.get("byteOffset",0),strides=(view.get("byteStride",12),4)).astype(float)
            points=np.column_stack((values[:,0],-values[:,2],values[:,1]))
            normal=points[:,0] if side else points[:,1]
            along=points[:,1]-.95 if side else points[:,0]
            radius=np.hypot(along,points[:,2]-1.05)
            selected=(normal>(1.1 if side else 1.8))&(radius>.6)&(radius<.85)
            records.extend(np.column_stack((normal[selected],radius[selected])))
    values=np.asarray(records)
    unique,counts=np.unique(np.round(values,5),axis=0,return_counts=True)
    rows.append({"asset":str(source.relative_to(root)),"side_wall":side,"plane_radius_counts":[[float(p),float(r),int(n)] for (p,r),n in zip(unique,counts)]})
out=Path(__file__).parent/"🗑️generated/native-window-profiles.json"
out.write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding="utf8")
print("[DEBUG] "+json.dumps(rows,ensure_ascii=True),flush=True)
