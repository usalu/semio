"""🏠 Build the real roof slab through the gravel window and stop at the next reserve."""
import importlib, sys, time
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, r"C:\git\semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
TAG = "RES-FIN-ROOF-GRAVEL-SL-R-705"
started = time.time()
state = {"passed": False}

def log(*items):
    """📣 Print one timed probe line."""
    print(round(time.time() - started, 1), *items, flush=True)

def progress(event):
    """📣 Record each finish reserve and request cancellation after the window."""
    if event.get("phase") != "finish-reservations":
        if event.get("status") == "completed":
            log("owner", event.get("tag"), event.get("cells"))
        return
    log("reserve", event.get("completed"), event.get("total"), event.get("tag"))
    if state.get("seen") and event.get("tag") != TAG:
        state["passed"] = True
    if event.get("tag") == TAG:
        state["seen"] = True

def cancel():
    """🛑 Stop once the following reserve proves this window returned."""
    return state["passed"]

model = api.load_model()
slab = api._find(model, "SL-R")
log("voids", len(slab["geometry"]["finish_voids"]))
try:
    api._slab_geometry_uncached(model, slab, exact=True, progress=progress, cancel=cancel)
    log("slab returned")
except InterruptedError:
    log("stopped after window", "passed" if state["passed"] else "not passed")
except ValueError as error:
    log("ERR", error)
