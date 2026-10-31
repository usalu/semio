"""🧩 Capture every physical capsule family from the shared public drawing interface."""
import importlib
from pathlib import Path
import sys

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
M,E=V._model,V._engine
OUTPUT=Path(__file__).resolve().parent/"🗑️generated"
OUTPUT.mkdir(exist_ok=True)
E.config.pixel_width,E.config.pixel_height=1920,1080
E.config.frame_width,E.config.frame_height=16,9
scene,drafting=V.NakaginDrawing(),V._d.Drafting()

for cutaway in (False,True):
    objects=[]
    for index,layout in enumerate(M.capsule_families()):
        x=(-5.2,0,5.2)[index%3]
        y=2.1 if index<3 else -2.1
        azimuth=55 if M._family(layout)["hand"]=="left" else 125
        drawing=scene.capsule_axon(M.capsule_type("A"),layout=layout,at=(x,y,0),scale=.72,view=(azimuth,38),cutaway=cutaway,interior="reference",bolts=True,glass_opacity=.25)
        assert {"window-glazing","window-rim","entry-door"}<=set(drawing.semantic_parts)
        assert {f"bolt-{index}-head" for index in range(4)}<=set(drawing.semantic_parts)
        if cutaway:
            assert {"fixture-mount-tv","fixture-mount-tap-tub-mixer"}<=set(drawing.semantic_parts)
        label=drafting.text(layout,(x,y,0),size=22,color=V.INK)
        label.next_to(drawing,E.DOWN,buff=.16)
        assert label.get_top()[1]<=drawing.get_bottom()[1]-.15
        assert label.get_bottom()[1]>=-E.config.frame_height/2+.05
        objects.extend((drawing,label))
        print("[DEBUG] family "+layout+" cutaway="+str(cutaway),flush=True)
    camera=E.Camera(background_color=V.BACKGROUND)
    camera.capture_mobjects(objects)
    name="nakagin-six-families-"+("cutaway" if cutaway else "closed")+".png"
    camera.get_image().save(OUTPUT/name)
    print("[DEBUG] saved "+name,flush=True)
