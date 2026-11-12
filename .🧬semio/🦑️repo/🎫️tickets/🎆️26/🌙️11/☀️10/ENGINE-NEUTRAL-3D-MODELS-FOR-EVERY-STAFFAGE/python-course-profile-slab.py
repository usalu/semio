import cProfile, importlib, pstats, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = core.load_model()
tag, reserve, length = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
windows = {"gravel": cProfile.Profile(), "reserves": cProfile.Profile()}
state = {"active": None, "stop": 0, "done": set()}
class Done(Exception): pass
def open_window(name):
    if name in state["done"] or state["active"]: return
    state["active"] = name; state["stop"] = time.time() + length; windows[name].enable(); print("[DEBUG] profiling", name, flush=True)
def progress(event):
    if state["active"] and time.time() > state["stop"]:
        windows[state["active"]].disable(); state["done"].add(state["active"]); state["active"] = None
        if len(state["done"]) == 2: raise Done
    if event.get("phase") == "finish-owner" and event.get("tag") == "FIN-ROOF-GRAVEL" and event.get("status") == "started": open_window("gravel")
    if event.get("phase") == "finish-reservations" and event["completed"] >= reserve: open_window("reserves")
try:
    core._slab_geometry_uncached(model, core._find(model, tag), exact=True, progress=progress)
except Done:
    pass
for name, profiler in windows.items():
    print("=====", name)
    stats = pstats.Stats(profiler).sort_stats("tottime"); stats.print_stats(14)
    pstats.Stats(profiler).sort_stats("cumulative").print_stats(24)
