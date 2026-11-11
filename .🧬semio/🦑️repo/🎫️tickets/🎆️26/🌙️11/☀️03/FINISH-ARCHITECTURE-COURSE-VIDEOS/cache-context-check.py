import importlib
import sys
from pathlib import Path
sys.path.insert(0, str(Path.cwd()))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
def trace(frame, event, arg):
    if frame.f_code.co_filename == cache.__file__ and event == "exception":
        print("exception", frame.f_code.co_name, frame.f_lineno, repr(arg[1]))
    if frame.f_code.co_name == "canonical_context" and event == "return":
        print("return", frame.f_lineno)
    return trace
sys.settrace(trace)
context = cache.canonical_context(api.geometry_producers())
print("eligible", context is not None)
if context:
    print("digest", cache.digest(cache.encode(context[2])))
    old = api.ARC_STEP
    api.ARC_STEP = old + 1
    print("mutated accepted", context[3](api.geometry_producers()))
    api.ARC_STEP = old
