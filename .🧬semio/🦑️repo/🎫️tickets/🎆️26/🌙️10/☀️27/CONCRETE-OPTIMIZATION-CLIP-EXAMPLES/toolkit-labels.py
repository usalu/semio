"""🏷️ Stills of the room_section demand label, dimension labels and plot_axes tick labels.

Usage: python toolkit-labels.py <en|de> [board ...]
Output: 🗑️generated/toolkit-labels/<board>-<language>.png, labels-<language>-layout.md and labels-<language>-extents.md
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/toolkit-labels/fonts")
OUTPUT = TICKET / "🗑️generated/toolkit-labels"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
report, extents = [], []


class Labels(visuals.ClipScene):
    """🖼️ One still per board."""

    TITLE = {"en": "Label Check", "de": "Beschriftungsprüfung"}
    NARRATION = [("a", "Caption bar line for layout checks.", "Beispiel-Untertitel für die Layoutprüfung.")]

    def save(self, name):
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
        conflicts = art.layout_conflicts(self)
        report.append(f"## {name}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")

    def board(self, name, *items):
        if only and name not in only:
            return
        self.add(*items)
        self.save(name)
        self.remove(*items)

    def construct(self):
        art.set_vo_language(language)
        fonts.apply_scene_style(self)
        self._title = fonts.scene_title(self.TITLE[language])
        self._caption = art.caption_bar(self.tr(self.NARRATION[0][1], self.NARRATION[0][2]))
        self.add(self._title, self._caption)
        for scale in (0.3, 0.45, 0.6):
            for season in ("winter", "summer"):
                trackers = self.study_trackers({"wwr": .6, "overhang": .6})
                point = (-3.3, 0.25, 0)
                room = self.room_section(trackers, point, scale, season)
                bars = self.energy_bars(trackers, (4.9, -2.0, 0), 3.4, maximum=1200)
                if not only or f"label-{scale}-{season}" in only:
                    box = room.get_critical_point(manim.LEFT)[0], room.get_critical_point(manim.RIGHT)[0], room.get_critical_point(manim.DOWN)[1], room.get_critical_point(manim.UP)[1]
                    extents.append(f"- scale {scale} {season}: x {box[0] - point[0]:.2f} … {box[1] - point[0]:.2f}, y {box[2] - point[1]:.2f} … {box[3] - point[1]:.2f} (relative to point, screen units)")
                self.board(f"label-{scale}-{season}", room, bars)
        for design in ({"orientation": 90, "wwr": .8, "overhang": 0}, {"wwr": .2, "overhang": 1.2}):
            trackers = self.study_trackers(design)
            self.board(f"label-extreme-{design['wwr']}", self.room_section(trackers, (-3.3, 0.25, 0), .45, "winter"), self.room_section(trackers, (3.3, 0.25, 0), .45, "summer", labels=False),
                       self.energy_bars(trackers, (0.0, -2.2, 0), 3.0, maximum=1200, compact=True))
        axes, titles = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label=self.tr("Window share, %", "Fensteranteil, %"), y_label=self.tr("Overhang, m", "Vordach, m"),
                                      point=(-3.65, -.1, 0), width=5.0, height=3.4, y_digits=1)
        plain, bare = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), point=(3.2, -.1, 0), width=5.0, height=3.4, y_digits=1, titles=False)
        self.board("axes", axes, titles, plain, bare)
        self.board("axes-left", *self.plot_axes(x_range=(0, 50, 10), y_range=(0, 900, 300), x_label="x", y_label="y", point=(-3.2, -.1, 0), width=6.4, height=3.4))
        (OUTPUT / f"labels-{language}-layout.md").write_text(f"# Layout labels {language}\n\n" + "\n".join(report), encoding="utf-8")
        (OUTPUT / f"labels-{language}-extents.md").write_text("\n".join(extents) + "\n", encoding="utf-8")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Labels().render()
print("[DEBUG] labels", language, len(report), "boards", flush=True)
