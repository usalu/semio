"""⏱️ Measure current detailed core-stage capsule projection and Cairo paint."""
import importlib
import os
import sys
import time
from pathlib import Path
from manim import tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"frame_rate":15}):
    scene=m.Beat2_Capsule()
    for angle in (55.0,35.0):
        start=time.perf_counter()
        drawing=scene.capsule_axon(m._nk.capsule_type("A"),at=(0,0,0),scale=m.SCALE_CORE,view=(angle,m.ELEVATION),base=m._canvas.PREVIEW_FILL)
        build=time.perf_counter()-start
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        print({"azimuth":angle,"build":round(build,3),"total":round(time.perf_counter()-start,3),"family":len(drawing.get_family())},flush=True)
