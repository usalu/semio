"""Measure the real four-capsule anatomy strip and snapshot."""
import importlib,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import psutil
import numpy as np
from manim import Camera,VGroup
nk=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
vis=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
scene=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
snap=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
cam=Camera(pixel_width=854,pixel_height=480,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
drawer=vis.NakaginDrawing()
proc=psutil.Process()
t=time.perf_counter()
items=[drawer.capsule_axon(k,at=scene._at(-3+2*i,.2),scale=.2,base=scene._canvas.PREVIEW_FILL,window=True) for i,k in enumerate(nk.capsule_types())]
group=VGroup(*items)
print("build_seconds",round(time.perf_counter()-t,3),"facets",len(group.get_family()),"rss_gib",round(proc.memory_info().rss/1024**3,3),flush=True)
t=time.perf_counter();cam.capture_mobjects([group]);reference=np.asarray(cam.get_image(),dtype=int).copy()
print("vector_frame_seconds",round(time.perf_counter()-t,3),flush=True)
t=time.perf_counter();image=snap.capture(group,cam)
print("capture_seconds",round(time.perf_counter()-t,3),"rss_gib",round(proc.memory_info().rss/1024**3,3),flush=True)
t=time.perf_counter();cam.reset();cam.capture_mobjects([image])
print("snapshot_frame_seconds",round(time.perf_counter()-t,3),"max_channel_delta",int(np.abs(reference-np.asarray(cam.get_image(),dtype=int)).max()),"image_bytes",image.pixel_array.nbytes,flush=True)
