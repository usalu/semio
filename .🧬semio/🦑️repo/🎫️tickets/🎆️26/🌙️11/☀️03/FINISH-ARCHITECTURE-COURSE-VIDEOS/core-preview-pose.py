"""🧭 Inspect physical moving capsule at several core-plane poses."""
import importlib
import os
import sys
from pathlib import Path
import numpy as np
from manim import tempconfig
from PIL import Image
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
with tempconfig({"dry_run":True,"pixel_width":480,"pixel_height":270,"frame_rate":15}):
    scene=m.Beat2_Capsule()
    core=m._nkd.NakaginDrawing3D(m._nkd.make_view((55.0,24.0),np.array([0,0,0]),0.25,(0,0,0)))
    capsule=m._nk.capsule_type("A")
    casing=m._nk.capsule_casing(capsule)
    images=[]
    for yaw in (0,-45,-90,-135,-180,-270):
        drawing=scene._core_preview(core,(0,0,0),yaw,capsule)
        np.testing.assert_allclose(drawing.at(0,(casing.back+casing.front)/2,casing.height/2),core.at(0,0,0),atol=1e-9)
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        images.append(scene.camera.get_image().convert("RGB"))
    contact=Image.new("RGB",(480*3,270*2),"white")
    for i,image in enumerate(images): contact.paste(image,((i%3)*480,(i//3)*270))
    contact.save(Path(__file__).parent/"🗑️generated"/"core-preview-pose.png")
print("six core-plane poses and projected centers passed",flush=True)
