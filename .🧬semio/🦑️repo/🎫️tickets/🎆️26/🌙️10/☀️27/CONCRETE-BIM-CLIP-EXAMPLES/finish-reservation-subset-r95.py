"""🔎 Isolate declared native roof reservation controls without claiming full-model acceptance."""
import copy,hashlib,importlib,json,sys,threading,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True;sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
ticket=Path(__file__).parent;model=api.load_model();source=api._geometry_source_key(model).hex()
count=int(sys.argv[1]);slab=api._find(model,"SL-R");all_reserves=slab["geometry"]["finish_voids"];selector=sys.argv[2]if len(sys.argv)>2 else None
selected=[row for row in all_reserves if row["tag"]==selector]if selector else all_reserves[:count]
if not selected:raise ValueError("The selected actual reserve does not exist")
count=len(selected);slab["geometry"]["finish_voids"]=selected
start=time.perf_counter();main_thread=threading.get_ident();done=threading.Event();cancelled=threading.Event()
def watch_cancel():
    """🛑 Poll only this script's cancellation file without putting filesystem calls in geometric inner loops."""
    while not done.wait(.2):
        if (ticket/"finish-reservation-subset-cancel-r95").exists():cancelled.set();return
threading.Thread(target=watch_cancel,daemon=True).start()
def monitor():
    while not done.wait(10):
        frame=sys._current_frames().get(main_thread);stack=[];detail={}
        while frame:
            stack.append(frame.f_code.co_name)
            if frame.f_code.co_name=="_finish_reservations":
                detail={key:frame.f_locals.get(key)for key in ("index","tag")}
                detail["reserve"]=frame.f_locals.get("reserve",{}).get("tag")
                detail["counts"]={key:len(frame.f_locals.get(key,[]))for key in ("host","common","expected","unoccupied","reserves")}
            frame=frame.f_back
        print("[DEBUG]",round(time.perf_counter()-start,2),stack[:8],detail,flush=True)
threading.Thread(target=monitor,daemon=True).start()
last=[start]
def progress(event):
    """📣 Surface bounded progress while preserving the original native construction callbacks."""
    now=time.perf_counter()
    if now-last[0]>=10:last[0]=now;print("[DEBUG] progress",event,flush=True)
result={"scope":"Actual roof reservations only; no final all-product interference claim.","source":source,"selected":count,"selector":selector,"total":len(all_reserves),"complete_roof":count==len(all_reserves),"api":hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest()}
try:
    value=api._slab_geometry_entry(model,slab,exact=True,progress=progress,cancel=cancelled.is_set)
    result.update(status="GREEN",counts={key:len(value.get(key,[]))for key in ("parts","finish_parts","shared_finish_parts")})
except Exception as error:
    result.update(status="RED",error=repr(error));traceback.print_exc()
finally:done.set()
result["seconds"]=time.perf_counter()-start
(ticket/f"🗑️generated/finish-reservation-subset-{selector or count}-r95.json").write_text(json.dumps(result,indent=2),encoding="utf8")
print("[DEBUG]",result,flush=True)
if result["status"]!="GREEN":raise SystemExit(1)
