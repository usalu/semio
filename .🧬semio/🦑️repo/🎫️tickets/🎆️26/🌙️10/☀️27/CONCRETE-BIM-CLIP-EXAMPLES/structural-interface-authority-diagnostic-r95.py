"""🔎 Inspect declared interface reference and actual owned material before changing any producer."""
import hashlib,importlib,json,sys,time
from collections import Counter
from pathlib import Path
sys.dont_write_bytecode=True;root=Path.cwd();sys.path.insert(0,str(root));ticket=Path(__file__).parent;path=root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json";raw=path.read_bytes();model=json.loads(raw);api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");results={}
for tag in ("GUARD-RM-1","GUARD-RM-2","BEAM-RAMP-FIRST-STRINGER","RM-1"):
    record={}
    for mode in (False,True):
        try:
            parts=api._interface_owner_parts(model,tag,reference=mode)
            record[str(mode)]={"parts":len(parts),"materials_roles":dict(Counter(str((p.get("material"),p.get("role")))for p in parts)),"volume":sum(api._polyhedron_volume(p)for p in parts),"bounds":[[float(min(api._material_bounds(p)[i][0]for p in parts)),float(max(api._material_bounds(p)[i][1]for p in parts))]for i in range(3)]}
        except Exception as error:record[str(mode)]={"error":repr(error)}
    results[tag]=record;print("[DEBUG]",tag,record,flush=True)
out={"source":hashlib.sha256(raw).hexdigest(),"api":hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),"scope":"Read-only source authority diagnosis, not repaired material or final all-pair acceptance","owners":results}
(ticket/"🗑️generated/structural-interface-authority-diagnostic-r95.json").write_text(json.dumps(out,indent=2),encoding="utf8")
