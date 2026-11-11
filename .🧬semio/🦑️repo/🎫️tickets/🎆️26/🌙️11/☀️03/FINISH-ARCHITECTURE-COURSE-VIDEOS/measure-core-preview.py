"""⏱️ Measure a continuous native core yaw and translation through Cairo."""
import importlib
import os
import sys
import time
from pathlib import Path
import numpy as np
from manim import tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"frame_rate":15}):
    scene=m.Beat2_Capsule()
    core=m._nkd.NakaginDrawing3D(m._nkd.make_view((55,24),np.array([0,0,0]),0.25,(0,0,0)))
    capsule=m._nk.capsule_type("A")
    times=[]
    for i in range(45):
        start=time.perf_counter()
        drawing=scene._core_preview(core,(i/44*3,0,i/44*4),-270*i/44,capsule)
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        times.append(time.perf_counter()-start)
    print({"frames":len(times),"seconds":sum(times),"maximum_frame_seconds":max(times)},flush=True)
