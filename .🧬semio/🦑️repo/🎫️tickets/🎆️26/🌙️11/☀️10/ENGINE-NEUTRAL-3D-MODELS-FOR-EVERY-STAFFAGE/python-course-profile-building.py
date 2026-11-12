import cProfile, importlib, pstats, signal, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
seconds = int(sys.argv[1])
profiler = cProfile.Profile()
class Done(Exception): pass
def stop(*_): raise Done
signal.signal(signal.SIGALRM, stop)
signal.alarm(seconds)
start = time.time()
profiler.enable()
try:
    core.building_geometry()
    print("[DEBUG] finished", time.time() - start)
except Done:
    print("[DEBUG] stopped after", seconds)
profiler.disable()
stats = pstats.Stats(profiler)
stats.sort_stats("cumulative").print_stats(45)
