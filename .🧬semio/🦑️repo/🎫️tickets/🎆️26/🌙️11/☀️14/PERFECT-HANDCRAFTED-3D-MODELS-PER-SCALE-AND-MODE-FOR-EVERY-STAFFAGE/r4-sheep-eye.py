"""👁️ Gives the far eye of animals/sheep at 1:50 its elevation twin (hidden behind the head): without it the front and plan shapes of `eye-f` have no depth to be carried at, collapse to a line in the lying pose and make the whole 1:50 drawing unprintable. Idempotent.

Usage: python3 r4-sheep-eye.py
"""
import copy
import importlib.util

spec = importlib.util.spec_from_file_location("io", "r3-sheep-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
reps = io.load()["representations"]
shapes = reps["50"]["geometry"]["elevation"]
if all(s["id"] != "eye-f" for s in shapes):
    near = next(s for s in shapes if s["id"] == "eye")
    far = copy.deepcopy(near)
    far["id"] = "eye-f"
    far["z"] = 0.45
    shapes.insert(next(i for i, s in enumerate(shapes) if s["id"] == "ear-f") + 1, far)
    io.put({"representations": reps})
    print("eye-f added to the 1:50 elevation")
else:
    print("eye-f already drawn")
