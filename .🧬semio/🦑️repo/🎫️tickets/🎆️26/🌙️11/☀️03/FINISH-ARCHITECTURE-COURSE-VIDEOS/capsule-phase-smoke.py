"""🧪 Render representative actual capsule construction stages through Cairo."""
import importlib
import os
import sys
import time
from pathlib import Path
from types import SimpleNamespace
from manim import ValueTracker, tempconfig
from PIL import Image

sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
names=("grid","env","hit","body","round","window","dw","dl","dh","dd","du","bolts","axes","plane2","lift","wrong","front","dr","over")
phases=[
    ("center",{"body":1,"grid":1,"dw":1,"dl":1,"dh":1}),
    ("lift",{"body":1,"grid":1,"dw":1,"dl":1,"dh":1,"lift":1,"plane2":1}),
    ("rounded",{"body":1,"grid":1,"lift":1,"round":1,"dr":1}),
    ("front",{"body":1,"grid":1,"lift":1,"round":1,"front":1}),
    ("window",{"body":1,"grid":1,"lift":1,"round":1,"window":1,"dd":1}),
    ("bolts",{"body":1,"grid":1,"lift":1,"round":1,"window":1,"bolts":1}),
    ("limit",{"body":1,"grid":1,"lift":1,"round":1,"window":1,"env":1,"over":1,"dw":1}),
]
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"frame_rate":15}):
    scene=m.Beat2_Capsule()
    scene._d=m._gh.canvas("capsule")
    scene._cache={}
    vals={key:ValueTracker(scene._d.obj(key).settings["value"]) for key in m.IDS+m.BOLT_IDS}
    scene.st=SimpleNamespace(vals=vals,az=ValueTracker(55),oy=ValueTracker(0),rv={key:ValueTracker(0) for key in names})
    images=[]
    for name,values in phases:
        for key,tracker in scene.st.rv.items():
            tracker.set_value(values.get(key,0))
        scene.st.az.set_value(-55 if name=="bolts" else 55)
        start=time.perf_counter()
        drawing=scene._stage()
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        images.append(scene.camera.get_image().convert("RGB").resize((427,240)))
        print(name,round(time.perf_counter()-start,3),len(drawing.get_family()),flush=True)
    contact=Image.new("RGB",(427*4,240*2),"white")
    for i,image in enumerate(images):
        contact.paste(image,((i%4)*427,(i//4)*240))
    contact.save(Path(__file__).parent/"🗑️generated"/"capsule-phase-smoke.png")
