"""🎬 Representative authored ventilation sections, rendered through Manim."""

import importlib.util
from pathlib import Path
from manim import *

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
TEST=ROOT/"🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🧪️architecture/🐍️.py"
spec=importlib.util.spec_from_file_location("vent_arch_tests",TEST)
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
visuals=module.VISUALS
OUTPUT=Path(__file__).resolve().parent/"🗑️generated"/"ventilation-preview"


class VentilationPreview(Scene):
    """🏛️ Closed solids, floor-anchored occupants, four house apertures and projected sashes."""

    def construct(self):
        self.camera.background_color=visuals.P_DEEP_DARK
        api=module.helpers("🪟️natural")
        room=api["_room"]([-3.4,1.15,0],4.8,2.5,window_side="both",window_y=1.45)
        person=api["_stand"]([-3.4,1.15,0],2.5,-3.0)
        windows=[visuals.window_glyph(x,0.875,2.025,depth=0.92,wall=0.16,inward=side)
                 for x,side in ((-5.8,1),(-1.0,-1))]
        for win in windows: self.add(win["group"])
        self.add(room,person)
        title=Text("Ventilation architectural sections",font_size=28,color=visuals.P_WHITE).to_edge(UP,buff=0.22)
        self.add(title)
        volume=module.helpers("📦️volume")
        house=volume["_pf_house"](volume["_house_section"](),[0.2,-1.2,0],0.4)
        self.add(house["group"])
        close=api["_vent_room"](-5.9,-2.6,0.7,"",visuals.P_CYAN)
        self.add(close["group"])
        self.play(*[visuals.open_window(w) for w in windows],
                  visuals.open_window(close["window"],fraction=0.5,tilt=True),
                  *[visuals.open_window(w) for w in house["windows"]],run_time=1.2)
        self.wait(0.8)


if __name__=="__main__":
    with tempconfig({"media_dir":str(OUTPUT),"pixel_width":1280,"pixel_height":720,"frame_rate":15,
                     "write_to_movie":True,"save_last_frame":False,"disable_caching":True,"verbosity":"WARNING"}):
        VentilationPreview().render()
