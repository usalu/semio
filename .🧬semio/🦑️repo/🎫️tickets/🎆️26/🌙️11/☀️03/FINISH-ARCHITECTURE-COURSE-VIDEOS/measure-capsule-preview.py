"""⏱️ Measure the continuous 45-frame authored width ramp with Cairo."""
import importlib
import os
import sys
import time
from pathlib import Path
import numpy as np
from manim import tempconfig

sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
module=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
p={"width":2.6,"length":4.2,"height":2.4,"back":-0.1,"front":4.1,"corner_radius":0.2,"window_diameter":1.2,"window_u":0.0,"window_v":1.2,"glazing_y":4.0}
rv={"round":0.0,"window":0.0,"lift":1.0}
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"frame_rate":15}):
    scene=module.Beat2_Capsule()
    view=(55.0,24.0)
    v=module._nkd.make_view(view,np.array([0.0,0.0,0.0]),module.SCALE,(0,0,0))
    times=[]
    for width in np.r_[np.linspace(2.6,3.1,23),np.linspace(3.1,2.6,22)]:
        p["width"]=float(width)
        start=time.perf_counter()
        drawing=scene._preview(p,rv,view,v,("width",))
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        times.append(time.perf_counter()-start)
        assert drawing.dims["width"].measurement_value==float(width)
    print({"frames":len(times),"seconds":sum(times),"maximum_frame_seconds":max(times),"mean_frame_seconds":sum(times)/len(times)},flush=True)

    for phase in ("rounded","window","camera"):
        times=[]
        for i in range(15):
            rv["round"]=float((i+1)/15) if phase=="rounded" else 1.0
            rv["window"]=1.0 if phase in ("window","camera") else 0.0
            p["window_diameter"]=1.0+0.4*i/14 if phase=="window" else 1.2
            view=(-55.0+110.0*i/14,24.0) if phase=="camera" else (55.0,24.0)
            v=module._nkd.make_view(view,np.array([0.0,0.0,0.0]),module.SCALE,(0,0,0))
            start=time.perf_counter()
            drawing=scene._preview(p,rv,view,v,())
            scene.camera.reset()
            scene.camera.capture_mobjects([drawing])
            times.append(time.perf_counter()-start)
        print({"phase":phase,"frames":len(times),"seconds":sum(times),"maximum_frame_seconds":max(times)},flush=True)
