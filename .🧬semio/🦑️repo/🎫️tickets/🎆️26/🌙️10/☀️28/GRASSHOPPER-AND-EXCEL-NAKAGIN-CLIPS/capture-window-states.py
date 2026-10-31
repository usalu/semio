"""🪭 Capture the actual isolated window mechanism; opened poses do not assert furnished-room clearance."""
import importlib
import json
from pathlib import Path
import sys

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
M,E=V._model,V._engine
OUTPUT=Path(__file__).resolve().parent/"🗑️generated"
OUTPUT.mkdir(exist_ok=True)
CASES=json.loads((ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🪟️unit/🔣️.json").read_text(encoding="utf8"))
E.config.pixel_width,E.config.pixel_height=1920,1080
E.config.frame_width,E.config.frame_height=16,9
drafting=V._d.Drafting()
scene=V.NakaginDrawing()
objects=[]
columns=(len(CASES["states"])+1)//2
for index,state in enumerate(CASES["states"]):
    first=index<columns
    count=columns if first else len(CASES["states"])-columns
    local=index if first else index-columns
    at=((local-(count-1)/2)*3.9,2.1 if first else -2.1,0)
    drawing=scene.window_unit_axon(M.capsule_type("A"),layout="Slash",state=M.WindowUnitState(**state),at=at,scale=1.7,view=(30,18),glass_opacity=.25)
    assert set(drawing.semantic_parts)=={p.key for p in drawing.source_assembly.parts}
    label=str(round(state["blind_fraction"]*100))+"% / "+str(state["sash_angle"])+"°"
    objects.extend((drawing,drafting.text(label,(at[0],at[1]-1.7,0),size=22,color=V.INK)))
    print("[DEBUG] isolated window "+label,flush=True)
camera=E.Camera(background_color=V.BACKGROUND)
camera.capture_mobjects(objects)
camera.get_image().save(OUTPUT/"nakagin-window-mechanism-states.png")
print("[DEBUG] saved isolated window mechanism states",flush=True)
