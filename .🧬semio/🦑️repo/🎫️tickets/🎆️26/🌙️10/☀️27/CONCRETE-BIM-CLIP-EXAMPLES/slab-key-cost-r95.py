"""📏 Measure exact private slab dependency serialization without building or mutating geometry."""
from pathlib import Path
import json,time,sys
m=json.loads(Path("🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").read_text(encoding="utf-8"));rows=[]
for slab in [o for o in m["elements"] if o["class"]=="IfcSlab"]:
    owners=set(slab["geometry"].get("column_reserves",[]))|{tag for v in slab["geometry"].get("finish_voids",[]) for tag in (v["owner"],*v.get("joint_owners",[]))}
    dependency=[slab,m["storeys"],m["materials"],m["types"],m["spaces"],m["car"],[o for o in m["elements"] if o["class"]=="IfcCurtainWall" or (o["class"]=="IfcOpeningElement" and o["geometry"].get("slab")==slab["tag"])],[o for o in m["elements"] if o["tag"] in owners]]
    started=time.perf_counter()
    for _ in range(10):value=json.dumps(dependency,sort_keys=True,separators=(",",":"))
    rows.append({"tag":slab["tag"],"key_bytes":len(value.encode()),"mean_seconds":(time.perf_counter()-started)/10})
report={"stdlib_python":sys.version,"scope":"read-only exact private slab input construction; no actual geometry or mutable cache","rows":rows,"whole_model_bytes":len(json.dumps(m).encode())}
(Path(__file__).parent/"🗑️generated/native-r92/slab-key-cost-r95.json").write_text(json.dumps(report,indent=2),encoding="utf-8");print("[DEBUG] "+json.dumps(report))
