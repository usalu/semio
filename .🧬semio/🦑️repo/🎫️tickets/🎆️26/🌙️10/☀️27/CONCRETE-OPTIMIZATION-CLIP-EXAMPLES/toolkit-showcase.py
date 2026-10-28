"""🖼️ Render stills of every room widget at several designs for visual verification.

Usage: python toolkit-showcase.py <en|de> [board ...]
Output: 🗑️generated/toolkit-r2/<board>-<language>.png and showcase-<language>-layout.md
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/toolkit/fonts")
OUTPUT = TICKET / "🗑️generated/toolkit-r2"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
strict = importlib.import_module("layout_strict")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
report = []


class Showcase(visuals.ClipScene):
    """🖼️ One still per board."""

    TITLE = {"en": "Toolkit Showcase", "de": "Werkzeug-Schau"}
    NARRATION = [("a", "Caption bar line for layout checks.", "Beispiel-Untertitel für die Layoutprüfung.")]

    def save(self, name):
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
        conflicts = art.layout_conflicts(self)
        report.append(f"## {name}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n" + strict.strict_section(strict.strict_conflicts(self)))

    def board(self, name, *items):
        if only and name not in only:
            return
        self.add(*items)
        self.save(name)
        self.remove(*items)

    def construct(self):
        art.set_vo_language(language)
        fonts.apply_scene_style(self)
        self._title = fonts.section_heading(self.TITLE[language])
        self._caption = art.caption_bar(self.tr(self.NARRATION[0][1], self.NARRATION[0][2]))
        self.add(self._title, self._caption)
        designs = {
            "baseline": {},
            "east": {"orientation": 90, "wwr": .6, "overhang": 0},
            "glass": {"wwr": .8, "overhang": 1.2},
            "north": {"orientation": 360, "wwr": .3, "overhang": 0, "insulation": .3},
        }
        for key, design in designs.items():
            trackers = self.study_trackers(design)
            room_w = self.room_section(trackers, (-3.7, 0.3, 0), .42, "winter")
            room_s = self.room_section(trackers, (3.1, 0.3, 0), .42, "summer")
            self.board(f"section-{key}", room_w, room_s)
        for name, (left, right) in {"daylight-a": (({"wwr": .3, "overhang": 0}), ({"wwr": .8, "overhang": 0})),
                                    "daylight-b": (({"wwr": .3, "overhang": 1.2}), ({"wwr": .8, "overhang": 1.2}))}.items():
            self.board(name, self.room_section(self.study_trackers(left), (-3.7, 0.3, 0), .42, "winter", light_label=True),
                       self.room_section(self.study_trackers(right), (3.1, 0.3, 0), .42, "summer", light_label=True))
        trackers = self.study_trackers({"orientation": 180, "wwr": .45, "overhang": .6})
        for name, design in {"daylight-mid": {}, "daylight-deep": {"wwr": .8, "overhang": 1.2}, "daylight-small": {"wwr": .1, "overhang": 0.0}}.items():
            live = self.study_trackers(design)
            self.board(name, self.room_section(live, (-1.4, 0.15, 0), .5, "summer", light_label=True, demand=False, labels=False),
                       self.electricity_formula(live, (0, -2.05, 0), size=20, frame=art.P_PRIMARY))
        wide = self.study_trackers({"wwr": .76, "overhang": .6})
        self.board("formula", self.electricity_formula(wide, (0, 2.0, 0), size=20, frame=art.P_PRIMARY, digits=1),
                   self.derivation(wide, "heating", (-3.4, 0.55, 0), size=17, width=6.6), self.derivation(wide, "cooling", (3.4, 0.55, 0), size=17, width=6.6),
                   self.derivation(wide, "lighting", (-3.4, -1.1, 0), size=17, width=6.6), self.derivation(wide, "bill", (3.4, -1.1, 0), size=17, width=6.6))
        self.board("formula-daylight", self.derivation(self.study_trackers({"wwr": .3, "overhang": 0}), "daylight", (0, 1.4, 0), size=20), self.derivation(wide, "daylight", (0, 0.2, 0), size=20),
                   self.derivation(self.study_trackers({"wwr": .8, "overhang": 1.2}), "daylight", (0, -1.0, 0), size=20))
        self.board("sum-bars", self.energy_bars(wide, (-3.0, -2.1, 0), 3.6, 800), self.electricity_formula(wide, (3.2, 0.5, 0), size=18, align="center", width=6.2),
                   self.derivation(wide, "lighting", (3.2, -1.2, 0), size=15, words=False, width=6.2))
        self.board("section-big", self.room_section(trackers, (-1.4, 0.0, 0), .66, "summer"),
                   self.energy_bars(trackers, (4.3, -2.1, 0), 3.6, 800))
        self.board("sun-note", self.sun_note(trackers, "summer", (-4.5, 1.0, 0)), self.sun_note(trackers, "winter", (-4.5, 0.2, 0)),
                   self.sun_note(self.study_trackers({"orientation": 360}), "summer", (0.5, 1.0, 0)), self.sun_note(self.study_trackers({"orientation": 90}), "summer", (0.5, 0.2, 0)),
                   self.room_section(self.study_trackers({"orientation": 90, "wwr": .6, "overhang": 0}), (4.2, 0.3, 0), .4, "summer", labels=True, demand=False))
        self.board("section-diffuse", self.room_section(self.study_trackers({"wwr": .8, "overhang": .6}), (-3.7, 0.3, 0), .42, "summer", sky="diffuse", light_label=True),
                   self.room_section(self.study_trackers({"wwr": .3, "overhang": 0}), (3.1, 0.3, 0), .42, "winter", sky="diffuse"))
        self.board("losses", self.room_section(self.study_trackers({"wwr": .3}), (-4.0, 0.2, 0), .5, "winter", labels=False),
                   self.room_section(self.study_trackers({"wwr": .8}), (2.2, 0.2, 0), .5, "winter", labels=False))
        self.board("facade", self.facade(trackers, (-3.2, .2, 0), .6, "summer"), self.facade(self.study_trackers({"wwr": .8, "overhang": 1.2}), (3.2, .2, 0), .6, "summer"))
        east = self.study_trackers({"orientation": 90})
        self.board("plan", self.plan(trackers, (-4.2, 0, 0), .26), self.plan(east, (0, 0, 0), .26, arcs=True),
                   self.plan(self.study_trackers({"orientation": 315}), (4.4, 0, 0), .2, arcs=True, sector=(90, 270, art.P_ORANGE), site=(180, 360), ghost=90, dimensions=True))
        self.board("bars", self.energy_bars(trackers, (-5.2, -2.1, 0), 3.6, 800), self.energy_bars(east, (0.3, -2.1, 0), 3.6, 800, reference={"orientation": 180}),
                   self.energy_bars(self.study_trackers({"wwr": .8, "overhang": 1.2}), (5.6, -2.1, 0), 3.6, 800, compact=True))
        spot = self.ramp()
        row = [self.thumbnail(d, (-5.4 + k * 1.0, 1.6, 0), .12, state="filled") for k, d in enumerate(
            [{"wwr": .1, "overhang": 0}, {"wwr": .3, "overhang": .3}, {"wwr": .45, "overhang": .6}, {"wwr": .6, "overhang": .9},
             {"wwr": .8, "overhang": 1.2}, {"wwr": .8, "overhang": 0}, {"wwr": .1, "overhang": 1.2}, {"wwr": .45, "overhang": 0}, {"wwr": .45, "overhang": 1.2},
             {"wwr": .3, "overhang": 0, "insulation": .3}, {"wwr": .3, "overhang": 0, "insulation": .1}])]
        ghosts = [self.thumbnail(d, (-5.4 + k * 1.0, 0.2, 0), .12, state="ghost") for k, d in enumerate([{"wwr": .1, "overhang": 0}, {"wwr": .45, "overhang": .6}, {"wwr": .8, "overhang": 1.2}])]
        big = [self.thumbnail({"wwr": w, "overhang": o}, (-4.5 + 3 * k, -1.2, 0), .5, state="filled", ring=art.P_GREEN) for k, (w, o) in enumerate([(.1, 0), (.45, .6), (.8, 1.2)])]
        self.board("thumbnails", *row, *ghosts, *big, self.ramp_strip(spot, (4.0, 1.6, 0), 3.0))
        axes, labels = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label=self.tr("Window-to-wall ratio WWR in %", "Fensteranteil WWR in %"),
                                     y_label=self.tr("Overhang d in m", "Vordachtiefe d in m"), point=(-3.2, .1, 0), width=6.2, height=3.4, y_digits=1)
        heat = self.heat_map(axes, colour=spot, levels=4, legend=True, legend_title=self.tr("Tile colour: electricity", "Kachelfarbe: Strom"), ticks=[600, 700, 800], avoid=[axes.c2p(45, .6), axes.c2p(45, .6) + manim.UP * .6])
        marker = self.live_marker(axes, trackers, lambda d: d["wwr"] * 100, lambda d: d["overhang"], x_label="WWR", x_unit="%", y_label="d", y_unit="m", y_digits=1)
        self.board("heatmap", axes, labels, heat["image"], heat["contours"], heat["star"], heat["legend"], marker,
                   self.room_section(trackers, (4.4, .9, 0), .42, "summer"), self.energy_bars(trackers, (4.4, -2.1, 0), 1.5, 800, labels=False, compact=True))
        small = self.study_trackers({"orientation": 90, "wwr": .6, "overhang": .3})
        self.board("scales", self.room_section(small, (-4.5, 1.4, 0), .25, "summer"), self.room_section(small, (-0.8, 1.4, 0), .35, "winter"),
                   self.room_section(trackers, (4.6, 1.2, 0), .7, "summer", labels=False),
                   self.facade(trackers, (-5.4, -1.2, 0), .2), self.facade(trackers, (-3.0, -1.2, 0), .25), self.facade(trackers, (-0.2, -1.2, 0), .3), self.facade(trackers, (3.2, -1.2, 0), .4))
        left_axes, left_labels = self.plot_axes(x_range=(-.3, 1.3, .2), y_range=(0, 600, 200), x_label=self.tr("Overhang in m", "Vordach in m"), y_label="kWh/a", point=(-3.6, 0, 0), width=4.6, height=2.8, x_digits=1)
        corner_axes, corner_labels = self.plot_axes(x_range=(-.3, 1.3, .2), y_range=(0, 600, 200), x_label=self.tr("Overhang in m", "Vordach in m"), y_label="kWh/a", point=(2.6, 0, 0), width=4.6, height=2.8, x_digits=1, corner=True)
        self.board("axes-corner", left_axes, left_labels, corner_axes, corner_labels,
                   manim.Dot(left_axes.c2p(.4, 300), color=art.P_YELLOW), manim.Dot(corner_axes.c2p(.4, 300), color=art.P_YELLOW))
        bare, bare_labels = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label=None, y_label=None, point=(-3.0, 0, 0), width=5.6, height=3.0, y_digits=1)
        flat = self.heat_map(bare, colour=spot, levels=3, columns=False, legend=True)
        self.board("axes-flat-map", bare, bare_labels, flat["image"], flat["contours"], flat["star"], flat["legend"])
        study = model.orientation_study()
        trackers = self.study_trackers({**study["design"], "orientation": 180})
        samples = study["samples"]
        low, high = min(v for _, v in samples), max(v for _, v in samples)
        study_axes, study_labels = self.plot_axes(x_range=(90, 450, 90), y_range=(round(low - 50, -2), round(high + 50, -2), 100), x_label="θ in °", y_label="kWh/a",
                                                  point=(-3.4, .1, 0), width=5.6, height=3.4)
        curve = manim.VMobject(color=art.P_CYAN, stroke_width=3)
        curve.set_points_smoothly([study_axes.c2p(a, v) for a, v in samples[::6]])
        mark = self.live_marker(study_axes, trackers, "orientation", "electricity", x_label="θ", x_unit="°", y_unit="kWh/a", avoid=[curve])
        widgets = [mark, self.plan(trackers, (2.4, 1.1, 0), .17, arcs=True), self.room_section(trackers, (3.2, -1.3, 0), .36, "summer"),
                   self.energy_bars(trackers, (6.0, -2.0, 0), 3.2, 1.2 * high, labels=False, compact=True)]
        self.add(study_axes, study_labels, curve)
        self.play(*[manim.FadeIn(widget) for widget in widgets], run_time=.5)
        if not only or "orientation" in only:
            self.save("orientation")
        self.play(self.set_design(trackers, {"orientation": 360}, 1))
        if not only or "orientation-moved" in only:
            self.save("orientation-moved")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Showcase().render()
(OUTPUT / f"showcase-{language}-layout.md").write_text(f"# Layout showcase {language}\n\n" + "\n".join(report), encoding="utf-8")
print("[DEBUG] showcase done", language, len(report), "boards", flush=True)
