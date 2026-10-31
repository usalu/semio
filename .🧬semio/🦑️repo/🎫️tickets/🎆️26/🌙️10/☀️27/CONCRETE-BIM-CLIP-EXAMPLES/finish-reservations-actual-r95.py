"""🧪 Probe current roof finish reservations against their declared native owners."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
ticket=Path(__file__).parent
original=api._native_parts_have_volume
def probe(parts):
    import inspect
    result=original(parts);frame=inspect.currentframe().f_back
    if result and frame.f_code.co_name=="_finish_reservations" and frame.f_locals.get("unoccupied")is parts:
        print("[DEBUG] GAP",frame.f_locals["reserve"],[(api._material_bounds(part),api._polyhedron_volume(part))for part in parts],flush=True)
    return result
api._native_parts_have_volume=probe
model=api.load_model();start=time.perf_counter()
result={"source":api._geometry_source_key(model).hex(),"api":hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest()}
print("[DEBUG] roof start",result,flush=True)
try:
    value=api._slab_geometry_entry(model,api._find(model,"SL-R"))
    result.update(status="GREEN",counts={key:len(value.get(key,[]))for key in ("parts","finish_parts","shared_finish_parts")})
except Exception as error:
    result.update(status="RED",error=repr(error));traceback.print_exc()
result["seconds"]=time.perf_counter()-start
(ticket/"🗑️generated/finish-reservations-actual-r95.json").write_text(json.dumps(result,indent=2),encoding="utf8")
print("[DEBUG]",json.dumps(result),flush=True)
if result["status"]!="GREEN":raise SystemExit(1)
