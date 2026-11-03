"""🧍 M7: still frame of the staffage call sites of excel program, bim consistency, optimization extrema and experiments, straight from the clip code."""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(HERE / "🗑️generated/fonts/r3")
import manim

PRE = "🎓️teaching.🏛️architecture."

info = importlib.import_module(PRE + "🏗️bim.🎬️clip.📊️information.🐍️")
index = importlib.import_module(PRE + "🎯️optimization.🎬️clip.⚖️index.🐍️")
room = importlib.import_module(PRE + "🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
program = importlib.import_module(PRE + "📊️excel.🎬️clip.📋️program.🐍️")
consistency = importlib.import_module(PRE + "🏗️bim.🎬️clip.🔗️consistency.🐍️")
extrema = importlib.import_module(PRE + "🎯️optimization.🎬️clip.📉️extrema.🐍️")
art = importlib.import_module(PRE + "⚡️energy.🏗️authoring.🎨️visuals.🐍️")


import json
_ids = [k for k in json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"] if k.startswith(("bim-information", "bim-consistency", "excel-program", "optimization-"))]
art.ensure_casts(_ids)


class Sheet(manim.Scene):
    def construct(self):
        self.camera.background_color = "#101418"
        prog = program.Beat3_Program()
        prog.lang = "en"
        prog.types = {k.key: k for k in program._nk.capsule_types()}
        t = prog.types["A"]
        for at, scale in (((-4.6, 0.6, 0), 0.82), ((-2.0, 0.6, 0), 0.42)):
            plan = prog.plan(t, at, scale, label=False, people=True)
            self.add(plan, plan.people)
        cons = consistency.Beat6_Consistency()
        for i, (x, size, cast) in enumerate([(0.6, 0.4, "bim-consistency-reviewer"), (1.6, 0.42, "bim-consistency-reviewer"), (2.6, 0.6, "bim-consistency-decider")]):
            self.add(cons.person((x, 1.0, 0), "#7CD992", size, cast))
        ex = extrema.Beat1_LocalGlobalExtrema()
        for i, icon in enumerate(ex._icons()):
            self.add(icon.move_to((-4.5 + 1.2 * i, -2.0, 0)))
        probe = info.Beat1_DataToInformation()
        probe._c = {"car_length": 4.5, "car_width": 1.75}
        for i, heading in enumerate((0, 270, 315)):
            self.add(probe._car(0.35, heading).move_to((-4.5 + 1.8 * i, -3.2, 0)))
        for i, c in enumerate(("electricity", "cost", "daylight", "carbon")):
            self.add(index.glyph(c, (2.0 + 0.8 * i, 3.2, 0), 0.5))
        self.add(room._cooler((5.0, 1.5, 0), 1.0, art.P_BLUE))
        for k, b in enumerate((0.0, 0.5, 1.0)):
            lamp = room.pendant((3.5 + 1.2 * k, 3.0, 0), 1.0)
            self.add(*room.pendant_lit(lamp, b))
        self.add(art.staffage("optimization-experiments-occupant", unit=.55 / 1.75, anchor=(0.9, -1.35 - .275, 0), color=art.P_ORANGE),
                 art.staffage("optimization-experiments-window", unit=.7 / 1.25, anchor=(2.3, -1.35 - .35, 0), color="#FFFFFF"))


out = HERE / "🗑️generated" / "r3"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Sheet().render()
print("[DEBUG] done", list(out.rglob("*.png")))
