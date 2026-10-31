"""🧱 Prove the measured masonry/finish interface with two actual windows under the shared own-RSS observer."""
import hashlib,importlib,importlib.util,json,os,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True;sys.path.insert(0,str(Path.cwd()))
ticket=Path(__file__).parent;monitor_path=ticket/"fixture-observer-r95.py"
spec=importlib.util.spec_from_file_location("owned_fixture_observer_r95",monitor_path);monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
observer=monitor.Observer(ticket,"roof-material-window-pair");start=time.perf_counter();result={"scope":"Two actual receiving windows with an in-memory declared masonry interface; no source publication or full-roof acceptance.","pid":os.getpid(),"observer_sha256":hashlib.sha256(monitor_path.read_bytes()).hexdigest()}
try:
    api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");model=api.load_model();result["source"]=api._geometry_source_key(model).hex();result["api"]=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest()
    host=api._find(model,"SL-R");tags={"RES-SALON-CHIMNEY-FINISH","RES-FIN-ROOF-GRAVEL-SL-R-151"};host["geometry"]["finish_voids"]=[row for row in host["geometry"]["finish_voids"]if row["tag"]in tags]
    finish=api._find(model,"FIN-ROOF-GRAVEL");refs=finish["geometry"].setdefault("interface_owners",[])
    if "CHIMNEY-SALON-ROOF"not in refs:refs.append("CHIMNEY-SALON-ROOF")
    result["declared_interface"]={"receiver":finish["tag"],"owner":"CHIMNEY-SALON-ROOF"};value=api._slab_geometry_entry(model,host,exact=True)
    result.update(status="GREEN",counts={key:len(value.get(key,[]))for key in ("parts","finish_parts","shared_finish_parts")})
except BaseException as error:
    result.update(status="CANCELLED"if isinstance(error,(KeyboardInterrupt,InterruptedError))else "RED",error=repr(error));traceback.print_exc()
finally:
    observation=observer.close();result["observer"]=observation;result["seconds"]=time.perf_counter()-start
(ticket/"🗑️generated/roof-material-window-pair-r95.json").write_text(json.dumps(result,indent=2,default=str),encoding="utf8")
print("[DEBUG]",json.dumps(result,default=str),flush=True)
if result["status"]!="GREEN":raise SystemExit(2 if result["status"]=="CANCELLED"else 1)
