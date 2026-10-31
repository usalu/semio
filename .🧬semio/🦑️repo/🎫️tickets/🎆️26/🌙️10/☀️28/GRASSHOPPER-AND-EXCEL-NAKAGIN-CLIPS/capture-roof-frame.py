"""🏭 Capture closed machinery heads from both formerly leaking camera directions."""
import importlib
from pathlib import Path

V=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
V._engine.config.pixel_width,V._engine.config.pixel_height=1920,1080
V._engine.config.frame_width,V._engine.config.frame_height=16,9
scene,drafting=V.NakaginDrawing(),V._d.Drafting()
objects=[]
for x,(tower,angle) in zip((-5.7,-1.9,1.9,5.7),((0,135),(0,225),(1,135),(1,225))):
    objects.append(scene.capital_axon(tower,at=(x,0,0),scale=.39,view=(angle,24)))
    objects.append(drafting.text(f"{V._model.towers()[tower].key} / {angle}°",(x,3.6,0),size=22,color=V.INK))
camera=V._engine.Camera(background_color=V.BACKGROUND)
camera.capture_mobjects(objects)
output=Path(__file__).parent/"🗑️generated/roof-frame-occlusion.png"
camera.get_image().save(output)
print("[DEBUG] saved "+str(output),flush=True)
