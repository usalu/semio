"""🏘️ Render current production house, plant and room sections through open/close states."""

from pathlib import Path
import hashlib
import importlib
import json
import os
import sys

WORKSPACE = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
TICKET = Path(__file__).resolve().parent
BASE = "🎓️teaching.🏛️architecture.⚡️energy"
SOURCES = [Path(__file__), WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py",
           WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py"]
SOURCES.append(SOURCES[1].with_name("🔣️.json"))
IDENTITY = hashlib.sha256(b"".join(path.read_bytes() for path in SOURCES)).hexdigest()[:12]
OUTPUT = TICKET / "🗑️generated" / "sections-preview" / IDENTITY
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)

from manim import Scene, VGroup, ORIGIN, LEFT, UP, tempconfig

VISUALS = importlib.import_module(BASE + ".🏗️authoring.🎨️visuals.🐍️")
HEATING = importlib.import_module(BASE + ".🔥️heating.🎬️clip.🧮️calculation.🐍️")


class SectionPreview(Scene):
    """🎞️ Capture each architectural state as both actual footage and a unique PNG."""

    def sequence(self, windows, tilts):
        self.wait(0.2)
        self.camera.get_image().save(OUTPUT / (type(self).__name__ + "-closed.png"))
        self.play(*(VISUALS.open_window(window, tilt=tilt) for window, tilt in zip(windows, tilts)), run_time=0.8)
        self.wait(0.2)
        self.camera.get_image().save(OUTPUT / (type(self).__name__ + "-open.png"))
        self.play(*(VISUALS.open_window(window, tilt=tilt, close=True) for window, tilt in zip(windows, tilts)), run_time=0.8)
        self.wait(0.2)
        self.camera.get_image().save(OUTPUT / (type(self).__name__ + "-closed-again.png"))


class UniformSectionOverview(SectionPreview):
    """🏠 Compare the actual plant helper with the shared two-storey section."""

    def construct(self):
        plant = HEATING._plant_house()
        keys = ("room", "cellar", "material", "windows", "radiators", "thermostat", "store", "boiler", "chimney", "pipes", "person")
        group = VGroup(*(plant[key] for key in keys)).scale(0.85).move_to(LEFT * 3.1 + UP * 0.1)
        self.add(group)
        house = VISUALS.house_section([3.3, 0.2, 0], 0.9)
        metre = house["w_height"] / 2 / 2.5
        person = VISUALS.person_glyph(ORIGIN).set(height=1.75 * metre)
        person.move_to([3.3, house["y_f"] + person.height / 2, 0])
        radiator = VISUALS.radiator(ORIGIN, height=0.65 * metre, width=1.3 * metre)
        radiator.move_to([house["x_l"] + 0.08 * metre + radiator.width / 2,
                          house["y_f"] + 0.12 * metre + radiator.height / 2, 0])
        self.add(house["group"], person, radiator)
        self.sequence(house["windows"], [False] * len(house["windows"]))


class UniformRoomOpenings(SectionPreview):
    """🪟 Inspect mirrored casement and tilt openings against solid room construction."""

    def construct(self):
        windows = []
        for cx, side, inward in ((-3.25, "left", 1), (3.25, "right", -1)):
            room = VISUALS.room_section([cx, 0, 0], w=4.5, h=2.5, window_side=side)
            room["glass"].set_opacity(0)
            if room["right_glass"] is not None:
                room["right_glass"].set_opacity(0)
            window = VISUALS.window_glyph(room["glass_x"] if side == "left" else room["right_glass_x"],
                                          room["win_lo"], room["win_hi"], wall=room["wall"], depth=0.9, inward=inward)
            person = VISUALS.person_glyph(ORIGIN).set(height=1.75)
            person.move_to([cx, room["y_f"] + person.height / 2, 0])
            self.add(room["group"], window["group"], person)
            windows.append(window)
        self.sequence(windows, [False, True])


with tempconfig({"pixel_width": 1600, "pixel_height": 900, "frame_rate": 10,
                 "write_to_movie": True, "save_last_frame": False, "media_dir": str(OUTPUT),
                 "progress_bar": "none", "verbosity": "ERROR", "background_color": VISUALS.P_DEEP_DARK}):
    for scene in (UniformSectionOverview, UniformRoomOpenings):
        scene().render()

(OUTPUT / "preview.json").write_text(json.dumps({"source_identity": IDENTITY, "output": str(OUTPUT)}, indent=2), encoding="utf-8")
print("[DEBUG] Uniform section previews: " + str(OUTPUT), flush=True)
