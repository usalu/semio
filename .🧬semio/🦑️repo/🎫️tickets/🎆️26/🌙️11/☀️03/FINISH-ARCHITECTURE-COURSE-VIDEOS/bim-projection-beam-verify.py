"""🔬 Replay the complete actual beam through the exact visual pipeline and Cairo."""
import importlib,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
p="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
model=importlib.import_module(p+"🧱️model.🐍️")
prims=importlib.import_module(p+"🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
axon=importlib.import_module(p+"🎨️visuals.🏛️savoye.🧊️axon.🐍️")
from manim import Camera
m=model.load_model();start=time.monotonic()
item=model.solid_geometry("BEAM-PRIMARY-F-7-24",m)
info=prims._item(item["tag"],item["guid"],"beams",item["class"],m,tuple(item["domains"]),(item["kind"],))
pieces=prims._surface_prims(item,info,3.17,6.6,-.15)
view=axon.AxonView((0,1.45,0),.13,(10,10,3),0)
painter=axon._Painter(view,axon._Selection((),(),.62),True)
visible=sum(sum(face[2] is None or view.facing(face[2])>1e-6 for face in part.faces) for part in pieces)
print("[DEBUG] actual beam pieces",len(pieces),"visible facets",visible,flush=True)
drawings=[painter.render(part,"structure",0) for part in pieces]
camera=Camera(pixel_width=480,pixel_height=270);camera.capture_mobjects(drawings)
print("[DEBUG] full beam painter/Cairo passed",round(time.monotonic()-start,3),"s",camera.pixel_array.shape,flush=True)
