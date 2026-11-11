"""Compare in-place lighting with a fresh one-slot tower render."""
import importlib,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from manim import Camera
nk=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
v=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
slot=next(s for s in nk.slots() if s.tower==0)
key=(slot.building,slot.tower,slot.storey,slot.facade,slot.item)
where=lambda s:(s.building,s.tower,s.storey,s.facade,s.item)==key
d=v.NakaginDrawing()
args=dict(where=where,towers=(0,),core=False,capital=False,podium=False,ground=False,segments=1,scale=.3)
t=time.perf_counter()
dim=d.tower_axon(lit=lambda k:False,**args)
fresh=d.tower_axon(lit=lambda k:True,**args)
col,on=v._slot_look(slot,lit=lambda k:True,tint=None,types=False,base=v.CAPSULE)
v._recolour(dim.capsules[key],col,on)
camera=Camera(pixel_width=320,pixel_height=180,frame_width=14.222222222,frame_height=8,background_opacity=1)
camera.capture_mobjects([fresh]); expected=np.asarray(camera.get_image(),dtype=int).copy()
camera.reset();camera.capture_mobjects([dim]); actual=np.asarray(camera.get_image(),dtype=int)
print("key",key,"seconds",round(time.perf_counter()-t,2),"max_delta",int(np.abs(expected-actual).max()),"changed",int(np.count_nonzero(np.any(expected!=actual,axis=2))),flush=True)
